use super::*;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkClearColorValue.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union ClearColorValue {
    pub float32: [f32; 4usize],
    pub int32: [i32; 4usize],
    pub uint32: [u32; 4usize],
}
impl Default for ClearColorValue {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkClearValue.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}
impl Default for ClearValue {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureOpInputNV.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union ClusterAccelerationStructureOpInputNV<'a> {
    pub p_clusters_bottom_level: *mut ClusterAccelerationStructureClustersBottomLevelInputNV<'a>,
    pub p_triangle_clusters: *mut ClusterAccelerationStructureTriangleClusterInputNV<'a>,
    pub p_move_objects: *mut ClusterAccelerationStructureMoveObjectsInputNV<'a>,
}
impl<'a> Default for ClusterAccelerationStructureOpInputNV<'a> {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterResultKHR.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union PerformanceCounterResultKHR {
    pub int32: i32,
    pub int64: i64,
    pub uint32: u32,
    pub uint64: u64,
    pub float32: f32,
    pub float64: f64,
}
impl Default for PerformanceCounterResultKHR {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceValueDataINTEL.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union PerformanceValueDataINTEL {
    pub value32: u32,
    pub value64: u64,
    pub value_float: f32,
    pub value_bool: Bool32,
    pub value_string: *const ffi::c_char,
}
impl Default for PerformanceValueDataINTEL {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineExecutableStatisticValueKHR.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union PipelineExecutableStatisticValueKHR {
    pub b32: Bool32,
    pub i64: i64,
    pub u64: u64,
    pub f64: f64,
}
impl Default for PipelineExecutableStatisticValueKHR {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceOrHostAddressKHR.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DeviceOrHostAddressKHR {
    pub device_address: DeviceAddress,
    pub host_address: *mut ffi::c_void,
}
impl Default for DeviceOrHostAddressKHR {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceOrHostAddressConstKHR.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DeviceOrHostAddressConstKHR {
    pub device_address: DeviceAddress,
    pub host_address: *const ffi::c_void,
}
impl Default for DeviceOrHostAddressConstKHR {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceOrHostAddressConstAMDX.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DeviceOrHostAddressConstAMDX {
    pub device_address: DeviceAddress,
    pub host_address: *const ffi::c_void,
}
impl Default for DeviceOrHostAddressConstAMDX {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureGeometryDataKHR.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union AccelerationStructureGeometryDataKHR<'a> {
    pub triangles: AccelerationStructureGeometryTrianglesDataKHR<'a>,
    pub aabbs: AccelerationStructureGeometryAabbsDataKHR<'a>,
    pub instances: AccelerationStructureGeometryInstancesDataKHR<'a>,
}
impl<'a> Default for AccelerationStructureGeometryDataKHR<'a> {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetInfoEXT.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union IndirectExecutionSetInfoEXT<'a> {
    pub p_pipeline_info: *const IndirectExecutionSetPipelineInfoEXT<'a>,
    pub p_shader_info: *const IndirectExecutionSetShaderInfoEXT<'a>,
}
impl<'a> Default for IndirectExecutionSetInfoEXT<'a> {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsTokenDataEXT.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union IndirectCommandsTokenDataEXT {
    pub p_push_constant: *const IndirectCommandsPushConstantTokenEXT,
    pub p_vertex_buffer: *const IndirectCommandsVertexBufferTokenEXT,
    pub p_index_buffer: *const IndirectCommandsIndexBufferTokenEXT,
    pub p_execution_set: *const IndirectCommandsExecutionSetTokenEXT,
}
impl Default for IndirectCommandsTokenDataEXT {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorDataEXT.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DescriptorDataEXT<'a> {
    pub p_sampler: *const Sampler,
    pub p_combined_image_sampler: *const DescriptorImageInfo,
    pub p_input_attachment_image: *const DescriptorImageInfo,
    pub p_sampled_image: *const DescriptorImageInfo,
    pub p_storage_image: *const DescriptorImageInfo,
    pub p_uniform_texel_buffer: *const DescriptorAddressInfoEXT<'a>,
    pub p_storage_texel_buffer: *const DescriptorAddressInfoEXT<'a>,
    pub p_uniform_buffer: *const DescriptorAddressInfoEXT<'a>,
    pub p_storage_buffer: *const DescriptorAddressInfoEXT<'a>,
    pub acceleration_structure: DeviceAddress,
}
impl<'a> Default for DescriptorDataEXT<'a> {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceDataNV.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union AccelerationStructureMotionInstanceDataNV {
    pub static_instance: AccelerationStructureInstanceKHR,
    pub matrix_motion_instance: AccelerationStructureMatrixMotionInstanceNV,
    pub srt_motion_instance: AccelerationStructureSRTMotionInstanceNV,
}
impl Default for AccelerationStructureMotionInstanceDataNV {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkResourceDescriptorDataEXT.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union ResourceDescriptorDataEXT<'a> {
    pub p_image: *const ImageDescriptorInfoEXT<'a>,
    pub p_texel_buffer: *const TexelBufferDescriptorInfoEXT<'a>,
    pub p_address_range: *const DeviceAddressRangeEXT,
    pub p_tensor_arm: *const TensorViewCreateInfoARM<'a>,
}
impl<'a> Default for ResourceDescriptorDataEXT<'a> {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourceDataEXT.html>"]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DescriptorMappingSourceDataEXT<'a> {
    pub constant_offset: DescriptorMappingSourceConstantOffsetEXT<'a>,
    pub push_index: DescriptorMappingSourcePushIndexEXT<'a>,
    pub indirect_index: DescriptorMappingSourceIndirectIndexEXT<'a>,
    pub indirect_index_array: DescriptorMappingSourceIndirectIndexArrayEXT<'a>,
    pub heap_data: DescriptorMappingSourceHeapDataEXT,
    pub push_data_offset: u32,
    pub push_address_offset: u32,
    pub indirect_address: DescriptorMappingSourceIndirectAddressEXT,
    pub shader_record_index: DescriptorMappingSourceShaderRecordIndexEXT<'a>,
    pub shader_record_data_offset: u32,
    pub shader_record_address_offset: u32,
}
impl<'a> Default for DescriptorMappingSourceDataEXT<'a> {
    #[inline]
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
