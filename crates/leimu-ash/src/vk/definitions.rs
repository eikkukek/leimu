use super::*;

use core::{
    ffi::{c_void, CStr},
    marker::PhantomData,
    fmt,
};

use leimu_proc::Structure;

/// *Extracted from [`ash`] source code.*
///
/// Iterates through the pointer chain. Includes the item that is passed into the function.
/// Stops at the last [`BaseOutStructure`] that has a null [`BaseOutStructure::p_next`] field.
///
/// # Safety
/// Blindly assumes struct layouts to be valid for this.
pub unsafe fn ptr_chain_iter<T: ?Sized>(
    ptr: &mut T,
) -> impl Iterator<Item = *mut BaseOutStructure<'_>> {
    unsafe {
        let ptr = <*mut T>::cast::<BaseOutStructure<'_>>(ptr);
        (0..).scan(ptr, |p_ptr, _| {
            if p_ptr.is_null() {
                return None;
            }
            let n_ptr = (**p_ptr).p_next;
            let old = *p_ptr;
            *p_ptr = n_ptr;
            Some(old)
        })
    }
}

/// *Extracted from [`ash`] source code.*
///
/// Iterates through the pointer chain. Includes the item that is passed into the function.
/// Stops at the last [`BaseOutStructure`] that has a null [`BaseOutStructure::p_next`] field.
///
/// # Safety
/// Blindly assumes struct layouts to be valid for this.
pub unsafe fn ptr_chain_iter_const<T: ?Sized>(
    ptr: &T,
) -> impl Iterator<Item = *const BaseOutStructure<'_>> {
    unsafe {
        let ptr = <*const T>::cast::<BaseOutStructure<'_>>(ptr);
        (0..).scan(ptr, |p_ptr, _| {
            if p_ptr.is_null() {
                return None;
            }
            let n_ptr = (**p_ptr).p_next;
            let old = *p_ptr;
            *p_ptr = n_ptr;
            Some(old)
        })
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkan14Features.html>
///
/// Provided by Vulkan 1.4.
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PhysicalDeviceVulkan14Features<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub global_priority_query: Bool32,
    pub shader_subgroup_rotate: Bool32,
    pub shader_subgroup_rotate_clustered: Bool32,
    pub shader_float_controls2: Bool32,
    pub shader_expect_assume: Bool32,
    pub rectangular_lines: Bool32,
    pub bresenham_lines: Bool32,
    pub smooth_lines: Bool32,
    pub stippled_rectangular_lines: Bool32,
    pub stippled_bresenham_lines: Bool32,
    pub stippled_smooth_lines: Bool32,
    pub vertex_attribute_instance_rate_divisor: Bool32,
    pub vertex_attribute_instance_rate_zero_divisor: Bool32,
    pub index_type_uint8: Bool32,
    pub dynamic_rendering_local_read: Bool32,
    pub maintenance5: Bool32,
    pub maintenance6: Bool32,
    pub pipeline_protected_access: Bool32,
    pub pipeline_robustness: Bool32,
    pub host_image_copy: Bool32,
    pub push_descriptor: Bool32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PhysicalDeviceVulkan14Features<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_VULKAN_1_4_FEATURES;
}

unsafe impl ExtendsPhysicalDeviceFeatures2 for PhysicalDeviceVulkan14Features<'_> {}
unsafe impl ExtendsDeviceCreateInfo for PhysicalDeviceVulkan14Features<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineRobustnessFeatures.html>
///
/// Provided by Vulkan 1.4.
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PhysicalDevicePipelineRobustnessFeatures<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_robustness: Bool32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PhysicalDevicePipelineRobustnessFeatures<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES;
}

unsafe impl ExtendsPhysicalDeviceFeatures2 for PhysicalDevicePipelineRobustnessFeatures<'_> {}
unsafe impl ExtendsDeviceCreateInfo for PhysicalDevicePipelineRobustnessFeatures<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineRobustnessFeatures.html>
///
/// Provided by VK_EXT_pipeline_robustness.
pub type PhysicalDevicePipelineRobustnessFeaturesEXT<'a> = PhysicalDevicePipelineRobustnessFeatures<'a>;

#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PhysicalDevicePipelineRobustnessProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub default_robustness_storage_buffers: PipelineRobustnessBufferBehavior,
    pub default_robustness_uniform_buffers: PipelineRobustnessBufferBehavior,
    pub default_robustness_vertex_inputs: PipelineRobustnessBufferBehavior,
    pub default_robustness_images: PipelineRobustnessImageBehavior,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PhysicalDevicePipelineRobustnessProperties<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES;
}

unsafe impl ExtendsPhysicalDeviceProperties2 for PhysicalDevicePipelineRobustnessProperties<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineRobustnessProperties.html>
///
/// Provided by VK_EXT_pipeline_robustness.
pub type PhysicalDevicePipelineRobustnessPropertiesEXT<'a> = PhysicalDevicePipelineRobustnessProperties<'a>;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessCreateInfo.html>
///
/// Provided by Vulkan 1.4.
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PipelineRobustnessCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub storage_buffers: PipelineRobustnessBufferBehavior,
    pub uniform_buffers: PipelineRobustnessBufferBehavior,
    pub vertex_input: PipelineRobustnessBufferBehavior,
    pub images: PipelineRobustnessImageBehavior,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PipelineRobustnessCreateInfo<'_> {
    
    const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_ROBUSTNESS_CREATE_INFO;
}

unsafe impl ExtendsGraphicsPipelineCreateInfo for PipelineRobustnessCreateInfo<'_> {}
unsafe impl ExtendsComputePipelineCreateInfo for PipelineRobustnessCreateInfo<'_> {}
unsafe impl ExtendsPipelineShaderStageCreateInfo for PipelineRobustnessCreateInfo<'_> {}
unsafe impl ExtendsRayTracingPipelineCreateInfoKHR for PipelineRobustnessCreateInfo<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessCreateInfo.html>
///
/// Provided by VK_EXT_pipeline_robustness.
pub type PipelineRobustnessCreateInfoEXT<'a> = PipelineRobustnessCreateInfo<'a>;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePushDescriptorProperties.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PhysicalDevicePushDescriptorProperties<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_push_descriptors: u32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PhysicalDevicePushDescriptorProperties<'_> {

    const STRUCTURE_TYPE: StructureType
        = StructureType::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES;
}

unsafe impl ExtendsPhysicalDeviceProperties2 for PhysicalDevicePushDescriptorProperties<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePushDescriptorProperties.html>
///
/// Provided by VK_KHR_push_descriptor.
pub type PhysicalDevicePushDescriptorPropertiesKHR<'a> = PhysicalDevicePushDescriptorProperties<'a>;

/// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance5Features.html
///
/// Provided by Vulkan 1.4.
pub type PhysicalDeviceMaintenance5Features<'a> = PhysicalDeviceMaintenance5FeaturesKHR<'a>;
/// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance6Features.html
///
/// Provided by Vulkan 1.4.
pub type PhysicalDeviceMaintenance6Features<'a> = PhysicalDeviceMaintenance6FeaturesKHR<'a>;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceIndexTypeUint8Features.html>
///
/// Provided by Vulkan 1.4.
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PhysicalDeviceIndexTypeUint8Features<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub index_type_uint8: Bool32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PhysicalDeviceIndexTypeUint8Features<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES;
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceIndexTypeUint8Features.html>
///
/// Provided by VK_KHR_index_type_uint8.
pub type PhysicalDeviceIndexTypeUint8FeaturesKHR<'a> = PhysicalDeviceIndexTypeUint8Features<'a>;

unsafe impl ExtendsPhysicalDeviceFeatures2 for PhysicalDeviceIndexTypeUint8Features<'_> {}
unsafe impl ExtendsDeviceCreateInfo for PhysicalDeviceIndexTypeUint8Features<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentLocationInfo.html>
///
/// Provided by Vulkan 1.4.
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct RenderingAttachmentLocationInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_locations: *const u32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for RenderingAttachmentLocationInfo<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::RENDERING_ATTACHMENT_LOCATION_INFO;
}

unsafe impl ExtendsGraphicsPipelineCreateInfo for RenderingAttachmentLocationInfo<'_> {}
unsafe impl ExtendsCommandBufferInheritanceInfo for RenderingAttachmentLocationInfo<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentLocationInfo.html>
///
/// Provided by VK_KHR_dynamic_rendering_local_read.
pub type RenderingAttachmentLocationInfoKHR<'a> = RenderingAttachmentLocationInfo<'a>;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingInputAttachmentIndexInfo.html>
///
/// Provided by of Vulkan 1.4.
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct RenderingInputAttachmentIndexInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_input_indices: *const u32,
    pub p_depth_input_attachment_index: *const u32,
    pub p_stencil_input_attachment_index: *const u32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for RenderingInputAttachmentIndexInfo<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::RENDERING_INPUT_ATTACHMENT_INDEX_INFO;
}

unsafe impl ExtendsGraphicsPipelineCreateInfo for RenderingInputAttachmentIndexInfo<'_> {}
unsafe impl ExtendsCommandBufferInheritanceInfo for RenderingInputAttachmentIndexInfo<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingInputAttachmentIndexInfo.html>
///
/// Provided by VK_KHR_dynamic_rendering_local_read.
pub type RenderingInputAttachmentIndexInfoKHR<'a> = RenderingInputAttachmentIndexInfo<'a>;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDynamicRenderingLocalReadFeatures.html>
///
/// Provided by Vulkan 1.4.
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PhysicalDeviceDynamicRenderingLocalReadFeatures<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dynamic_rendering_local_read: Bool32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PhysicalDeviceDynamicRenderingLocalReadFeatures<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES;
}

unsafe impl ExtendsPhysicalDeviceFeatures2 for PhysicalDeviceDynamicRenderingLocalReadFeatures<'_> {}
unsafe impl ExtendsDeviceCreateInfo for PhysicalDeviceDynamicRenderingLocalReadFeatures<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDynamicRenderingLocalReadFeatures.html>
///
/// Provided by VK_KHR_dynamic_rendering_local_read.
pub type PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR<'a> = PhysicalDeviceDynamicRenderingLocalReadFeatures<'a>;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html>
///
/// Provided by Vulkan 1.4.
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRenderingAttachmentLocations = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_location_info: *const RenderingAttachmentLocationInfo,
);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html>
///
/// Provided by VK_KHR_dynamic_rendering_local_read.
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = PFN_vkCmdSetRenderingAttachmentLocations;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html>
///
/// Provided by Vulkan 1.4.
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRenderingInputAttachmentIndices = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html>
///
/// Provided by VK_KHR_dynamic_rendering_local_read.
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR = PFN_vkCmdSetRenderingInputAttachmentIndices;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags2CreateInfo.html>
///
/// Provided by Vulkan 1.4.
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PipelineCreateFlags2CreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags2,
    pub _marker: PhantomData<&'a ()>,
}

/// Provided by VK_KHR_maintenance5.
pub type PipelineCreateFlags2CreateInfoKHR<'a> 
    = PipelineCreateFlags2CreateInfo<'a>;

unsafe impl TaggedStructure for PipelineCreateFlags2CreateInfo<'_> {

    const STRUCTURE_TYPE: StructureType
        = StructureType::PIPELINE_CREATE_FLAGS_2_CREATE_INFO;
}

unsafe impl ExtendsGraphicsPipelineCreateInfo for PipelineCreateFlags2CreateInfo<'_> {}
unsafe impl ExtendsComputePipelineCreateInfo for PipelineCreateFlags2CreateInfo<'_> {}
unsafe impl ExtendsRayTracingPipelineCreateInfoKHR for PipelineCreateFlags2CreateInfo<'_> {}
unsafe impl ExtendsRayTracingPipelineCreateInfoNV for PipelineCreateFlags2CreateInfo<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBindDescriptorSetsInfo.html>
///
/// Provided by Vulkan 1.4.
pub type BindDescriptorSetsInfo<'a> = BindDescriptorSetsInfoKHR<'a>;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPushConstantsInfo.html>
///
/// Provided by Vulkan 1.4.
pub type PushConstantsInfo<'a> = PushConstantsInfoKHR<'a>;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html>
///
/// Provided by Vulkan 1.4.
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorSets2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html>
///
/// Provided by Vulkan 1.4.
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushConstants2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_push_constants_info: *const PushConstantsInfo,
);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html>
///
/// Provided by Vulkan 1.4
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSet = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet,
);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPushDescriptorSetInfo.html>
///
/// Provided by Vulkan 1.4.
pub type PushDescriptorSetInfo<'a> = PushDescriptorSetInfoKHR<'a>;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html>
///
/// Provided by of Vulkan 1.4.
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSet2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_push_descriptor_set_info: *const PushDescriptorSetInfo<'_>,
);

/// VK_KHR_present_id2 name.
pub const KHR_PRESENT_ID_2_NAME: &CStr = c"VK_KHR_present_id2";
/// VK_KHR_present_id2 spec version.
pub const KHR_PRESENT_ID_2_SPEC_VERSION: u32 = 1;

/// VK_KHR_present_wait2 name.
pub const KHR_PRESENT_WAIT_2_NAME: &CStr = c"VK_KHR_present_wait2";
/// VK_KHR_present_wait2 spec version.
pub const KHR_PRESENT_WAIT_2_SPEC_VERSION: u32 = 1;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCapabilitiesPresentId2KHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct SurfaceCapabilitiesPresentId2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id2_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for SurfaceCapabilitiesPresentId2KHR<'_> {

    const STRUCTURE_TYPE: ash::vk::StructureType =
        StructureType::SURFACE_CAPABILITIES_PRESENT_ID_2_KHR;
}

unsafe impl ExtendsSurfaceCapabilities2KHR for SurfaceCapabilitiesPresentId2KHR<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentId2KHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PresentId2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_present_ids: *const u64,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PresentId2KHR<'_> {
    
    const STRUCTURE_TYPE: ash::vk::StructureType =
        StructureType::PRESENT_ID_2_KHR;
}

unsafe impl ExtendsPresentInfoKHR for PresentId2KHR<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentId2FeaturesKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PhysicalDevicePresentId2FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id2: Bool32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PhysicalDevicePresentId2FeaturesKHR<'_> {

    const STRUCTURE_TYPE: ash::vk::StructureType =
        StructureType::PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR;
}

unsafe impl ExtendsPhysicalDeviceFeatures2 for PhysicalDevicePresentId2FeaturesKHR<'_> {}
unsafe impl ExtendsDeviceCreateInfo for PhysicalDevicePresentId2FeaturesKHR<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCapabilitiesPresentWait2KHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct SurfaceCapabilitiesPresentWait2KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_wait2_supported: Bool32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for SurfaceCapabilitiesPresentWait2KHR<'_> {

    const STRUCTURE_TYPE: StructureType =
        StructureType::SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR;
}

unsafe impl ExtendsSurfaceCapabilities2KHR for SurfaceCapabilitiesPresentWait2KHR<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentWait2FeaturesKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PhysicalDevicePresentWait2FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_wait2: Bool32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PhysicalDevicePresentWait2FeaturesKHR<'_> {

    const STRUCTURE_TYPE: StructureType =
        StructureType::PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR;
}

unsafe impl ExtendsPhysicalDeviceFeatures2 for PhysicalDevicePresentWait2FeaturesKHR<'_> {}
unsafe impl ExtendsDeviceCreateInfo for PhysicalDevicePresentWait2FeaturesKHR<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentWait2InfoKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PresentWait2InfoKHR<'a> { pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_id: u64,
    pub timeout: u64,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PresentWait2InfoKHR<'_> {

    const STRUCTURE_TYPE: StructureType =
        StructureType::PRESENT_WAIT_INFO_2_KHR;
}

/// Extension trait for [`SwapchainCreateFlagsKHR`].
///
/// New values:
///
/// [`SwapchainCreateFlagsKHR::PRESENT_ID_2`]
/// [`SwapchainCreateFlagsKHR::PRESENT_WAIT_2`]
pub trait SwapchainCreateFlagsKHR2 {

    /// Provided by VK_KHR_present_id2.
    const PRESENT_ID_2: Self;
    /// Provided by VK_KHR_present_wait2.
    const PRESENT_WAIT_2: Self;
}

impl SwapchainCreateFlagsKHR2 for SwapchainCreateFlagsKHR {

    const PRESENT_ID_2: Self = Self::from_raw(0x00000040);
    const PRESENT_WAIT_2: Self = Self::from_raw(0x00000080);
}

/// Provided by VK_KHR_present_wait2.
#[allow(non_camel_case_types)]
pub type PFN_vkWaitForPresent2KHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_present_wait2_info: *const PresentWait2InfoKHR,
) -> Result;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html>
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    index_type: super::enums::IndexType,
);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html>
///
/// Provided by Vulkan 1.4.
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindIndexBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: super::enums::IndexType,
);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html>
///
/// Provided by VK_KHR_maintenance5.
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindIndexBuffer2KHR = PFN_vkCmdBindIndexBuffer2;

/// VK_KHR_pipeline_binary name.
pub const KHR_PIPELINE_BINARY_NAME: &CStr = c"VK_KHR_pipeline_binary";
/// VK_KHR_pipeline_binary spec version.
pub const KHR_PIPELINE_BINARY_SPEC_VERSION: u32 = 1;

ash::handle_nondispatchable!(
    PipelineBinaryKHR,
    PIPELINE_BINARY_KHR,
    doc = "Provided by VK_KHR_pipeline_binary\n\n<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKHR.html>"
);

pub const MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKeyKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PipelineBinaryKeyKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub key_size: u32,
    pub key: [u8; MAX_PIPELINE_BINARY_KEY_SIZE_KHR as usize],
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PipelineBinaryKeyKHR<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_BINARY_KEY_KHR;
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryDataKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PipelineBinaryDataKHR {
    pub data_size: usize,
    pub p_data: *mut c_void,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKeysAndDataKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PipelineBinaryKeysAndDataKHR<'a> {
    pub binary_count: u32,
    pub p_pipeline_binary_keys: *const PipelineBinaryKeyKHR<'a>,
    pub p_pipeline_binary_data: *const PipelineBinaryDataKHR,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateInfoKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PipelineCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PipelineCreateInfoKHR<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_CREATE_INFO_KHR;
}

/// # Safety
/// You should only implement this if the structure truly extends [`PipelineCreateInfoKHR`].
pub unsafe trait ExtendsPipelineCreateInfoKHR {}

unsafe impl ExtendsPipelineCreateInfoKHR for GraphicsPipelineCreateInfo<'_> {}
unsafe impl ExtendsPipelineCreateInfoKHR for ExecutionGraphPipelineCreateInfoAMDX<'_> {}
unsafe impl ExtendsPipelineCreateInfoKHR for RayTracingPipelineCreateInfoKHR<'_> {}
unsafe impl ExtendsPipelineCreateInfoKHR for ComputePipelineCreateInfo<'_> {}

impl<'a> PipelineCreateInfoKHR<'a> {

    /// Prepends the given extension struct between the root and the first pointer.
    /// This method only exists on structs that can be passed to a function directly.
    /// Only valid extension structs can be pushed into the chain. If the chain
    /// looks like A -> B -> C, and you call x.push_next(&mut D),
    /// then the chain will look like A -> D -> B -> C.
    #[inline(always)]
    pub fn push_next<T: ExtendsPipelineCreateInfoKHR + ?Sized>(
        mut self,
        next: &'a mut T,
    ) -> Self {
        unsafe {
            let next_ptr = <*const T>::cast(next);
            let last_next = ptr_chain_iter(next).last().unwrap();
            (*last_next).p_next = self.p_next as _;
            self.p_next = next_ptr;
        }
        self
    }
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryCreateInfoKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PipelineBinaryCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_keys_and_data_info: *const PipelineBinaryKeysAndDataKHR<'a>,
    pub pipeline: Pipeline,
    pub pipeline_create_info: *const PipelineCreateInfoKHR<'a>,
}

unsafe impl TaggedStructure for PipelineBinaryCreateInfoKHR<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_BINARY_CREATE_INFO_KHR;
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryHandlesInfoKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PipelineBinaryHandlesInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_binary_count: u32,
    pub p_pipeline_binaries: *mut PipelineBinaryKHR,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PipelineBinaryHandlesInfoKHR<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_BINARY_HANDLES_INFO_KHR;
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryDataInfoKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PipelineBinaryDataInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_binary: PipelineBinaryKHR,
    pub _marker: PhantomData<&'a ()>
}

unsafe impl TaggedStructure for PipelineBinaryDataInfoKHR<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::PIPELINE_BINARY_DATA_INFO_KHR;
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkReleaseCapturedPipelineDataInfoKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct ReleaseCapturedPipelineDataInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline: Pipeline,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for ReleaseCapturedPipelineDataInfoKHR<'_> {

    const STRUCTURE_TYPE: StructureType = StructureType::RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR;
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDevicePipelineBinaryInternalCacheControlKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct DevicePipelineBinaryInternalCacheControlKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disable_internal_cache: Bool32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for DevicePipelineBinaryInternalCacheControlKHR<'_> {

    const STRUCTURE_TYPE: StructureType
        = StructureType::DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR;
}

unsafe impl ExtendsDeviceCreateInfo for DevicePipelineBinaryInternalCacheControlKHR<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryInfoKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PipelineBinaryInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub binary_count: u32,
    pub p_pipeline_binaries: *const PipelineBinaryKHR,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PipelineBinaryInfoKHR<'_> {

    const STRUCTURE_TYPE: ash::vk::StructureType = StructureType::PIPELINE_BINARY_INFO_KHR;
}

unsafe impl ExtendsGraphicsPipelineCreateInfo for PipelineBinaryInfoKHR<'_> {}
unsafe impl ExtendsComputePipelineCreateInfo for PipelineBinaryInfoKHR<'_> {}
unsafe impl ExtendsRayTracingPipelineCreateInfoKHR for PipelineBinaryInfoKHR<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineBinaryFeaturesKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PhysicalDevicePipelineBinaryFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_binaries: Bool32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PhysicalDevicePipelineBinaryFeaturesKHR<'_> {

    const STRUCTURE_TYPE: StructureType
        = StructureType::PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR;
}

unsafe impl ExtendsPhysicalDeviceFeatures2 for PhysicalDevicePipelineBinaryFeaturesKHR<'_> {}
unsafe impl ExtendsDeviceCreateInfo for PhysicalDevicePipelineBinaryFeaturesKHR<'_> {}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineBinaryPropertiesKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Structure)]
pub struct PhysicalDevicePipelineBinaryPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_binary_internal_cache: Bool32,
    pub pipeline_binary_internal_cache_control: Bool32,
    pub pipeline_binary_prefers_internal_cache: Bool32,
    pub pipeline_binary_precompiled_internal_cache: Bool32,
    pub pipeline_binary_compressed_data: Bool32,
    pub _marker: PhantomData<&'a ()>,
}

unsafe impl TaggedStructure for PhysicalDevicePipelineBinaryPropertiesKHR<'_> {

    const STRUCTURE_TYPE: StructureType
        = StructureType::PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR;
}

unsafe impl ExtendsPhysicalDeviceProperties2 for PhysicalDevicePipelineBinaryPropertiesKHR<'_> {}

/// Provided by VK_KHR_pipeline_binary.
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePipelineBinariesKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PipelineBinaryCreateInfoKHR<>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_binaries: *mut PipelineBinaryHandlesInfoKHR<'_>,
) -> Result;

/// Provided by VK_KHR_pipeline_binary.
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipelineBinaryKHR = unsafe extern "system" fn(
    device: Device,
    pipeline_binary: PipelineBinaryKHR,
    allocator: *const AllocationCallbacks<'_>,
);

/// Provided by VK_KHR_pipeline_binary.
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineBinaryDataKHR = unsafe extern "system" fn(
    device: Device,
    p_info: *const PipelineBinaryDataInfoKHR<'_>,
    p_pipeline_binary_key: *mut PipelineBinaryKeyKHR<'_>,
    p_pipeline_binary_data_size: *mut usize,
    p_pipeline_binary_data: *mut c_void,
) -> Result;

/// Provided by VK_KHR_pipeline_binary.
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineKeyKHR = unsafe extern "system" fn (
    device: Device,
    p_pipeline_create_info: *const PipelineCreateInfoKHR<'_>,
    p_pipeline_key: *mut PipelineBinaryKeyKHR,
) -> Result;

/// Provided by VK_KHR_pipeline_binary.
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseCapturedPipelineDataKHR = unsafe extern "system" fn (
    device: Device,
    p_info: *const ReleaseCapturedPipelineDataInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>
) -> Result;
