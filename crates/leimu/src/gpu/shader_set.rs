use core::{
    ffi::CStr,
    slice,
    hash::{Hasher, Hash},
};

use ahash::AHashMap;

use tuhka::vk;
use leimu_mem::{
    alloc::{self, Layout}, int::Integer,
    pack_alloc,
    slot_map::{SlotIndex, SlotMap},
    vec::{ArrayVec, FixedVec32, Pointer, Vec32},
    vec32,
};

use futures::future::RemoteHandle;

use crate::{
    bitflags, core::{OptionExt, TryExtend},
    error::*,
    executor::{SpawnExt, ThreadPool},
    gpu::{
        MtContext, ext, prelude::*
    },
    macros::impl_id_display,
    sync::{Arc, FutureLock, RwLock},
};

bitflags!(
    /// Specifies how a descriptor set *can* be used.
    #[default = Self::empty()]
    pub struct DescriptorSetLayoutFlags: Flags32 {
        /// Specifies that the descriptor set *must* not be allocated from a
        /// [`DescriptorPool`], but instead should be pushed by `push descriptor set`
        /// commands provided by [`DrawPipelineCommands`] and [`PipelineCommands`].
        ///
        /// # Valid usage
        /// - The [`push_descriptor`][1] device extension *must* be enabled.
        ///
        /// [1]: ext::push_descriptor
        PUSH_DESCRIPTOR = 0x1,
    }
);

/// Specifies how a binding in a descriptor set *can* be used.
#[derive(Default, Clone, Copy)]
pub struct DescriptorBindingAttributes {
    variable_descriptor_count: Option<u32>,
}

impl DescriptorBindingAttributes {

    /// Specifies that the binding has variable descriptor count up to `upper_bound`.
    ///
    /// Requires enabling the [`descriptor indexing`][1] extension with
    /// [`variable descriptor count`][2] enabled.
    ///
    /// [1]: ext::descriptor_indexing
    /// [2]: ext::descriptor_indexing::Features::descriptor_binding_variable_descriptor_count
    #[inline]
    pub fn with_variable_descriptor_count(mut self, upper_bound: u32) -> Self
    {
        self.variable_descriptor_count = Some(upper_bound);
        self
    }
}

/// Contains the handle and metadata of a [`descriptor set layout`][1].
///
/// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayout.html
#[derive(Clone)]
pub struct DescriptorSetLayout {
    pub(crate) handle: vk::DescriptorSetLayout,
    pub(crate) bindings: Vec32<DescriptorSetLayoutBinding>,
    pub(crate) stage_flags: ShaderStageFlags,
    pub(crate) flags: DescriptorSetLayoutFlags,
}

impl DescriptorSetLayout {

    #[inline(always)]
    pub fn is_push_descriptor(&self) -> bool {
        self.flags.contains(DescriptorSetLayoutFlags::PUSH_DESCRIPTOR)
    }
}

struct ShaderSetInnerHandle(FutureLock<Arc<ShaderSetInner>, RemoteHandle<Result<Arc<ShaderSetInner>>>>);

#[derive(Clone)]
struct ShaderSetHandle {
    inner: Arc<ShaderSetInnerHandle>,
}

unsafe impl Sync for ShaderSetHandle {}

impl ShaderSetHandle {

    fn new(f: RemoteHandle<Result<Arc<ShaderSetInner>>>) -> Self {
        Self {
            inner: Arc::new(ShaderSetInnerHandle(FutureLock::new(f)))
        }
    }

    #[inline(always)]
    fn load(&self) -> impl Future<Output = Result<&Arc<ShaderSetInner>>> + Send {
        self.inner.0.load()
    }
}

/// Contains the handle and metadata of a [`shader module`][1].
///
/// [1]: https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModule.html
pub struct ShaderModule {
    handle: vk::ShaderModule,
    spirv: ShaderSourceCompiled,
    entry_point: Arc<CStr>,
}

impl ShaderModule { 

    #[inline(always)]
    pub(crate) fn handle(&self) -> vk::ShaderModule {
        self.handle
    }

    /// Gets the shader stage of the module.
    #[inline(always)]
    pub fn stage(&self) -> ShaderStage {
        self.spirv.stage()
    }

    /// Gets the raw spirv code of the shader module.
    #[inline(always)]
    pub fn spirv(&self) -> &[u32] {
        self.spirv.spirv()
    }

    /// Gets the entry point name of the shader module.
    #[inline(always)]
    pub fn entry_point(&self) -> &CStr {
        &self.entry_point
    }
}

pub(crate) struct ShaderSetInner {
    device: Device,
    id: ShaderSetId,
    pipeline_layout: vk::PipelineLayout,
    n_descriptor_set_layouts: u32,
    descriptor_set_layouts: Pointer<DescriptorSetLayout>,
    n_push_constant_ranges: u32,
    push_constant_ranges: Pointer<PushConstantRange>,
    n_shaders: u32,
    shaders: Pointer<ShaderModule>,
    pack_ptr: *mut u8,
    pack_layout: Layout,
    push_descriptor_bindings: AHashMap<Arc<CStr>, (u32, u32)>,
}

unsafe impl Send for ShaderSetInner {}
unsafe impl Sync for ShaderSetInner {}

impl ShaderSetInner {

    #[inline(always)]
    pub(crate) fn new(
        device: Device,
        id: ShaderSetId,
        descriptor_set_layouts: &[DescriptorSetLayout],
        push_constant_ranges: &[PushConstantRange],
        shaders: impl ExactSizeIterator<Item = ShaderModule>,
        pipeline_layout: vk::PipelineLayout,
    ) -> Self {
        let n_descriptor_set_layouts = descriptor_set_layouts.len() as u32;
        let n_push_constant_ranges = push_constant_ranges.len() as u32;
        let n_shaders = shaders.len() as u32;
        let layout;
        let ptr;
        let p_descriptor_set_layouts;
        let p_push_constant_ranges;
        let p_shaders;
        unsafe {
            pack_alloc!(
                layout as Layout,
                ptr as *mut u8,
                p_descriptor_set_layouts
                    as [DescriptorSetLayout; n_descriptor_set_layouts as usize],
                p_push_constant_ranges as [PushConstantRange; n_push_constant_ranges as usize],
                p_shaders as [ShaderModule; n_shaders as usize],
            );
        }
        unsafe {
            Pointer
                ::new_unchecked(descriptor_set_layouts.as_ptr().cast_mut())
                .clone_elements(
                    Pointer::new_unchecked(p_descriptor_set_layouts),
                    n_descriptor_set_layouts,
                );
            Pointer
                ::new_unchecked(push_constant_ranges.as_ptr().cast_mut())
                .clone_elements(
                    Pointer::new_unchecked(p_push_constant_ranges),
                    n_push_constant_ranges
                );
            for (i, shader) in shaders.enumerate() {
                p_shaders.add(i).write(shader);
            }
        };
        let mut push_descriptor_bindings = AHashMap::default();
        for (i, layout) in descriptor_set_layouts.iter().enumerate() {
            let set = i as u32;
            if layout.is_push_descriptor() {
                for (j, binding) in layout.bindings.iter().enumerate() {
                    push_descriptor_bindings
                        .entry(binding.name.clone())
                        .or_insert((set, j as u32));
                }
            }
        }
        unsafe { Self {
            device,
            id,
            n_descriptor_set_layouts,
            descriptor_set_layouts: Pointer::new_unchecked(p_descriptor_set_layouts),
            n_push_constant_ranges,
            push_constant_ranges: Pointer::new_unchecked(p_push_constant_ranges),
            n_shaders,
            shaders: Pointer::new_unchecked(p_shaders),
            pipeline_layout,
            pack_ptr: ptr,
            pack_layout: layout,
            push_descriptor_bindings,
        } }
    } 
}

/// A set of [`shaders`][1] compiled into [`shader modules`][2], [`descriptor set layouts`][3] and
/// a [`pipeline layout`][4].
///
/// Shader sets can be created with [`create_shader_set`][5].
///
/// [1]: Shader
/// [2]: ShaderModule
/// [3]: DescriptorSetLayout
/// [4]: https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayout.html
/// [5]: Gpu::create_shader_set
#[derive(Clone)]
pub struct ShaderSet {
    inner: Arc<ShaderSetInner>,
}

impl ShaderSet {

    #[inline]
    pub fn id(&self) -> ShaderSetId {
        self.inner.id
    }

    #[inline]
    pub fn pipeline_layout(&self) -> vk::PipelineLayout {
        self.inner.pipeline_layout
    }

    #[inline]
    pub fn set_count(&self) -> u32 {
        self.inner.n_descriptor_set_layouts
    }

    #[inline]
    pub fn descriptor_set_layouts(
        &self,
    ) -> &[DescriptorSetLayout]
    {
        unsafe {
            slice::from_raw_parts(
                self.inner.descriptor_set_layouts.as_ptr(),
                self.inner.n_descriptor_set_layouts as usize,
            )
        }
    }

    /// Gets the descriptor set and [`binding`][1] of a [`descriptor set layout`][2] created with
    /// the [`push descriptor flag`][3] set.
    ///
    /// Returns [`None`] if a binding with `name` is not found from a push descriptor set.
    ///
    /// Otherwise returns a tuple containing the set number and [`binding`][1] with `name`.
    ///
    /// [1]: DescriptorSetLayoutBinding
    /// [2]: DescriptorSetLayout
    /// [3]: DescriptorSetLayoutFlags::PUSH_DESCRIPTOR
    #[inline(always)]
    pub fn push_descriptor_binding(
        &self,
        name: &CStr,
    ) -> Option<(u32, &DescriptorSetLayoutBinding)> {
        self.inner.push_descriptor_bindings
            .get(name)
            .map(|&(set, binding_idx)| {
                (set,
                    &self.descriptor_set_layouts()[set as usize]
                    .bindings[binding_idx as usize]
                )
            })
    }

    #[inline(always)]
    pub fn push_constant_ranges(&self) -> &[PushConstantRange] {
        unsafe {
            slice::from_raw_parts(
                self.inner.push_constant_ranges.as_ptr(),
                self.inner.n_push_constant_ranges as usize,
            )
        }
    }

    #[inline(always)]
    pub fn shaders(&self) -> &[ShaderModule] {
        unsafe {
            slice::from_raw_parts(
                self.inner.shaders.as_ptr(),
                self.inner.n_shaders as usize
            )
        }
    }
}

impl Drop for ShaderSetInner {

    fn drop(&mut self) {
        unsafe {
            self.descriptor_set_layouts.drop_in_place(self.n_descriptor_set_layouts as usize);
            self.push_constant_ranges.drop_in_place(self.n_push_constant_ranges as usize);
            for module in slice::from_raw_parts(self.shaders.as_ptr(), self.n_shaders as usize) {
                self.device.destroy_shader_module(module.handle(), None);
            }
            self.shaders.drop_in_place(self.n_shaders as usize);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            alloc::dealloc(self.pack_ptr, self.pack_layout);
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ShaderSetId(SlotIndex<ShaderSetHandle>);

impl_id_display!(ShaderSetId);

#[derive(Clone)]
struct SetAttributes {
    set_flags: DescriptorSetLayoutFlags,
    binding_attributes: Option<Vec32<(u32, DescriptorBindingAttributes)>>
}

#[must_use]
#[derive(Default, Clone)]
pub struct ShaderSetAttributes {
    set_attributes: Vec32<(u32, SetAttributes)>,
    inline_uniform_blocks: Vec32<(u32, u32)>,
}

pub fn default_shader_set_attributes() -> ShaderSetAttributes {
    ShaderSetAttributes {
        set_attributes: vec32![],
        inline_uniform_blocks: vec32![],
    }
}

impl ShaderSetAttributes {

    /// Specifies [`flags`][1] for a specific descriptor set in the shader set.
    ///
    /// # Valid usage
    /// - Only one set with the [`push descriptor`][2] flag **may** be set per shader set.
    ///
    /// [1]: DescriptorSetLayoutFlags
    /// [2]: DescriptorSetLayoutFlags::PUSH_DESCRIPTOR
    #[inline(always)]
    pub fn with_descriptor_set_layout_flags(
        mut self,
        set: u32,
        flags: DescriptorSetLayoutFlags,
    ) -> Self {
        if let Some((_, contained)) = self.set_attributes
            .iter_mut()
            .find(|&&mut(s, _)| set == s)
        {
            contained.set_flags |= flags;
        } else {
            self.set_attributes.push((set, SetAttributes {
                set_flags: flags,
                binding_attributes: None,
            }));
        }
        self
    }

    /// Sets [`descriptor binding attributes`][1] for the given `binding` in `set`.
    ///
    /// This requires enabling the [`descriptor_indexing`][2] extension.
    ///
    /// [1]: DescriptorBindingAttributes
    /// [2]: ext::descriptor_indexing
    pub fn with_descriptor_binding_attribute(
        mut self,
        set: u32,
        binding: u32,
        attributes: DescriptorBindingAttributes,
    ) -> Self {
        if let Some((_, contained)) = self.set_attributes
            .iter_mut()
            .find(|&&mut(s, _)| set == s)
        {
            if let Some(attrs) = &mut contained.binding_attributes {
                attrs.push((binding, attributes));
            } else {
                contained.binding_attributes = Some(vec32![(binding, attributes); 1]);
            }
        } else {
            self.set_attributes.push((set, SetAttributes {
                set_flags: DescriptorSetLayoutFlags::empty(),
                binding_attributes: Some(vec32![(binding, attributes); 1])
            }));
        }
        self
    }


    /// Specifies that a `binding` in set index `set` is an [`inline uniform block`][1].
    ///
    /// # Valid usage
    /// - The [`inline uniform block`][1] extension *must* be enabled.
    /// - The the descriptor set layout flags of `set` *must* not contain the [`push descriptor flag`][2].
    /// - The binding's [`descriptor type`][3] *must* be [`uniform buffer`][4].
    ///
    /// [1]: ext::inline_uniform_block
    /// [2]: DescriptorSetLayoutFlags::PUSH_DESCRIPTOR
    /// [3]: DescriptorType
    /// [4]: DescriptorType::UniformBuffer
    #[inline(always)]
    pub fn with_inline_uniform_block(
        mut self,
        set: u32,
        binding: u32,
    ) -> Self {
        self.inline_uniform_blocks.push((set, binding));
        self
    }
}

#[derive(Clone)]
struct DescriptorSetLayoutKey {
    flags: DescriptorSetLayoutFlags,
    bindings: Vec32<DescriptorSetLayoutBinding>,
    binding_flags: Option<Vec32<vk::DescriptorBindingFlags>>,
    hash: u64,
}

impl DescriptorSetLayoutKey {

    fn new(
        flags: DescriptorSetLayoutFlags,
        bindings: Vec32<DescriptorSetLayoutBinding>,
        binding_flags: Option<Vec32<vk::DescriptorBindingFlags>>,
    ) -> Self {
        let mut hasher = ahash::AHasher::default();
        flags.hash(&mut hasher);
        bindings.hash(&mut hasher);
        binding_flags.hash(&mut hasher);
        let hash = hasher.finish();
        Self {
            flags,
            bindings,
            binding_flags,
            hash,
        }
    }
}

impl PartialEq for DescriptorSetLayoutKey {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags &&
        self.bindings == other.bindings &&
        self.binding_flags == other.binding_flags
    }
}

impl Eq for DescriptorSetLayoutKey {}

impl Hash for DescriptorSetLayoutKey {
    
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

pub(crate) struct ShaderCache {
    device: Device,
    shader_sets: SlotMap<ShaderSetHandle>,
    descriptor_set_layouts: Arc<RwLock<AHashMap<
        DescriptorSetLayoutKey,
        vk::DescriptorSetLayout
    >>>,
}

impl ShaderCache {

    #[inline(always)]
    pub(crate) fn new(device: Device) -> Self {
        Self {
            device,
            shader_sets: Default::default(),
            descriptor_set_layouts: Arc::new(RwLock::new(
                AHashMap::default()
            )),
        }
    } 

    #[inline(always)]
    pub fn create_shader_set<const N_SHADERS: usize>(
        &mut self,
        shaders: [Shader; N_SHADERS],
        mut attributes: ShaderSetAttributes,
        thread_pool: ThreadPool,
        mt_ctx: Arc<MtContext>,
    ) -> Result<ShaderSetId>
    {
        let descriptor_set_layout_cache = self.descriptor_set_layouts.clone();
        let device = self.device.clone();
        let max_push_descriptors = device
            .get_device_attribute(ext::push_descriptor::Attributes::MAX_PUSH_DESCRIPTORS)
            .u32();
        if !attributes.inline_uniform_blocks.is_empty() &&
            !device.get_device_attribute(ext::inline_uniform_block::Attributes::IS_ENABLED)
            .bool().unwrap_or(false)
        {
            return Err(Error::just_context(
                "attempting to use inline uniform blocks without enabling the extension"
            ))
        }
        let index = self.shader_sets.try_insert_with_index(|index| Ok(ShaderSetHandle::new(
            thread_pool.spawn_with_handle(async move {
                let mut shaders_inner = ArrayVec::<_, N_SHADERS>::new();
                let mut all_stage_flags = ShaderStageFlags::empty();
                let mut max_set = 0;
                for shader in &shaders {
                    let inner = shader
                        .inner()
                        .await
                        .context("failed to create shader")?;
                    all_stage_flags |= inner.stage().into();
                    max_set = max_set.max(inner
                        .uniforms()
                        .iter()
                        .map(|u| u.set)
                        .max().unwrap_or(0)
                    );
                    shaders_inner.push(inner);
                }
                let tmp_alloc = mt_ctx.tmp_alloc();
                let tmp_alloc = tmp_alloc.guard();
                #[derive(Default, Clone)]
                struct Set {
                    bindings: Vec32<DescriptorSetLayoutBinding>,
                    binding_attributes: Option<Vec32<(u32, DescriptorBindingAttributes)>>,
                    stage_flags: ShaderStageFlags,
                    flags: DescriptorSetLayoutFlags,
                    inline_ubos: u32,
                }
                let mut sets = FixedVec32::<Set, _>::with_capacity(max_set + 1, &tmp_alloc)
                .context("alloc failed")?;
                let mut per_stage_inline_ubos = FixedVec32::with_capacity(
                    all_stage_flags.count_ones(),
                    &tmp_alloc
                ).context("alloc failed")?;
                for stage in all_stage_flags.bit_iter() {
                    per_stage_inline_ubos.push((ShaderStageFlags::from_raw(stage), 0u32));
                }
                let mut push_constant_ranges = Vec32::new();
                let mut any_binding_attributes = false;
                for shader in &mut shaders_inner {
                    for uniform in shader.uniforms() {
                        let set = uniform.set;
                        if set >= sets.len() {
                            let mut i = sets.len();
                            sets.resize_with(set + 1, || {
                                let mut new_set = Set::default();
                                if let Some((_, attrs)) = attributes.set_attributes
                                    .iter_mut().find(|&&mut(s, _)| {
                                        s == i
                                    })
                                {
                                    new_set.flags = attrs.set_flags;
                                    new_set.binding_attributes = attrs.binding_attributes.take();
                                    if new_set.binding_attributes.is_some() {
                                        any_binding_attributes = true;
                                    }
                                }
                                i += 1;
                                new_set
                            });
                        }
                        let set = unsafe {
                            sets.get_unchecked_mut(set as usize)
                        };
                        set.stage_flags |= shader.stage().into();
                        let inline_uniform_block = attributes.inline_uniform_blocks
                            .contains(&(uniform.set, uniform.binding));
                        set.inline_ubos += inline_uniform_block as u32;
                        if inline_uniform_block {
                            let (_, ubos) = per_stage_inline_ubos
                                .iter_mut()
                                .find(|(stage, _)| stage == &shader.stage().into())
                                .unwrap();
                            *ubos += 1;
                        }
                        let b = uniform.as_layout_binding(
                            inline_uniform_block,
                            |count| {
                                match count {
                                    DescriptorCount::Static(count) => count as u32,
                                    DescriptorCount::Runtime { declared } => {
                                        if let Some(attrs) = &set.binding_attributes &&
                                            let Some((_, attrs)) = attrs
                                                .iter()
                                                .find(|&&(b, _)| b == uniform.binding) &&
                                            let Some(count) = attrs.variable_descriptor_count
                                        {
                                            declared as u32 * count
                                        } else {
                                            declared as u32
                                        }
                                    },
                                }
                            },
                        ).context_with(|| format!(
                            "failed to convert uniform (set {}, binding {})",
                            uniform.set, uniform.binding,
                        ))?;
                        set.bindings.push(b);
                    }
                    for &pc in shader.push_constant_ranges() {
                        push_constant_ranges.push(pc);
                    }
                }
                if let Some(max) = device
                    .get_device_attribute(ext::inline_uniform_block::Attributes
                        ::MAX_PER_STAGE_DESCRIPTOR_INLINE_UNIFORM_BLOCKS
                    ).u32()
                {
                    for (stage, ubos) in per_stage_inline_ubos {
                        if ubos > max {
                            return Err(Error::just_context(format!(
                                "{}{}",
                                format_args!("shader stage {stage} contains {ubos} inline uniform blocks, "),
                                format_args!("but the max per stage inline uniform block count is {max}"),
                            )))
                        }
                    }
                }
                let mut descriptor_set_layouts = FixedVec32
                    ::with_capacity(sets.len(), &tmp_alloc)
                    .context("alloc failed")?;
                let max_descriptor_set_inline_ubos = device
                    .get_device_attribute(ext::inline_uniform_block::Attributes
                        ::MAX_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCKS
                    ).u32().unwrap_or(0);
                if any_binding_attributes &&
                    !device.get_device_attribute(ext::descriptor_indexing::Attributes::IS_ENABLED)
                    .bool().unwrap_or(false)
                {
                    return Err(Error::just_context(
                        "attempting to use descriptor indexing features without enabling the extension"
                    ))
                }
                let mut contains_push_descriptor = false;
                for (i, Set { mut bindings, binding_attributes, stage_flags, flags, inline_ubos }) in
                    sets.into_iter().enumerate()
                {
                    if inline_ubos > max_descriptor_set_inline_ubos {
                        return Err(Error::just_context(format!(
                            "{}{}",
                            format_args!("set {i} contains {inline_ubos} inline uniform blocks, "),
                            format_args!("but the max descriptor set inline uniform block count is {}",
                                max_descriptor_set_inline_ubos
                            ),
                        )))
                    }
                    if inline_ubos != 0 && flags.contains(DescriptorSetLayoutFlags::PUSH_DESCRIPTOR) {
                        return Err(Error::just_context(
                            "inline uniform buffer block descriptor type can't be used for push descriptors"
                        ))
                    }
                    bindings.sort_unstable_by_key(|a| a.binding);
                    let mut prev = bindings.first().map(|b| b.binding).unwrap_or(0);
                    for binding in bindings.iter().skip(1) {
                        if prev == binding.binding {
                            return Err(Error::just_context(format!(
                                "binding {prev} is duplicated"
                            )))
                        }
                        prev = binding.binding;
                    }
                    let mut layout_flags = vk::DescriptorSetLayoutCreateFlags::empty();
                    if flags.contains(DescriptorSetLayoutFlags::PUSH_DESCRIPTOR)
                    {
                        if contains_push_descriptor {
                            return Err(Error::just_context(
                                "more than one push descriptor set found"
                            ))
                        }
                        contains_push_descriptor = true;
                        layout_flags |= vk::DescriptorSetLayoutCreateFlags::PUSH_DESCRIPTOR;
                        let binding_count: u32 = bindings
                            .iter()
                            .map(|binding| binding.descriptor_count)
                            .sum();
                        let Some(max_push_descriptors) = max_push_descriptors else {
                            return Err(Error::just_context(
                                "push descriptor extension is not enabled"
                            ))
                        };
                        if binding_count > max_push_descriptors {
                            return Err(Error::just_context(format!(
                                "{}{}",
                                format_args!(
                                    "descriptor with push descriptor flag has {binding_count} descriptors "),
                                format_args!("when max push descriptors count is {max_push_descriptors}"),
                            )))
                        }
                    }
                    let binding_flags =
                        if let Some(attributes) = binding_attributes {
                            let mut flags = vec32![vk::DescriptorBindingFlags::empty(); bindings.len()];
                            for (binding, attr) in attributes {
                                let index_of = bindings
                                    .iter().enumerate()
                                    .find_map(|(idx, b)| {
                                        (b.binding == binding).then_some(idx)
                                    }).ok_or_else(|| Error::just_context(format!(
                                        "{}{}",
                                        format_args!(
                                            "descriptor binding attributes references binding {binding}, ",
                                        ),
                                        format_args!(
                                            "which is not present in set {i}",
                                        )
                                    )))?;
                                let flags = &mut flags[index_of];
                                if attr.variable_descriptor_count.is_some() {
                                    *flags |= vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT;
                                }
                            }
                            Some(flags)
                        } else {
                            None
                        };
                    let key = DescriptorSetLayoutKey::new(flags, bindings, binding_flags);
                    let mut cache = descriptor_set_layout_cache.write();
                    let handle = cache
                        .get(&key)
                        .copied()
                        .unwrap_or_try_else(|| {
                            let binding_count = key.bindings.len();
                            let mut vk_bindings = FixedVec32::with_capacity(
                                binding_count, &tmp_alloc
                            ).context("alloc failed")?;
                            vk_bindings.extend(key.bindings
                                .iter()
                                .map(|b| vk::DescriptorSetLayoutBinding {
                                    binding: b.binding,
                                    descriptor_type: b.descriptor_type.into(),
                                    descriptor_count: b.descriptor_count,
                                    stage_flags: b.stage_flags.into(),
                                    ..Default::default()
                                }),
                            );
                            let mut create_info = vk::DescriptorSetLayoutCreateInfo {
                                s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
                                flags: layout_flags,
                                binding_count,
                                p_bindings: vk_bindings.as_ptr(),
                                ..Default::default()
                            };
                            let mut binding_info = vk::DescriptorSetLayoutBindingFlagsCreateInfo
                                ::default();
                            if let Some(flags) = &key.binding_flags {
                                binding_info = binding_info
                                    .binding_count(flags.len())
                                    .p_binding_flags(flags.as_ptr());
                                create_info = create_info.push_next(&mut binding_info);
                            }
                            let handle = unsafe {
                                device
                                    .create_descriptor_set_layout(&create_info, None)
                            }.context("failed to create descriptor set layout")?
                            .value;
                            cache.insert(key.clone(), handle);
                            Ok(handle)
                        })?;
                    descriptor_set_layouts.push(DescriptorSetLayout {
                        handle,
                        bindings: key.bindings,
                        stage_flags,
                        flags,
                    });
                }
                let mut vk_set_layouts = FixedVec32
                    ::with_capacity(descriptor_set_layouts.len(), &tmp_alloc)
                    .context("alloc failed")?;
                vk_set_layouts.extend(descriptor_set_layouts
                    .iter()
                    .map(|layout| layout.handle)
                );
                let mut vk_push_constants = FixedVec32
                    ::with_capacity(push_constant_ranges.len(), &tmp_alloc)
                    .context("alloc failed")?;
                vk_push_constants.extend(push_constant_ranges
                    .iter()
                    .map(|&r| r.into())
                );
                let create_info = vk::PipelineLayoutCreateInfo {
                    s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
                    set_layout_count: vk_set_layouts.len(),
                    p_set_layouts: vk_set_layouts.as_ptr(),
                    push_constant_range_count: vk_push_constants.len(),
                    p_push_constant_ranges: vk_push_constants.as_ptr(),
                    ..Default::default()
                };
                let pipeline_layout = unsafe {
                    device
                        .create_pipeline_layout(&create_info, None)
                }.context("failed to create pipeline layout")?.value;
                let mut shader_modules = ArrayVec::<_, N_SHADERS>::new();
                shader_modules.try_extend(
                    shaders_inner.iter_mut().map(|shader| {
                        let info = vk::ShaderModuleCreateInfo {
                            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
                            code_size: size_of_val(shader.source().spirv()),
                            p_code: shader.source().spirv().as_ptr(),
                            ..Default::default()
                        };
                        Ok(unsafe { RaiiHandle::new(
                            device
                                .create_shader_module(&info, None)
                                .context("failed to create shader module")?
                                .value,
                            |module| {
                                device.destroy_shader_module(module, None);
                            }
                        )})
                    }),
                )?;
                Ok(Arc::new(ShaderSetInner::new(
                    device.clone(),
                    ShaderSetId(index),
                    &descriptor_set_layouts,
                    &push_constant_ranges,
                    shader_modules
                        .into_iter()
                        .enumerate()
                        .map(|(i, handle)| {
                            let shader = &shaders_inner[i];
                            ShaderModule {
                                handle: handle.into_inner(),
                                spirv: shader.source().clone(),
                                entry_point: shader.entry_point().into()
                            }
                            
                        }),
                    pipeline_layout,
                )))
            }).context("failed to spawn")?)
        ))?;
        Ok(ShaderSetId(index))
    }

    #[inline(always)]
    pub fn delete_shader_set(&mut self, id: ShaderSetId) {
        self.shader_sets.remove(id.0).ok();
    }

    #[inline(always)]
    pub fn get_shader_set<'a>(&self, id: ShaderSetId) -> impl Future<
        Output = Result<ShaderSet>> + Send + Sync + use<'a
    > {
        let set = self.shader_sets
            .get(id.0)
            .context_with(|| format!(
                "invalid shader set id {id}",
            )).cloned();
        async move {
            set?.load().await
                .cloned()
                .map(|set| ShaderSet { inner: set })
        }
    }
}

impl Drop for ShaderCache {

    fn drop(&mut self) {
        self.shader_sets.clear(); 
        for &layout in self.descriptor_set_layouts.read().values() {
            unsafe {
                self.device.destroy_descriptor_set_layout(layout, None);
            }
        }
    }
}
