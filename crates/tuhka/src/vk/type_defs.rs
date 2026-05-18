use super::*;
#[allow(non_camel_case_types)]
pub type PFN_vkInternalAllocationNotification = unsafe extern "system" fn(
    p_user_data: *mut ffi::c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);
impl PFN_Default for PFN_vkInternalAllocationNotification {
    #[inline]
    fn default() -> Self {
        unsafe extern "system" fn f(
            _p_user_data: *mut ffi::c_void,
            _size: usize,
            _allocation_type: InternalAllocationType,
            _allocation_scope: SystemAllocationScope,
        ) {
            panic!(concat!(
                stringify!(PFN_vkInternalAllocationNotification),
                " not loaded"
            ))
        }
        f
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn(
    p_user_data: *mut ffi::c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);
#[allow(non_camel_case_types)]
pub type PFN_vkReallocationFunction = unsafe extern "system" fn(
    p_user_data: *mut ffi::c_void,
    p_original: *mut ffi::c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut ffi::c_void;
impl PFN_Default for PFN_vkReallocationFunction {
    #[inline]
    fn default() -> Self {
        unsafe extern "system" fn f(
            _p_user_data: *mut ffi::c_void,
            _p_original: *mut ffi::c_void,
            _size: usize,
            _alignment: usize,
            _allocation_scope: SystemAllocationScope,
        ) -> *mut ffi::c_void {
            panic!(concat!(
                stringify!(PFN_vkReallocationFunction),
                " not loaded"
            ))
        }
        f
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkAllocationFunction = unsafe extern "system" fn(
    p_user_data: *mut ffi::c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut ffi::c_void;
impl PFN_Default for PFN_vkAllocationFunction {
    #[inline]
    fn default() -> Self {
        unsafe extern "system" fn f(
            _p_user_data: *mut ffi::c_void,
            _size: usize,
            _alignment: usize,
            _allocation_scope: SystemAllocationScope,
        ) -> *mut ffi::c_void {
            panic!(concat!(stringify!(PFN_vkAllocationFunction), " not loaded"))
        }
        f
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkFreeFunction =
    unsafe extern "system" fn(p_user_data: *mut ffi::c_void, p_memory: *mut ffi::c_void);
impl PFN_Default for PFN_vkFreeFunction {
    #[inline]
    fn default() -> Self {
        unsafe extern "system" fn f(_p_user_data: *mut ffi::c_void, _p_memory: *mut ffi::c_void) {
            panic!(concat!(stringify!(PFN_vkFreeFunction), " not loaded"))
        }
        f
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkVoidFunction = unsafe extern "system" fn();
impl PFN_Default for PFN_vkVoidFunction {
    #[inline]
    fn default() -> Self {
        unsafe extern "system" fn f() {
            panic!(concat!(stringify!(PFN_vkVoidFunction), " not loaded"))
        }
        f
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkDebugReportCallbackEXT = unsafe extern "system" fn(
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const ffi::c_char,
    p_message: *const ffi::c_char,
    p_user_data: *mut ffi::c_void,
) -> Bool32;
impl PFN_Default for PFN_vkDebugReportCallbackEXT {
    #[inline]
    fn default() -> Self {
        unsafe extern "system" fn f(
            _flags: DebugReportFlagsEXT,
            _object_type: DebugReportObjectTypeEXT,
            _object: u64,
            _location: usize,
            _message_code: i32,
            _p_layer_prefix: *const ffi::c_char,
            _p_message: *const ffi::c_char,
            _p_user_data: *mut ffi::c_void,
        ) -> Bool32 {
            panic!(concat!(
                stringify!(PFN_vkDebugReportCallbackEXT),
                " not loaded"
            ))
        }
        f
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut ffi::c_void,
) -> Bool32;
impl PFN_Default for PFN_vkDebugUtilsMessengerCallbackEXT {
    #[inline]
    fn default() -> Self {
        unsafe extern "system" fn f(
            _message_severity: DebugUtilsMessageSeverityFlagsEXT,
            _message_types: DebugUtilsMessageTypeFlagsEXT,
            _p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
            _p_user_data: *mut ffi::c_void,
        ) -> Bool32 {
            panic!(concat!(
                stringify!(PFN_vkDebugUtilsMessengerCallbackEXT),
                " not loaded"
            ))
        }
        f
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkDeviceMemoryReportCallbackEXT = unsafe extern "system" fn(
    p_callback_data: *const DeviceMemoryReportCallbackDataEXT,
    p_user_data: *mut ffi::c_void,
);
impl PFN_Default for PFN_vkDeviceMemoryReportCallbackEXT {
    #[inline]
    fn default() -> Self {
        unsafe extern "system" fn f(
            _p_callback_data: *const DeviceMemoryReportCallbackDataEXT,
            _p_user_data: *mut ffi::c_void,
        ) {
            panic!(concat!(
                stringify!(PFN_vkDeviceMemoryReportCallbackEXT),
                " not loaded"
            ))
        }
        f
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetInstanceProcAddrLUNARG = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    p_name: *const ffi::c_char,
) -> PFN_vkVoidFunction;
impl PFN_Default for PFN_vkGetInstanceProcAddrLUNARG {
    #[inline]
    fn default() -> Self {
        unsafe extern "system" fn f(
            _instance: crate::vk::Instance,
            _p_name: *const ffi::c_char,
        ) -> PFN_vkVoidFunction {
            panic!(concat!(
                stringify!(PFN_vkGetInstanceProcAddrLUNARG),
                " not loaded"
            ))
        }
        f
    }
}
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateInstance = unsafe extern "system" fn(
    create_info: *const InstanceCreateInfo,
    allocator: *const AllocationCallbacks,
    instance: *mut crate::vk::Instance,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyInstance =
    unsafe extern "system" fn(instance: crate::vk::Instance, allocator: *const AllocationCallbacks);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    physical_device_count: *mut u32,
    physical_devices: *mut PhysicalDevice,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceProcAddr = unsafe extern "system" fn(
    device: crate::vk::Device,
    name: *const ffi::c_char,
) -> PFN_vkVoidFunction;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    name: *const ffi::c_char,
) -> PFN_vkVoidFunction;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    properties: *mut PhysicalDeviceProperties,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_property_count: *mut u32,
    queue_family_properties: *mut QueueFamilyProperties,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    memory_properties: *mut PhysicalDeviceMemoryProperties,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    features: *mut PhysicalDeviceFeatures,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    format_properties: *mut FormatProperties,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    ty: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    image_format_properties: *mut ImageFormatProperties,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDevice = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    create_info: *const DeviceCreateInfo,
    allocator: *const AllocationCallbacks,
    device: *mut crate::vk::Device,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDevice =
    unsafe extern "system" fn(device: crate::vk::Device, allocator: *const AllocationCallbacks);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceVersion =
    unsafe extern "system" fn(api_version: *mut u32) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceLayerProperties =
    unsafe extern "system" fn(property_count: *mut u32, properties: *mut LayerProperties) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(
    layer_name: *const ffi::c_char,
    property_count: *mut u32,
    properties: *mut ExtensionProperties,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    property_count: *mut u32,
    properties: *mut LayerProperties,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    layer_name: *const ffi::c_char,
    property_count: *mut u32,
    properties: *mut ExtensionProperties,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(
    device: crate::vk::Device,
    queue_family_index: u32,
    queue_index: u32,
    queue: *mut Queue,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSubmit = unsafe extern "system" fn(
    queue: Queue,
    submit_count: u32,
    submits: *const SubmitInfo,
    fence: Fence,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: Queue) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: crate::vk::Device) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
    device: crate::vk::Device,
    allocate_info: *const MemoryAllocateInfo,
    allocator: *const AllocationCallbacks,
    memory: *mut DeviceMemory,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkFreeMemory = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory: DeviceMemory,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkMapMemory = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory: DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    pp_data: *mut *mut ffi::c_void,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkUnmapMemory =
    unsafe extern "system" fn(device: crate::vk::Device, memory: DeviceMemory);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory_range_count: u32,
    memory_ranges: *const MappedMemoryRange,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory_range_count: u32,
    memory_ranges: *const MappedMemoryRange,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory: DeviceMemory,
    committed_memory_in_bytes: *mut DeviceSize,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(
    device: crate::vk::Device,
    buffer: Buffer,
    memory_requirements: *mut MemoryRequirements,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(
    device: crate::vk::Device,
    buffer: Buffer,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    memory_requirements: *mut MemoryRequirements,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    ty: ImageType,
    samples: SampleCountFlags,
    usage: ImageUsageFlags,
    tiling: ImageTiling,
    property_count: *mut u32,
    properties: *mut SparseImageFormatProperties,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(
    queue: Queue,
    bind_info_count: u32,
    bind_info: *const BindSparseInfo,
    fence: Fence,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateFence = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const FenceCreateInfo,
    allocator: *const AllocationCallbacks,
    fence: *mut Fence,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyFence = unsafe extern "system" fn(
    device: crate::vk::Device,
    fence: Fence,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkResetFences = unsafe extern "system" fn(
    device: crate::vk::Device,
    fence_count: u32,
    fences: *const Fence,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceStatus =
    unsafe extern "system" fn(device: crate::vk::Device, fence: Fence) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkWaitForFences = unsafe extern "system" fn(
    device: crate::vk::Device,
    fence_count: u32,
    fences: *const Fence,
    wait_all: Bool32,
    timeout: u64,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const SemaphoreCreateInfo,
    allocator: *const AllocationCallbacks,
    semaphore: *mut Semaphore,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(
    device: crate::vk::Device,
    semaphore: Semaphore,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateEvent = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const EventCreateInfo,
    allocator: *const AllocationCallbacks,
    event: *mut Event,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyEvent = unsafe extern "system" fn(
    device: crate::vk::Device,
    event: Event,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetEventStatus =
    unsafe extern "system" fn(device: crate::vk::Device, event: Event) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetEvent =
    unsafe extern "system" fn(device: crate::vk::Device, event: Event) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkResetEvent =
    unsafe extern "system" fn(device: crate::vk::Device, event: Event) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const QueryPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    query_pool: *mut QueryPool,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    query_pool: QueryPool,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(
    device: crate::vk::Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    data_size: usize,
    data: *mut ffi::c_void,
    stride: DeviceSize,
    flags: QueryResultFlags,
) -> Result;
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkResetQueryPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const BufferCreateInfo,
    allocator: *const AllocationCallbacks,
    buffer: *mut Buffer,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(
    device: crate::vk::Device,
    buffer: Buffer,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const BufferViewCreateInfo,
    allocator: *const AllocationCallbacks,
    view: *mut BufferView,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(
    device: crate::vk::Device,
    buffer_view: BufferView,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImage = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const ImageCreateInfo,
    allocator: *const AllocationCallbacks,
    image: *mut Image,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyImage = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    subresource: *const ImageSubresource,
    layout: *mut SubresourceLayout,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImageView = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const ImageViewCreateInfo,
    allocator: *const AllocationCallbacks,
    view: *mut ImageView,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyImageView = unsafe extern "system" fn(
    device: crate::vk::Device,
    image_view: ImageView,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const ShaderModuleCreateInfo,
    allocator: *const AllocationCallbacks,
    shader_module: *mut ShaderModule,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(
    device: crate::vk::Device,
    shader_module: ShaderModule,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const PipelineCacheCreateInfo,
    allocator: *const AllocationCallbacks,
    pipeline_cache: *mut PipelineCache,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_cache: PipelineCache,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_cache: PipelineCache,
    data_size: *mut usize,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(
    device: crate::vk::Device,
    dst_cache: PipelineCache,
    src_cache_count: u32,
    src_caches: *const PipelineCache,
) -> Result;
#[doc = "Provided by VK_KHR_pipeline_binary."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePipelineBinariesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const PipelineBinaryCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    binaries: *mut PipelineBinaryHandlesInfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_pipeline_binary."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipelineBinaryKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_binary: PipelineBinaryKHR,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_KHR_pipeline_binary."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineKeyKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_create_info: *const PipelineCreateInfoKHR,
    pipeline_key: *mut PipelineBinaryKeyKHR,
) -> Result;
#[doc = "Provided by VK_KHR_pipeline_binary."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineBinaryDataKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const PipelineBinaryDataInfoKHR,
    pipeline_binary_key: *mut PipelineBinaryKeyKHR,
    pipeline_binary_data_size: *mut usize,
    pipeline_binary_data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_KHR_pipeline_binary."]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseCapturedPipelineDataKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const ReleaseCapturedPipelineDataInfoKHR,
    allocator: *const AllocationCallbacks,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const GraphicsPipelineCreateInfo,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const ComputePipelineCreateInfo,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> Result;
#[doc = "Provided by VK_HUAWEI_subpass_shading."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = unsafe extern "system" fn(
    device: crate::vk::Device,
    renderpass: RenderPass,
    max_workgroup_size: *mut Extent2D,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline: Pipeline,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const PipelineLayoutCreateInfo,
    allocator: *const AllocationCallbacks,
    pipeline_layout: *mut PipelineLayout,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_layout: PipelineLayout,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSampler = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const SamplerCreateInfo,
    allocator: *const AllocationCallbacks,
    sampler: *mut Sampler,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySampler = unsafe extern "system" fn(
    device: crate::vk::Device,
    sampler: Sampler,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const DescriptorSetLayoutCreateInfo,
    allocator: *const AllocationCallbacks,
    set_layout: *mut DescriptorSetLayout,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_set_layout: DescriptorSetLayout,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const DescriptorPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    descriptor_pool: *mut DescriptorPool,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_pool: DescriptorPool,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_pool: DescriptorPool,
    flags: DescriptorPoolResetFlags,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(
    device: crate::vk::Device,
    allocate_info: *const DescriptorSetAllocateInfo,
    descriptor_sets: *mut DescriptorSet,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_pool: DescriptorPool,
    descriptor_set_count: u32,
    descriptor_sets: *const DescriptorSet,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_write_count: u32,
    descriptor_writes: *const WriteDescriptorSet,
    descriptor_copy_count: u32,
    descriptor_copies: *const CopyDescriptorSet,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const FramebufferCreateInfo,
    allocator: *const AllocationCallbacks,
    framebuffer: *mut Framebuffer,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(
    device: crate::vk::Device,
    framebuffer: Framebuffer,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const RenderPassCreateInfo,
    allocator: *const AllocationCallbacks,
    render_pass: *mut RenderPass,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(
    device: crate::vk::Device,
    render_pass: RenderPass,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(
    device: crate::vk::Device,
    render_pass: RenderPass,
    granularity: *mut Extent2D,
);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRenderingAreaGranularity = unsafe extern "system" fn(
    device: crate::vk::Device,
    rendering_area_info: *const RenderingAreaInfo,
    granularity: *mut Extent2D,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const CommandPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    command_pool: *mut CommandPool,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    command_pool: CommandPool,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkResetCommandPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    command_pool: CommandPool,
    flags: CommandPoolResetFlags,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(
    device: crate::vk::Device,
    allocate_info: *const CommandBufferAllocateInfo,
    command_buffers: *mut CommandBuffer,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(
    device: crate::vk::Device,
    command_pool: CommandPool,
    command_buffer_count: u32,
    command_buffers: *const CommandBuffer,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    begin_info: *const CommandBufferBeginInfo,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkEndCommandBuffer =
    unsafe extern "system" fn(command_buffer: CommandBuffer) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    flags: CommandBufferResetFlags,
) -> Result;
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);
#[doc = "Provided by VK_EXT_primitive_restart_index."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveRestartIndexEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_index: u32);
#[doc = "Provided by VK_EXT_attachment_feedback_loop_dynamic_state."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, aspect_mask: ImageAspectFlags);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    viewports: *const Viewport,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    scissors: *const Rect2D,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineWidth =
    unsafe extern "system" fn(command_buffer: CommandBuffer, line_width: f32);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetBlendConstants =
    unsafe extern "system" fn(command_buffer: CommandBuffer, blend_constants: *const [f32; 4usize]);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    compare_mask: u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    write_mask: u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    reference: u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    descriptor_sets: *const DescriptorSet,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    index_type: IndexType,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const DeviceSize,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDraw = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
);
#[doc = "Provided by VK_EXT_multi_draw."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMultiEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    draw_count: u32,
    vertex_info: *const MultiDrawInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
);
#[doc = "Provided by VK_EXT_multi_draw."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMultiIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    draw_count: u32,
    index_info: *const MultiDrawIndexedInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    vertex_offset: *const i32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatch = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchIndirect =
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);
#[doc = "Provided by VK_HUAWEI_subpass_shading."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSubpassShadingHUAWEI = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[doc = "Provided by VK_HUAWEI_cluster_culling_shader."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawClusterHUAWEI = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
#[doc = "Provided by VK_HUAWEI_cluster_culling_shader."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawClusterIndirectHUAWEI =
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);
#[doc = "Provided by VK_NV_device_generated_commands_compute."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdUpdatePipelineIndirectBufferNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_buffer: Buffer,
    region_count: u32,
    regions: *const BufferCopy,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const ImageCopy,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const ImageBlit,
    filter: Filter,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const BufferImageCopy,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_buffer: Buffer,
    region_count: u32,
    regions: *const BufferImageCopy,
);
#[doc = "Provided by VK_NV_copy_memory_indirect."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
);
#[doc = "Provided by VK_KHR_copy_memory_indirect."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR,
);
#[doc = "Provided by VK_NV_copy_memory_indirect."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryToImageIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    image_subresources: *const ImageSubresourceLayers,
);
#[doc = "Provided by VK_KHR_copy_memory_indirect."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryToImageIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    data_size: DeviceSize,
    data: *const ffi::c_void,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    size: DeviceSize,
    data: u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    color: *const ClearColorValue,
    range_count: u32,
    ranges: *const ImageSubresourceRange,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    depth_stencil: *const ClearDepthStencilValue,
    range_count: u32,
    ranges: *const ImageSubresourceRange,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    attachments: *const ClearAttachment,
    rect_count: u32,
    rects: *const ClearRect,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const ImageResolve,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    events: *const Event,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    memory_barrier_count: u32,
    memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    image_memory_barriers: *const ImageMemoryBarrier,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    dependency_flags: DependencyFlags,
    memory_barrier_count: u32,
    memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    image_memory_barriers: *const ImageMemoryBarrier,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndQuery =
    unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32);
#[doc = "Provided by VK_EXT_conditional_rendering."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
);
#[doc = "Provided by VK_EXT_conditional_rendering."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndConditionalRenderingEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer);
#[doc = "Provided by VK_EXT_custom_resolve."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginCustomResolveEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    begin_custom_resolve_info: *const BeginCustomResolveInfoEXT,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    query_pool: QueryPool,
    query: u32,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    stride: DeviceSize,
    flags: QueryResultFlags,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    layout: PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    values: *const ffi::c_void,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    render_pass_begin: *const RenderPassBeginInfo,
    contents: SubpassContents,
);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdNextSubpass =
    unsafe extern "system" fn(command_buffer: CommandBuffer, contents: SubpassContents);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[doc = "Provided by Vulkan version 1.0."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    command_buffer_count: u32,
    command_buffers: *const CommandBuffer,
);
#[doc = "Provided by VK_KHR_android_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const AndroidSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_OHOS_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSurfaceOHOS = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const SurfaceCreateInfoOHOS,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_KHR_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    property_count: *mut u32,
    properties: *mut DisplayPropertiesKHR,
) -> Result;
#[doc = "Provided by VK_KHR_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    property_count: *mut u32,
    properties: *mut DisplayPlanePropertiesKHR,
) -> Result;
#[doc = "Provided by VK_KHR_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    plane_index: u32,
    display_count: *mut u32,
    displays: *mut DisplayKHR,
) -> Result;
#[doc = "Provided by VK_KHR_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    property_count: *mut u32,
    properties: *mut DisplayModePropertiesKHR,
) -> Result;
#[doc = "Provided by VK_KHR_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    create_info: *const DisplayModeCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    mode: *mut DisplayModeKHR,
) -> Result;
#[doc = "Provided by VK_KHR_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    mode: DisplayModeKHR,
    plane_index: u32,
    capabilities: *mut DisplayPlaneCapabilitiesKHR,
) -> Result;
#[doc = "Provided by VK_KHR_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const DisplaySurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_KHR_display_swapchain."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain_count: u32,
    create_infos: *const SwapchainCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    swapchains: *mut SwapchainKHR,
) -> Result;
#[doc = "Provided by VK_KHR_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    surface: SurfaceKHR,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_KHR_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    surface: SurfaceKHR,
    supported: *mut Bool32,
) -> Result;
#[doc = "Provided by VK_KHR_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    surface_capabilities: *mut SurfaceCapabilitiesKHR,
) -> Result;
#[doc = "Provided by VK_KHR_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    surface_format_count: *mut u32,
    surface_formats: *mut SurfaceFormatKHR,
) -> Result;
#[doc = "Provided by VK_KHR_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    present_mode_count: *mut u32,
    present_modes: *mut PresentModeKHR,
) -> Result;
#[doc = "Provided by VK_KHR_swapchain."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const SwapchainCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    swapchain: *mut SwapchainKHR,
) -> Result;
#[doc = "Provided by VK_KHR_swapchain."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySwapchainKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_KHR_swapchain."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    swapchain_image_count: *mut u32,
    swapchain_images: *mut Image,
) -> Result;
#[doc = "Provided by VK_KHR_swapchain."]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    timeout: u64,
    semaphore: Semaphore,
    fence: Fence,
    image_index: *mut u32,
) -> Result;
#[doc = "Provided by VK_KHR_swapchain."]
#[allow(non_camel_case_types)]
pub type PFN_vkQueuePresentKHR =
    unsafe extern "system" fn(queue: Queue, present_info: *const PresentInfoKHR) -> Result;
#[doc = "Provided by VK_NN_vi_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const ViSurfaceCreateInfoNN,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_KHR_wayland_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const WaylandSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_KHR_wayland_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    display: *mut wl_display,
)
    -> Bool32;
#[doc = "Provided by VK_SEC_ubm_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateUbmSurfaceSEC = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const UbmSurfaceCreateInfoSEC,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_SEC_ubm_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    device: *mut ubm_device,
) -> Bool32;
#[doc = "Provided by VK_KHR_win32_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const Win32SurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_KHR_win32_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
    unsafe extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32) -> Bool32;
#[doc = "Provided by VK_KHR_xlib_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const XlibSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_KHR_xlib_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    dpy: *mut Display,
    visual_id: VisualID,
) -> Bool32;
#[doc = "Provided by VK_KHR_xcb_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const XcbSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_KHR_xcb_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> Bool32;
#[doc = "Provided by VK_EXT_directfb_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const DirectFBSurfaceCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_EXT_directfb_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32;
#[doc = "Provided by VK_FUCHSIA_imagepipe_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_GGP_stream_descriptor_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const StreamDescriptorSurfaceCreateInfoGGP,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_QNX_screen_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateScreenSurfaceQNX = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const ScreenSurfaceCreateInfoQNX,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_QNX_screen_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    window: *mut _screen_window,
) -> Bool32;
#[doc = "Provided by VK_EXT_debug_report."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const DebugReportCallbackCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    callback: *mut DebugReportCallbackEXT,
) -> Result;
#[doc = "Provided by VK_EXT_debug_report."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    callback: DebugReportCallbackEXT,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_EXT_debug_report."]
#[allow(non_camel_case_types)]
pub type PFN_vkDebugReportMessageEXT = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    layer_prefix: *const ffi::c_char,
    message: *const ffi::c_char,
);
#[doc = "Provided by VK_EXT_debug_marker."]
#[allow(non_camel_case_types)]
pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    name_info: *const DebugMarkerObjectNameInfoEXT,
) -> Result;
#[doc = "Provided by VK_EXT_debug_marker."]
#[allow(non_camel_case_types)]
pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    tag_info: *const DebugMarkerObjectTagInfoEXT,
) -> Result;
#[doc = "Provided by VK_EXT_debug_marker."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    marker_info: *const DebugMarkerMarkerInfoEXT,
);
#[doc = "Provided by VK_EXT_debug_marker."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[doc = "Provided by VK_EXT_debug_marker."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    marker_info: *const DebugMarkerMarkerInfoEXT,
);
#[doc = "Provided by VK_NV_external_memory_capabilities."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
        external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
    ) -> Result;
#[doc = "Provided by VK_NV_external_memory_win32."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory: DeviceMemory,
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    handle: *mut HANDLE,
) -> Result;
#[doc = "Provided by VK_NV_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    is_preprocessed: Bool32,
    generated_commands_info: *const GeneratedCommandsInfoNV,
);
#[doc = "Provided by VK_NV_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    generated_commands_info: *const GeneratedCommandsInfoNV,
);
#[doc = "Provided by VK_NV_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
    group_index: u32,
);
#[doc = "Provided by VK_NV_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const GeneratedCommandsMemoryRequirementsInfoNV,
    memory_requirements: *mut MemoryRequirements2,
);
#[doc = "Provided by VK_NV_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const IndirectCommandsLayoutCreateInfoNV,
    allocator: *const AllocationCallbacks,
    indirect_commands_layout: *mut IndirectCommandsLayoutNV,
) -> Result;
#[doc = "Provided by VK_NV_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    indirect_commands_layout: IndirectCommandsLayoutNV,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_EXT_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdExecuteGeneratedCommandsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    is_preprocessed: Bool32,
    generated_commands_info: *const GeneratedCommandsInfoEXT,
);
#[doc = "Provided by VK_EXT_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPreprocessGeneratedCommandsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    generated_commands_info: *const GeneratedCommandsInfoEXT,
    state_command_buffer: CommandBuffer,
);
#[doc = "Provided by VK_EXT_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const GeneratedCommandsMemoryRequirementsInfoEXT,
    memory_requirements: *mut MemoryRequirements2,
);
#[doc = "Provided by VK_EXT_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateIndirectCommandsLayoutEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const IndirectCommandsLayoutCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
) -> Result;
#[doc = "Provided by VK_EXT_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyIndirectCommandsLayoutEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    indirect_commands_layout: IndirectCommandsLayoutEXT,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_EXT_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateIndirectExecutionSetEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const IndirectExecutionSetCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    indirect_execution_set: *mut IndirectExecutionSetEXT,
) -> Result;
#[doc = "Provided by VK_EXT_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyIndirectExecutionSetEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    indirect_execution_set: IndirectExecutionSetEXT,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_EXT_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateIndirectExecutionSetPipelineEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    indirect_execution_set: IndirectExecutionSetEXT,
    execution_set_write_count: u32,
    execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT,
);
#[doc = "Provided by VK_EXT_device_generated_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateIndirectExecutionSetShaderEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    indirect_execution_set: IndirectExecutionSetEXT,
    execution_set_write_count: u32,
    execution_set_writes: *const WriteIndirectExecutionSetShaderEXT,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    features: *mut PhysicalDeviceFeatures2,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    properties: *mut PhysicalDeviceProperties2,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    format_properties: *mut FormatProperties2,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    image_format_info: *const PhysicalDeviceImageFormatInfo2,
    image_format_properties: *mut ImageFormatProperties2,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_property_count: *mut u32,
    queue_family_properties: *mut QueueFamilyProperties2,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    memory_properties: *mut PhysicalDeviceMemoryProperties2,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format_info: *const PhysicalDeviceSparseImageFormatInfo2,
    property_count: *mut u32,
    properties: *mut SparseImageFormatProperties2,
);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSet = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    descriptor_writes: *const WriteDescriptorSet,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
    device: crate::vk::Device,
    command_pool: CommandPool,
    flags: CommandPoolTrimFlags,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
    external_buffer_properties: *mut ExternalBufferProperties,
);
#[doc = "Provided by VK_KHR_external_memory_win32."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR,
    handle: *mut HANDLE,
) -> Result;
#[doc = "Provided by VK_KHR_external_memory_win32."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    handle: HANDLE,
    memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
) -> Result;
#[doc = "Provided by VK_KHR_external_memory_fd."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_fd_info: *const MemoryGetFdInfoKHR,
    fd: *mut ffi::c_int,
) -> Result;
#[doc = "Provided by VK_KHR_external_memory_fd."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    fd: ffi::c_int,
    memory_fd_properties: *mut MemoryFdPropertiesKHR,
) -> Result;
#[doc = "Provided by VK_FUCHSIA_external_memory."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA,
    zircon_handle: *mut zx_handle_t,
) -> Result;
#[doc = "Provided by VK_FUCHSIA_external_memory."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(
    device: crate::vk::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    zircon_handle: zx_handle_t,
    memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA,
) -> Result;
#[doc = "Provided by VK_NV_external_memory_rdma."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV,
    address: *mut RemoteAddressNV,
) -> Result;
#[doc = "Provided by VK_NV_external_memory_sci_buf."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemorySciBufNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_sci_buf_info: *const MemoryGetSciBufInfoNV,
    handle: *mut NvSciBufObj,
) -> Result;
#[doc = "Provided by VK_NV_external_memory_sci_buf."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: NvSciBufObj,
        memory_sci_buf_properties: *mut MemorySciBufPropertiesNV,
    ) -> Result;
#[doc = "Provided by VK_NV_external_memory_sci_buf."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSciBufAttributesNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    attributes: NvSciBufAttrList,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
    external_semaphore_properties: *mut ExternalSemaphoreProperties,
);
#[doc = "Provided by VK_KHR_external_semaphore_win32."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
    handle: *mut HANDLE,
) -> Result;
#[doc = "Provided by VK_KHR_external_semaphore_win32."]
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_external_semaphore_fd."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_fd_info: *const SemaphoreGetFdInfoKHR,
    fd: *mut ffi::c_int,
) -> Result;
#[doc = "Provided by VK_KHR_external_semaphore_fd."]
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
) -> Result;
#[doc = "Provided by VK_FUCHSIA_external_semaphore."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA,
    zircon_handle: *mut zx_handle_t,
) -> Result;
#[doc = "Provided by VK_FUCHSIA_external_semaphore."]
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: crate::vk::Device,
    import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    external_fence_info: *const PhysicalDeviceExternalFenceInfo,
    external_fence_properties: *mut ExternalFenceProperties,
);
#[doc = "Provided by VK_KHR_external_fence_win32."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_win32_handle_info: *const FenceGetWin32HandleInfoKHR,
    handle: *mut HANDLE,
) -> Result;
#[doc = "Provided by VK_KHR_external_fence_win32."]
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_external_fence_fd."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_fd_info: *const FenceGetFdInfoKHR,
    fd: *mut ffi::c_int,
) -> Result;
#[doc = "Provided by VK_KHR_external_fence_fd."]
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceFdKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    import_fence_fd_info: *const ImportFenceFdInfoKHR,
) -> Result;
#[doc = "Provided by VK_NV_external_sci_sync."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceSciSyncFenceNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_sci_sync_handle_info: *const FenceGetSciSyncInfoNV,
    handle: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_NV_external_sci_sync."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceSciSyncObjNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_sci_sync_handle_info: *const FenceGetSciSyncInfoNV,
    handle: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_NV_external_sci_sync."]
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceSciSyncFenceNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    import_fence_sci_sync_info: *const ImportFenceSciSyncInfoNV,
) -> Result;
#[doc = "Provided by VK_NV_external_sci_sync."]
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceSciSyncObjNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    import_fence_sci_sync_info: *const ImportFenceSciSyncInfoNV,
) -> Result;
#[doc = "Provided by VK_NV_external_sci_sync."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreSciSyncObjNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_sci_sync_info: *const SemaphoreGetSciSyncInfoNV,
    handle: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_NV_external_sci_sync."]
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreSciSyncObjNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    import_semaphore_sci_sync_info: *const ImportSemaphoreSciSyncInfoNV,
) -> Result;
#[doc = "Provided by VK_NV_external_sci_sync."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSciSyncAttributesNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    sci_sync_attributes_info: *const SciSyncAttributesInfoNV,
    attributes: NvSciSyncAttrList,
) -> Result;
#[doc = "Provided by VK_NV_external_sci_sync2."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSemaphoreSciSyncPoolNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const SemaphoreSciSyncPoolCreateInfoNV,
    allocator: *const AllocationCallbacks,
    semaphore_pool: *mut SemaphoreSciSyncPoolNV,
) -> Result;
#[doc = "Provided by VK_NV_external_sci_sync2."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySemaphoreSciSyncPoolNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    semaphore_pool: SemaphoreSciSyncPoolNV,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_EXT_direct_mode_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseDisplayEXT =
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> Result;
#[doc = "Provided by VK_EXT_acquire_xlib_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireXlibDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    dpy: *mut Display,
    display: DisplayKHR,
) -> Result;
#[doc = "Provided by VK_EXT_acquire_xlib_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRandROutputDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    dpy: *mut Display,
    rr_output: RROutput,
    display: *mut DisplayKHR,
) -> Result;
#[doc = "Provided by VK_NV_acquire_winrt_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireWinrtDisplayNV =
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> Result;
#[doc = "Provided by VK_NV_acquire_winrt_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetWinrtDisplayNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    device_relative_id: u32,
    display: *mut DisplayKHR,
) -> Result;
#[doc = "Provided by VK_EXT_display_control."]
#[allow(non_camel_case_types)]
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    display: DisplayKHR,
    display_power_info: *const DisplayPowerInfoEXT,
) -> Result;
#[doc = "Provided by VK_EXT_display_control."]
#[allow(non_camel_case_types)]
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    device_event_info: *const DeviceEventInfoEXT,
    allocator: *const AllocationCallbacks,
    fence: *mut Fence,
) -> Result;
#[doc = "Provided by VK_EXT_display_control."]
#[allow(non_camel_case_types)]
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    display: DisplayKHR,
    display_event_info: *const DisplayEventInfoEXT,
    allocator: *const AllocationCallbacks,
    fence: *mut Fence,
) -> Result;
#[doc = "Provided by VK_EXT_display_control."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    counter: SurfaceCounterFlagsEXT,
    counter_value: *mut u64,
) -> Result;
#[doc = "Provided by VK_EXT_display_surface_counter."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    surface_capabilities: *mut SurfaceCapabilities2EXT,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    physical_device_group_count: *mut u32,
    physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
    device: crate::vk::Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    peer_memory_features: *mut PeerMemoryFeatureFlags,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
    device: crate::vk::Device,
    bind_info_count: u32,
    bind_infos: *const BindBufferMemoryInfo,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
    device: crate::vk::Device,
    bind_info_count: u32,
    bind_infos: *const BindImageMemoryInfo,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDeviceMask =
    unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32);
#[doc = "Provided by VK_KHR_swapchain."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
) -> Result;
#[doc = "Provided by VK_KHR_swapchain."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    surface: SurfaceKHR,
    modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> Result;
#[doc = "Provided by VK_KHR_swapchain."]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireNextImage2KHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    acquire_info: *const AcquireNextImageInfoKHR,
    image_index: *mut u32,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
#[doc = "Provided by VK_KHR_swapchain."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    rect_count: *mut u32,
    rects: *mut Rect2D,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const DescriptorUpdateTemplateCreateInfo,
    allocator: *const AllocationCallbacks,
    descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_update_template: DescriptorUpdateTemplate,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_set: DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    data: *const ffi::c_void,
);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetWithTemplate = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    descriptor_update_template: DescriptorUpdateTemplate,
    layout: PipelineLayout,
    set: u32,
    data: *const ffi::c_void,
);
#[doc = "Provided by VK_EXT_hdr_metadata."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain_count: u32,
    swapchains: *const SwapchainKHR,
    metadata: *const HdrMetadataEXT,
);
#[doc = "Provided by VK_KHR_shared_presentable_image."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainStatusKHR =
    unsafe extern "system" fn(device: crate::vk::Device, swapchain: SwapchainKHR) -> Result;
#[doc = "Provided by VK_GOOGLE_display_timing."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    display_timing_properties: *mut RefreshCycleDurationGOOGLE,
) -> Result;
#[doc = "Provided by VK_GOOGLE_display_timing."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    presentation_timing_count: *mut u32,
    presentation_timings: *mut PastPresentationTimingGOOGLE,
) -> Result;
#[doc = "Provided by VK_MVK_ios_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const IOSSurfaceCreateInfoMVK,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_MVK_macos_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const MacOSSurfaceCreateInfoMVK,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_EXT_metal_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const MetalSurfaceCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_NV_clip_space_w_scaling."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWScalingNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    viewport_wscalings: *const ViewportWScalingNV,
);
#[doc = "Provided by VK_EXT_discard_rectangles."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_discard_rectangle: u32,
    discard_rectangle_count: u32,
    discard_rectangles: *const Rect2D,
);
#[doc = "Provided by VK_EXT_discard_rectangles."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDiscardRectangleEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, discard_rectangle_enable: Bool32);
#[doc = "Provided by VK_EXT_discard_rectangles."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDiscardRectangleModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    discard_rectangle_mode: DiscardRectangleModeEXT,
);
#[doc = "Provided by VK_EXT_sample_locations."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    sample_locations_info: *const SampleLocationsInfoEXT,
);
#[doc = "Provided by VK_EXT_sample_locations."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    samples: SampleCountFlags,
    multisample_properties: *mut MultisamplePropertiesEXT,
);
#[doc = "Provided by VK_KHR_get_surface_capabilities2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    surface_capabilities: *mut SurfaceCapabilities2KHR,
) -> Result;
#[doc = "Provided by VK_KHR_get_surface_capabilities2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    surface_format_count: *mut u32,
    surface_formats: *mut SurfaceFormat2KHR,
) -> Result;
#[doc = "Provided by VK_KHR_get_display_properties2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    property_count: *mut u32,
    properties: *mut DisplayProperties2KHR,
) -> Result;
#[doc = "Provided by VK_KHR_get_display_properties2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    property_count: *mut u32,
    properties: *mut DisplayPlaneProperties2KHR,
) -> Result;
#[doc = "Provided by VK_KHR_get_display_properties2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    property_count: *mut u32,
    properties: *mut DisplayModeProperties2KHR,
) -> Result;
#[doc = "Provided by VK_KHR_get_display_properties2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display_plane_info: *const DisplayPlaneInfo2KHR,
    capabilities: *mut DisplayPlaneCapabilities2KHR,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const BufferMemoryRequirementsInfo2,
    memory_requirements: *mut MemoryRequirements2,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const ImageMemoryRequirementsInfo2,
    memory_requirements: *mut MemoryRequirements2,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const ImageSparseMemoryRequirementsInfo2,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const DeviceBufferMemoryRequirements,
    memory_requirements: *mut MemoryRequirements2,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const DeviceImageMemoryRequirements,
    memory_requirements: *mut MemoryRequirements2,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const DeviceImageMemoryRequirements,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const SamplerYcbcrConversionCreateInfo,
    allocator: *const AllocationCallbacks,
    ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
    device: crate::vk::Device,
    ycbcr_conversion: SamplerYcbcrConversion,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
    device: crate::vk::Device,
    queue_info: *const DeviceQueueInfo2,
    queue: *mut Queue,
);
#[doc = "Provided by VK_EXT_validation_cache."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateValidationCacheEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const ValidationCacheCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    validation_cache: *mut ValidationCacheEXT,
) -> Result;
#[doc = "Provided by VK_EXT_validation_cache."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    validation_cache: ValidationCacheEXT,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_EXT_validation_cache."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    validation_cache: ValidationCacheEXT,
    data_size: *mut usize,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_EXT_validation_cache."]
#[allow(non_camel_case_types)]
pub type PFN_vkMergeValidationCachesEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    dst_cache: ValidationCacheEXT,
    src_cache_count: u32,
    src_caches: *const ValidationCacheEXT,
) -> Result;
#[doc = "Provided by Vulkan version 1.1."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const DescriptorSetLayoutCreateInfo,
    support: *mut DescriptorSetLayoutSupport,
);
#[doc = "Provided by VK_AMD_shader_info."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderInfoAMD = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline: Pipeline,
    shader_stage: ShaderStageFlags,
    info_type: ShaderInfoTypeAMD,
    info_size: *mut usize,
    info: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_AMD_display_native_hdr."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
    device: crate::vk::Device,
    swap_chain: SwapchainKHR,
    local_dimming_enable: Bool32,
);
#[doc = "Provided by VK_KHR_calibrated_timestamps."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    time_domain_count: *mut u32,
    time_domains: *mut TimeDomainKHR,
) -> Result;
#[doc = "Provided by VK_KHR_calibrated_timestamps."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetCalibratedTimestampsKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    timestamp_count: u32,
    timestamp_infos: *const CalibratedTimestampInfoKHR,
    timestamps: *mut u64,
    max_deviation: *mut u64,
) -> Result;
#[doc = "Provided by VK_EXT_debug_utils."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    name_info: *const DebugUtilsObjectNameInfoEXT,
) -> Result;
#[doc = "Provided by VK_EXT_debug_utils."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    tag_info: *const DebugUtilsObjectTagInfoEXT,
) -> Result;
#[doc = "Provided by VK_EXT_debug_utils."]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueBeginDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: Queue, label_info: *const DebugUtilsLabelEXT);
#[doc = "Provided by VK_EXT_debug_utils."]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueEndDebugUtilsLabelEXT = unsafe extern "system" fn(queue: Queue);
#[doc = "Provided by VK_EXT_debug_utils."]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueInsertDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: Queue, label_info: *const DebugUtilsLabelEXT);
#[doc = "Provided by VK_EXT_debug_utils."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginDebugUtilsLabelEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, label_info: *const DebugUtilsLabelEXT);
#[doc = "Provided by VK_EXT_debug_utils."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndDebugUtilsLabelEXT = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[doc = "Provided by VK_EXT_debug_utils."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdInsertDebugUtilsLabelEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, label_info: *const DebugUtilsLabelEXT);
#[doc = "Provided by VK_EXT_debug_utils."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const DebugUtilsMessengerCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    messenger: *mut DebugUtilsMessengerEXT,
) -> Result;
#[doc = "Provided by VK_EXT_debug_utils."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    messenger: DebugUtilsMessengerEXT,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_EXT_debug_utils."]
#[allow(non_camel_case_types)]
pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const DebugUtilsMessengerCallbackDataEXT,
);
#[doc = "Provided by VK_EXT_external_memory_host."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    host_pointer: *const ffi::c_void,
    memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
) -> Result;
#[doc = "Provided by VK_AMD_buffer_marker."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const RenderPassCreateInfo2,
    allocator: *const AllocationCallbacks,
    render_pass: *mut RenderPass,
) -> Result;
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    render_pass_begin: *const RenderPassBeginInfo,
    subpass_begin_info: *const SubpassBeginInfo,
);
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdNextSubpass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    subpass_begin_info: *const SubpassBeginInfo,
    subpass_end_info: *const SubpassEndInfo,
);
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    subpass_end_info: *const SubpassEndInfo,
);
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreCounterValue = unsafe extern "system" fn(
    device: crate::vk::Device,
    semaphore: Semaphore,
    value: *mut u64,
) -> Result;
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkWaitSemaphores = unsafe extern "system" fn(
    device: crate::vk::Device,
    wait_info: *const SemaphoreWaitInfo,
    timeout: u64,
) -> Result;
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkSignalSemaphore = unsafe extern "system" fn(
    device: crate::vk::Device,
    signal_info: *const SemaphoreSignalInfo,
) -> Result;
#[doc = "Provided by VK_ANDROID_external_memory_android_hardware_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = unsafe extern "system" fn(
    device: crate::vk::Device,
    buffer: *const AHardwareBuffer,
    properties: *mut AndroidHardwareBufferPropertiesANDROID,
) -> Result;
#[doc = "Provided by VK_ANDROID_external_memory_android_hardware_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
    buffer: *mut *mut AHardwareBuffer,
) -> Result;
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
#[doc = "Provided by VK_NV_device_diagnostic_checkpoints."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCheckpointNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, checkpoint_marker: *const ffi::c_void);
#[doc = "Provided by VK_NV_device_diagnostic_checkpoints."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "system" fn(
    queue: Queue,
    checkpoint_data_count: *mut u32,
    checkpoint_data: *mut CheckpointDataNV,
);
#[doc = "Provided by VK_EXT_transform_feedback."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const DeviceSize,
    sizes: *const DeviceSize,
);
#[doc = "Provided by VK_EXT_transform_feedback."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    counter_buffers: *const Buffer,
    counter_buffer_offsets: *const DeviceSize,
);
#[doc = "Provided by VK_EXT_transform_feedback."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    counter_buffers: *const Buffer,
    counter_buffer_offsets: *const DeviceSize,
);
#[doc = "Provided by VK_EXT_transform_feedback."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
    index: u32,
);
#[doc = "Provided by VK_EXT_transform_feedback."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    index: u32,
);
#[doc = "Provided by VK_EXT_transform_feedback."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectByteCountEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    instance_count: u32,
    first_instance: u32,
    counter_buffer: Buffer,
    counter_buffer_offset: DeviceSize,
    counter_offset: u32,
    vertex_stride: u32,
);
#[doc = "Provided by VK_NV_scissor_exclusive."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    exclusive_scissors: *const Rect2D,
);
#[doc = "Provided by VK_NV_scissor_exclusive."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetExclusiveScissorEnableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    exclusive_scissor_enables: *const Bool32,
);
#[doc = "Provided by VK_NV_shading_rate_image."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image_view: ImageView,
    image_layout: ImageLayout,
);
#[doc = "Provided by VK_NV_shading_rate_image."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    shading_rate_palettes: *const ShadingRatePaletteNV,
);
#[doc = "Provided by VK_NV_shading_rate_image."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    sample_order_type: CoarseSampleOrderTypeNV,
    custom_sample_order_count: u32,
    custom_sample_orders: *const CoarseSampleOrderCustomNV,
);
#[doc = "Provided by VK_NV_mesh_shader."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, task_count: u32, first_task: u32);
#[doc = "Provided by VK_NV_mesh_shader."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[doc = "Provided by VK_NV_mesh_shader."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
#[doc = "Provided by VK_EXT_mesh_shader."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
#[doc = "Provided by VK_EXT_mesh_shader."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[doc = "Provided by VK_EXT_mesh_shader."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
#[doc = "Provided by VK_NV_ray_tracing."]
#[allow(non_camel_case_types)]
pub type PFN_vkCompileDeferredNV =
    unsafe extern "system" fn(device: crate::vk::Device, pipeline: Pipeline, shader: u32) -> Result;
#[doc = "Provided by VK_NV_ray_tracing."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const AccelerationStructureCreateInfoNV,
    allocator: *const AllocationCallbacks,
    acceleration_structure: *mut AccelerationStructureNV,
) -> Result;
#[doc = "Provided by VK_HUAWEI_invocation_mask."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindInvocationMaskHUAWEI = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image_view: ImageView,
    image_layout: ImageLayout,
);
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    acceleration_structure: AccelerationStructureKHR,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_NV_ray_tracing."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    acceleration_structure: AccelerationStructureNV,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_NV_ray_tracing."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const AccelerationStructureMemoryRequirementsInfoNV,
    memory_requirements: *mut MemoryRequirements2,
);
#[doc = "Provided by VK_NV_ray_tracing."]
#[allow(non_camel_case_types)]
pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    bind_info_count: u32,
    bind_infos: *const BindAccelerationStructureMemoryInfoNV,
) -> Result;
#[doc = "Provided by VK_NV_ray_tracing."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    mode: CopyAccelerationStructureModeKHR,
);
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info: *const CopyAccelerationStructureInfoKHR,
);
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyAccelerationStructureInfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info: *const CopyAccelerationStructureToMemoryInfoKHR,
);
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyAccelerationStructureToMemoryInfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info: *const CopyMemoryToAccelerationStructureInfoKHR,
);
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMemoryToAccelerationStructureInfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    acceleration_structure_count: u32,
    acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
#[doc = "Provided by VK_NV_ray_tracing."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    acceleration_structure_count: u32,
    acceleration_structures: *const AccelerationStructureNV,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
#[doc = "Provided by VK_NV_ray_tracing."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info: *const AccelerationStructureInfoNV,
    instance_data: Buffer,
    instance_offset: DeviceSize,
    update: Bool32,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    scratch: Buffer,
    scratch_offset: DeviceSize,
);
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    acceleration_structure_count: u32,
    acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    data_size: usize,
    data: *mut ffi::c_void,
    stride: usize,
) -> Result;
#[doc = "Provided by VK_KHR_ray_tracing_pipeline."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
);
#[doc = "Provided by VK_NV_ray_tracing."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    raygen_shader_binding_table_buffer: Buffer,
    raygen_shader_binding_offset: DeviceSize,
    miss_shader_binding_table_buffer: Buffer,
    miss_shader_binding_offset: DeviceSize,
    miss_shader_binding_stride: DeviceSize,
    hit_shader_binding_table_buffer: Buffer,
    hit_shader_binding_offset: DeviceSize,
    hit_shader_binding_stride: DeviceSize,
    callable_shader_binding_table_buffer: Buffer,
    callable_shader_binding_offset: DeviceSize,
    callable_shader_binding_stride: DeviceSize,
    width: u32,
    height: u32,
    depth: u32,
);
#[doc = "Provided by VK_KHR_ray_tracing_pipeline."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_KHR_ray_tracing_pipeline."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
    unsafe extern "system" fn(
        device: crate::vk::Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        data: *mut ffi::c_void,
    ) -> Result;
#[doc = "Provided by VK_NV_ray_tracing."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureHandleNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    acceleration_structure: AccelerationStructureNV,
    data_size: usize,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_NV_ray_tracing."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const RayTracingPipelineCreateInfoNV,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> Result;
#[doc = "Provided by VK_KHR_ray_tracing_pipeline."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const RayTracingPipelineCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> Result;
#[doc = "Provided by VK_NV_cooperative_matrix."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    property_count: *mut u32,
    properties: *mut CooperativeMatrixPropertiesNV,
)
    -> Result;
#[doc = "Provided by VK_KHR_ray_tracing_pipeline."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    indirect_device_address: DeviceAddress,
);
#[doc = "Provided by VK_KHR_ray_tracing_maintenance1."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysIndirect2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    indirect_device_address: DeviceAddress,
);
#[doc = "Provided by VK_NV_cluster_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetClusterAccelerationStructureBuildSizesNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const ClusterAccelerationStructureInputInfoNV,
    size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);
#[doc = "Provided by VK_NV_cluster_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildClusterAccelerationStructureIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    command_infos: *const ClusterAccelerationStructureCommandsInfoNV,
);
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    version_info: *const AccelerationStructureVersionInfoKHR,
    compatibility: *mut AccelerationStructureCompatibilityKHR,
);
#[doc = "Provided by VK_KHR_ray_tracing_pipeline."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline: Pipeline,
    group: u32,
    group_shader: ShaderGroupShaderKHR,
) -> DeviceSize;
#[doc = "Provided by VK_KHR_ray_tracing_pipeline."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
    unsafe extern "system" fn(command_buffer: CommandBuffer, pipeline_stack_size: u32);
#[doc = "Provided by VK_NVX_image_view_handle."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewHandleNVX = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const ImageViewHandleInfoNVX,
) -> u32;
#[doc = "Provided by VK_NVX_image_view_handle."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewHandle64NVX = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const ImageViewHandleInfoNVX,
) -> u64;
#[doc = "Provided by VK_NVX_image_view_handle."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
    device: crate::vk::Device,
    image_view: ImageView,
    properties: *mut ImageViewAddressPropertiesNVX,
) -> Result;
#[doc = "Provided by VK_NVX_image_view_handle."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceCombinedImageSamplerIndexNVX = unsafe extern "system" fn(
    device: crate::vk::Device,
    image_view_index: u64,
    sampler_index: u64,
) -> u64;
#[doc = "Provided by VK_EXT_full_screen_exclusive."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    present_mode_count: *mut u32,
    present_modes: *mut PresentModeKHR,
) -> Result;
#[doc = "Provided by VK_EXT_full_screen_exclusive."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> Result;
#[doc = "Provided by VK_EXT_full_screen_exclusive."]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: crate::vk::Device, swapchain: SwapchainKHR) -> Result;
#[doc = "Provided by VK_EXT_full_screen_exclusive."]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: crate::vk::Device, swapchain: SwapchainKHR) -> Result;
#[doc = "Provided by VK_KHR_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        counter_count: *mut u32,
        counters: *mut PerformanceCounterKHR,
        counter_descriptions: *mut PerformanceCounterDescriptionKHR,
    ) -> Result;
#[doc = "Provided by VK_KHR_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
        num_passes: *mut u32,
    );
#[doc = "Provided by VK_KHR_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireProfilingLockKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const AcquireProfilingLockInfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseProfilingLockKHR = unsafe extern "system" fn(device: crate::vk::Device);
#[doc = "Provided by VK_EXT_image_drm_format_modifier."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    properties: *mut ImageDrmFormatModifierPropertiesEXT,
) -> Result;
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferOpaqueCaptureAddress = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const BufferDeviceAddressInfo,
) -> u64;
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferDeviceAddress = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const BufferDeviceAddressInfo,
) -> DeviceAddress;
#[doc = "Provided by VK_EXT_headless_surface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
    instance: crate::vk::Instance,
    create_info: *const HeadlessSurfaceCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> Result;
#[doc = "Provided by VK_NV_coverage_reduction_mode."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        combination_count: *mut u32,
        combinations: *mut FramebufferMixedSamplesCombinationNV,
    ) -> Result;
#[doc = "Provided by VK_INTEL_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn(
    device: crate::vk::Device,
    initialize_info: *const InitializePerformanceApiInfoINTEL,
) -> Result;
#[doc = "Provided by VK_INTEL_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkUninitializePerformanceApiINTEL =
    unsafe extern "system" fn(device: crate::vk::Device);
#[doc = "Provided by VK_INTEL_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    marker_info: *const PerformanceMarkerInfoINTEL,
) -> Result;
#[doc = "Provided by VK_INTEL_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    marker_info: *const PerformanceStreamMarkerInfoINTEL,
) -> Result;
#[doc = "Provided by VK_INTEL_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    override_info: *const PerformanceOverrideInfoINTEL,
) -> Result;
#[doc = "Provided by VK_INTEL_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: crate::vk::Device,
    acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
    configuration: *mut PerformanceConfigurationINTEL,
) -> Result;
#[doc = "Provided by VK_INTEL_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: crate::vk::Device,
    configuration: PerformanceConfigurationINTEL,
) -> Result;
#[doc = "Provided by VK_INTEL_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSetPerformanceConfigurationINTEL =
    unsafe extern "system" fn(queue: Queue, configuration: PerformanceConfigurationINTEL) -> Result;
#[doc = "Provided by VK_INTEL_performance_query."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "system" fn(
    device: crate::vk::Device,
    parameter: PerformanceParameterTypeINTEL,
    value: *mut PerformanceValueINTEL,
) -> Result;
#[doc = "Provided by Vulkan version 1.2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64;
#[doc = "Provided by VK_KHR_pipeline_executable_properties."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_info: *const PipelineInfoKHR,
    executable_count: *mut u32,
    properties: *mut PipelineExecutablePropertiesKHR,
) -> Result;
#[doc = "Provided by VK_KHR_pipeline_executable_properties."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    executable_info: *const PipelineExecutableInfoKHR,
    statistic_count: *mut u32,
    statistics: *mut PipelineExecutableStatisticKHR,
) -> Result;
#[doc = "Provided by VK_KHR_pipeline_executable_properties."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR =
    unsafe extern "system" fn(
        device: crate::vk::Device,
        executable_info: *const PipelineExecutableInfoKHR,
        internal_representation_count: *mut u32,
        internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
    ) -> Result;
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineStipple = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    tool_count: *mut u32,
    tool_properties: *mut PhysicalDeviceToolProperties,
) -> Result;
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const AccelerationStructureCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    acceleration_structure: *mut AccelerationStructureKHR,
) -> Result;
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
);
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    infos: *const AccelerationStructureBuildGeometryInfoKHR,
    indirect_device_addresses: *const DeviceAddress,
    indirect_strides: *const u32,
    pp_max_primitive_counts: *const *const u32,
);
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR =
    unsafe extern "system" fn(
        device: crate::vk::Device,
        info: *const AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress;
#[doc = "Provided by VK_KHR_deferred_host_operations."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    allocator: *const AllocationCallbacks,
    deferred_operation: *mut DeferredOperationKHR,
) -> Result;
#[doc = "Provided by VK_KHR_deferred_host_operations."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    operation: DeferredOperationKHR,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_KHR_deferred_host_operations."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
    unsafe extern "system" fn(device: crate::vk::Device, operation: DeferredOperationKHR) -> u32;
#[doc = "Provided by VK_KHR_deferred_host_operations."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationResultKHR =
    unsafe extern "system" fn(device: crate::vk::Device, operation: DeferredOperationKHR) -> Result;
#[doc = "Provided by VK_KHR_deferred_host_operations."]
#[allow(non_camel_case_types)]
pub type PFN_vkDeferredOperationJoinKHR =
    unsafe extern "system" fn(device: crate::vk::Device, operation: DeferredOperationKHR) -> Result;
#[doc = "Provided by VK_NV_device_generated_commands_compute."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineIndirectMemoryRequirementsNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const ComputePipelineCreateInfo,
    memory_requirements: *mut MemoryRequirements2,
);
#[doc = "Provided by VK_NV_device_generated_commands_compute."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineIndirectDeviceAddressNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const PipelineIndirectDeviceAddressInfoNV,
) -> DeviceAddress;
#[doc = "Provided by VK_AMD_anti_lag."]
#[allow(non_camel_case_types)]
pub type PFN_vkAntiLagUpdateAMD =
    unsafe extern "system" fn(device: crate::vk::Device, data: *const AntiLagDataAMD);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCullMode =
    unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFrontFace =
    unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveTopology =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_topology: PrimitiveTopology);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWithCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    viewport_count: u32,
    viewports: *const Viewport,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetScissorWithCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scissor_count: u32,
    scissors: *const Rect2D,
);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindIndexBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: IndexType,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const DeviceSize,
    sizes: *const DeviceSize,
    strides: *const DeviceSize,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_test_enable: Bool32);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthWriteEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_write_enable: Bool32);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthCompareOp =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_compare_op: CompareOp);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBoundsTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bounds_test_enable: Bool32);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, stencil_test_enable: Bool32);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilOp = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state2."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPatchControlPointsEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRasterizerDiscardEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, rasterizer_discard_enable: Bool32);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBiasEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bias_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state2."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLogicOpEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveRestartEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetTessellationDomainOriginEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    domain_origin: TessellationDomainOrigin,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthClampEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clamp_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPolygonModeEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, polygon_mode: PolygonMode);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRasterizationSamplesEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    rasterization_samples: SampleCountFlags,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetSampleMaskEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    samples: SampleCountFlags,
    sample_mask: *const SampleMask,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetAlphaToCoverageEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_coverage_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetAlphaToOneEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_one_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLogicOpEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorBlendEnableEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    color_blend_enables: *const Bool32,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorBlendEquationEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    color_blend_equations: *const ColorBlendEquationEXT,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorWriteMaskEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    color_write_masks: *const ColorComponentFlags,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRasterizationStreamEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, rasterization_stream: u32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetConservativeRasterizationModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    conservative_rasterization_mode: ConservativeRasterizationModeEXT,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    extra_primitive_overestimation_size: f32,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthClipEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clip_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetSampleLocationsEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, sample_locations_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorBlendAdvancedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    color_blend_advanced: *const ColorBlendAdvancedEXT,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetProvokingVertexModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    provoking_vertex_mode: ProvokingVertexModeEXT,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineRasterizationModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    line_rasterization_mode: LineRasterizationModeEXT,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineStippleEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, stippled_line_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, negative_one_to_one: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWScalingEnableNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, viewport_wscaling_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportSwizzleNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    viewport_swizzles: *const ViewportSwizzleNV,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageToColorEnableNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageToColorLocationNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_location: u32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageModulationModeNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_modulation_mode: CoverageModulationModeNV,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageModulationTableEnableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_modulation_table_enable: Bool32,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageModulationTableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_modulation_table_count: u32,
    coverage_modulation_table: *const f32,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetShadingRateImageEnableNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, shading_rate_image_enable: Bool32);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageReductionModeNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_reduction_mode: CoverageReductionModeNV,
);
#[doc = "Provided by VK_EXT_extended_dynamic_state3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    representative_fragment_test_enable: Bool32,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const PrivateDataSlotCreateInfo,
    allocator: *const AllocationCallbacks,
    private_data_slot: *mut PrivateDataSlot,
) -> Result;
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "system" fn(
    device: crate::vk::Device,
    private_data_slot: PrivateDataSlot,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetPrivateData = unsafe extern "system" fn(
    device: crate::vk::Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: u64,
) -> Result;
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPrivateData = unsafe extern "system" fn(
    device: crate::vk::Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: *mut u64,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_buffer_info: *const CopyBufferInfo2,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_image_info: *const CopyImageInfo2,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBlitImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    blit_image_info: *const BlitImageInfo2,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResolveImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    resolve_image_info: *const ResolveImageInfo2,
);
#[doc = "Provided by VK_KHR_object_refresh."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdRefreshObjectsKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    refresh_objects: *const RefreshObjectListKHR,
);
#[doc = "Provided by VK_KHR_object_refresh."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceRefreshableObjectTypesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    refreshable_object_type_count: *mut u32,
    refreshable_object_types: *mut ObjectType,
) -> Result;
#[doc = "Provided by VK_KHR_fragment_shading_rate."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    fragment_size: *const Extent2D,
    combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2usize],
);
#[doc = "Provided by VK_KHR_fragment_shading_rate."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    fragment_shading_rate_count: *mut u32,
    fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR,
) -> Result;
#[doc = "Provided by VK_NV_fragment_shading_rate_enums."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFragmentShadingRateEnumNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    shading_rate: FragmentShadingRateNV,
    combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2usize],
);
#[doc = "Provided by VK_KHR_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    build_type: AccelerationStructureBuildTypeKHR,
    build_info: *const AccelerationStructureBuildGeometryInfoKHR,
    max_primitive_counts: *const u32,
    size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);
#[doc = "Provided by VK_EXT_vertex_input_dynamic_state."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_binding_description_count: u32,
    vertex_binding_descriptions: *const VertexInputBindingDescription2EXT,
    vertex_attribute_description_count: u32,
    vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT,
);
#[doc = "Provided by VK_EXT_color_write_enable."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    color_write_enables: *const Bool32,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetEvent2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    dependency_info: *const DependencyInfo,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetEvent2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags2,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWaitEvents2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    events: *const Event,
    dependency_infos: *const DependencyInfo,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPipelineBarrier2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dependency_info: *const DependencyInfo,
);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSubmit2 = unsafe extern "system" fn(
    queue: Queue,
    submit_count: u32,
    submits: *const SubmitInfo2,
    fence: Fence,
) -> Result;
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: QueryPool,
    query: u32,
);
#[doc = "Provided by VK_AMD_buffer_marker."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteBufferMarker2AMD = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage: PipelineStageFlags2,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);
#[doc = "Provided by VK_NV_device_diagnostic_checkpoints."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "system" fn(
    queue: Queue,
    checkpoint_data_count: *mut u32,
    checkpoint_data: *mut CheckpointData2NV,
);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMemoryToImage = unsafe extern "system" fn(
    device: crate::vk::Device,
    copy_memory_to_image_info: *const CopyMemoryToImageInfo,
) -> Result;
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyImageToMemory = unsafe extern "system" fn(
    device: crate::vk::Device,
    copy_image_to_memory_info: *const CopyImageToMemoryInfo,
) -> Result;
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyImageToImage = unsafe extern "system" fn(
    device: crate::vk::Device,
    copy_image_to_image_info: *const CopyImageToImageInfo,
) -> Result;
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkTransitionImageLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    transition_count: u32,
    transitions: *const HostImageLayoutTransitionInfo,
) -> Result;
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    video_profile: *const VideoProfileInfoKHR,
    capabilities: *mut VideoCapabilitiesKHR,
) -> Result;
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    video_format_info: *const PhysicalDeviceVideoFormatInfoKHR,
    video_format_property_count: *mut u32,
    video_format_properties: *mut VideoFormatPropertiesKHR,
) -> Result;
#[doc = "Provided by VK_KHR_video_encode_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR,
    ) -> Result;
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const VideoSessionCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    video_session: *mut VideoSessionKHR,
) -> Result;
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    video_session: VideoSessionKHR,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const VideoSessionParametersCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    video_session_parameters: *mut VideoSessionParametersKHR,
) -> Result;
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    video_session_parameters: VideoSessionParametersKHR,
    update_info: *const VideoSessionParametersUpdateInfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_video_encode_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetEncodedVideoSessionParametersKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR,
    feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
    data_size: *mut usize,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    video_session_parameters: VideoSessionParametersKHR,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    video_session: VideoSessionKHR,
    memory_requirements_count: *mut u32,
    memory_requirements: *mut VideoSessionMemoryRequirementsKHR,
) -> Result;
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    video_session: VideoSessionKHR,
    bind_session_memory_info_count: u32,
    bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_video_decode_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDecodeVideoKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    decode_info: *const VideoDecodeInfoKHR,
);
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    begin_info: *const VideoBeginCodingInfoKHR,
);
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coding_control_info: *const VideoCodingControlInfoKHR,
);
#[doc = "Provided by VK_KHR_video_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    end_coding_info: *const VideoEndCodingInfoKHR,
);
#[doc = "Provided by VK_KHR_video_encode_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEncodeVideoKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    encode_info: *const VideoEncodeInfoKHR,
);
#[doc = "Provided by VK_NV_memory_decompression."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDecompressMemoryNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    decompress_region_count: u32,
    decompress_memory_regions: *const DecompressMemoryRegionNV,
);
#[doc = "Provided by VK_NV_memory_decompression."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDecompressMemoryIndirectCountNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    stride: u32,
);
#[doc = "Provided by VK_NV_partitioned_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const PartitionedAccelerationStructureInstancesInputNV,
    size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);
#[doc = "Provided by VK_NV_partitioned_acceleration_structure."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildPartitionedAccelerationStructuresNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    build_info: *const BuildPartitionedAccelerationStructureInfoNV,
);
#[doc = "Provided by VK_EXT_memory_decompression."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDecompressMemoryEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    decompress_memory_info_ext: *const DecompressMemoryInfoEXT,
);
#[doc = "Provided by VK_EXT_memory_decompression."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDecompressMemoryIndirectCountEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    decompression_method: MemoryDecompressionMethodFlagsEXT,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    max_decompression_count: u32,
    stride: u32,
);
#[doc = "Provided by VK_NVX_binary_import."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const CuModuleCreateInfoNVX,
    allocator: *const AllocationCallbacks,
    module: *mut CuModuleNVX,
) -> Result;
#[doc = "Provided by VK_NVX_binary_import."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const CuFunctionCreateInfoNVX,
    allocator: *const AllocationCallbacks,
    function: *mut CuFunctionNVX,
) -> Result;
#[doc = "Provided by VK_NVX_binary_import."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCuModuleNVX = unsafe extern "system" fn(
    device: crate::vk::Device,
    module: CuModuleNVX,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_NVX_binary_import."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "system" fn(
    device: crate::vk::Device,
    function: CuFunctionNVX,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_NVX_binary_import."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCuLaunchKernelNVX =
    unsafe extern "system" fn(command_buffer: CommandBuffer, launch_info: *const CuLaunchInfoNVX);
#[doc = "Provided by VK_EXT_descriptor_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutSizeEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    layout: DescriptorSetLayout,
    layout_size_in_bytes: *mut DeviceSize,
);
#[doc = "Provided by VK_EXT_descriptor_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutBindingOffsetEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    layout: DescriptorSetLayout,
    binding: u32,
    offset: *mut DeviceSize,
);
#[doc = "Provided by VK_EXT_descriptor_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_info: *const DescriptorGetInfoEXT,
    data_size: usize,
    descriptor: *mut ffi::c_void,
);
#[doc = "Provided by VK_EXT_descriptor_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorBuffersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer_count: u32,
    binding_infos: *const DescriptorBufferBindingInfoEXT,
);
#[doc = "Provided by VK_EXT_descriptor_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDescriptorBufferOffsetsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    set_count: u32,
    buffer_indices: *const u32,
    offsets: *const DeviceSize,
);
#[doc = "Provided by VK_EXT_descriptor_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
);
#[doc = "Provided by VK_EXT_descriptor_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const BufferCaptureDescriptorDataInfoEXT,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_EXT_descriptor_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const ImageCaptureDescriptorDataInfoEXT,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_EXT_descriptor_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const ImageViewCaptureDescriptorDataInfoEXT,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_EXT_descriptor_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const SamplerCaptureDescriptorDataInfoEXT,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_EXT_descriptor_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT =
    unsafe extern "system" fn(
        device: crate::vk::Device,
        info: *const AccelerationStructureCaptureDescriptorDataInfoEXT,
        data: *mut ffi::c_void,
    ) -> Result;
#[doc = "Provided by VK_EXT_pageable_device_local_memory."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetDeviceMemoryPriorityEXT =
    unsafe extern "system" fn(device: crate::vk::Device, memory: DeviceMemory, priority: f32);
#[doc = "Provided by VK_EXT_acquire_drm_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireDrmDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    drm_fd: i32,
    display: DisplayKHR,
) -> Result;
#[doc = "Provided by VK_EXT_acquire_drm_display."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDrmDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    drm_fd: i32,
    connector_id: u32,
    display: *mut DisplayKHR,
) -> Result;
#[doc = "Provided by VK_KHR_present_wait2."]
#[allow(non_camel_case_types)]
pub type PFN_vkWaitForPresent2KHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    present_wait2_info: *const PresentWait2InfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_present_wait."]
#[allow(non_camel_case_types)]
pub type PFN_vkWaitForPresentKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    present_id: u64,
    timeout: u64,
) -> Result;
#[doc = "Provided by VK_FUCHSIA_buffer_collection."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const BufferCollectionCreateInfoFUCHSIA,
    allocator: *const AllocationCallbacks,
    collection: *mut BufferCollectionFUCHSIA,
) -> Result;
#[doc = "Provided by VK_FUCHSIA_buffer_collection."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(
    device: crate::vk::Device,
    collection: BufferCollectionFUCHSIA,
    buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
) -> Result;
#[doc = "Provided by VK_FUCHSIA_buffer_collection."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(
    device: crate::vk::Device,
    collection: BufferCollectionFUCHSIA,
    image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
) -> Result;
#[doc = "Provided by VK_FUCHSIA_buffer_collection."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: crate::vk::Device,
    collection: BufferCollectionFUCHSIA,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_FUCHSIA_buffer_collection."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(
    device: crate::vk::Device,
    collection: BufferCollectionFUCHSIA,
    properties: *mut BufferCollectionPropertiesFUCHSIA,
) -> Result;
#[doc = "Provided by VK_NV_cuda_kernel_launch."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCudaModuleNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const CudaModuleCreateInfoNV,
    allocator: *const AllocationCallbacks,
    module: *mut CudaModuleNV,
) -> Result;
#[doc = "Provided by VK_NV_cuda_kernel_launch."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetCudaModuleCacheNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    module: CudaModuleNV,
    cache_size: *mut usize,
    cache_data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_NV_cuda_kernel_launch."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCudaFunctionNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const CudaFunctionCreateInfoNV,
    allocator: *const AllocationCallbacks,
    function: *mut CudaFunctionNV,
) -> Result;
#[doc = "Provided by VK_NV_cuda_kernel_launch."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCudaModuleNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    module: CudaModuleNV,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_NV_cuda_kernel_launch."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCudaFunctionNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    function: CudaFunctionNV,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_NV_cuda_kernel_launch."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCudaLaunchKernelNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, launch_info: *const CudaLaunchInfoNV);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRendering =
    unsafe extern "system" fn(command_buffer: CommandBuffer, rendering_info: *const RenderingInfo);
#[doc = "Provided by Vulkan version 1.3."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRendering = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[doc = "Provided by VK_KHR_maintenance10."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRendering2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    rendering_end_info: *const RenderingEndInfoKHR,
);
#[doc = "Provided by VK_VALVE_descriptor_set_host_mapping."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "system" fn(
    device: crate::vk::Device,
    binding_reference: *const DescriptorSetBindingReferenceVALVE,
    host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE,
);
#[doc = "Provided by VK_VALVE_descriptor_set_host_mapping."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetHostMappingVALVE = unsafe extern "system" fn(
    device: crate::vk::Device,
    descriptor_set: DescriptorSet,
    pp_data: *mut *mut ffi::c_void,
);
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMicromapEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const MicromapCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    micromap: *mut MicromapEXT,
) -> Result;
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildMicromapsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    infos: *const MicromapBuildInfoEXT,
);
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkBuildMicromapsEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    infos: *const MicromapBuildInfoEXT,
) -> Result;
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyMicromapEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    micromap: MicromapEXT,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMicromapEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, info: *const CopyMicromapInfoEXT);
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMicromapEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMicromapInfoEXT,
) -> Result;
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info: *const CopyMicromapToMemoryInfoEXT,
);
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMicromapToMemoryInfoEXT,
) -> Result;
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info: *const CopyMemoryToMicromapInfoEXT,
);
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMemoryToMicromapInfoEXT,
) -> Result;
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    micromap_count: u32,
    micromaps: *const MicromapEXT,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    micromap_count: u32,
    micromaps: *const MicromapEXT,
    query_type: QueryType,
    data_size: usize,
    data: *mut ffi::c_void,
    stride: usize,
) -> Result;
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMicromapCompatibilityEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    version_info: *const MicromapVersionInfoEXT,
    compatibility: *mut AccelerationStructureCompatibilityKHR,
);
#[doc = "Provided by VK_EXT_opacity_micromap."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMicromapBuildSizesEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    build_type: AccelerationStructureBuildTypeKHR,
    build_info: *const MicromapBuildInfoEXT,
    size_info: *mut MicromapBuildSizesInfoEXT,
);
#[doc = "Provided by VK_EXT_shader_module_identifier."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderModuleIdentifierEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    shader_module: ShaderModule,
    identifier: *mut ShaderModuleIdentifierEXT,
);
#[doc = "Provided by VK_EXT_shader_module_identifier."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const ShaderModuleCreateInfo,
    identifier: *mut ShaderModuleIdentifierEXT,
);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSubresourceLayout2 = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    subresource: *const ImageSubresource2,
    layout: *mut SubresourceLayout2,
);
#[doc = "Provided by VK_EXT_pipeline_properties."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_info: *const PipelineInfoKHR,
    pipeline_properties: *mut BaseOutStructure,
) -> Result;
#[doc = "Provided by VK_EXT_metal_objects."]
#[allow(non_camel_case_types)]
pub type PFN_vkExportMetalObjectsEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    metal_objects_info: *mut ExportMetalObjectsInfoEXT,
);
#[doc = "Provided by VK_QCOM_tile_memory_heap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindTileMemoryQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    tile_memory_bind_info: *const TileMemoryBindInfoQCOM,
);
#[doc = "Provided by VK_QCOM_tile_properties."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetFramebufferTilePropertiesQCOM = unsafe extern "system" fn(
    device: crate::vk::Device,
    framebuffer: Framebuffer,
    properties_count: *mut u32,
    properties: *mut TilePropertiesQCOM,
) -> Result;
#[doc = "Provided by VK_QCOM_tile_properties."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = unsafe extern "system" fn(
    device: crate::vk::Device,
    rendering_info: *const RenderingInfo,
    properties: *mut TilePropertiesQCOM,
) -> Result;
#[doc = "Provided by VK_NV_optical_flow."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV,
    format_count: *mut u32,
    image_format_properties: *mut OpticalFlowImageFormatPropertiesNV,
) -> Result;
#[doc = "Provided by VK_NV_optical_flow."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateOpticalFlowSessionNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const OpticalFlowSessionCreateInfoNV,
    allocator: *const AllocationCallbacks,
    session: *mut OpticalFlowSessionNV,
) -> Result;
#[doc = "Provided by VK_NV_optical_flow."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyOpticalFlowSessionNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    session: OpticalFlowSessionNV,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_NV_optical_flow."]
#[allow(non_camel_case_types)]
pub type PFN_vkBindOpticalFlowSessionImageNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    session: OpticalFlowSessionNV,
    binding_point: OpticalFlowSessionBindingPointNV,
    view: ImageView,
    layout: ImageLayout,
) -> Result;
#[doc = "Provided by VK_NV_optical_flow."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdOpticalFlowExecuteNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    session: OpticalFlowSessionNV,
    execute_info: *const OpticalFlowExecuteInfoNV,
);
#[doc = "Provided by VK_EXT_device_fault."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceFaultInfoEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    fault_counts: *mut DeviceFaultCountsEXT,
    fault_info: *mut DeviceFaultInfoEXT,
) -> Result;
#[doc = "Provided by VK_KHR_device_fault."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceFaultReportsKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    timeout: u64,
    fault_counts: *mut u32,
    fault_info: *mut DeviceFaultInfoKHR,
) -> Result;
#[doc = "Provided by VK_KHR_device_fault."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceFaultDebugInfoKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    debug_info: *mut DeviceFaultDebugInfoKHR,
) -> Result;
#[doc = "Provided by VK_EXT_depth_bias_control."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBias2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    depth_bias_info: *const DepthBiasInfoEXT,
);
#[doc = "Provided by VK_KHR_swapchain_maintenance1."]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseSwapchainImagesKHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    release_info: *const ReleaseSwapchainImagesInfoKHR,
) -> Result;
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageSubresourceLayout = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const DeviceImageSubresourceInfo,
    layout: *mut SubresourceLayout2,
);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkMapMemory2 = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory_map_info: *const MemoryMapInfo,
    pp_data: *mut *mut ffi::c_void,
) -> Result;
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkUnmapMemory2 = unsafe extern "system" fn(
    device: crate::vk::Device,
    memory_unmap_info: *const MemoryUnmapInfo,
) -> Result;
#[doc = "Provided by VK_EXT_shader_object."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateShadersEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info_count: u32,
    create_infos: *const ShaderCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    shaders: *mut ShaderEXT,
) -> Result;
#[doc = "Provided by VK_EXT_shader_object."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyShaderEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    shader: ShaderEXT,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_EXT_shader_object."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderBinaryDataEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    shader: ShaderEXT,
    data_size: *mut usize,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_EXT_shader_object."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindShadersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage_count: u32,
    stages: *const ShaderStageFlags,
    shaders: *const ShaderEXT,
);
#[doc = "Provided by VK_EXT_present_timing."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetSwapchainPresentTimingQueueSizeEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    size: u32,
) -> Result;
#[doc = "Provided by VK_EXT_present_timing."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainTimingPropertiesEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT,
    swapchain_timing_properties_counter: *mut u64,
) -> Result;
#[doc = "Provided by VK_EXT_present_timing."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainTimeDomainPropertiesEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT,
    time_domains_counter: *mut u64,
) -> Result;
#[doc = "Provided by VK_EXT_present_timing."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPastPresentationTimingEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    past_presentation_timing_info: *const PastPresentationTimingInfoEXT,
    past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT,
) -> Result;
#[doc = "Provided by VK_QNX_external_memory_screen_buffer."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetScreenBufferPropertiesQNX = unsafe extern "system" fn(
    device: crate::vk::Device,
    buffer: *const _screen_buffer,
    properties: *mut ScreenBufferPropertiesQNX,
) -> Result;
#[doc = "Provided by VK_KHR_cooperative_matrix."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        property_count: *mut u32,
        properties: *mut CooperativeMatrixPropertiesKHR,
    ) -> Result;
#[doc = "Provided by VK_AMDX_shader_enqueue."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetExecutionGraphPipelineScratchSizeAMDX = unsafe extern "system" fn(
    device: crate::vk::Device,
    execution_graph: Pipeline,
    size_info: *mut ExecutionGraphPipelineScratchSizeAMDX,
) -> Result;
#[doc = "Provided by VK_AMDX_shader_enqueue."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetExecutionGraphPipelineNodeIndexAMDX = unsafe extern "system" fn(
    device: crate::vk::Device,
    execution_graph: Pipeline,
    node_info: *const PipelineShaderStageNodeCreateInfoAMDX,
    node_index: *mut u32,
) -> Result;
#[doc = "Provided by VK_AMDX_shader_enqueue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateExecutionGraphPipelinesAMDX = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const ExecutionGraphPipelineCreateInfoAMDX,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> Result;
#[doc = "Provided by VK_AMDX_shader_enqueue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdInitializeGraphScratchMemoryAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    execution_graph: Pipeline,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
);
#[doc = "Provided by VK_AMDX_shader_enqueue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchGraphAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: *const DispatchGraphCountInfoAMDX,
);
#[doc = "Provided by VK_AMDX_shader_enqueue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchGraphIndirectAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: *const DispatchGraphCountInfoAMDX,
);
#[doc = "Provided by VK_AMDX_shader_enqueue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchGraphIndirectCountAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: DeviceAddress,
);
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateGpaSessionAMD = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const GpaSessionCreateInfoAMD,
    allocator: *const AllocationCallbacks,
    gpa_session: *mut GpaSessionAMD,
) -> Result;
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyGpaSessionAMD = unsafe extern "system" fn(
    device: crate::vk::Device,
    gpa_session: GpaSessionAMD,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetGpaDeviceClockModeAMD = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *mut GpaDeviceClockModeInfoAMD,
) -> Result;
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetGpaDeviceClockInfoAMD = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *mut GpaDeviceGetClockInfoAMD,
) -> Result;
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginGpaSessionAMD =
    unsafe extern "system" fn(command_buffer: CommandBuffer, gpa_session: GpaSessionAMD) -> Result;
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndGpaSessionAMD =
    unsafe extern "system" fn(command_buffer: CommandBuffer, gpa_session: GpaSessionAMD) -> Result;
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginGpaSampleAMD = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    gpa_session: GpaSessionAMD,
    gpa_sample_begin_info: *const GpaSampleBeginInfoAMD,
    sample_id: *mut u32,
) -> Result;
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndGpaSampleAMD = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    gpa_session: GpaSessionAMD,
    sample_id: u32,
);
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetGpaSessionStatusAMD =
    unsafe extern "system" fn(device: crate::vk::Device, gpa_session: GpaSessionAMD) -> Result;
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetGpaSessionResultsAMD = unsafe extern "system" fn(
    device: crate::vk::Device,
    gpa_session: GpaSessionAMD,
    sample_id: u32,
    size_in_bytes: *mut usize,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkResetGpaSessionAMD =
    unsafe extern "system" fn(device: crate::vk::Device, gpa_session: GpaSessionAMD) -> Result;
#[doc = "Provided by VK_AMD_gpa_interface."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyGpaSessionResultsAMD =
    unsafe extern "system" fn(command_buffer: CommandBuffer, gpa_session: GpaSessionAMD);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorSets2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushConstants2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    push_constants_info: *const PushConstantsInfo,
);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSet2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    push_descriptor_set_info: *const PushDescriptorSetInfo,
);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetWithTemplate2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo,
);
#[doc = "Provided by VK_KHR_maintenance6."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT,
);
#[doc = "Provided by VK_KHR_maintenance6."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = unsafe extern "system" fn (command_buffer : CommandBuffer , bind_descriptor_buffer_embedded_samplers_info : * const BindDescriptorBufferEmbeddedSamplersInfoEXT ,) ;
#[doc = "Provided by VK_NV_low_latency2."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetLatencySleepModeNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    sleep_mode_info: *const LatencySleepModeInfoNV,
) -> Result;
#[doc = "Provided by VK_NV_low_latency2."]
#[allow(non_camel_case_types)]
pub type PFN_vkLatencySleepNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    sleep_info: *const LatencySleepInfoNV,
) -> Result;
#[doc = "Provided by VK_NV_low_latency2."]
#[allow(non_camel_case_types)]
pub type PFN_vkSetLatencyMarkerNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    latency_marker_info: *const SetLatencyMarkerInfoNV,
);
#[doc = "Provided by VK_NV_low_latency2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetLatencyTimingsNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    swapchain: SwapchainKHR,
    latency_marker_info: *mut GetLatencyMarkerInfoNV,
);
#[doc = "Provided by VK_NV_low_latency2."]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueNotifyOutOfBandNV =
    unsafe extern "system" fn(queue: Queue, queue_type_info: *const OutOfBandQueueTypeInfoNV);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRenderingAttachmentLocations = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    location_info: *const RenderingAttachmentLocationInfo,
);
#[doc = "Provided by Vulkan version 1.4."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRenderingInputAttachmentIndices = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
);
#[doc = "Provided by VK_EXT_shader_object."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthClampRangeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    depth_clamp_mode: DepthClampModeEXT,
    depth_clamp_range: *const DepthClampRangeEXT,
);
#[doc = "Provided by VK_NV_cooperative_matrix2."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        property_count: *mut u32,
        properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV,
    ) -> Result;
#[doc = "Provided by VK_EXT_external_memory_metal."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryMetalHandleEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    get_metal_handle_info: *const MemoryGetMetalHandleInfoEXT,
    handle: *mut *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_EXT_external_memory_metal."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryMetalHandlePropertiesEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    handle: *const ffi::c_void,
    memory_metal_handle_properties: *mut MemoryMetalHandlePropertiesEXT,
) -> Result;
#[doc = "Provided by VK_NV_cooperative_vector."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    property_count: *mut u32,
    properties: *mut CooperativeVectorPropertiesNV,
)
    -> Result;
#[doc = "Provided by VK_NV_cooperative_vector."]
#[allow(non_camel_case_types)]
pub type PFN_vkConvertCooperativeVectorMatrixNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const ConvertCooperativeVectorMatrixInfoNV,
) -> Result;
#[doc = "Provided by VK_NV_cooperative_vector."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdConvertCooperativeVectorMatrixNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    infos: *const ConvertCooperativeVectorMatrixInfoNV,
);
#[doc = "Provided by VK_QCOM_tile_shading."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchTileQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dispatch_tile_info: *const DispatchTileInfoQCOM,
);
#[doc = "Provided by VK_QCOM_tile_shading."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginPerTileExecutionQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    per_tile_begin_info: *const PerTileBeginInfoQCOM,
);
#[doc = "Provided by VK_QCOM_tile_shading."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndPerTileExecutionQCOM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    per_tile_end_info: *const PerTileEndInfoQCOM,
);
#[doc = "Provided by VK_NV_external_compute_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateExternalComputeQueueNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const ExternalComputeQueueCreateInfoNV,
    allocator: *const AllocationCallbacks,
    external_queue: *mut ExternalComputeQueueNV,
) -> Result;
#[doc = "Provided by VK_NV_external_compute_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyExternalComputeQueueNV = unsafe extern "system" fn(
    device: crate::vk::Device,
    external_queue: ExternalComputeQueueNV,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_NV_external_compute_queue."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetExternalComputeQueueDataNV = unsafe extern "system" fn(
    external_queue: ExternalComputeQueueNV,
    params: *mut ExternalComputeQueueDataParamsNV,
    data: *mut ffi::c_void,
);
#[doc = "Provided by VK_ARM_shader_instrumentation."]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        description_count: *mut u32,
        descriptions: *mut ShaderInstrumentationMetricDescriptionARM,
    ) -> Result;
#[doc = "Provided by VK_ARM_shader_instrumentation."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateShaderInstrumentationARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const ShaderInstrumentationCreateInfoARM,
    allocator: *const AllocationCallbacks,
    instrumentation: *mut ShaderInstrumentationARM,
) -> Result;
#[doc = "Provided by VK_ARM_shader_instrumentation."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyShaderInstrumentationARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    instrumentation: ShaderInstrumentationARM,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_ARM_shader_instrumentation."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginShaderInstrumentationARM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    instrumentation: ShaderInstrumentationARM,
);
#[doc = "Provided by VK_ARM_shader_instrumentation."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndShaderInstrumentationARM =
    unsafe extern "system" fn(command_buffer: CommandBuffer);
#[doc = "Provided by VK_ARM_shader_instrumentation."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderInstrumentationValuesARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    instrumentation: ShaderInstrumentationARM,
    metric_block_count: *mut u32,
    metric_values: *mut ffi::c_void,
    flags: ShaderInstrumentationValuesFlagsARM,
) -> Result;
#[doc = "Provided by VK_ARM_shader_instrumentation."]
#[allow(non_camel_case_types)]
pub type PFN_vkClearShaderInstrumentationMetricsARM =
    unsafe extern "system" fn(device: crate::vk::Device, instrumentation: ShaderInstrumentationARM);
#[doc = "Provided by VK_ARM_tensors."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateTensorARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const TensorCreateInfoARM,
    allocator: *const AllocationCallbacks,
    tensor: *mut TensorARM,
) -> Result;
#[doc = "Provided by VK_ARM_tensors."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyTensorARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    tensor: TensorARM,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_ARM_tensors."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateTensorViewARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const TensorViewCreateInfoARM,
    allocator: *const AllocationCallbacks,
    view: *mut TensorViewARM,
) -> Result;
#[doc = "Provided by VK_ARM_tensors."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyTensorViewARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    tensor_view: TensorViewARM,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_ARM_tensors."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const TensorMemoryRequirementsInfoARM,
    memory_requirements: *mut MemoryRequirements2,
);
#[doc = "Provided by VK_ARM_tensors."]
#[allow(non_camel_case_types)]
pub type PFN_vkBindTensorMemoryARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    bind_info_count: u32,
    bind_infos: *const BindTensorMemoryInfoARM,
) -> Result;
#[doc = "Provided by VK_ARM_tensors."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const DeviceTensorMemoryRequirementsARM,
    memory_requirements: *mut MemoryRequirements2,
);
#[doc = "Provided by VK_ARM_tensors."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyTensorARM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_tensor_info: *const CopyTensorInfoARM,
);
#[doc = "Provided by VK_ARM_tensors."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetTensorOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const TensorCaptureDescriptorDataInfoARM,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_ARM_tensors."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const TensorViewCaptureDescriptorDataInfoARM,
    data: *mut ffi::c_void,
) -> Result;
#[doc = "Provided by VK_ARM_tensors."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM,
    external_tensor_properties: *mut ExternalTensorPropertiesARM,
);
#[doc = "Provided by VK_ARM_data_graph."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDataGraphPipelinesARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const DataGraphPipelineCreateInfoARM,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> Result;
#[doc = "Provided by VK_ARM_data_graph."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDataGraphPipelineSessionARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const DataGraphPipelineSessionCreateInfoARM,
    allocator: *const AllocationCallbacks,
    session: *mut DataGraphPipelineSessionARM,
) -> Result;
#[doc = "Provided by VK_ARM_data_graph."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM =
    unsafe extern "system" fn(
        device: crate::vk::Device,
        info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM,
        bind_point_requirement_count: *mut u32,
        bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM,
    ) -> Result;
#[doc = "Provided by VK_ARM_data_graph."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM,
    memory_requirements: *mut MemoryRequirements2,
);
#[doc = "Provided by VK_ARM_data_graph."]
#[allow(non_camel_case_types)]
pub type PFN_vkBindDataGraphPipelineSessionMemoryARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    bind_info_count: u32,
    bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM,
) -> Result;
#[doc = "Provided by VK_ARM_data_graph."]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDataGraphPipelineSessionARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    session: DataGraphPipelineSessionARM,
    allocator: *const AllocationCallbacks,
);
#[doc = "Provided by VK_ARM_data_graph."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchDataGraphARM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    session: DataGraphPipelineSessionARM,
    info: *const DataGraphPipelineDispatchInfoARM,
);
#[doc = "Provided by VK_ARM_data_graph."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDataGraphPipelineAvailablePropertiesARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_info: *const DataGraphPipelineInfoARM,
    properties_count: *mut u32,
    properties: *mut DataGraphPipelinePropertyARM,
) -> Result;
#[doc = "Provided by VK_ARM_data_graph."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDataGraphPipelinePropertiesARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    pipeline_info: *const DataGraphPipelineInfoARM,
    properties_count: u32,
    properties: *mut DataGraphPipelinePropertyQueryResultARM,
) -> Result;
#[doc = "Provided by VK_ARM_data_graph."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_property_count: *mut u32,
        queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM,
    ) -> Result;
#[doc = "Provided by VK_ARM_data_graph."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM = unsafe extern "system" fn (physical_device : PhysicalDevice , queue_family_data_graph_processing_engine_info : * const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM , queue_family_data_graph_processing_engine_properties : * mut QueueFamilyDataGraphProcessingEnginePropertiesARM ,) ;
#[doc = "Provided by VK_OHOS_external_memory."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetNativeBufferPropertiesOHOS = unsafe extern "system" fn(
    device: crate::vk::Device,
    buffer: *const OH_NativeBuffer,
    properties: *mut NativeBufferPropertiesOHOS,
) -> Result;
#[doc = "Provided by VK_OHOS_external_memory."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryNativeBufferOHOS = unsafe extern "system" fn(
    device: crate::vk::Device,
    info: *const MemoryGetNativeBufferInfoOHOS,
    buffer: *mut *mut OH_NativeBuffer,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainGrallocUsageOHOS = unsafe extern "system" fn(
    device: crate::vk::Device,
    format: Format,
    image_usage: ImageUsageFlags,
    gralloc_usage: *mut u64,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireImageOHOS = unsafe extern "system" fn(
    device: crate::vk::Device,
    image: Image,
    native_fence_fd: i32,
    semaphore: Semaphore,
    fence: Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSignalReleaseImageOHOS = unsafe extern "system" fn(
    queue: Queue,
    wait_semaphore_count: u32,
    wait_semaphores: *const Semaphore,
    image: Image,
    native_fence_fd: *mut i32,
) -> Result;
#[doc = "Provided by VK_QCOM_queue_perf_hint."]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSetPerfHintQCOM =
    unsafe extern "system" fn(queue: Queue, perf_hint_info: *const PerfHintInfoQCOM) -> Result;
#[doc = "Provided by VK_ARM_performance_counters_by_region."]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        counter_count: *mut u32,
        counters: *mut PerformanceCounterARM,
        counter_descriptions: *mut PerformanceCounterDescriptionARM,
    ) -> Result;
#[doc = "Provided by VK_NV_compute_occupancy_priority."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetComputeOccupancyPriorityNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    parameters: *const ComputeOccupancyPriorityParametersNV,
);
#[doc = "Provided by VK_EXT_descriptor_heap."]
#[allow(non_camel_case_types)]
pub type PFN_vkWriteSamplerDescriptorsEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    sampler_count: u32,
    samplers: *const SamplerCreateInfo,
    descriptors: *const HostAddressRangeEXT,
) -> Result;
#[doc = "Provided by VK_EXT_descriptor_heap."]
#[allow(non_camel_case_types)]
pub type PFN_vkWriteResourceDescriptorsEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    resource_count: u32,
    resources: *const ResourceDescriptorInfoEXT,
    descriptors: *const HostAddressRangeEXT,
) -> Result;
#[doc = "Provided by VK_EXT_descriptor_heap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindSamplerHeapEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, bind_info: *const BindHeapInfoEXT);
#[doc = "Provided by VK_EXT_descriptor_heap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindResourceHeapEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, bind_info: *const BindHeapInfoEXT);
#[doc = "Provided by VK_EXT_descriptor_heap."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDataEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    push_data_info: *const PushDataInfoEXT,
);
#[doc = "Provided by VK_EXT_descriptor_heap."]
#[allow(non_camel_case_types)]
pub type PFN_vkRegisterCustomBorderColorEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    border_color: *const SamplerCustomBorderColorCreateInfoEXT,
    request_index: Bool32,
    index: *mut u32,
) -> Result;
#[doc = "Provided by VK_EXT_descriptor_heap."]
#[allow(non_camel_case_types)]
pub type PFN_vkUnregisterCustomBorderColorEXT =
    unsafe extern "system" fn(device: crate::vk::Device, index: u32);
#[doc = "Provided by VK_EXT_descriptor_heap."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageOpaqueCaptureDataEXT = unsafe extern "system" fn(
    device: crate::vk::Device,
    image_count: u32,
    images: *const Image,
    datas: *mut HostAddressRangeEXT,
) -> Result;
#[doc = "Provided by VK_EXT_descriptor_heap."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDescriptorSizeEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    descriptor_type: DescriptorType,
) -> DeviceSize;
#[doc = "Provided by VK_EXT_descriptor_heap."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetTensorOpaqueCaptureDataARM = unsafe extern "system" fn(
    device: crate::vk::Device,
    tensor_count: u32,
    tensors: *const TensorARM,
    datas: *mut HostAddressRangeEXT,
) -> Result;
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_memory_info: *const CopyDeviceMemoryInfoKHR,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryToImageKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_memory_info: *const CopyDeviceMemoryImageInfoKHR,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImageToMemoryKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_memory_info: *const CopyDeviceMemoryImageInfoKHR,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdUpdateMemoryKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_range: *const DeviceAddressRangeKHR,
    dst_flags: AddressCommandFlagsKHR,
    data_size: DeviceSize,
    data: *const ffi::c_void,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdFillMemoryKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_range: *const DeviceAddressRangeKHR,
    dst_flags: AddressCommandFlagsKHR,
    data: u32,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyQueryPoolResultsToMemoryKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    dst_range: *const StridedDeviceAddressRangeKHR,
    dst_flags: AddressCommandFlagsKHR,
    query_result_flags: QueryResultFlags,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginConditionalRendering2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    conditional_rendering_begin: *const ConditionalRenderingBeginInfo2EXT,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindTransformFeedbackBuffers2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    binding_infos: *const BindTransformFeedbackBuffer2InfoEXT,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginTransformFeedback2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_range: u32,
    counter_range_count: u32,
    counter_infos: *const BindTransformFeedbackBuffer2InfoEXT,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndTransformFeedback2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_range: u32,
    counter_range_count: u32,
    counter_infos: *const BindTransformFeedbackBuffer2InfoEXT,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectByteCount2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    instance_count: u32,
    first_instance: u32,
    counter_info: *const BindTransformFeedbackBuffer2InfoEXT,
    counter_offset: u32,
    vertex_stride: u32,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteMarkerToMemoryAMD =
    unsafe extern "system" fn(command_buffer: CommandBuffer, info: *const MemoryMarkerInfoAMD);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindIndexBuffer3KHR =
    unsafe extern "system" fn(command_buffer: CommandBuffer, info: *const BindIndexBuffer3InfoKHR);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindVertexBuffers3KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    binding_infos: *const BindVertexBuffer3InfoKHR,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirect2KHR =
    unsafe extern "system" fn(command_buffer: CommandBuffer, info: *const DrawIndirect2InfoKHR);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirect2KHR =
    unsafe extern "system" fn(command_buffer: CommandBuffer, info: *const DrawIndirect2InfoKHR);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectCount2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info: *const DrawIndirectCount2InfoKHR,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirectCount2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info: *const DrawIndirectCount2InfoKHR,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirect2EXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, info: *const DrawIndirect2InfoKHR);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectCount2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info: *const DrawIndirectCount2InfoKHR,
);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchIndirect2KHR =
    unsafe extern "system" fn(command_buffer: CommandBuffer, info: *const DispatchIndirect2InfoKHR);
#[doc = "Provided by VK_KHR_device_address_commands."]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAccelerationStructure2KHR = unsafe extern "system" fn(
    device: crate::vk::Device,
    create_info: *const AccelerationStructureCreateInfo2KHR,
    allocator: *const AllocationCallbacks,
    acceleration_structure: *mut AccelerationStructureKHR,
) -> Result;
#[doc = "Provided by VK_ARM_data_graph_instruction_set_tosa."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM,
        properties: *mut BaseOutStructure,
    ) -> Result;
#[doc = "Provided by VK_ARM_scheduling_controls."]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDispatchParametersARM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dispatch_parameters: *const DispatchParametersARM,
);
#[doc = "Provided by VK_ARM_data_graph_optical_flow."]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM,
        optical_flow_image_format_info: *const DataGraphOpticalFlowImageFormatInfoARM,
        format_count: *mut u32,
        image_format_properties: *mut DataGraphOpticalFlowImageFormatPropertiesARM,
    ) -> Result;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlagsKHR.html>"]
pub type MemoryUnmapFlagsKHR = MemoryUnmapFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagsNV.html>"]
pub type GeometryFlagsNV = GeometryFlagsKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagsNV.html>"]
pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagsNV.html>"]
pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateFlagsEXT.html>"]
pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateCreateFlagsKHR.html>"]
pub type DescriptorUpdateTemplateCreateFlagsKHR = DescriptorUpdateTemplateCreateFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlagsEXT.html>"]
pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlagsKHR.html>"]
pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlags2KHR.html>"]
pub type AccessFlags2KHR = AccessFlags2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlags2KHR.html>"]
pub type PipelineStageFlags2KHR = PipelineStageFlags2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlags2KHR.html>"]
pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagsNV.html>"]
pub type MemoryDecompressionMethodFlagsNV = MemoryDecompressionMethodFlagsEXT;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlagsKHR.html>"]
pub type RenderingFlagsKHR = RenderingFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags2KHR.html>"]
pub type PipelineCreateFlags2KHR = PipelineCreateFlags2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlags2KHR.html>"]
pub type BufferUsageFlags2KHR = BufferUsageFlags2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlagsKHR.html>"]
pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagsKHR.html>"]
pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolTrimFlagsKHR.html>"]
pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagsKHR.html>"]
pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagsKHR.html>"]
pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlagsKHR.html>"]
pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlagsKHR.html>"]
pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlagsKHR.html>"]
pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlagsKHR.html>"]
pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlagsKHR.html>"]
pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlagsKHR.html>"]
pub type FenceImportFlagsKHR = FenceImportFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlagsEXT.html>"]
pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlagsKHR.html>"]
pub type ResolveModeFlagsKHR = ResolveModeFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlagsEXT.html>"]
pub type ToolPurposeFlagsEXT = ToolPurposeFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlagsKHR.html>"]
pub type SubmitFlagsKHR = SubmitFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlagsEXT.html>"]
pub type HostImageCopyFlagsEXT = HostImageCopyFlags;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentScalingFlagsEXT.html>"]
pub type PresentScalingFlagsEXT = PresentScalingFlagsKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentGravityFlagsEXT.html>"]
pub type PresentGravityFlagsEXT = PresentGravityFlagsKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateKHR.html>"]
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionKHR.html>"]
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotEXT.html>"]
pub type PrivateDataSlotEXT = PrivateDataSlot;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingInvocationReorderModeNV.html>"]
pub type RayTracingInvocationReorderModeNV = RayTracingInvocationReorderModeEXT;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateTypeKHR.html>"]
pub type DescriptorUpdateTemplateTypeKHR = DescriptorUpdateTemplateType;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPointClippingBehaviorKHR.html>"]
pub type PointClippingBehaviorKHR = PointClippingBehavior;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueGlobalPriorityKHR.html>"]
pub type QueueGlobalPriorityKHR = QueueGlobalPriority;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueGlobalPriorityEXT.html>"]
pub type QueueGlobalPriorityEXT = QueueGlobalPriority;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTimeDomainEXT.html>"]
pub type TimeDomainEXT = TimeDomainKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreTypeKHR.html>"]
pub type SemaphoreTypeKHR = SemaphoreType;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyAccelerationStructureModeNV.html>"]
pub type CopyAccelerationStructureModeNV = CopyAccelerationStructureModeKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureTypeNV.html>"]
pub type AccelerationStructureTypeNV = AccelerationStructureTypeKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryTypeNV.html>"]
pub type GeometryTypeNV = GeometryTypeKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingShaderGroupTypeNV.html>"]
pub type RayTracingShaderGroupTypeNV = RayTracingShaderGroupTypeKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkLineRasterizationModeKHR.html>"]
pub type LineRasterizationModeKHR = LineRasterizationMode;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkLineRasterizationModeEXT.html>"]
pub type LineRasterizationModeEXT = LineRasterizationMode;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessBufferBehaviorEXT.html>"]
pub type PipelineRobustnessBufferBehaviorEXT = PipelineRobustnessBufferBehavior;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessImageBehaviorEXT.html>"]
pub type PipelineRobustnessImageBehaviorEXT = PipelineRobustnessImageBehavior;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpacityMicromapFormatEXT.html>"]
pub type OpacityMicromapFormatEXT = OpacityMicromapFormatKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkOpacityMicromapSpecialIndexEXT.html>"]
pub type OpacityMicromapSpecialIndexEXT = OpacityMicromapSpecialIndexKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultVendorBinaryHeaderVersionEXT.html>"]
pub type DeviceFaultVendorBinaryHeaderVersionEXT = DeviceFaultVendorBinaryHeaderVersionKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkScopeNV.html>"]
pub type ScopeNV = ScopeKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkComponentTypeNV.html>"]
pub type ComponentTypeNV = ComponentTypeKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTessellationDomainOriginKHR.html>"]
pub type TessellationDomainOriginKHR = TessellationDomainOrigin;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrModelConversionKHR.html>"]
pub type SamplerYcbcrModelConversionKHR = SamplerYcbcrModelConversion;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrRangeKHR.html>"]
pub type SamplerYcbcrRangeKHR = SamplerYcbcrRange;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkChromaLocationKHR.html>"]
pub type ChromaLocationKHR = ChromaLocation;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerReductionModeEXT.html>"]
pub type SamplerReductionModeEXT = SamplerReductionMode;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderFloatControlsIndependenceKHR.html>"]
pub type ShaderFloatControlsIndependenceKHR = ShaderFloatControlsIndependence;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultAddressTypeEXT.html>"]
pub type DeviceFaultAddressTypeEXT = DeviceFaultAddressTypeKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDriverIdKHR.html>"]
pub type DriverIdKHR = DriverId;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlags2CreateInfoKHR.html>"]
pub type BufferUsageFlags2CreateInfoKHR<'a> = BufferUsageFlags2CreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryIndirectCommandNV.html>"]
pub type CopyMemoryIndirectCommandNV = CopyMemoryIndirectCommandKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryToImageIndirectCommandNV.html>"]
pub type CopyMemoryToImageIndirectCommandNV = CopyMemoryToImageIndirectCommandKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlags2CreateInfoKHR.html>"]
pub type PipelineCreateFlags2CreateInfoKHR<'a> = PipelineCreateFlags2CreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalSciBufFeaturesNV.html>"]
pub type PhysicalDeviceExternalSciBufFeaturesNV<'a> =
    PhysicalDeviceExternalMemorySciBufFeaturesNV<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDevicePrivateDataCreateInfoEXT.html>"]
pub type DevicePrivateDataCreateInfoEXT<'a> = DevicePrivateDataCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlotCreateInfoEXT.html>"]
pub type PrivateDataSlotCreateInfoEXT<'a> = PrivateDataSlotCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePrivateDataFeaturesEXT.html>"]
pub type PhysicalDevicePrivateDataFeaturesEXT<'a> = PhysicalDevicePrivateDataFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFeatures2KHR.html>"]
pub type PhysicalDeviceFeatures2KHR<'a> = PhysicalDeviceFeatures2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceProperties2KHR.html>"]
pub type PhysicalDeviceProperties2KHR<'a> = PhysicalDeviceProperties2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatProperties2KHR.html>"]
pub type FormatProperties2KHR<'a> = FormatProperties2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatProperties2KHR.html>"]
pub type ImageFormatProperties2KHR<'a> = ImageFormatProperties2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageFormatInfo2KHR.html>"]
pub type PhysicalDeviceImageFormatInfo2KHR<'a> = PhysicalDeviceImageFormatInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyProperties2KHR.html>"]
pub type QueueFamilyProperties2KHR<'a> = QueueFamilyProperties2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryProperties2KHR.html>"]
pub type PhysicalDeviceMemoryProperties2KHR<'a> = PhysicalDeviceMemoryProperties2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatProperties2KHR.html>"]
pub type SparseImageFormatProperties2KHR<'a> = SparseImageFormatProperties2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSparseImageFormatInfo2KHR.html>"]
pub type PhysicalDeviceSparseImageFormatInfo2KHR<'a> = PhysicalDeviceSparseImageFormatInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePushDescriptorPropertiesKHR.html>"]
pub type PhysicalDevicePushDescriptorPropertiesKHR<'a> = PhysicalDevicePushDescriptorProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkConformanceVersionKHR.html>"]
pub type ConformanceVersionKHR = ConformanceVersion;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDriverPropertiesKHR.html>"]
pub type PhysicalDeviceDriverPropertiesKHR<'a> = PhysicalDeviceDriverProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVariablePointersFeaturesKHR.html>"]
pub type PhysicalDeviceVariablePointersFeaturesKHR<'a> = PhysicalDeviceVariablePointersFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVariablePointerFeaturesKHR.html>"]
pub type PhysicalDeviceVariablePointerFeaturesKHR<'a> = PhysicalDeviceVariablePointersFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVariablePointerFeatures.html>"]
pub type PhysicalDeviceVariablePointerFeatures<'a> = PhysicalDeviceVariablePointersFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryPropertiesKHR.html>"]
pub type ExternalMemoryPropertiesKHR = ExternalMemoryProperties;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalImageFormatInfoKHR.html>"]
pub type PhysicalDeviceExternalImageFormatInfoKHR<'a> = PhysicalDeviceExternalImageFormatInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalImageFormatPropertiesKHR.html>"]
pub type ExternalImageFormatPropertiesKHR<'a> = ExternalImageFormatProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalBufferInfoKHR.html>"]
pub type PhysicalDeviceExternalBufferInfoKHR<'a> = PhysicalDeviceExternalBufferInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalBufferPropertiesKHR.html>"]
pub type ExternalBufferPropertiesKHR<'a> = ExternalBufferProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceIDPropertiesKHR.html>"]
pub type PhysicalDeviceIDPropertiesKHR<'a> = PhysicalDeviceIDProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryImageCreateInfoKHR.html>"]
pub type ExternalMemoryImageCreateInfoKHR<'a> = ExternalMemoryImageCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryBufferCreateInfoKHR.html>"]
pub type ExternalMemoryBufferCreateInfoKHR<'a> = ExternalMemoryBufferCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMemoryAllocateInfoKHR.html>"]
pub type ExportMemoryAllocateInfoKHR<'a> = ExportMemoryAllocateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalSemaphoreInfoKHR.html>"]
pub type PhysicalDeviceExternalSemaphoreInfoKHR<'a> = PhysicalDeviceExternalSemaphoreInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphorePropertiesKHR.html>"]
pub type ExternalSemaphorePropertiesKHR<'a> = ExternalSemaphoreProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExportSemaphoreCreateInfoKHR.html>"]
pub type ExportSemaphoreCreateInfoKHR<'a> = ExportSemaphoreCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExternalFenceInfoKHR.html>"]
pub type PhysicalDeviceExternalFenceInfoKHR<'a> = PhysicalDeviceExternalFenceInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFencePropertiesKHR.html>"]
pub type ExternalFencePropertiesKHR<'a> = ExternalFenceProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkExportFenceCreateInfoKHR.html>"]
pub type ExportFenceCreateInfoKHR<'a> = ExportFenceCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewFeaturesKHR.html>"]
pub type PhysicalDeviceMultiviewFeaturesKHR<'a> = PhysicalDeviceMultiviewFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMultiviewPropertiesKHR.html>"]
pub type PhysicalDeviceMultiviewPropertiesKHR<'a> = PhysicalDeviceMultiviewProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassMultiviewCreateInfoKHR.html>"]
pub type RenderPassMultiviewCreateInfoKHR<'a> = RenderPassMultiviewCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGroupPropertiesKHR.html>"]
pub type PhysicalDeviceGroupPropertiesKHR<'a> = PhysicalDeviceGroupProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagsInfoKHR.html>"]
pub type MemoryAllocateFlagsInfoKHR<'a> = MemoryAllocateFlagsInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBindBufferMemoryInfoKHR.html>"]
pub type BindBufferMemoryInfoKHR<'a> = BindBufferMemoryInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBindBufferMemoryDeviceGroupInfoKHR.html>"]
pub type BindBufferMemoryDeviceGroupInfoKHR<'a> = BindBufferMemoryDeviceGroupInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBindImageMemoryInfoKHR.html>"]
pub type BindImageMemoryInfoKHR<'a> = BindImageMemoryInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBindImageMemoryDeviceGroupInfoKHR.html>"]
pub type BindImageMemoryDeviceGroupInfoKHR<'a> = BindImageMemoryDeviceGroupInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupRenderPassBeginInfoKHR.html>"]
pub type DeviceGroupRenderPassBeginInfoKHR<'a> = DeviceGroupRenderPassBeginInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupCommandBufferBeginInfoKHR.html>"]
pub type DeviceGroupCommandBufferBeginInfoKHR<'a> = DeviceGroupCommandBufferBeginInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupSubmitInfoKHR.html>"]
pub type DeviceGroupSubmitInfoKHR<'a> = DeviceGroupSubmitInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupBindSparseInfoKHR.html>"]
pub type DeviceGroupBindSparseInfoKHR<'a> = DeviceGroupBindSparseInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupDeviceCreateInfoKHR.html>"]
pub type DeviceGroupDeviceCreateInfoKHR<'a> = DeviceGroupDeviceCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateEntryKHR.html>"]
pub type DescriptorUpdateTemplateEntryKHR = DescriptorUpdateTemplateEntry;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateCreateInfoKHR.html>"]
pub type DescriptorUpdateTemplateCreateInfoKHR<'a> = DescriptorUpdateTemplateCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkInputAttachmentAspectReferenceKHR.html>"]
pub type InputAttachmentAspectReferenceKHR = InputAttachmentAspectReference;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassInputAttachmentAspectCreateInfoKHR.html>"]
pub type RenderPassInputAttachmentAspectCreateInfoKHR<'a> =
    RenderPassInputAttachmentAspectCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice16BitStorageFeaturesKHR.html>"]
pub type PhysicalDevice16BitStorageFeaturesKHR<'a> = PhysicalDevice16BitStorageFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR.html>"]
pub type PhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR<'a> =
    PhysicalDeviceShaderSubgroupExtendedTypesFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferMemoryRequirementsInfo2KHR.html>"]
pub type BufferMemoryRequirementsInfo2KHR<'a> = BufferMemoryRequirementsInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceBufferMemoryRequirementsKHR.html>"]
pub type DeviceBufferMemoryRequirementsKHR<'a> = DeviceBufferMemoryRequirements<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageMemoryRequirementsInfo2KHR.html>"]
pub type ImageMemoryRequirementsInfo2KHR<'a> = ImageMemoryRequirementsInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSparseMemoryRequirementsInfo2KHR.html>"]
pub type ImageSparseMemoryRequirementsInfo2KHR<'a> = ImageSparseMemoryRequirementsInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceImageMemoryRequirementsKHR.html>"]
pub type DeviceImageMemoryRequirementsKHR<'a> = DeviceImageMemoryRequirements<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryRequirements2KHR.html>"]
pub type MemoryRequirements2KHR<'a> = MemoryRequirements2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageMemoryRequirements2KHR.html>"]
pub type SparseImageMemoryRequirements2KHR<'a> = SparseImageMemoryRequirements2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePointClippingPropertiesKHR.html>"]
pub type PhysicalDevicePointClippingPropertiesKHR<'a> = PhysicalDevicePointClippingProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDedicatedRequirementsKHR.html>"]
pub type MemoryDedicatedRequirementsKHR<'a> = MemoryDedicatedRequirements<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDedicatedAllocateInfoKHR.html>"]
pub type MemoryDedicatedAllocateInfoKHR<'a> = MemoryDedicatedAllocateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewUsageCreateInfoKHR.html>"]
pub type ImageViewUsageCreateInfoKHR<'a> = ImageViewUsageCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineTessellationDomainOriginStateCreateInfoKHR.html>"]
pub type PipelineTessellationDomainOriginStateCreateInfoKHR<'a> =
    PipelineTessellationDomainOriginStateCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionInfoKHR.html>"]
pub type SamplerYcbcrConversionInfoKHR<'a> = SamplerYcbcrConversionInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionCreateInfoKHR.html>"]
pub type SamplerYcbcrConversionCreateInfoKHR<'a> = SamplerYcbcrConversionCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBindImagePlaneMemoryInfoKHR.html>"]
pub type BindImagePlaneMemoryInfoKHR<'a> = BindImagePlaneMemoryInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImagePlaneMemoryRequirementsInfoKHR.html>"]
pub type ImagePlaneMemoryRequirementsInfoKHR<'a> = ImagePlaneMemoryRequirementsInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR.html>"]
pub type PhysicalDeviceSamplerYcbcrConversionFeaturesKHR<'a> =
    PhysicalDeviceSamplerYcbcrConversionFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversionImageFormatPropertiesKHR.html>"]
pub type SamplerYcbcrConversionImageFormatPropertiesKHR<'a> =
    SamplerYcbcrConversionImageFormatProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT.html>"]
pub type PhysicalDeviceSamplerFilterMinmaxPropertiesEXT<'a> =
    PhysicalDeviceSamplerFilterMinmaxProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerReductionModeCreateInfoEXT.html>"]
pub type SamplerReductionModeCreateInfoEXT<'a> = SamplerReductionModeCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceInlineUniformBlockFeaturesEXT.html>"]
pub type PhysicalDeviceInlineUniformBlockFeaturesEXT<'a> =
    PhysicalDeviceInlineUniformBlockFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceInlineUniformBlockPropertiesEXT.html>"]
pub type PhysicalDeviceInlineUniformBlockPropertiesEXT<'a> =
    PhysicalDeviceInlineUniformBlockProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkWriteDescriptorSetInlineUniformBlockEXT.html>"]
pub type WriteDescriptorSetInlineUniformBlockEXT<'a> = WriteDescriptorSetInlineUniformBlock<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolInlineUniformBlockCreateInfoEXT.html>"]
pub type DescriptorPoolInlineUniformBlockCreateInfoEXT<'a> =
    DescriptorPoolInlineUniformBlockCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageFormatListCreateInfoKHR.html>"]
pub type ImageFormatListCreateInfoKHR<'a> = ImageFormatListCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance3PropertiesKHR.html>"]
pub type PhysicalDeviceMaintenance3PropertiesKHR<'a> = PhysicalDeviceMaintenance3Properties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance4FeaturesKHR.html>"]
pub type PhysicalDeviceMaintenance4FeaturesKHR<'a> = PhysicalDeviceMaintenance4Features<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance4PropertiesKHR.html>"]
pub type PhysicalDeviceMaintenance4PropertiesKHR<'a> = PhysicalDeviceMaintenance4Properties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance5FeaturesKHR.html>"]
pub type PhysicalDeviceMaintenance5FeaturesKHR<'a> = PhysicalDeviceMaintenance5Features<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance5PropertiesKHR.html>"]
pub type PhysicalDeviceMaintenance5PropertiesKHR<'a> = PhysicalDeviceMaintenance5Properties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance6FeaturesKHR.html>"]
pub type PhysicalDeviceMaintenance6FeaturesKHR<'a> = PhysicalDeviceMaintenance6Features<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMaintenance6PropertiesKHR.html>"]
pub type PhysicalDeviceMaintenance6PropertiesKHR<'a> = PhysicalDeviceMaintenance6Properties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAreaInfoKHR.html>"]
pub type RenderingAreaInfoKHR<'a> = RenderingAreaInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutSupportKHR.html>"]
pub type DescriptorSetLayoutSupportKHR<'a> = DescriptorSetLayoutSupport<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderDrawParameterFeatures.html>"]
pub type PhysicalDeviceShaderDrawParameterFeatures<'a> =
    PhysicalDeviceShaderDrawParametersFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderFloat16Int8FeaturesKHR.html>"]
pub type PhysicalDeviceShaderFloat16Int8FeaturesKHR<'a> =
    PhysicalDeviceShaderFloat16Int8Features<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFloat16Int8FeaturesKHR.html>"]
pub type PhysicalDeviceFloat16Int8FeaturesKHR<'a> = PhysicalDeviceShaderFloat16Int8Features<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFloatControlsPropertiesKHR.html>"]
pub type PhysicalDeviceFloatControlsPropertiesKHR<'a> = PhysicalDeviceFloatControlsProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceHostQueryResetFeaturesEXT.html>"]
pub type PhysicalDeviceHostQueryResetFeaturesEXT<'a> = PhysicalDeviceHostQueryResetFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueGlobalPriorityCreateInfoKHR.html>"]
pub type DeviceQueueGlobalPriorityCreateInfoKHR<'a> = DeviceQueueGlobalPriorityCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueGlobalPriorityCreateInfoEXT.html>"]
pub type DeviceQueueGlobalPriorityCreateInfoEXT<'a> = DeviceQueueGlobalPriorityCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR.html>"]
pub type PhysicalDeviceGlobalPriorityQueryFeaturesKHR<'a> =
    PhysicalDeviceGlobalPriorityQueryFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT.html>"]
pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXT<'a> =
    PhysicalDeviceGlobalPriorityQueryFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyGlobalPriorityPropertiesKHR.html>"]
pub type QueueFamilyGlobalPriorityPropertiesKHR<'a> = QueueFamilyGlobalPriorityProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFamilyGlobalPriorityPropertiesEXT.html>"]
pub type QueueFamilyGlobalPriorityPropertiesEXT<'a> = QueueFamilyGlobalPriorityProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCalibratedTimestampInfoEXT.html>"]
pub type CalibratedTimestampInfoEXT<'a> = CalibratedTimestampInfoKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorIndexingFeaturesEXT.html>"]
pub type PhysicalDeviceDescriptorIndexingFeaturesEXT<'a> =
    PhysicalDeviceDescriptorIndexingFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDescriptorIndexingPropertiesEXT.html>"]
pub type PhysicalDeviceDescriptorIndexingPropertiesEXT<'a> =
    PhysicalDeviceDescriptorIndexingProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutBindingFlagsCreateInfoEXT.html>"]
pub type DescriptorSetLayoutBindingFlagsCreateInfoEXT<'a> =
    DescriptorSetLayoutBindingFlagsCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetVariableDescriptorCountAllocateInfoEXT.html>"]
pub type DescriptorSetVariableDescriptorCountAllocateInfoEXT<'a> =
    DescriptorSetVariableDescriptorCountAllocateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetVariableDescriptorCountLayoutSupportEXT.html>"]
pub type DescriptorSetVariableDescriptorCountLayoutSupportEXT<'a> =
    DescriptorSetVariableDescriptorCountLayoutSupport<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescription2KHR.html>"]
pub type AttachmentDescription2KHR<'a> = AttachmentDescription2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentReference2KHR.html>"]
pub type AttachmentReference2KHR<'a> = AttachmentReference2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescription2KHR.html>"]
pub type SubpassDescription2KHR<'a> = SubpassDescription2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDependency2KHR.html>"]
pub type SubpassDependency2KHR<'a> = SubpassDependency2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateInfo2KHR.html>"]
pub type RenderPassCreateInfo2KHR<'a> = RenderPassCreateInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassBeginInfoKHR.html>"]
pub type SubpassBeginInfoKHR<'a> = SubpassBeginInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassEndInfoKHR.html>"]
pub type SubpassEndInfoKHR<'a> = SubpassEndInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTimelineSemaphoreFeaturesKHR.html>"]
pub type PhysicalDeviceTimelineSemaphoreFeaturesKHR<'a> =
    PhysicalDeviceTimelineSemaphoreFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTimelineSemaphorePropertiesKHR.html>"]
pub type PhysicalDeviceTimelineSemaphorePropertiesKHR<'a> =
    PhysicalDeviceTimelineSemaphoreProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreTypeCreateInfoKHR.html>"]
pub type SemaphoreTypeCreateInfoKHR<'a> = SemaphoreTypeCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTimelineSemaphoreSubmitInfoKHR.html>"]
pub type TimelineSemaphoreSubmitInfoKHR<'a> = TimelineSemaphoreSubmitInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitInfoKHR.html>"]
pub type SemaphoreWaitInfoKHR<'a> = SemaphoreWaitInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreSignalInfoKHR.html>"]
pub type SemaphoreSignalInfoKHR<'a> = SemaphoreSignalInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputBindingDivisorDescriptionKHR.html>"]
pub type VertexInputBindingDivisorDescriptionKHR = VertexInputBindingDivisorDescription;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputBindingDivisorDescriptionEXT.html>"]
pub type VertexInputBindingDivisorDescriptionEXT = VertexInputBindingDivisorDescription;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineVertexInputDivisorStateCreateInfoKHR.html>"]
pub type PipelineVertexInputDivisorStateCreateInfoKHR<'a> =
    PipelineVertexInputDivisorStateCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineVertexInputDivisorStateCreateInfoEXT.html>"]
pub type PipelineVertexInputDivisorStateCreateInfoEXT<'a> =
    PipelineVertexInputDivisorStateCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR.html>"]
pub type PhysicalDeviceVertexAttributeDivisorPropertiesKHR<'a> =
    PhysicalDeviceVertexAttributeDivisorProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice8BitStorageFeaturesKHR.html>"]
pub type PhysicalDevice8BitStorageFeaturesKHR<'a> = PhysicalDevice8BitStorageFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVulkanMemoryModelFeaturesKHR.html>"]
pub type PhysicalDeviceVulkanMemoryModelFeaturesKHR<'a> =
    PhysicalDeviceVulkanMemoryModelFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderAtomicInt64FeaturesKHR.html>"]
pub type PhysicalDeviceShaderAtomicInt64FeaturesKHR<'a> =
    PhysicalDeviceShaderAtomicInt64Features<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR.html>"]
pub type PhysicalDeviceVertexAttributeDivisorFeaturesKHR<'a> =
    PhysicalDeviceVertexAttributeDivisorFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT.html>"]
pub type PhysicalDeviceVertexAttributeDivisorFeaturesEXT<'a> =
    PhysicalDeviceVertexAttributeDivisorFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDepthStencilResolvePropertiesKHR.html>"]
pub type PhysicalDeviceDepthStencilResolvePropertiesKHR<'a> =
    PhysicalDeviceDepthStencilResolveProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionDepthStencilResolveKHR.html>"]
pub type SubpassDescriptionDepthStencilResolveKHR<'a> = SubpassDescriptionDepthStencilResolve<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceComputeShaderDerivativesFeaturesNV.html>"]
pub type PhysicalDeviceComputeShaderDerivativesFeaturesNV<'a> =
    PhysicalDeviceComputeShaderDerivativesFeaturesKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV.html>"]
pub type PhysicalDeviceFragmentShaderBarycentricFeaturesNV<'a> =
    PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceCopyMemoryIndirectPropertiesNV.html>"]
pub type PhysicalDeviceCopyMemoryIndirectPropertiesNV<'a> =
    PhysicalDeviceCopyMemoryIndirectPropertiesKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryDecompressionFeaturesNV.html>"]
pub type PhysicalDeviceMemoryDecompressionFeaturesNV<'a> =
    PhysicalDeviceMemoryDecompressionFeaturesEXT<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMemoryDecompressionPropertiesNV.html>"]
pub type PhysicalDeviceMemoryDecompressionPropertiesNV<'a> =
    PhysicalDeviceMemoryDecompressionPropertiesEXT<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageStencilUsageCreateInfoEXT.html>"]
pub type ImageStencilUsageCreateInfoEXT<'a> = ImageStencilUsageCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM.html>"]
pub type PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM<'a> =
    PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM.html>"]
pub type PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM<'a> =
    PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassFragmentDensityMapOffsetEndInfoQCOM.html>"]
pub type SubpassFragmentDensityMapOffsetEndInfoQCOM<'a> =
    RenderPassFragmentDensityMapOffsetEndInfoEXT<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceScalarBlockLayoutFeaturesEXT.html>"]
pub type PhysicalDeviceScalarBlockLayoutFeaturesEXT<'a> =
    PhysicalDeviceScalarBlockLayoutFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR.html>"]
pub type PhysicalDeviceUniformBufferStandardLayoutFeaturesKHR<'a> =
    PhysicalDeviceUniformBufferStandardLayoutFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceBufferDeviceAddressFeaturesKHR.html>"]
pub type PhysicalDeviceBufferDeviceAddressFeaturesKHR<'a> =
    PhysicalDeviceBufferDeviceAddressFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceBufferAddressFeaturesEXT.html>"]
pub type PhysicalDeviceBufferAddressFeaturesEXT<'a> =
    PhysicalDeviceBufferDeviceAddressFeaturesEXT<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferDeviceAddressInfoKHR.html>"]
pub type BufferDeviceAddressInfoKHR<'a> = BufferDeviceAddressInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferDeviceAddressInfoEXT.html>"]
pub type BufferDeviceAddressInfoEXT<'a> = BufferDeviceAddressInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferOpaqueCaptureAddressCreateInfoKHR.html>"]
pub type BufferOpaqueCaptureAddressCreateInfoKHR<'a> = BufferOpaqueCaptureAddressCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImagelessFramebufferFeaturesKHR.html>"]
pub type PhysicalDeviceImagelessFramebufferFeaturesKHR<'a> =
    PhysicalDeviceImagelessFramebufferFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferAttachmentsCreateInfoKHR.html>"]
pub type FramebufferAttachmentsCreateInfoKHR<'a> = FramebufferAttachmentsCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferAttachmentImageInfoKHR.html>"]
pub type FramebufferAttachmentImageInfoKHR<'a> = FramebufferAttachmentImageInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassAttachmentBeginInfoKHR.html>"]
pub type RenderPassAttachmentBeginInfoKHR<'a> = RenderPassAttachmentBeginInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT.html>"]
pub type PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT<'a> =
    PhysicalDeviceTextureCompressionASTCHDRFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackEXT.html>"]
pub type PipelineCreationFeedbackEXT = PipelineCreationFeedback;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackCreateInfoEXT.html>"]
pub type PipelineCreationFeedbackCreateInfoEXT<'a> = PipelineCreationFeedbackCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateInfoINTEL.html>"]
pub type QueryPoolCreateInfoINTEL<'a> = QueryPoolPerformanceQueryCreateInfoINTEL<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceIndexTypeUint8FeaturesKHR.html>"]
pub type PhysicalDeviceIndexTypeUint8FeaturesKHR<'a> = PhysicalDeviceIndexTypeUint8Features<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceIndexTypeUint8FeaturesEXT.html>"]
pub type PhysicalDeviceIndexTypeUint8FeaturesEXT<'a> = PhysicalDeviceIndexTypeUint8Features<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR.html>"]
pub type PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR<'a> =
    PhysicalDeviceSeparateDepthStencilLayoutsFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentReferenceStencilLayoutKHR.html>"]
pub type AttachmentReferenceStencilLayoutKHR<'a> = AttachmentReferenceStencilLayout<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionStencilLayoutKHR.html>"]
pub type AttachmentDescriptionStencilLayoutKHR<'a> = AttachmentDescriptionStencilLayout<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineInfoEXT.html>"]
pub type PipelineInfoEXT<'a> = PipelineInfoKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT.html>"]
pub type PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT<'a> =
    PhysicalDeviceShaderDemoteToHelperInvocationFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT.html>"]
pub type PhysicalDeviceTexelBufferAlignmentPropertiesEXT<'a> =
    PhysicalDeviceTexelBufferAlignmentProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubgroupSizeControlFeaturesEXT.html>"]
pub type PhysicalDeviceSubgroupSizeControlFeaturesEXT<'a> =
    PhysicalDeviceSubgroupSizeControlFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSubgroupSizeControlPropertiesEXT.html>"]
pub type PhysicalDeviceSubgroupSizeControlPropertiesEXT<'a> =
    PhysicalDeviceSubgroupSizeControlProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT.html>"]
pub type PipelineShaderStageRequiredSubgroupSizeCreateInfoEXT<'a> =
    PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderRequiredSubgroupSizeCreateInfoEXT.html>"]
pub type ShaderRequiredSubgroupSizeCreateInfoEXT<'a> =
    PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryOpaqueCaptureAddressAllocateInfoKHR.html>"]
pub type MemoryOpaqueCaptureAddressAllocateInfoKHR<'a> = MemoryOpaqueCaptureAddressAllocateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryOpaqueCaptureAddressInfoKHR.html>"]
pub type DeviceMemoryOpaqueCaptureAddressInfoKHR<'a> = DeviceMemoryOpaqueCaptureAddressInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLineRasterizationFeaturesKHR.html>"]
pub type PhysicalDeviceLineRasterizationFeaturesKHR<'a> =
    PhysicalDeviceLineRasterizationFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLineRasterizationFeaturesEXT.html>"]
pub type PhysicalDeviceLineRasterizationFeaturesEXT<'a> =
    PhysicalDeviceLineRasterizationFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLineRasterizationPropertiesKHR.html>"]
pub type PhysicalDeviceLineRasterizationPropertiesKHR<'a> =
    PhysicalDeviceLineRasterizationProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLineRasterizationPropertiesEXT.html>"]
pub type PhysicalDeviceLineRasterizationPropertiesEXT<'a> =
    PhysicalDeviceLineRasterizationProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationLineStateCreateInfoKHR.html>"]
pub type PipelineRasterizationLineStateCreateInfoKHR<'a> =
    PipelineRasterizationLineStateCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRasterizationLineStateCreateInfoEXT.html>"]
pub type PipelineRasterizationLineStateCreateInfoEXT<'a> =
    PipelineRasterizationLineStateCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT.html>"]
pub type PhysicalDevicePipelineCreationCacheControlFeaturesEXT<'a> =
    PhysicalDevicePipelineCreationCacheControlFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceToolPropertiesEXT.html>"]
pub type PhysicalDeviceToolPropertiesEXT<'a> = PhysicalDeviceToolProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAabbPositionsNV.html>"]
pub type AabbPositionsNV = AabbPositionsKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkTransformMatrixNV.html>"]
pub type TransformMatrixNV = TransformMatrixKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureInstanceNV.html>"]
pub type AccelerationStructureInstanceNV = AccelerationStructureInstanceKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR.html>"]
pub type PhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR<'a> =
    PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRobustness2FeaturesEXT.html>"]
pub type PhysicalDeviceRobustness2FeaturesEXT<'a> = PhysicalDeviceRobustness2FeaturesKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRobustness2PropertiesEXT.html>"]
pub type PhysicalDeviceRobustness2PropertiesEXT<'a> = PhysicalDeviceRobustness2PropertiesKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceImageRobustnessFeaturesEXT.html>"]
pub type PhysicalDeviceImageRobustnessFeaturesEXT<'a> = PhysicalDeviceImageRobustnessFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCopy2KHR.html>"]
pub type BufferCopy2KHR<'a> = BufferCopy2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCopy2KHR.html>"]
pub type ImageCopy2KHR<'a> = ImageCopy2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageBlit2KHR.html>"]
pub type ImageBlit2KHR<'a> = ImageBlit2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferImageCopy2KHR.html>"]
pub type BufferImageCopy2KHR<'a> = BufferImageCopy2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageResolve2KHR.html>"]
pub type ImageResolve2KHR<'a> = ImageResolve2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyBufferInfo2KHR.html>"]
pub type CopyBufferInfo2KHR<'a> = CopyBufferInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageInfo2KHR.html>"]
pub type CopyImageInfo2KHR<'a> = CopyImageInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBlitImageInfo2KHR.html>"]
pub type BlitImageInfo2KHR<'a> = BlitImageInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyBufferToImageInfo2KHR.html>"]
pub type CopyBufferToImageInfo2KHR<'a> = CopyBufferToImageInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageToBufferInfo2KHR.html>"]
pub type CopyImageToBufferInfo2KHR<'a> = CopyImageToBufferInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveImageInfo2KHR.html>"]
pub type ResolveImageInfo2KHR<'a> = ResolveImageInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR.html>"]
pub type PhysicalDeviceShaderTerminateInvocationFeaturesKHR<'a> =
    PhysicalDeviceShaderTerminateInvocationFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE.html>"]
pub type PhysicalDeviceMutableDescriptorTypeFeaturesVALVE<'a> =
    PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMutableDescriptorTypeListVALVE.html>"]
pub type MutableDescriptorTypeListVALVE<'a> = MutableDescriptorTypeListEXT<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMutableDescriptorTypeCreateInfoVALVE.html>"]
pub type MutableDescriptorTypeCreateInfoVALVE<'a> = MutableDescriptorTypeCreateInfoEXT<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryBarrier2KHR.html>"]
pub type MemoryBarrier2KHR<'a> = MemoryBarrier2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageMemoryBarrier2KHR.html>"]
pub type ImageMemoryBarrier2KHR<'a> = ImageMemoryBarrier2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferMemoryBarrier2KHR.html>"]
pub type BufferMemoryBarrier2KHR<'a> = BufferMemoryBarrier2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyInfoKHR.html>"]
pub type DependencyInfoKHR<'a> = DependencyInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreSubmitInfoKHR.html>"]
pub type SemaphoreSubmitInfoKHR<'a> = SemaphoreSubmitInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferSubmitInfoKHR.html>"]
pub type CommandBufferSubmitInfoKHR<'a> = CommandBufferSubmitInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitInfo2KHR.html>"]
pub type SubmitInfo2KHR<'a> = SubmitInfo2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSynchronization2FeaturesKHR.html>"]
pub type PhysicalDeviceSynchronization2FeaturesKHR<'a> = PhysicalDeviceSynchronization2Features<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceHostImageCopyFeaturesEXT.html>"]
pub type PhysicalDeviceHostImageCopyFeaturesEXT<'a> = PhysicalDeviceHostImageCopyFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceHostImageCopyPropertiesEXT.html>"]
pub type PhysicalDeviceHostImageCopyPropertiesEXT<'a> = PhysicalDeviceHostImageCopyProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryToImageCopyEXT.html>"]
pub type MemoryToImageCopyEXT<'a> = MemoryToImageCopy<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageToMemoryCopyEXT.html>"]
pub type ImageToMemoryCopyEXT<'a> = ImageToMemoryCopy<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMemoryToImageInfoEXT.html>"]
pub type CopyMemoryToImageInfoEXT<'a> = CopyMemoryToImageInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageToMemoryInfoEXT.html>"]
pub type CopyImageToMemoryInfoEXT<'a> = CopyImageToMemoryInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyImageToImageInfoEXT.html>"]
pub type CopyImageToImageInfoEXT<'a> = CopyImageToImageInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageLayoutTransitionInfoEXT.html>"]
pub type HostImageLayoutTransitionInfoEXT<'a> = HostImageLayoutTransitionInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubresourceHostMemcpySizeEXT.html>"]
pub type SubresourceHostMemcpySizeEXT<'a> = SubresourceHostMemcpySize<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyDevicePerformanceQueryEXT.html>"]
pub type HostImageCopyDevicePerformanceQueryEXT<'a> = HostImageCopyDevicePerformanceQuery<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineProtectedAccessFeaturesEXT.html>"]
pub type PhysicalDevicePipelineProtectedAccessFeaturesEXT<'a> =
    PhysicalDevicePipelineProtectedAccessFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR.html>"]
pub type PhysicalDeviceShaderIntegerDotProductFeaturesKHR<'a> =
    PhysicalDeviceShaderIntegerDotProductFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR.html>"]
pub type PhysicalDeviceShaderIntegerDotProductPropertiesKHR<'a> =
    PhysicalDeviceShaderIntegerDotProductProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatProperties3KHR.html>"]
pub type FormatProperties3KHR<'a> = FormatProperties3<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRenderingCreateInfoKHR.html>"]
pub type PipelineRenderingCreateInfoKHR<'a> = PipelineRenderingCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingInfoKHR.html>"]
pub type RenderingInfoKHR<'a> = RenderingInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingEndInfoEXT.html>"]
pub type RenderingEndInfoEXT<'a> = RenderingEndInfoKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentInfoKHR.html>"]
pub type RenderingAttachmentInfoKHR<'a> = RenderingAttachmentInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDynamicRenderingFeaturesKHR.html>"]
pub type PhysicalDeviceDynamicRenderingFeaturesKHR<'a> = PhysicalDeviceDynamicRenderingFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferInheritanceRenderingInfoKHR.html>"]
pub type CommandBufferInheritanceRenderingInfoKHR<'a> = CommandBufferInheritanceRenderingInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentSampleCountInfoNV.html>"]
pub type AttachmentSampleCountInfoNV<'a> = AttachmentSampleCountInfoAMD<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM.html>"]
pub type PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM<'a> =
    PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSubresource2KHR.html>"]
pub type ImageSubresource2KHR<'a> = ImageSubresource2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkImageSubresource2EXT.html>"]
pub type ImageSubresource2EXT<'a> = ImageSubresource2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubresourceLayout2KHR.html>"]
pub type SubresourceLayout2KHR<'a> = SubresourceLayout2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSubresourceLayout2EXT.html>"]
pub type SubresourceLayout2EXT<'a> = SubresourceLayout2<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapTriangleEXT.html>"]
pub type MicromapTriangleEXT = MicromapTriangleKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineRobustnessFeaturesEXT.html>"]
pub type PhysicalDevicePipelineRobustnessFeaturesEXT<'a> =
    PhysicalDevicePipelineRobustnessFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessCreateInfoEXT.html>"]
pub type PipelineRobustnessCreateInfoEXT<'a> = PipelineRobustnessCreateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePipelineRobustnessPropertiesEXT.html>"]
pub type PhysicalDevicePipelineRobustnessPropertiesEXT<'a> =
    PhysicalDevicePipelineRobustnessProperties<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDepthClampZeroOneFeaturesEXT.html>"]
pub type PhysicalDeviceDepthClampZeroOneFeaturesEXT<'a> =
    PhysicalDeviceDepthClampZeroOneFeaturesKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultAddressInfoEXT.html>"]
pub type DeviceFaultAddressInfoEXT = DeviceFaultAddressInfoKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultVendorInfoEXT.html>"]
pub type DeviceFaultVendorInfoEXT = DeviceFaultVendorInfoKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultVendorBinaryHeaderVersionOneEXT.html>"]
pub type DeviceFaultVendorBinaryHeaderVersionOneEXT = DeviceFaultVendorBinaryHeaderVersionOneKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfacePresentModeEXT.html>"]
pub type SurfacePresentModeEXT<'a> = SurfacePresentModeKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfacePresentScalingCapabilitiesEXT.html>"]
pub type SurfacePresentScalingCapabilitiesEXT<'a> = SurfacePresentScalingCapabilitiesKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfacePresentModeCompatibilityEXT.html>"]
pub type SurfacePresentModeCompatibilityEXT<'a> = SurfacePresentModeCompatibilityKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT.html>"]
pub type PhysicalDeviceSwapchainMaintenance1FeaturesEXT<'a> =
    PhysicalDeviceSwapchainMaintenance1FeaturesKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentFenceInfoEXT.html>"]
pub type SwapchainPresentFenceInfoEXT<'a> = SwapchainPresentFenceInfoKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentModesCreateInfoEXT.html>"]
pub type SwapchainPresentModesCreateInfoEXT<'a> = SwapchainPresentModesCreateInfoKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentModeInfoEXT.html>"]
pub type SwapchainPresentModeInfoEXT<'a> = SwapchainPresentModeInfoKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainPresentScalingCreateInfoEXT.html>"]
pub type SwapchainPresentScalingCreateInfoEXT<'a> = SwapchainPresentScalingCreateInfoKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkReleaseSwapchainImagesInfoEXT.html>"]
pub type ReleaseSwapchainImagesInfoEXT<'a> = ReleaseSwapchainImagesInfoKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceImageSubresourceInfoKHR.html>"]
pub type DeviceImageSubresourceInfoKHR<'a> = DeviceImageSubresourceInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapInfoKHR.html>"]
pub type MemoryMapInfoKHR<'a> = MemoryMapInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapInfoKHR.html>"]
pub type MemoryUnmapInfoKHR<'a> = MemoryUnmapInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBindMemoryStatusKHR.html>"]
pub type BindMemoryStatusKHR<'a> = BindMemoryStatus<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkBindDescriptorSetsInfoKHR.html>"]
pub type BindDescriptorSetsInfoKHR<'a> = BindDescriptorSetsInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPushConstantsInfoKHR.html>"]
pub type PushConstantsInfoKHR<'a> = PushConstantsInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPushDescriptorSetInfoKHR.html>"]
pub type PushDescriptorSetInfoKHR<'a> = PushDescriptorSetInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPushDescriptorSetWithTemplateInfoKHR.html>"]
pub type PushDescriptorSetWithTemplateInfoKHR<'a> = PushDescriptorSetWithTemplateInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderSubgroupRotateFeaturesKHR.html>"]
pub type PhysicalDeviceShaderSubgroupRotateFeaturesKHR<'a> =
    PhysicalDeviceShaderSubgroupRotateFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderExpectAssumeFeaturesKHR.html>"]
pub type PhysicalDeviceShaderExpectAssumeFeaturesKHR<'a> =
    PhysicalDeviceShaderExpectAssumeFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceShaderFloatControls2FeaturesKHR.html>"]
pub type PhysicalDeviceShaderFloatControls2FeaturesKHR<'a> =
    PhysicalDeviceShaderFloatControls2Features<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDynamicRenderingLocalReadFeaturesKHR.html>"]
pub type PhysicalDeviceDynamicRenderingLocalReadFeaturesKHR<'a> =
    PhysicalDeviceDynamicRenderingLocalReadFeatures<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentLocationInfoKHR.html>"]
pub type RenderingAttachmentLocationInfoKHR<'a> = RenderingAttachmentLocationInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingInputAttachmentIndexInfoKHR.html>"]
pub type RenderingInputAttachmentIndexInfoKHR<'a> = RenderingInputAttachmentIndexInfo<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevicePresentModeFifoLatestReadyFeaturesEXT.html>"]
pub type PhysicalDevicePresentModeFifoLatestReadyFeaturesEXT<'a> =
    PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'a>;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressRangeEXT.html>"]
pub type DeviceAddressRangeEXT = DeviceAddressRangeKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkResetQueryPoolEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkResetQueryPoolEXT = PFN_vkResetQueryPool;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetRenderingAreaGranularityKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRenderingAreaGranularityKHR = PFN_vkGetRenderingAreaGranularity;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceFeatures2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFeatures2KHR = PFN_vkGetPhysicalDeviceFeatures2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceProperties2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceProperties2KHR = PFN_vkGetPhysicalDeviceProperties2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceFormatProperties2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = PFN_vkGetPhysicalDeviceFormatProperties2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceImageFormatProperties2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR =
    PFN_vkGetPhysicalDeviceImageFormatProperties2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR =
    PFN_vkGetPhysicalDeviceQueueFamilyProperties2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceMemoryProperties2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = PFN_vkGetPhysicalDeviceMemoryProperties2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR =
    PFN_vkGetPhysicalDeviceSparseImageFormatProperties2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdPushDescriptorSetKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetKHR = PFN_vkCmdPushDescriptorSet;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkTrimCommandPoolKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkTrimCommandPoolKHR = PFN_vkTrimCommandPool;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalBufferProperties;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalSemaphoreProperties;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR =
    PFN_vkGetPhysicalDeviceExternalFenceProperties;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkEnumeratePhysicalDeviceGroupsKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = PFN_vkEnumeratePhysicalDeviceGroups;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR = PFN_vkGetDeviceGroupPeerMemoryFeatures;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkBindBufferMemory2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory2KHR = PFN_vkBindBufferMemory2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkBindImageMemory2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory2KHR = PFN_vkBindImageMemory2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetDeviceMaskKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDeviceMaskKHR = PFN_vkCmdSetDeviceMask;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdDispatchBaseKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchBaseKHR = PFN_vkCmdDispatchBase;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCreateDescriptorUpdateTemplateKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorUpdateTemplateKHR = PFN_vkCreateDescriptorUpdateTemplate;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDestroyDescriptorUpdateTemplateKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = PFN_vkDestroyDescriptorUpdateTemplate;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkUpdateDescriptorSetWithTemplateKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = PFN_vkUpdateDescriptorSetWithTemplate;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdPushDescriptorSetWithTemplateKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = PFN_vkCmdPushDescriptorSetWithTemplate;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetBufferMemoryRequirements2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements2KHR = PFN_vkGetBufferMemoryRequirements2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetImageMemoryRequirements2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements2KHR = PFN_vkGetImageMemoryRequirements2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetImageSparseMemoryRequirements2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = PFN_vkGetImageSparseMemoryRequirements2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetDeviceBufferMemoryRequirementsKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = PFN_vkGetDeviceBufferMemoryRequirements;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetDeviceImageMemoryRequirementsKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = PFN_vkGetDeviceImageMemoryRequirements;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetDeviceImageSparseMemoryRequirementsKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR =
    PFN_vkGetDeviceImageSparseMemoryRequirements;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCreateSamplerYcbcrConversionKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSamplerYcbcrConversionKHR = PFN_vkCreateSamplerYcbcrConversion;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDestroySamplerYcbcrConversionKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySamplerYcbcrConversionKHR = PFN_vkDestroySamplerYcbcrConversion;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetDescriptorSetLayoutSupportKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutSupportKHR = PFN_vkGetDescriptorSetLayoutSupport;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT =
    PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetCalibratedTimestampsEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetCalibratedTimestampsEXT = PFN_vkGetCalibratedTimestampsKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCreateRenderPass2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRenderPass2KHR = PFN_vkCreateRenderPass2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdBeginRenderPass2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderPass2KHR = PFN_vkCmdBeginRenderPass2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdNextSubpass2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdNextSubpass2KHR = PFN_vkCmdNextSubpass2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdEndRenderPass2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderPass2KHR = PFN_vkCmdEndRenderPass2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetSemaphoreCounterValueKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreCounterValueKHR = PFN_vkGetSemaphoreCounterValue;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkWaitSemaphoresKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkWaitSemaphoresKHR = PFN_vkWaitSemaphores;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkSignalSemaphoreKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkSignalSemaphoreKHR = PFN_vkSignalSemaphore;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdDrawIndirectCountKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectCountKHR = PFN_vkCmdDrawIndirectCount;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdDrawIndirectCountAMD.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectCountAMD = PFN_vkCmdDrawIndirectCount;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdDrawIndexedIndirectCountKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirectCountKHR = PFN_vkCmdDrawIndexedIndirectCount;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdDrawIndexedIndirectCountAMD.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirectCountAMD = PFN_vkCmdDrawIndexedIndirectCount;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetRayTracingShaderGroupHandlesNV.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupHandlesNV = PFN_vkGetRayTracingShaderGroupHandlesKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetBufferOpaqueCaptureAddressKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferOpaqueCaptureAddressKHR = PFN_vkGetBufferOpaqueCaptureAddress;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetBufferDeviceAddressKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferDeviceAddressKHR = PFN_vkGetBufferDeviceAddress;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetBufferDeviceAddressEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferDeviceAddressEXT = PFN_vkGetBufferDeviceAddress;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR = PFN_vkGetDeviceMemoryOpaqueCaptureAddress;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetLineStippleKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineStippleKHR = PFN_vkCmdSetLineStipple;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetLineStippleEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineStippleEXT = PFN_vkCmdSetLineStipple;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPhysicalDeviceToolPropertiesEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT = PFN_vkGetPhysicalDeviceToolProperties;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetCullModeEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCullModeEXT = PFN_vkCmdSetCullMode;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetFrontFaceEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFrontFaceEXT = PFN_vkCmdSetFrontFace;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetPrimitiveTopologyEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveTopologyEXT = PFN_vkCmdSetPrimitiveTopology;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetViewportWithCountEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWithCountEXT = PFN_vkCmdSetViewportWithCount;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetScissorWithCountEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetScissorWithCountEXT = PFN_vkCmdSetScissorWithCount;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdBindIndexBuffer2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindIndexBuffer2KHR = PFN_vkCmdBindIndexBuffer2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdBindVertexBuffers2EXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindVertexBuffers2EXT = PFN_vkCmdBindVertexBuffers2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetDepthTestEnableEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthTestEnableEXT = PFN_vkCmdSetDepthTestEnable;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetDepthWriteEnableEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthWriteEnableEXT = PFN_vkCmdSetDepthWriteEnable;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetDepthCompareOpEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthCompareOpEXT = PFN_vkCmdSetDepthCompareOp;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetDepthBoundsTestEnableEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBoundsTestEnableEXT = PFN_vkCmdSetDepthBoundsTestEnable;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetStencilTestEnableEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilTestEnableEXT = PFN_vkCmdSetStencilTestEnable;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetStencilOpEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilOpEXT = PFN_vkCmdSetStencilOp;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetRasterizerDiscardEnableEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRasterizerDiscardEnableEXT = PFN_vkCmdSetRasterizerDiscardEnable;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetDepthBiasEnableEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBiasEnableEXT = PFN_vkCmdSetDepthBiasEnable;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetPrimitiveRestartEnableEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveRestartEnableEXT = PFN_vkCmdSetPrimitiveRestartEnable;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCreatePrivateDataSlotEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePrivateDataSlotEXT = PFN_vkCreatePrivateDataSlot;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDestroyPrivateDataSlotEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPrivateDataSlotEXT = PFN_vkDestroyPrivateDataSlot;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkSetPrivateDataEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkSetPrivateDataEXT = PFN_vkSetPrivateData;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetPrivateDataEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetPrivateDataEXT = PFN_vkGetPrivateData;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdCopyBuffer2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBuffer2KHR = PFN_vkCmdCopyBuffer2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdCopyImage2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImage2KHR = PFN_vkCmdCopyImage2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdBlitImage2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBlitImage2KHR = PFN_vkCmdBlitImage2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdCopyBufferToImage2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBufferToImage2KHR = PFN_vkCmdCopyBufferToImage2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdCopyImageToBuffer2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImageToBuffer2KHR = PFN_vkCmdCopyImageToBuffer2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdResolveImage2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResolveImage2KHR = PFN_vkCmdResolveImage2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetEvent2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetEvent2KHR = PFN_vkCmdSetEvent2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdResetEvent2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetEvent2KHR = PFN_vkCmdResetEvent2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdWaitEvents2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWaitEvents2KHR = PFN_vkCmdWaitEvents2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdPipelineBarrier2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPipelineBarrier2KHR = PFN_vkCmdPipelineBarrier2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkQueueSubmit2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSubmit2KHR = PFN_vkQueueSubmit2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdWriteTimestamp2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteTimestamp2KHR = PFN_vkCmdWriteTimestamp2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCopyMemoryToImageEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMemoryToImageEXT = PFN_vkCopyMemoryToImage;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCopyImageToMemoryEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyImageToMemoryEXT = PFN_vkCopyImageToMemory;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCopyImageToImageEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCopyImageToImageEXT = PFN_vkCopyImageToImage;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkTransitionImageLayoutEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkTransitionImageLayoutEXT = PFN_vkTransitionImageLayout;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdBeginRenderingKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderingKHR = PFN_vkCmdBeginRendering;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdEndRendering2EXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRendering2EXT = PFN_vkCmdEndRendering2KHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdEndRenderingKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderingKHR = PFN_vkCmdEndRendering;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetImageSubresourceLayout2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSubresourceLayout2KHR = PFN_vkGetImageSubresourceLayout2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetImageSubresourceLayout2EXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSubresourceLayout2EXT = PFN_vkGetImageSubresourceLayout2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkReleaseSwapchainImagesEXT.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseSwapchainImagesEXT = PFN_vkReleaseSwapchainImagesKHR;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetDeviceImageSubresourceLayoutKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageSubresourceLayoutKHR = PFN_vkGetDeviceImageSubresourceLayout;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkMapMemory2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkMapMemory2KHR = PFN_vkMapMemory2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkUnmapMemory2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkUnmapMemory2KHR = PFN_vkUnmapMemory2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdBindDescriptorSets2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorSets2KHR = PFN_vkCmdBindDescriptorSets2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdPushConstants2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushConstants2KHR = PFN_vkCmdPushConstants2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdPushDescriptorSet2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSet2KHR = PFN_vkCmdPushDescriptorSet2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdPushDescriptorSetWithTemplate2KHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = PFN_vkCmdPushDescriptorSetWithTemplate2;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetRenderingAttachmentLocationsKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = PFN_vkCmdSetRenderingAttachmentLocations;
#[doc = "<https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkCmdSetRenderingInputAttachmentIndicesKHR.html>"]
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR =
    PFN_vkCmdSetRenderingInputAttachmentIndices;
