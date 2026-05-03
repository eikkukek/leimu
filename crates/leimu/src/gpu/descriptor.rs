use core::{
    ptr::NonNull,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    fmt::{self, Display},
};

use ahash::{AHashMap, AHashSet};

use leimu_proc::BuildStructure;
use leimu_mem::{
    alloc::{Layout, LocalAlloc, dealloc},
    arena::{self, Arena},
    slot_map::*,
    vec::{FixedVec32, NonNullVec32, Vec32},
};
use tuhka::vk;

use crate::{
    core::{slice, TryExtend},
    gpu::prelude::*,
    error::*,
    sync::*,
    macros::impl_id_display,
};

macro_rules! match_display {
    (match $name:ident {$(
        $var:ident
            $({ $($field:ident $($ft1:tt $ft2:tt)?),* $(,)? })?
            $(($($tuple:tt),* $(,)?))?
        => $fmt:literal
    ),+ $(,)?}) => {
        impl ::core::fmt::Display for $name {

            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {$(
                    Self::$var $({ $($field $($ft1 $ft2)?),* })? $(($($tuple),*))? => write!(f, $fmt)
                ),+}
            }
        }
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DepthStencilAttachmentType {
    Depth,
    Stencil,
    DepthStencil,
}

match_display! {
    match DepthStencilAttachmentType {
        Depth => "depth",
        Stencil => "stencil",
        DepthStencil => "depth stencil",
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AttachmentImageLayout {
    Color,
    DepthStencil(DepthStencilAttachmentType),
    RenderingLocalRead { is_color: bool, },
}

match_display! {
    match AttachmentImageLayout {
        Color => "color",
        DepthStencil(ty) => "{ty} attachment",
        RenderingLocalRead {
            is_color: _,
        } => "rendering local read",
    }
}

impl AttachmentImageLayout {

    #[must_use]
    #[inline(always)]
    pub fn combine(self, other: Self) -> Self {
        if self != other {
            AttachmentImageLayout::RenderingLocalRead {
                is_color:
                    matches!(self, AttachmentImageLayout::Color) ||
                    matches!(other, AttachmentImageLayout::Color)
            }
        } else {
            self
        }
    }

    pub fn access_mask(self) -> vk::AccessFlags2 {
        match self {
            Self::Color =>
                vk::AccessFlags2::COLOR_ATTACHMENT_READ |
                vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
            Self::DepthStencil(_) =>
                vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ |
                vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
            Self::RenderingLocalRead { is_color } => {
                let mut mask = vk::AccessFlags2::INPUT_ATTACHMENT_READ;
                if is_color {
                    mask |=
                        vk::AccessFlags2::COLOR_ATTACHMENT_READ |
                        vk::AccessFlags2::COLOR_ATTACHMENT_WRITE;
                } else {
                    mask |=
                        vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ |
                        vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE;
                }
                mask
            }
        }
    }
}

impl From<AttachmentImageLayout> for vk::ImageLayout {

    #[inline(always)]
    fn from(value: AttachmentImageLayout) -> Self {
        match value {
            AttachmentImageLayout::Color => vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            AttachmentImageLayout::DepthStencil(ty) => match ty {
                DepthStencilAttachmentType::Depth => vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL,
                DepthStencilAttachmentType::Stencil => vk::ImageLayout::STENCIL_ATTACHMENT_OPTIMAL,
                DepthStencilAttachmentType::DepthStencil => vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            }
            AttachmentImageLayout::RenderingLocalRead { is_color: _ } => vk::ImageLayout::GENERAL,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ShaderImageLayout {
    General(vk::AccessFlags2),
    SampledReadOnly,
    Attachment(AttachmentImageLayout),
}

match_display! {
    match ShaderImageLayout {
        General(_) => "general",
        SampledReadOnly => "sampled read only",
        Attachment(layout) => "{layout}",
    }
}

impl ShaderImageLayout {

    #[must_use]
    #[inline(always)]
    pub fn combine(self, other: Self) -> Self {
        if self != other {
            if let Self::Attachment(a) = self &&
                let Self::Attachment(b) = other
            {
                Self::Attachment(a.combine(b))
            } else {
                ShaderImageLayout::General(self.access_mask() | other.access_mask())
            }
        } else {
            self
        }
    }

    #[inline(always)]
    pub fn access_mask(self) -> vk::AccessFlags2 {
        match self {
            Self::General(access) => access,
            Self::SampledReadOnly => vk::AccessFlags2::SHADER_SAMPLED_READ,
            Self::Attachment(layout) => layout.access_mask(),
        }
    }
}

impl From<ShaderImageLayout> for vk::ImageLayout {

    #[inline(always)] 
    fn from(value: ShaderImageLayout) -> vk::ImageLayout {
        match value {
            ShaderImageLayout::General(_) => vk::ImageLayout::GENERAL,
            ShaderImageLayout::SampledReadOnly => vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            ShaderImageLayout::Attachment(attachment) => attachment.into(),
        }
    }
}

/// Parameters used to [`allocate a descriptor set`][1].
///
/// See [`descriptor_set_info`] for full description.
///
/// [1]: Gpu::allocate_descriptor_sets
#[must_use]
pub struct DescriptorSetInfo<'a> {
    /// A mutable reference to where the newly allocated descriptor set's [`id`][1] will be stored.
    ///
    /// [1]: DescriptorSetId
    pub out_id: &'a mut DescriptorSetId,
    /// The [`shader set`][1] containing the [`DescriptorSetLayout`], which will be used to allocate the
    /// descriptor set (see [`descriptor_set_index`][Self::descriptor_set_index]).
    ///
    /// [1]: ShaderSet
    pub shader_set: ShaderSet,
    /// The index of the descriptor set within [`shader_set`][1], which will be used to allocate the
    /// descriptor set.
    ///
    /// [1]: Self::shader_set
    pub descriptor_set_index: u32,
}

/// Creates new [`DescriptorSetInfo`]
///
/// # Parameters
/// - `out_id`: A mutable reference to where the newly allocated descriptor set's [`id`][1] will be
///   stored.
/// - `shader_set`: The [`shader set`][2] containing the [`DescriptorSetLayout`], which will be used
///   to allocate the descriptor set.
/// - `descriptor_set_index`: The index of the descriptor set within `shader_set`, which will be
///   used to allocate the descriptor set.
///
/// [1]: DescriptorSetId
/// [2]: ShaderSet
#[inline]
pub fn descriptor_set_info<'a>(
    out_id: &'a mut DescriptorSetId,
    shader_set: ShaderSet,
    descriptor_set_index: u32,
) -> DescriptorSetInfo<'a> {
    DescriptorSetInfo {
        out_id,
        shader_set,
        descriptor_set_index,
    }
}

#[derive(Clone, Copy)]
pub(crate) struct DescriptorSetBuffer {
    pub buffer: Option<(BufferId, DeviceSize, DeviceSize)>,
    pub buffer_track_id: TrackedDescriptorSetId<BufferMeta>,
}

#[derive(Clone)]
pub(crate) struct ImageDescriptor {
    pub sampler: Option<Sampler>,
    pub image: Option<(ImageViewId, ShaderImageLayout)>,
    pub image_track_id: TrackedDescriptorSetId<ImageView>,
}

pub(super) enum DescriptorSetDescriptors {
    Buffers(NonNullVec32<'static, DescriptorSetBuffer>),
    Images(NonNullVec32<'static, ImageDescriptor>),
    InlineUniformBlock(u32),
}

match_display! {
    match DescriptorSetDescriptors {
        Buffers(_) => "buffers",
        Images(_) => "images",
        InlineUniformBlock(_) => "inline uniform block",
    }
}

pub(crate) struct DescriptorSetBinding {
    binding: u32,
    ty: DescriptorType,
    descriptors: DescriptorSetDescriptors,
}

impl DescriptorSetBinding {

    #[inline(always)]
    pub fn binding(&self) -> u32 {
        self.binding
    }

    #[inline(always)]
    pub fn ty(&self) -> DescriptorType {
        self.ty
    }

    #[inline(always)]
    pub fn descriptor_count(&self) -> u32 {
        match &self.descriptors {
            DescriptorSetDescriptors::Buffers(b) => b.len(),
            DescriptorSetDescriptors::Images(i) => i.len(),
            &DescriptorSetDescriptors::InlineUniformBlock(count) => count,
        }
    }

    #[inline(always)]
    pub fn buffer_descriptors(&self) -> &[DescriptorSetBuffer] {
        match &self.descriptors {
            DescriptorSetDescriptors::Buffers(descriptors) => {
                descriptors
            },
            _ => &[],
        }
    }

    #[inline(always)]
    pub fn buffer_descriptors_mut(&mut self) -> &mut [DescriptorSetBuffer] {
        match &mut self.descriptors {
            DescriptorSetDescriptors::Buffers(descriptors) => {
                descriptors
            },
            _ => &mut [],
        }
    }

    #[inline(always)]
    pub fn image_descriptors(&self) -> &[ImageDescriptor] {
        match &self.descriptors {
            DescriptorSetDescriptors::Images(descriptors) => {
                descriptors
            },
            _ => &[],
        }
    }

    #[inline(always)]
    pub fn image_descriptors_mut(&mut self) -> &mut [ImageDescriptor] {
        match &mut self.descriptors {
            DescriptorSetDescriptors::Images(descriptors) => {
                descriptors
            },
            _ => &mut [],
        }
    }
}

pub(crate) struct DescriptorSet {
    handle: vk::DescriptorSet,
    bindings: NonNullVec32<'static, DescriptorSetBinding>,
    stage_flags: ShaderStageFlags,
    arena: (NonNull<u8>, usize),
    last_used_frame: u64,
    flags: u32,
}

unsafe impl Send for DescriptorSet {}
unsafe impl Sync for DescriptorSet {}

impl DescriptorSet {

    const MAYBE_POISONED: u32 = 0x1;
    const IS_VALID: u32 = 0x2;

    #[inline(always)]
    fn new(
        bindings: NonNullVec32<'static, DescriptorSetBinding>,
        stage_flags: ShaderStageFlags,
        arena: Arena,
    ) -> Self
    {
        Self {
            handle: vk::DescriptorSet::null(),
            bindings,
            stage_flags,
            arena: arena.into_raw_parts(),
            last_used_frame: 0,
            flags: 0,
        }
    }

    #[inline(always)]
    pub fn handle(&self) -> vk::DescriptorSet {
        self.handle
    }

    #[inline(always)]
    pub fn bindings(&self) -> &[DescriptorSetBinding] {
        &self.bindings
    }

    #[inline(always)]
    pub fn is_maybe_poisoned(&self) -> bool {
        self.flags & Self::MAYBE_POISONED == Self::MAYBE_POISONED
    }

    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.flags & Self::IS_VALID == Self::IS_VALID
    }

    fn validate_bindings(
        &mut self,
        gpu: &Gpu,
    ) {
        self.flags = 0;
        if self.is_maybe_poisoned() {
            for binding in &mut self.bindings {
                for buffer in binding.buffer_descriptors_mut() {
                    buffer.buffer.take();
                }
                for image in binding.image_descriptors_mut() {
                    image.sampler.take();
                    image.image.take();
                }
            }
            self.flags &= !(Self::MAYBE_POISONED | Self::IS_VALID);
            return
        }
        for binding in &mut self.bindings {
            let ty = binding.ty();
            for buffer in binding.buffer_descriptors() {
                if let Some((id, _, _)) = buffer.buffer {
                    if !gpu.is_buffer_valid(id) {
                        self.flags &= !Self::IS_VALID;
                        return
                    }
                } else {
                    self.flags &= !Self::IS_VALID;
                    return 
                }
            }
            let requires_sampler = ty.requires_sampler();
            let requires_image = ty.requires_image();
            for image in binding.image_descriptors() {
                if requires_sampler && image.sampler.is_none() {
                    self.flags &= !Self::IS_VALID;
                    return
                }
                if requires_image {
                    if let Some((id, _,)) = image.image {
                        if !gpu.is_image_view_valid(id) {
                            self.flags &= !Self::IS_VALID;
                            return
                        }
                    } else {
                        self.flags &= !Self::IS_VALID;
                        return
                    }
                }
            }
        }
        self.flags |= Self::IS_VALID;
    }
}

impl Drop for DescriptorSet {

    fn drop(&mut self) {
        unsafe {
            for binding in &mut self.bindings {
                match &mut binding.descriptors {
                    DescriptorSetDescriptors::Buffers(bufs) => {
                        bufs.drop_in_place();
                    },
                    DescriptorSetDescriptors::Images(imgs) => {
                        imgs.drop_in_place();
                    },
                    _ => {},
                }
            }
            self.bindings.drop_in_place();
            let layout = Layout::from_size_align_unchecked(
                self.arena.1, arena::max_align(),
            );
            dealloc(self.arena.0.as_ptr(), layout);
        }
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DescriptorPoolId(SlotIndex<DescriptorPool>);

impl_id_display!(DescriptorPoolId);

impl DescriptorPoolId {

    #[inline(always)]
    pub(crate) fn new(index: SlotIndex<DescriptorPool>) -> Self {
        Self(index)
    }

    #[inline(always)]
    pub(crate) fn slot_index(self) -> SlotIndex<DescriptorPool> {
        self.0
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct DescriptorSetId(DescriptorPoolId, DescriptorSetInnerId);

impl Display for DescriptorSetId {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}", self.0, self.1)
    }
}

impl DescriptorSetId {

    #[inline(always)]
    pub fn pool_id(self) -> DescriptorPoolId {
        self.0
    }

    #[inline(always)]
    pub(crate) fn inner_id(self) -> DescriptorSetInnerId {
        self.1
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct TrackedDescriptorSetId<T>(pub SlotIndex<DescriptorSetInnerId>, pub PhantomData<T>);

impl<T> Default for TrackedDescriptorSetId<T> {

    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

impl<T> Clone for TrackedDescriptorSetId<T> {

    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for TrackedDescriptorSetId<T> {}

/// Specifies the parameters for descriptor buffer [`allocation`][Gpu::allocate_descriptor_sets].
///
/// See [`descriptor_buffer_info`] for creating default parameters.
///
/// # Valid usage
/// - [`buffer_id`][1] **must** be a valid [`BufferId`].
/// - [`offset`][2] **must** be less than the size of the buffer.
/// - If [`range`][3] is [`Some`], [`offset`][2] + [`range`][3] **must** be less than or equal to
///   the size of the buffer.
///
/// [1]: Self::buffer_id
/// [2]: Self::offset
/// [3]: Self::range
#[derive(Clone, Copy, BuildStructure)]
pub struct DescriptorBufferInfo {
    /// The source [`buffer id`][1].
    ///
    /// [1]: BufferId
    pub buffer_id: BufferId,
    /// An offset into the buffer.
    pub offset: DeviceSize,
    /// The size in bytes that is used for this descriptor update.
    ///
    /// Set to [`None`] to specify the end of the buffer from [`offset`][1].
    ///
    /// [1]: Self::offset
    #[skip]
    pub range: Option<NonZeroDeviceSize>,
}

/// Creates default [`DescriptorBufferInfo`].
#[inline]
pub fn descriptor_buffer_info(id: BufferId) -> DescriptorBufferInfo {
    DescriptorBufferInfo {
        buffer_id: id,
        offset: 0,
        range: None,
    }
}

/// Creates an inline uniform block [`DescriptorInfos`].
#[inline]
pub fn inline_uniform_block<'a, T: Copy>(
    data: &'a [T]
) -> Result<DescriptorInfos<'a>> {
    DescriptorInfos::inline_uniform_block(data)
}

impl DescriptorBufferInfo {

    /// The size in bytes that is used for this descriptor update.
    ///
    /// Set to zero to specify the end of the buffer from [`offset`][1].
    ///
    /// [1]: Self::offset
    #[inline]
    pub fn range(mut self, range: u64) -> Self {
        self.range = NonZeroDeviceSize::new(range);
        self
    }
}

/// Specifies the parameters for descriptor image [`allocation`][1].
///
/// [1]: Gpu::allocate_descriptor_sets
#[derive(Clone, BuildStructure)]
pub struct DescriptorImageInfo {
    pub sampler: Option<Sampler>,
    pub image_view_id: Option<ImageViewId>,
}

/// Creates default [`DescriptorImageInfo`].
#[inline]
pub fn descriptor_image_info<S, I>(
    sampler: S,
    image_view_id: I,
) -> DescriptorImageInfo 
    where
        S: Into<Option<Sampler>>,
        I: Into<Option<ImageViewId>>,
{
    DescriptorImageInfo {
        sampler: sampler.into(),
        image_view_id: image_view_id.into(),
    }
}

/// Specifies how descriptors will be updated in a descriptor set write operation either through
/// [`descriptor set update`][1] or [`push descriptor bindings`][2] call.
///
/// [1]: Gpu::update_descriptor_sets
/// [2]: ext::push_descriptor
#[derive(Clone)]
pub struct DescriptorInfos<'a>(DescriptorInfosInner<'a>);

impl<'a> DescriptorInfos<'a> {

    /// Buffer descriptor writes.
    #[inline(always)]
    pub fn buffers(buffers: &'a [DescriptorBufferInfo]) -> Self {
        Self(DescriptorInfosInner::Buffer(buffers))
    }

    #[inline(always)]
    pub fn single_buffer(buffer: &'a DescriptorBufferInfo) -> Self {
        Self(DescriptorInfosInner::Buffer(slice::from_ref(buffer)))
    }

    /// Image descriptor writes.
    #[inline(always)]
    pub fn images(images: &'a [DescriptorImageInfo]) -> Self {
        Self(DescriptorInfosInner::Image(images))
    }

    #[inline(always)]
    pub fn single_image(image: &'a DescriptorImageInfo) -> Self {
        Self(DescriptorInfosInner::Image(slice::from_ref(image)))
    }

    /// [`Inline uniform block`][1] write.
    ///
    /// # Valid usage
    /// - Size of `data`, in bytes, *must* be a multiple of 4.
    /// - Alignment of `data` *must* be a multiple of 4.
    ///
    /// [1]: ext::inline_uniform_block
    #[inline(always)]
    pub fn inline_uniform_block<T: Copy>(data: &'a [T]) -> Result<Self>
    {
        let bytes = unsafe {
            slice::cast(data)
        }.ok_or_else(||
            if !size_of::<T>().is_multiple_of(4) {
                Error::just_context(format!(
                    "size of data {} is not a multiple of 4",
                    size_of_val(data)
                ))
            } else {
                Error::just_context(format!(
                    "alignment of data {} is not a multiple of 4",
                    align_of::<T>()
                ))
            }
        )?;
        Ok(Self(DescriptorInfosInner::InlineUniformBlock(bytes)))
    }

    #[inline(always)]
    pub fn descriptor_count(&self) -> u32 {
        match self.0 {
            DescriptorInfosInner::Buffer(b) => b.len() as u32,
            DescriptorInfosInner::Image(i) => i.len() as u32,
            DescriptorInfosInner::InlineUniformBlock(b) => b.len() as u32,
        }
    }

    #[inline(always)]
    pub fn is_buffers(&self) -> bool {
        matches!(self.0, DescriptorInfosInner::Buffer(_))
    }

    #[inline(always)]
    pub fn is_images(&self) -> bool {
        matches!(self.0, DescriptorInfosInner::Image(_))
    }

    #[inline(always)]
    pub fn is_inline_uniform_block(&self) -> bool {
        matches!(self.0, DescriptorInfosInner::InlineUniformBlock(_))
    }

    #[inline(always)]
    pub fn as_buffers(&self) -> Option<&[DescriptorBufferInfo]> {
        match self.0 {
            DescriptorInfosInner::Buffer(b) => Some(b),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn as_images(&self) -> Option<&[DescriptorImageInfo]> {
        match self.0 {
            DescriptorInfosInner::Image(i) => Some(i),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn as_inline_uniform_block(&self) -> Option<&[u32]> {
        match self.0 {
            DescriptorInfosInner::InlineUniformBlock(b) => Some(b),
            _ => None,
        }
    }
}

impl<'a> From<&'a DescriptorBufferInfo> for DescriptorInfos<'a> {

    #[inline]
    fn from(value: &'a DescriptorBufferInfo) -> Self {
        Self::single_buffer(value)
    }
}

impl<'a> From<&'a DescriptorImageInfo> for DescriptorInfos<'a> {

    #[inline]
    fn from(value: &'a DescriptorImageInfo) -> Self {
        Self::single_image(value)
    }
}

#[derive(Clone)]
enum DescriptorInfosInner<'a> {
    Buffer(&'a [DescriptorBufferInfo]),
    Image(&'a [DescriptorImageInfo]),
    InlineUniformBlock(&'a [u32]),
}

/// Specifies the parameters of a [`descriptor set write operation`][1].
///
/// See [`write_descriptor_set`] for a full description.
///
/// [1]: Gpu::update_descriptor_sets
#[derive(Clone)]
pub struct WriteDescriptorSet<'a> {
    pub(crate) set_id: DescriptorSetId,
    pub(crate) binding: u32,
    pub(crate) starting_index: u32,
    pub(crate) infos: DescriptorInfos<'a>,
}

/// Creates new [`WriteDescriptorSet`] for a [`descriptor set write operation`][1].
///
/// # Parameters
/// - `set_id`: specifies the [`descriptor set`][2] to update.
/// - `binding`: specifies the binding within the descriptor set to update.
/// - `starting_index`: specifies either the starting descriptor array element or a byte
///   offset to an [`inline uniform block`][3] to update.
/// - `infos`: specifies what each descriptor from [`starting_index`] to the number of
///   elements in the [`variant`][4] will be updated with.
///
/// # Valid usage
/// - `set_id` **must** be a valid [`DescriptorSetId`]
/// - `binding` **must** be a valid binding in the descriptor set
/// - The [`descriptor type`][6] of the binding **must** be compatible with the [`variant`][4] of
///   `infos` as described in the variants of [`DescriptorType`].
/// - `starting_index` + the [`number of elements`][5] in `infos` **must** not overflow the
///   descriptor count or the number of bytes in an [`inline uniform block`][3] in that binding.
/// - If the write is an [`inline uniform block`][3] write, offset **must** be a multiple of 4.
///
/// [1]: Gpu::update_descriptor_sets
/// [2]: DescriptorSetId
/// [3]: ext::inline_uniform_block
/// [4]: DescriptorInfos
/// [5]: DescriptorInfos::descriptor_count
/// [6]: DescriptorType
pub fn write_descriptor_set<'a, D>(
    set_id: DescriptorSetId,
    binding: u32,
    starting_index: u32,
    infos: D,
) -> Result<WriteDescriptorSet<'a>>
    where D: Into<DescriptorInfos<'a>>
{
    let infos = infos.into();
    if infos.as_inline_uniform_block().is_some() &&
        !starting_index.is_multiple_of(4)
    {
        return Err(Error::just_context(format!(
            "starting index {starting_index} is not a multiple of 4"
        )))
    }
    Ok(WriteDescriptorSet {
        set_id,
        binding,
        starting_index,
        infos,
    })
}

impl<'a> WriteDescriptorSet<'a> {
    #[inline(always)]
    pub fn new(
        set_id: DescriptorSetId,
        binding: u32,
        starting_index: u32,
        infos: DescriptorInfos<'a>,
    ) -> Result<Self>
    {
        if infos.as_inline_uniform_block().is_some() &&
            !starting_index.is_multiple_of(4)
        {
            return Err(Error::just_context(format!(
                "starting index {starting_index} is not a multiple of 4"
            )))
        }
        Ok(Self {
            set_id,
            binding,
            starting_index,
            infos,
        })
    }

    #[inline(always)]
    pub fn buffer_infos(&self) -> Option<&[DescriptorBufferInfo]> {
        self.infos.as_buffers()
    }

    #[inline(always)]
    pub fn image_infos(&self) -> Option<&[DescriptorImageInfo]> {
        self.infos.as_images()
    }

    #[inline(always)]
    pub fn inline_uniform_block(&self) -> Option<&[u32]> {
        self.infos.as_inline_uniform_block()
    }
}

#[derive(Clone, Copy)]
pub struct CopyDescriptorSet {
    pub src_set_id: DescriptorSetId,
    pub src_binding: u32,
    pub src_starting_index: u32,
    pub dst_set_id: DescriptorSetId,
    pub dst_starting_index: u32,
    pub dst_binding: u32,
    pub array_count: u32,
}

struct Pool {
    size: u32,
    used: u32,
}

mod inner {

    use super::*;

    pub(crate) struct Inner {
        pub device: Device,
        pub id: DescriptorPoolId,
        pub handle: vk::DescriptorPool,
        pub max_sets: u32,
        pub allocated_sets: SlotMap<DescriptorSet>,
        pub max_inline_uniform_block_bindings: u32,
        pub allocated_inline_uniform_block_bindings: u32,
        pub(super) pools: AHashMap<DescriptorType, Pool>,
        pub pending_validations: AHashSet<DescriptorSetInnerId>,
        pub tracked_buffers: AHashMap<BufferId, SlotMap<DescriptorSetInnerId>>,
        pub tracked_image_views: AHashMap<ImageViewId, SlotMap<DescriptorSetInnerId>>,
        pub poisoned: bool,
    }

    impl Inner {

        #[inline(always)]
        pub(super) fn update(&mut self, gpu: &Gpu) {
            for &id in self.pending_validations.iter() {
                if let Ok(set) = self.allocated_sets.get_mut(id.0) {
                    set.validate_bindings(gpu);
                }
            }
            self.pending_validations.clear();
            self.poisoned = false;
        }
    }
}

#[derive(Clone)]
pub(crate) struct DescriptorPool {
    inner: Arc<RwLock<inner::Inner>>,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct DescriptorSetInnerId(pub SlotIndex<DescriptorSet>);

impl_id_display!(DescriptorSetInnerId);

pub(crate) struct DescriptorPoolWriteGuard<'a> {
    inner: RwLockWriteGuard<'a, inner::Inner>,
}

#[derive(Clone, Copy)]
pub struct DescriptorPoolSize {
    pub descriptor_type: DescriptorType,
    pub size: u32,
}

pub fn descriptor_pool_size(
    descriptor_type: DescriptorType,
    size: u32
) -> DescriptorPoolSize {
    DescriptorPoolSize { descriptor_type, size }
}

impl DescriptorPool {

    pub fn new(
        device: Device,
        id: DescriptorPoolId,
        pool_sizes: impl IntoIterator<Item = DescriptorPoolSize>,
        max_sets: u32,
        max_inline_uniform_block_bindings: u32,
    ) -> Result<Self>
    {
        let mut pools = AHashMap::<DescriptorType, Pool>::default();
        for DescriptorPoolSize { descriptor_type, size } in pool_sizes.into_iter() {
            pools.entry(descriptor_type)
                .and_modify(|pool| pool.size += size)
                .or_insert(Pool { size, used: 0, });

        }
        let mut pool_sizes = Vec32::default();
        let mut uniform_block_info = None;
        for (&ty, pool) in &pools {
            if ty.is_inline_uniform_block() {
                if !pool.size.is_multiple_of(4) {
                    return Err(Error::just_context(
                        "inline uniform block pool size is not a multiple of 4"
                    ))
                }
                uniform_block_info = Some(vk::DescriptorPoolInlineUniformBlockCreateInfo {
                    max_inline_uniform_block_bindings,
                    ..Default::default()
                });
            }
            pool_sizes.push(vk::DescriptorPoolSize {
                ty: ty.into(),
                descriptor_count: pool.size,
            });
        }
        let mut info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            flags: vk::DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET |
                    vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND,
            max_sets,
            pool_size_count: pool_sizes.len(),
            p_pool_sizes: pool_sizes.as_ptr(),
            ..Default::default()
        };
        if let Some(uniform_block_info) = &mut uniform_block_info {
            if !device.get_device_attribute(ext::inline_uniform_block::Attributes::IS_ENABLED)
                .bool().unwrap_or(false)
            {
                return Err(Error::just_context(
                    "attempting to use inline uniform block extension when it's not enabled"
                ))
            }
            info = info.push_next(uniform_block_info);
        }
        let handle = unsafe {
            device
                .create_descriptor_pool(&info, None)
                .context("failed to create descriptor pool")?
        }.value;
        Ok(Self {
            inner: Arc::new(RwLock::new(inner::Inner {
                device,
                id,
                handle,
                max_sets,
                allocated_sets: SlotMap::default(),
                max_inline_uniform_block_bindings,
                allocated_inline_uniform_block_bindings: 0,
                pools,
                tracked_buffers: AHashMap::default(),
                tracked_image_views: AHashMap::default(),
                pending_validations: AHashSet::default(),
                poisoned: false,
            }))
        })
    }

    pub fn buffer_delete(&self, id: BufferId) {
        let mut inner = self.inner.write();
        if let Some(tracked) = inner.tracked_buffers.remove(&id) {
            for &set_id in tracked.values() {
                inner.pending_validations.insert(set_id);
            }
        }
    }

    pub fn image_view_delete(&self, id: ImageViewId) {
        let mut inner = self.inner.write();
        if let Some(tracked) = inner.tracked_image_views.remove(&id) {
            for &set_id in tracked.values() {
                inner.pending_validations.insert(set_id);
            }
        }
    }

    pub fn allocate<'a, I, Alloc>(
        &self,
        set_infos: I,
        tmp_alloc: &Alloc,
    ) -> Result<()>
        where
            Alloc: LocalAlloc<Error = arena::Error>,
            I: IntoIterator<Item = DescriptorSetInfo<'a>>,
            I::IntoIter: ExactSizeIterator,
    {
        let set_infos = set_infos.into_iter();
        let mut set_infos = {
            let mut infos = FixedVec32::with_capacity(
                set_infos.len() as u32,
                tmp_alloc
            ).context("alloc failed")?;
            infos.extend(set_infos);
            infos
        };
        let count = set_infos.len();
        {
            let inner = self.inner.read();
            if inner.allocated_sets.len() + count > inner.max_sets {
                return Err(Error::just_context(format!(
                    "pool was full with max sets {}", inner.max_sets,
                )))
            }
        }
        let mut set_layouts = FixedVec32
            ::with_capacity(count, tmp_alloc)
            .context("alloc failed")?;
        let mut sets = FixedVec32
            ::with_capacity(count, tmp_alloc)
            .context("alloc failed")?;
        let mut inner = self.inner.write();
        for info in set_infos.iter() {
            let layout = info.shader_set.descriptor_set_layouts()
                .get(info.descriptor_set_index as usize)
                .ok_or_else(|| Error::just_context(format!(
                    "invalid descriptor set index {} for shader set {}",
                    info.descriptor_set_index, info.shader_set.id(),
                )))?;
            if layout.is_push_descriptor() {
                return Err(Error::just_context(format!(
                    "{}{}",
                    format_args!("attempting to allocate descriptor set with index {} of shader set {} ",
                        info.descriptor_set_index, info.shader_set.id()
                    ),
                    "that has the push descriptor flag set",
                )))
            }
            set_layouts.push(layout.handle);
            let set = &info.shader_set.descriptor_set_layouts()[info.descriptor_set_index as usize];
            let alloc_size: usize = set.bindings
                .iter().map(|binding|
                    size_of::<DescriptorSetBinding>() +
                    if binding.descriptor_type.is_buffer() {
                        size_of::<DescriptorSetBuffer>() * binding.descriptor_count as usize
                    } else if binding.descriptor_type.is_image() {
                        size_of::<ImageDescriptor>() * binding.descriptor_count as usize
                    } else {
                        0
                    }
                ).sum();
            let alloc = Arena
                ::new(alloc_size)
                .context("failed to create arena")?;
            let mut bindings = NonNullVec32
                ::with_capacity(set.bindings.len(), &alloc)
                .context("alloc failed")?
                .into_static();
            bindings.try_extend(set.bindings
                .iter()
                .map(|binding| {
                    if let Some(pool) = inner.pools.get_mut(&binding.descriptor_type) {
                        let used = pool.used + binding.descriptor_count;
                        if used > pool.size {
                            Err(Error::just_context(format!(
                                "maximum capacity {} for descriptor type {} reached",
                                pool.size, binding.descriptor_type,
                            )))
                        } else {
                            pool.used = used;
                            let descriptors =
                                match binding.descriptor_type {
                                    crate::buffer_descriptor_types!() => {
                                        let mut bufs = NonNullVec32::with_capacity(
                                            binding.descriptor_count,
                                            &alloc,
                                        ).context("alloc failed")?.into_static();
                                        bufs.resize(
                                            binding.descriptor_count,
                                            DescriptorSetBuffer {
                                                buffer: None, buffer_track_id: Default::default(),
                                            },
                                        );
                                        DescriptorSetDescriptors::Buffers(bufs)
                                    },
                                    crate::image_descriptor_types!() => {
                                        let mut imgs = NonNullVec32::with_capacity(
                                            binding.descriptor_count,
                                            &alloc
                                        ).context("alloc failed")?.into_static();
                                        imgs.resize(
                                            binding.descriptor_count,
                                            ImageDescriptor {
                                                sampler: None,
                                                image: None, image_track_id: Default::default(),
                                            },
                                        );
                                        DescriptorSetDescriptors::Images(imgs)
                                    },
                                    DescriptorType::InlineUniformBlock => {
                                        let n = inner.allocated_inline_uniform_block_bindings + binding.descriptor_count;
                                        if n > inner.max_inline_uniform_block_bindings {
                                            return Err(Error::just_context(format!(
                                                "maximum number of inline uniform block bindings {} reached",
                                                inner.max_inline_uniform_block_bindings
                                            )))
                                        }
                                        inner.allocated_inline_uniform_block_bindings = n;
                                        DescriptorSetDescriptors::InlineUniformBlock(binding.descriptor_count)
                                    },
                                    DescriptorType::Unknown => {
                                        return Err(Error::just_context("unknown descriptor type"))
                                    },
                                };
                            Ok(DescriptorSetBinding {
                                binding: binding.binding,
                                ty: binding.descriptor_type,
                                descriptors,
                            })
                        }
                    } else {
                        Err(Error::just_context(format!(
                            "no space allocated for descriptor type {}",
                            binding.descriptor_type,
                        )))
                    }
                })
            )?;
            sets.push(DescriptorSet::new(
                bindings, set.stage_flags,
                alloc,
            ));
        }
        let info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            descriptor_pool: inner.handle,
            descriptor_set_count: count,
            p_set_layouts: set_layouts.as_ptr(),
            ..Default::default()
        };
        let mut handles = FixedVec32
            ::with_len(count, Default::default(), tmp_alloc)
            .context("alloc failed")?;
        unsafe {
            inner.device.allocate_descriptor_sets(&info, &mut handles)
            .context("failed to allocate descriptor sets")?
        };
        for (i, mut set) in sets.into_iter().enumerate() {
            let handle = handles[i];
            set.handle = handle;
            let index = inner.allocated_sets.insert(set);
            *set_infos[i].out_id = DescriptorSetId(inner.id, DescriptorSetInnerId(index));
        }
        Ok(())
    }

    pub unsafe fn free<Alloc>(
        &self,
        gpu: &Gpu,
        queue_scheduler: &QueueSchedulerReadGuard,
        ids: &[DescriptorSetId],
        tmp_alloc: &Alloc,
    ) -> Result<()>
        where 
            Alloc: LocalAlloc<Error = arena::Error>,
    {
        let mut inner = self.inner.write();
        let mut descriptor_sets = FixedVec32
            ::with_capacity(ids.len() as u32, tmp_alloc)
            .context("alloc failed")?;
        let finsihed_frame = gpu
            .get_semaphore_counter_value(queue_scheduler.get_frame_semaphore_id())?;
        for &id in ids {
            if let Ok(set) = inner.allocated_sets.remove(id.1.0) {
                if set.last_used_frame > finsihed_frame {
                    return Err(Error::just_context(format!(
                        "attempting to free descriptor set {id} while it is still in use by a queue"
                    )))
                }
                for binding in set.bindings.iter() {
                    let buffer_descriptors = binding.buffer_descriptors();
                    for descriptor in buffer_descriptors {
                        if let Some((buffer_id, _, _)) = descriptor.buffer {
                            inner.untrack_buffer(buffer_id, descriptor.buffer_track_id)?;
                        }
                    }
                    let image_descriptors = binding.image_descriptors();
                    for descriptor in image_descriptors {
                        if let Some((id, _)) = descriptor.image {
                            inner.untrack_image(id, descriptor.image_track_id)?;
                        }
                    }
                    if let Some(pool) = inner.pools.get_mut(&binding.ty) {
                        pool.used -= binding.descriptor_count();
                    }
                    if binding.ty().is_inline_uniform_block() {
                        inner.allocated_inline_uniform_block_bindings -= 1;
                    }
                }
                descriptor_sets.push(set.handle);
            }
        }
        unsafe {
            inner.device.free_descriptor_sets(
                inner.handle,
                &descriptor_sets,
            ).context("failed to free descriptor sets")?;
        }
        Ok(())
    }

    pub fn update(
        &self,
        gpu: &Gpu,
    ) {
        self.inner.write().update(gpu);
    }

    #[inline(always)]
    pub fn write(&self) -> DescriptorPoolWriteGuard<'_> {
        DescriptorPoolWriteGuard {
            inner: self.inner.write(),
        }
    }
}

impl inner::Inner {

    #[inline(always)]
    fn add_pending_validation(&mut self, id: DescriptorSetInnerId) {
        self.pending_validations.insert(id);
    }

    #[inline(always)]
    fn track_buffer(
        &mut self,
        buffer_id: BufferId,
        set_id: DescriptorSetInnerId,
    ) -> TrackedDescriptorSetId<BufferMeta> {
        TrackedDescriptorSetId(
            self.tracked_buffers.entry(buffer_id)
                .or_default()
                .insert(set_id),
            PhantomData,
        )
    }

    #[inline(always)]
    fn untrack_buffer(
        &mut self,
        buffer_id: BufferId,
        tracked_id: TrackedDescriptorSetId<BufferMeta>
    ) -> Result<()>
    {
        let Some(tracked) = self.tracked_buffers
            .get_mut(&buffer_id) else
        {
            return Ok(())
        };
        tracked
            .remove(tracked_id.0)
            .context("failed to untrack buffer")?;
        Ok(())
    } 

    #[inline(always)]
    fn track_image(
        &mut self,
        view_id: ImageViewId,
        set_id: DescriptorSetInnerId,
    ) -> TrackedDescriptorSetId<ImageView> {
        TrackedDescriptorSetId(
            self.tracked_image_views.entry(view_id)
                .or_default()
                .insert(set_id),
            PhantomData,
        )
    }

    #[inline(always)]
    fn untrack_image(
        &mut self,
        view_id: ImageViewId,
        tracked_id: TrackedDescriptorSetId<ImageView>,
    ) -> Result<()>
    {
        let Some(tracked) = self.tracked_image_views
            .get_mut(&view_id) else {
            return Ok(())
        };
        tracked
            .remove(tracked_id.0)
            .context("failed to untrack image")?;
        Ok(())
    }
}

pub(crate) struct DescriptorSetHandle(*mut DescriptorSet, DescriptorSetInnerId);

impl DescriptorSetHandle {

    #[inline(always)]
    pub unsafe fn get(&mut self) -> (&mut DescriptorSet, DescriptorSetInnerId) {
        unsafe {
            (self.0.as_mut().unwrap(), self.1)
        }
    }

    #[inline(always)]
    pub unsafe fn unpoison(&mut self) {
        unsafe {
            self.get().0
        }.flags &= !DescriptorSet::MAYBE_POISONED;
    }
}

impl<'a> DescriptorPoolWriteGuard<'a> {

    #[inline(always)]
    pub fn get_descriptor_set_for_update(
        &mut self,
        id: DescriptorSetId,
        finished_frame: u64,
    ) -> Result<DescriptorSetUpdateContext<'_, 'a>>
    {
        if self.inner.poisoned {
            return Err(Error::just_context("descriptor pool poisoned"))
        }
        let mut handle = self.get_descriptor_set_handle(id)?;
        unsafe {
            let set = handle.get().0;
            if set.last_used_frame > finished_frame {
                return Err(Error::just_context(format!(
                    "attempting to update descriptor set {id} while it is still in use by a queue"
                )))
            }
        }
        Ok(DescriptorSetUpdateContext {
            handle,
            pool: self,
        })
    }

    #[inline(always)]
    pub fn get_descriptor_set_for_submit(
        &mut self,
        id: DescriptorSetId,
        current_frame: u64,
    ) -> Result<DescriptorSetSubmitContext<'_, 'a>> {
        let mut handle = self.get_descriptor_set_handle(id)?;
        unsafe {
            let set = handle.get().0;
            if !set.is_valid() {
                return Err(Error::just_context(format!(
                    "descriptor set {id} is in an invalid state",
                )))
            }
            set.last_used_frame = current_frame;
        }
        Ok(DescriptorSetSubmitContext {
            _pool: self,
            handle 
        })
    }

    #[inline(always)]
    pub fn get_descriptor_set_handle(
        &mut self,
        id: DescriptorSetId,
    ) -> Result<DescriptorSetHandle>
    {
        if self.inner.poisoned {
            return Err(Error::just_context("pool poisoned"))
        }
        let set = self.inner.allocated_sets
            .get_mut(id.inner_id().0)
            .context_with(|| format!(
                "invalid descriptor set id {id}"
            ))?;
        Ok(DescriptorSetHandle(set as *mut _, id.inner_id()))
    }

    #[inline(always)]
    pub fn get_descriptor_set(
        &mut self,
        id: DescriptorSetId,
    ) -> Result<&mut DescriptorSet> {
        self.allocated_sets
            .get_mut(id.inner_id().0)
            .context_with(|| format!(
                "invalid descriptor set id {id}"
            ))
    }
}

impl<'a> Deref for DescriptorPoolWriteGuard<'a> {

    type Target = inner::Inner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> DerefMut for DescriptorPoolWriteGuard<'a> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Drop for inner::Inner {

    fn drop(&mut self) {
        unsafe {
            self.device.destroy_descriptor_pool(
                self.handle, None,
            );
        }
    }
}

pub(crate) struct DescriptorSetUpdateContext<'a, 'b> {
    pool: &'a mut DescriptorPoolWriteGuard<'b>,
    handle: DescriptorSetHandle,
}

pub(crate) enum DescriptorUpdateInfos<'c, Alloc>
    where Alloc: LocalAlloc<Error = arena::Error>
{
    Buffer(FixedVec32<'c, vk::DescriptorBufferInfo, Alloc>),
    Image(FixedVec32<'c, vk::DescriptorImageInfo, Alloc>),
    InlineUniformBlock(vk::WriteDescriptorSetInlineUniformBlock<'static>),
}

impl<'a, 'b> DescriptorSetUpdateContext<'a, 'b> {

    #[inline(always)]
    pub fn descriptor_set(&mut self) -> vk::DescriptorSet {
        unsafe {
            self.handle.get().0
        }.handle
    }

    #[inline(always)]
    pub fn into_inner(self) -> DescriptorSetHandle {
        self.handle
    }

    #[inline(always)]
    pub fn update<'c, Alloc>(
        &mut self,
        gpu: &Gpu,
        write: &WriteDescriptorSet,
        alloc: &'c Alloc,
    ) -> Result<(vk::DescriptorType, DescriptorUpdateInfos<'c, Alloc>)>
        where 
            Alloc: LocalAlloc<Error = arena::Error>,
    {
        if let Some(infos) = write.infos.as_buffers() {
            let mut pool = RaiiHandle::new(&mut *self.pool, |pool| {
                pool.poisoned = true;
            });
            let (set, id) = unsafe {
                self.handle.get()
            };
            pool.add_pending_validation(id);
            set.flags |= DescriptorSet::MAYBE_POISONED;
            let Some(binding) = set.bindings.iter_mut().find(|b| b.binding == write.binding) else {
                return Err(Error::just_context(format!(
                    "invalid buffer write binding {} for descriptor set {}",
                    write.binding, write.set_id,
                )))
            };
            let count = binding.buffer_descriptors().len() as u32;
            if infos.len() as u32 + write.starting_index > count {
                return Err(Error::just_context(format!(
                    "{}{}",
                    format_args!("buffer write descriptor starting index {} + count {} ",
                        write.starting_index, infos.len()
                    ),
                    format_args!("is out of range of descriptor binding {} descriptor count {}",
                        write.binding, count,
                    ),
                )))
            }
            let ty = binding.ty;
            let Some(usage) = ty.buffer_usage() else {
                return Err(Error::just_context(format!(
                    "non-buffer descriptor on buffer write (binding {} for descriptor set {})",
                    write.binding, write.set_id,
                )))
            };
            let mut vk_infos = FixedVec32
                ::with_capacity(infos.len() as u32, alloc)
                .context("alloc failed")?;
            let starting_idx = write.starting_index as usize;
            for (i, descriptor) in binding.buffer_descriptors_mut()[starting_idx..starting_idx + infos.len()]
                .iter_mut()
                .enumerate()
            {
                let info = infos[i];
                if let Some((id, _, _)) = descriptor.buffer {
                    pool.untrack_buffer(id, descriptor.buffer_track_id)?;
                }
                let buffers = gpu.read_buffers();
                let buffer = buffers
                    .get(info.buffer_id)
                    .context_with(|| format!(
                        "failed to get buffer for descriptor set {} write (binding {}, index {}, type {ty})",
                        write.set_id, write.binding, starting_idx + i,
                    ))?;
                if let Some(err) = buffer.validate_usage(usage) {
                    return Err(Error::new(err, format!(
                        "buffer {} usage mismatch for descriptor set {}, (binding {}, index {}, type {ty})",
                        info.buffer_id, write.set_id, write.binding, starting_idx + i,
                    )))
                }
                let properties = buffer.properties();
                let (vk_range, effective) = match info.range {
                    Some(range) => {
                        let range = range.get();
                        if info.offset + range > properties.size
                        {
                            return Err(Error::just_context(format!(
                                "{}{}",
                                format_args!("descriptor set {} write (binding {}, index {}, type {ty}) ",
                                    write.set_id, write.binding, starting_idx + i,
                                ),
                                format_args!("buffer {} offset {} + size {} is out of range of buffer size {}",
                                    info.buffer_id, info.offset, range, properties.size,
                                )
                            )))
                        }
                        (range, range)
                    },
                    None => {
                        if info.offset >= properties.size {
                            return Err(Error::just_context(format!(
                                "{}{}",
                                format_args!("descriptor set {} write (binding {}, index {}, type {ty}) ",
                                    write.set_id, write.binding, starting_idx + i,
                                ),
                                format_args!("buffer {} offset {} is greater than or equal buffer size {}",
                                    info.buffer_id, info.offset, properties.size,
                                )
                            )))
                        }
                        (vk::WHOLE_SIZE, properties.size - info.offset)
                    }
                };
                descriptor.buffer = Some((info.buffer_id, info.offset, effective));
                descriptor.buffer_track_id = pool.track_buffer(info.buffer_id, id);
                let vk_info = vk::DescriptorBufferInfo {
                    buffer: buffer.handle(),
                    offset: info.offset,
                    range: vk_range,
                };  
                vk_infos.push(vk_info);
            }
            pool.into_inner();
            Ok((ty.into(), DescriptorUpdateInfos::Buffer(vk_infos)))
        } else if let Some(infos) = write.image_infos() {
            let mut pool = RaiiHandle::new(&mut *self.pool, |pool| {
                pool.poisoned = true;
            });
            let (set, id) = unsafe {
                self.handle.get()
            };
            pool.add_pending_validation(id);
            set.flags |= DescriptorSet::MAYBE_POISONED;
            let Some(binding) = set.bindings.iter_mut().find(|b| b.binding == write.binding) else {
                return Err(Error::just_context(format!(
                    "invalid image write binding {} for descriptor set {}",
                    write.binding, write.set_id,
                )))
            };
            let ty = binding.ty();
            let Some(image_usage) = ty.image_usage() else {
                return Err(Error::just_context(format!(
                    "non-image descriptor on image write (binding {} for descriptor set {})",
                    write.binding, write.set_id,
                )))
            };
            let image_layout = ty.shader_image_layout();
            let count = binding.image_descriptors().len() as u32;
            if infos.len() as u32 + write.starting_index > count {
                return Err(Error::just_context(format!(
                    "image write descriptor starting index {} + count {} is out of range of descriptor binding {} descriptor count {}",
                    write.starting_index, infos.len(), write.binding, count,
                )))
            }
            let mut vk_infos = FixedVec32
                ::with_capacity(infos.len() as u32, alloc)
                .context("alloc failed")?;
            let starting_idx = write.starting_index as usize;
            for (i, descriptor) in binding.image_descriptors_mut()[starting_idx..starting_idx + infos.len()]
                .iter_mut()
                .enumerate()
            {
                let info = infos[i].clone();
                let sampler =
                    if ty.requires_sampler() {
                        let Some(sampler) = info.sampler else {
                            return Err(Error::just_context(format!(
                                "descriptor set {} (binding {}, index {}, type {ty}) requires a sampler, but none was given",
                                write.set_id, write.binding, starting_idx + i,
                            )))
                        };
                        descriptor.sampler = Some(sampler.clone());
                        sampler.handle().into_inner()
                    } else {
                        if info.sampler.is_some() {
                            log::warn!(
                                "descriptor set {} (binding {}, type {ty}) doesn't require a sampler, but a sampler was given",
                                write.set_id, write.binding,
                            );
                        }
                        vk::Sampler::null()
                    };
                let (image_view, layout) =
                    if let Some(layout) = image_layout {
                        if let Some((view, _)) = descriptor.image {
                            pool.untrack_image(view, descriptor.image_track_id)?;
                        }
                        let Some(image_view) = info.image_view_id else {
                            return Err(Error::just_context(format!(
                                "descriptor set {} (binding {}, type {ty}) requires an image, but none was given",
                                write.set_id, write.binding,
                            )))
                        };
                        let images = gpu.read_images();
                        let image = images.get(image_view.image_id())?;
                        if let Some(err) = image.validate_usage(image_usage) {
                            return Err(Error::new(
                                err,
                                format!(
                                    "descriptor set {} (binding {}, type (ty)) image usage mismatch",
                                    write.set_id, write.binding,
                                )
                            ))
                        }
                        descriptor.image = Some((image_view, layout));
                        descriptor.image_track_id = pool.track_image(image_view, write.set_id.inner_id());
                        (image.get_view(image_view)?.handle, layout.into())
                    } else {
                        if info.image_view_id.is_some() {
                            log::warn!(
                                "descriptor set {} (binding {}, type {ty}) doesn't require an image, but an image view was given",
                                write.set_id, write.binding,
                            );
                        }
                        (vk::ImageView::null(), vk::ImageLayout::UNDEFINED)
                    };
                let vk_info = vk::DescriptorImageInfo {
                    sampler,
                    image_view,
                    image_layout: layout,
                };
                vk_infos.push(vk_info);
            }
            pool.into_inner();
            Ok((ty.into(), DescriptorUpdateInfos::Image(vk_infos)))
        } else if let Some(data) = write.infos.as_inline_uniform_block() {
            let (set, _) = unsafe {
                self.handle.get()
            };
            let Some(binding) = set.bindings.iter_mut().find(|b| b.binding == write.binding) else {
                return Err(Error::just_context(format!(
                    "invalid inline uniform block write binding {} for descriptor set {}",
                    write.binding, write.set_id,
                )))
            };
            let ty = binding.ty();
            if !ty.is_inline_uniform_block() {
                return Err(Error::just_context(format!(
                    "expected inline uniform block descriptor type, but binding {} descriptor type is {ty}",
                    write.binding,
                )))
            }
            if write.starting_index + data.len() as u32 > binding.descriptor_count() {
                return Err(Error::just_context(format!(
                    "inline uniform block write offset {} + count {} is out of range of descriptor binding {} block size {}",
                    write.starting_index, data.len(), write.binding, binding.descriptor_count(),
                )))
            }
            let info = vk::WriteDescriptorSetInlineUniformBlock {
                data_size: data.len() as u32,
                p_data: data.as_ptr().cast(),
                ..Default::default()
            };
            Ok((ty.into(), DescriptorUpdateInfos::InlineUniformBlock(info)))
        } else {
            unreachable!()
        }
    }

    #[inline(always)]
    pub unsafe fn copy_from<'c>(
        &mut self,
        mut from: DescriptorSetHandle,
        src_binding: u32,
        src_starting_index: u32,
        dst_binding: u32,
        dst_starting_index: u32,
        array_count: u32,
    ) -> Result<vk::CopyDescriptorSet<'c>>
    {
        let mut pool = RaiiHandle::new(&mut *self.pool, |pool| {
            pool.poisoned = true;
        });
        let (from, from_id) = unsafe {
            from.get()
        };
        let (to, to_id) = unsafe {
            self.handle.get()
        };
        pool.add_pending_validation(to_id);
        to.flags |= DescriptorSet::MAYBE_POISONED;
        let src_binding_id = src_binding;
        let src_binding = from.bindings.iter().find(|b| b.binding == src_binding_id)
            .ok_or_else(|| Error::just_context(format!(
                "invalid source binding {}", src_binding,
            )))?;
        let dst_binding_id = dst_binding;
        let dst_binding = to.bindings.iter_mut().find(|b| b.binding == dst_binding)
            .ok_or_else(|| Error::just_context(format!(
                "invalid destination binding {}", dst_binding,
            )))?;
        let src_ty = src_binding.ty();
        let dst_ty = dst_binding.ty();
        if src_ty != dst_ty {
            return Err(Error::just_context(format!(
                "descriptor type mismatch, source type was {src_ty} while destination type {dst_ty}",
            )))
        }
        if to_id == from_id &&
            src_starting_index + array_count > dst_starting_index &&
            dst_starting_index + array_count > src_starting_index
        {
            return Err(Error::just_context(format!(
                "{}{}",
                format_args!("descriptor set copy source and destination (id: {from_id}) overlap, "),
                format_args!("if source and destination descriptor set are equal, the source and destination ranges must not overlap")
            )))?;
        }
        let ty = src_ty;
        if ty.is_buffer() {
            let src_descriptors = src_binding.buffer_descriptors();
            let dst_descriptors = dst_binding.buffer_descriptors_mut();
            if src_starting_index + array_count > src_descriptors.len() as u32 {
                return Err(Error::just_context(format!(
                    "descriptor set copy source starting index {} + array count {} is out of range of source count {}",
                    src_starting_index, array_count, src_descriptors.len(),
                )))
            }
            if dst_starting_index + array_count > dst_descriptors.len() as u32 {
                return Err(Error::just_context(format!(
                    "descriptor set copy destination starting index {} + array count {} is out of range of destination count {}",
                    dst_starting_index, array_count, dst_descriptors.len(),
                )))
            }
            let src_starting_index = src_starting_index as usize;
            for (i, dst_descriptor) in dst_descriptors[dst_starting_index as usize..(dst_starting_index + array_count) as usize]
                .iter_mut()
                .enumerate()
            {
                if let Some((id, _, _)) = dst_descriptor.buffer {
                    pool.untrack_buffer(id, dst_descriptor.buffer_track_id)?;
                }
                let new_buffer = src_descriptors[src_starting_index + i].buffer
                    .ok_or_else(|| Error::just_context(format!(
                        "source buffer descriptor set (index: {}) had uninitialized values",
                        src_starting_index + i,
                    )))?;
                dst_descriptor.buffer = Some(new_buffer);
                dst_descriptor.buffer_track_id = pool.track_buffer(new_buffer.0, to_id);
            }
        } else if ty.is_image() {
            let src_descriptors = src_binding.image_descriptors();
            let dst_descriptors = dst_binding.image_descriptors_mut();
            if src_starting_index + array_count > src_descriptors.len() as u32 {
                return Err(Error::just_context(format!(
                    "{}{}",
                    format_args!("descriptor set copy source starting index {} + array count {} ",
                        src_starting_index, array_count
                    ),
                    format_args!("is out of range of source count {}",
                        src_descriptors.len()
                    ),
                )))
            }
            if dst_starting_index + array_count > dst_descriptors.len() as u32 {
                return Err(Error::just_context(format!(
                    "{}{}",
                    format_args!("descriptor set copy destination starting index {} + array count {} ",
                        dst_starting_index, array_count,
                    ),
                    format_args!("is out of range of destination count {}",
                        dst_descriptors.len(),
                    )
                )))
            }
            let src_starting_index = src_starting_index as usize;
            for (i, dst_descriptor) in dst_descriptors[
                    dst_starting_index as usize..(dst_starting_index + array_count) as usize
                ].iter_mut()
                .enumerate()
            {
                if let Some((image_id, _)) = dst_descriptor.image {
                    pool.untrack_image(image_id, dst_descriptor.image_track_id)?;
                }
                let src_descriptor = src_descriptors[src_starting_index + i].clone();
                if let Some((image_id, _)) = src_descriptor.image {
                    dst_descriptor.image_track_id = pool.track_image(image_id, to_id);
                }
                dst_descriptor.sampler = src_descriptor.sampler.clone();
                dst_descriptor.image = src_descriptor.image;
            }
        }
        pool.into_inner();
        Ok(vk::CopyDescriptorSet {
            s_type: vk::StructureType::COPY_DESCRIPTOR_SET,
            src_set: from.handle,
            src_binding: src_binding_id,
            src_array_element: src_starting_index,
            dst_set: to.handle,
            dst_binding: dst_binding_id,
            dst_array_element: dst_starting_index,
            descriptor_count: array_count,
            ..Default::default()
        })
    }
}

pub(crate) struct DescriptorSetSubmitContext<'a, 'b> {
    _pool: &'a mut DescriptorPoolWriteGuard<'b>,
    handle: DescriptorSetHandle,
}

impl<'a, 'b> DescriptorSetSubmitContext<'a, 'b> {

    #[inline(always)]
    pub fn binding_iter(&mut self) -> impl Iterator<Item = &'_ DescriptorSetBinding> {
        unsafe {
            self.handle.get()
        }.0.bindings.iter()
    }

    #[inline(always)]
    pub fn stage_flags(&mut self) -> ShaderStageFlags {
        unsafe {
            self.handle.get()
        }.0.stage_flags
    }
}
