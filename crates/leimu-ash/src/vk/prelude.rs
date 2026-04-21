pub use ash::vk::*;
pub use super::definitions::*;

use core::ops::{Deref, DerefMut};

use paste::paste;

/// Trait for creating Vulkan `p_next` chains.
///
/// # Safety
/// You should not implement this trait yourself.
pub unsafe trait ExtendsStructure<What: ?Sized>: Send + Sync {

    /// Extracts the structure type field of self.
    fn s_type(&self) -> StructureType;

    /// Clears the `p_next` field of self.
    ///
    /// This along with [`ExtendsStructure::as_mut`] can be used to dynamically and safely build
    /// p_next chains of Vulkan structures.
    fn clear_p_next(&mut self);

    /// Gets a mutable reference to self as the underlying *Extends...* trait. For example, if this
    /// extends [`PhysicalDeviceFeatures2`] this returns a mutable reference to
    /// [`ExtendsPhysicalDeviceFeatures2`] trait object.
    ///
    /// This along with [`ExtendsStructure::clear_p_next`] can be used to dynamically and safely
    /// build p_next chains of Vulkan structures.
    fn as_mut(&mut self) -> &mut What;

    /// Clones self to a box.
    fn boxed(&self) -> Box<dyn ExtendsStructure<What>>;

    /// Gets a pointer to the underlying structure.
    fn as_ptr(&self) -> *const ();
}

/// A non dyn-compatible extension trait for [`ExtendsStructure`], that can be used to create
/// [`ExtendsStructure`] trait objects.
///
/// # Safety
/// You should not implement this trait yourself.
pub unsafe trait ExtendsStructureExt<What: ?Sized>:
    ExtendsStructure<What> + TaggedStructure + Sized
{

    type ExtendsObj;

    /// Boxes [`Self::default`] as an Extends* trait object.
    fn obj_default() -> Self::ExtendsObj;

    /// Boxes [`self`] as an Extends* trait object.
    fn into_obj(self) -> Self::ExtendsObj;

    /// Makes getting a reference to self easier.
    ///
    /// # Safety
    /// Only checks if the structure type of `obj` matches [`self`].
    unsafe fn as_ref(obj: &Self::ExtendsObj) -> Option<&Self>;
}

pub trait Extendable<'a> {

    type Obj: ?Sized;

    #[must_use]
    fn extend_next<T>(self, chain: impl IntoIterator<Item = &'a mut T>) -> Self
        where T: DerefMut<Target = dyn ExtendsStructure<Self::Obj>> + 'a;
}

macro_rules! impl_extends_structure_obj {
    ($($name:ident),+ $(,)?) => {
        $(paste! {
    
            /// A trait object that can be used to extend Vulkan structures.
            pub struct [<Extends $name Obj>](
                Box<dyn ExtendsStructure<dyn [<Extends $name>]>>
            );

            impl [<Extends $name Obj>] {

                #[inline(always)]
                pub fn new(
                    s: impl ExtendsStructure<dyn [<Extends $name>]> + 'static,
                ) -> Self {
                    Self(Box::new(s))
                }
            }

            impl Deref for [<Extends $name Obj>] {

                type Target = dyn ExtendsStructure<dyn [<Extends $name>]>;

                #[inline(always)]
                fn deref(&self) -> &Self::Target {
                    &*self.0
                }
            }

            impl DerefMut for [<Extends $name Obj>] {

                #[inline(always)]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut *self.0
                }
            }

            impl Clone for [<Extends $name Obj>] {
                
                #[inline(always)]
                fn clone(&self) -> Self {
                    Self(self.0.boxed())
                }
            }

            unsafe impl<T> ExtendsStructure<dyn [<Extends $name>]> for T
                where T: [<Extends $name>] + Copy + Send + Sync + 'static
            {

                #[inline(always)]
                fn s_type(&self) -> StructureType {
                    unsafe {
                        self.as_ptr()
                        .cast::<StructureType>()
                        .read()
                    }
                }

                #[inline(always)]
                fn clear_p_next(&mut self) {
                    unsafe {
                        self.as_ptr()
                        .cast::<u8>()
                        .add(size_of::<usize>())
                        .cast::<*mut core::ffi::c_void>()
                        .cast_mut()
                        .write(core::ptr::null_mut())
                    }
                }

                #[inline(always)]
                fn as_mut(&mut self) -> &mut (dyn [<Extends $name>] + 'static) {
                    self
                }

                #[inline(always)]
                fn boxed(&self) -> Box<dyn ExtendsStructure<dyn [<Extends $name>]>> {
                    Box::new(*self)
                }

                #[inline(always)]
                fn as_ptr(&self) -> *const () {
                    (self as *const Self).cast()
                }
            }

            unsafe impl<T> ExtendsStructureExt<dyn [<Extends $name>]> for T
                where T:
                    [<Extends $name>] + Default +
                    Copy + Send + Sync +
                    TaggedStructure + 'static
            {

                paste! {

                    type ExtendsObj = [<Extends $name Obj>];
                    
                    #[inline(always)]
                    fn obj_default() -> Self::ExtendsObj {
                        Self::ExtendsObj::new(Self::default())
                    }

                    #[inline(always)]
                    fn into_obj(self) -> Self::ExtendsObj {
                        Self::ExtendsObj::new(self)
                    }

                    #[inline(always)]
                    unsafe fn as_ref(obj: &Self::ExtendsObj) -> Option<&Self> {
                        (obj.s_type() == Self::STRUCTURE_TYPE).then(|| unsafe {
                            obj.as_ptr()
                            .cast::<Self>()
                            .as_ref().unwrap()
                        })
                    }
                }
            }

            impl<'a> Extendable<'a> for $name<'a> {

                type Obj = dyn [<Extends $name>];

                fn extend_next<T>(mut self, chain: impl IntoIterator<Item = &'a mut T>) -> Self
                    where T: DerefMut<Target = dyn ExtendsStructure<Self::Obj>> + 'a
                {
                    for t in chain.into_iter() {
                        self = self.push_next(t.as_mut());
                    }
                    self
                }
            }
        })+
    };
}

impl_extends_structure_obj!(
    AccelerationStructureCreateInfoKHR,
    AccelerationStructureCreateInfoNV,
    AccelerationStructureGeometryTrianglesDataKHR,
    AndroidHardwareBufferPropertiesANDROID,
    AttachmentDescription2,
    AttachmentReference2,
    BindBufferMemoryInfo,
    BindDescriptorBufferEmbeddedSamplersInfoEXT,
    BindDescriptorSetsInfoKHR,
    BindImageMemoryInfo,
    BindSparseInfo,
    BlitImageInfo2,
    BufferCreateInfo,
    BufferImageCopy2,
    BufferMemoryBarrier,
    BufferMemoryBarrier2,
    BufferViewCreateInfo,
    CommandBufferBeginInfo,
    CommandBufferInheritanceInfo,
    CommandBufferSubmitInfo,
    ComputePipelineCreateInfo,
    DebugUtilsMessengerCallbackDataEXT,
    DepthBiasInfoEXT,
    DescriptorBufferBindingInfoEXT,
    DescriptorPoolCreateInfo,
    DescriptorSetAllocateInfo,
    DescriptorSetLayoutCreateInfo,
    DescriptorSetLayoutSupport,
    DeviceCreateInfo,
    DeviceQueueCreateInfo,
    EventCreateInfo,
    ExecutionGraphPipelineCreateInfoAMDX,
    ExportMetalObjectsInfoEXT,
    FenceCreateInfo,
    FormatProperties2,
    FramebufferCreateInfo,
    GraphicsPipelineCreateInfo,
    ImageBlit2,
    ImageCreateInfo,
    ImageFormatProperties2,
    ImageMemoryBarrier,
    ImageMemoryBarrier2,
    ImageMemoryRequirementsInfo2,
    ImageViewCreateInfo,
    InstanceCreateInfo,
    MemoryAllocateInfo,
    MemoryMapInfoKHR,
    MemoryRequirements2,
    OpticalFlowSessionCreateInfoNV,
    PhysicalDeviceClusterCullingShaderFeaturesHUAWEI,
    PhysicalDeviceExternalBufferInfo,
    PhysicalDeviceExternalSemaphoreInfo,
    PhysicalDeviceFeatures2,
    PhysicalDeviceImageFormatInfo2,
    PhysicalDeviceMemoryProperties2,
    PhysicalDeviceProperties2,
    PhysicalDeviceSurfaceInfo2KHR,
    PhysicalDeviceVideoFormatInfoKHR,
    PipelineColorBlendStateCreateInfo,
    PipelineMultisampleStateCreateInfo,
    PipelineRasterizationStateCreateInfo,
    PipelineShaderStageCreateInfo,
    PipelineTessellationStateCreateInfo,
    PipelineVertexInputStateCreateInfo,
    PipelineViewportStateCreateInfo,
    PresentInfoKHR,
    PushConstantsInfoKHR,
    PushDescriptorSetInfoKHR,
    PushDescriptorSetWithTemplateInfoKHR,
    QueryPoolCreateInfo,
    QueueFamilyProperties2,
    RayTracingPipelineCreateInfoKHR,
    RayTracingPipelineCreateInfoNV,
    RenderPassBeginInfo,
    RenderPassCreateInfo,
    RenderPassCreateInfo2,
    RenderingInfo,
    SamplerCreateInfo,
    SamplerYcbcrConversionCreateInfo,
    ScreenBufferPropertiesQNX,
    SemaphoreCreateInfo,
    SetDescriptorBufferOffsetsInfoEXT,
    ShaderCreateInfoEXT,
    ShaderModuleCreateInfo,
    SubmitInfo,
    SubmitInfo2,
    SubpassDependency2,
    SubpassDescription2,
    SubpassEndInfo,
    SubresourceLayout2KHR,
    SurfaceCapabilities2KHR,
    SurfaceFormat2KHR,
    SwapchainCreateInfoKHR,
    VideoBeginCodingInfoKHR,
    VideoCapabilitiesKHR,
    VideoCodingControlInfoKHR,
    VideoDecodeInfoKHR,
    VideoEncodeInfoKHR,
    VideoEncodeQualityLevelPropertiesKHR,
    VideoEncodeRateControlLayerInfoKHR,
    VideoEncodeSessionParametersFeedbackInfoKHR,
    VideoEncodeSessionParametersGetInfoKHR,
    VideoProfileInfoKHR,
    VideoReferenceSlotInfoKHR,
    VideoSessionCreateInfoKHR,
    VideoSessionParametersCreateInfoKHR,
    VideoSessionParametersUpdateInfoKHR,
    WriteDescriptorSet,
    PipelineCreateInfoKHR,
);
