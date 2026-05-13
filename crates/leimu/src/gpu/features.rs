mod obj;
mod context;
mod attributes;

use tuhka::vk;

use leimu_mem::{vec::Vec32, vec32};

use crate::error::*;
use super::prelude::*;

pub use obj::*;
pub use context::*;
pub use attributes::*;

pub use crate::gpu::features;

mod inner {
    use super::*;
    leimu_mem::smallbox!(
        pub(super) struct Support: Fn[(&mut DeviceFeatureContext) -> Result<()>]
    );
    leimu_mem::smallbox!(
        pub(super) struct Enable: Fn[(&mut DeviceFeatureContext)]
    );
}

enum Inner {
    Vulkan10(vulkan_10::Feature),
    Vulkan11(vulkan_11::Feature),
    Vulkan12(vulkan_12::Feature),
    Ext(FeatureExt),
}

pub struct DeviceFeature {
    inner: Inner,
    name: ConstName,
    enable: bool,
}

impl DeviceFeature {

    pub fn query_support(
        &self,
        ctx: &mut DeviceFeatureContext,
    ) -> Result<()> {
        match &self.inner {
            Inner::Vulkan10(f) => f.query_support(ctx.vulkan_10_features()),
            Inner::Vulkan11(f) => f.query_support(ctx.vulkan_11_features()),
            Inner::Vulkan12(f) => f.query_support(ctx.vulkan_12_features()),
            Inner::Ext(f) => f.query_support(ctx),
        }
    }

    pub fn enable_features(
        &self,
        ctx: &mut DeviceFeatureContext,
    ) {
        match &self.inner {
            Inner::Vulkan10(f) => f.enable(&mut ctx.out_vulkan_10_features),
            Inner::Vulkan11(f) => ctx.set_features(|mut features| { f.enable(&mut features); features }),
            Inner::Vulkan12(f) => ctx.set_features(|mut features| { f.enable(&mut features); features }),
            Inner::Ext(f) => f.enable(ctx),
        }
    }
}

struct FeatureExt(inner::Support<8>, inner::Enable<8>);

impl FeatureExt {

    #[inline]
    fn query_support(
        &self,
        ctx: &mut DeviceFeatureContext
    ) -> Result<()> {
        (*self.0)(ctx)
    }

    #[inline]
    fn enable(
        &self,
        ctx: &mut DeviceFeatureContext,
    ) {
        (*self.1)(ctx)
    }
}

/// Creates a [`DeviceFeature`].
///
/// You shouldn't call this function unless you are implementing a custom feature implementation.
///
/// # Safety
/// Any structure [`added`][1] by custom feature implementations **must** have a valid
/// [`StructureType`][2] for that structure.
///
/// [1]: DeviceFeatureContext::set_features
/// [2]: vk::StructureType
pub unsafe fn feature_ext<S, E>(
    fn_support: S,
    fn_enable: E,
    name: ConstName,
    enable: bool,
) -> DeviceFeature
    where 
        S: Fn(&mut DeviceFeatureContext) -> Result<()> + 'static,
        E: Fn(&mut DeviceFeatureContext) + 'static
{
    DeviceFeature {
        inner: Inner::Ext(FeatureExt(inner::Support::new(fn_support), inner::Enable::new(fn_enable))),
        name,
        enable,
    }
}

macro_rules! core_features {
    (
        $(#[doc = $doc:literal])*
        $field:ident $(as $name:ident)?,
        $(
            $(#[doc = $rest_doc:literal])*
            $rest_field:ident $(as $rest_name:ident)?
        ),* $(,)?
    ) => {
        core_features!(
            $(#[doc = $doc])*
            $field $(as $name)?
        );
        core_features!($(
            $(#[doc = $rest_doc])*
            $rest_field $(as $rest_name)?
        ),*);
    };
    () => {};
    (
        $(#[doc = $doc:literal])*
        $field:ident
    ) => {

        $(#[doc = $doc])*
        pub mod $field {
            use super::*;
            /// The name of this feature.
            pub const NAME: ConstName = ConstName::new(stringify!($field));
        }

        $(#[doc = $doc])*
        pub fn $field(enable: bool) -> DeviceFeature {
            feature(
                Inner::new(move |query_support, features| {
                    if query_support {
                        if features.$field == 0 {
                            return Err(Error::just_context(concat!(
                                "missing device feature ", stringify!($field)
                            )))
                        } else {
                            Ok(())
                        }
                    } else {
                        *features = features.$field(enable);
                        Ok(())
                    }
                }),
                $field::NAME,
                enable,
            )
        }
    };
    (
        $(#[doc = $doc:literal])*
        $field:ident as $name:ident
    ) => {

        $(#[doc = $doc])*
        pub mod $name {
            use super::*;
            /// The name of this feature.
            pub const NAME: ConstName = ConstName::new(stringify!($name));
        }

        $(#[doc = $doc])*
        pub fn $name(enable: bool) -> DeviceFeature {
            feature(
                Inner::new(move |query_support, features| {
                    if query_support {
                        if features.$field == 0 {
                            return Err(Error::just_context(concat!(
                                "missing device feature ", stringify!($name)
                            )))
                        } else {
                            Ok(())
                        }
                    } else {
                        *features = features.$field(enable);
                        Ok(())
                    }
                }),
                $name::NAME,
                enable,
            )
        }
    };
}

/// Core Vulkan 1.0 features.
///
/// [1]: Attribute
pub mod vulkan_10 {

    use super::*;

    leimu_mem::smallbox!(
        struct Inner: Fn[(bool, &mut vk::PhysicalDeviceFeatures) -> Result<()>]
    );

    pub(super) struct Feature(Inner<8>);

    impl Feature {

        #[inline]
        pub fn query_support(
            &self,
            features: &vk::PhysicalDeviceFeatures,
        ) -> Result<()> {
            (*self.0)(true, &mut features.clone())
        }

        pub fn enable(
            &self,
            features: &mut vk::PhysicalDeviceFeatures,
        ) {
            (*self.0)(false, features).ok();
        }
    }

    #[inline]
    fn feature(
        inner: Inner<8>,
        name: ConstName,
        enable: bool,
    ) -> DeviceFeature {
        DeviceFeature {
            inner: features::Inner::Vulkan10(Feature(inner)),
            name,
            enable,
        }
    } 

    core_features!(
        /// Enables [`robust buffer access`][1] guarantees for shader buffer accesses.
        ///
        /// [1]: https://docs.vulkan.org/spec/latest/chapters/shaders.html#shaders-robust-buffer-access
        robust_buffer_access,
        /// Specifies the full 32-bit range of indices is supported for indexed draw calls when using
        /// a [`IndexType::U32`].
        full_draw_index_uint32,
        /// Specifies whether [`image views`][1] **can** be created with the [`Cube`][2] and
        /// [`CubeArray`][3] view types.
        ///
        /// [1]: Gpu::create_image_view
        /// [2]: ImageViewType::Cube
        /// [3]: ImageViewType::CubeArray
        image_cube_array,
        /// Specifies whether the [`color blend`][1] settings are controlled independently per-attachment.
        ///
        /// [1]: ColorOutputBlendState
        independent_blend,
        /// Specifies whether geometry shaders are supported.
        geometry_shader,
        /// Specifies whether tessellation shaders are supported.
        tessellation_shader,
        /// Specifies whether samples shading and multisample interpolation are supported.
        sample_rate_shading,
        /// Specifies whether blend operations which take two sources are supported.
        dual_src_blend,
        /// Specifies whether [`logic operations`][1] are supported. 
        ///
        /// [1]: GraphicsPipelineCreateInfo::with_logic_op
        logic_op,
        /// Specifies whether multiple draw indirect is supported.
        multi_draw_indirect,
        /// Specifies whether indirect drawing calls support the `first_instance` parameter.
        draw_indirect_first_instance,
        /// Specifies whether [`depth clamping`][1] is supported.
        ///
        /// [1]: GraphicsPipelineCreateInfo::with_depth_clamp
        depth_clamp,
        /// Specifies whether [`depth bias clamping`][1] is supported.
        ///
        /// [1]: DepthBiasInfo::clamp
        depth_bias_clamp,
        /// Specifies whether [`point`][1] and [`wireframe`][2] fill modes are supported.
        ///
        /// [1]: PolygonMode::Point
        /// [2]: PolygonMode::Line
        fill_mode_non_solid,
        /// Specifies whether depth bounds tests are supported.
        depth_bounds,
        /// Specifies whether lines with width other than 1.0 are supported.
        wide_lines,
        /// Specifies whether points with size greater than 1.0 are supported.
        large_points,
        /// Specifies whether replacing the alpha value of the fragment shader color output in the
        /// multisample coverage fragment operation.
        ///
        /// If this feature is not enabled, then [`alpha_to_one`][1] member of [`SampleShadingInfo`]
        /// **must** be `false`.
        ///
        /// [1]: SampleShadingInfo::alpha_to_one
        alpha_to_one,
        /// Specifies whether more than one viewport is supported.
        multi_viewport,
        /// Specifies whether anisotropic filtering is supported.
        sampler_anisotropy,
        /// Specifies whether all of the ETC2 and EAC compressed texture [`formats`][1] are supported.
        ///
        /// [1]: Format
        texture_compression_etc2,
        /// Specifies whether all of the ASTC LDR compressed texture [`formats`][1] are supported.
        ///
        /// [1]: Format
        texture_compression_astc_ldr,
        /// Specifies whether all of the BC compressed texture [`formats`][1] are supported.
        ///
        /// [1]: Format
        texture_compression_bc,
        /// Specifies whether occlusion queries returning actual sample counts are supported.
        occlusion_query_precise,
        /// Specifies whether the pipeline statistics queries are supported.
        pipeline_statistics_query,
        /// Specifies whether storage buffers and images support stores and atomic operations in the
        /// vertex, tessellation, and geometry shader stages.
        vertex_pipeline_stores_and_atomics,
        /// Specifies whether storage buffers and images support stores and atomic operations in the
        /// fragment shader stage.
        fragment_stores_and_atomics,
        /// Specifies whether the PointSize built-in decoration is available in the tessellation
        /// control, tessellation evaluation, and geometry shader stages.
        shader_tessellation_and_geometry_point_size,
        /// Specifies whether the extended set of image gather instructions are available in shader
        /// code.
        shader_image_gather_extended,
        /// Specifies whether all the “storage image extended formats” are supported.
        ///
        /// # Storage image extended formats
        /// - [`Format::R16g16_Sfloat`]
        /// - [`Format::B10g11r11_Ufloat_Pack32`]
        /// - [`Format::R16_Sfloat`]
        /// - [`Format::R16g16b16a16_Unorm`]
        /// - [`Format::A2b10g10r10_Unorm_Pack32`]
        /// - [`Format::R16g16_Unorm`]
        /// - [`Format::R8g8_Unorm`]
        /// - [`Format::R16_Unorm`]
        /// - [`Format::R8_Unorm`]
        /// - [`Format::R16g16b16a16_Snorm`]
        /// - [`Format::R16g16_Snorm`]
        /// - [`Format::R8g8_Snorm`]
        /// - [`Format::R16_Snorm`]
        /// - [`Format::R8_Snorm`]
        /// - [`Format::R16g16_Sint`]
        /// - [`Format::R8g8_Sint`]
        /// - [`Format::R16_Sint`]
        /// - [`Format::R8_Sint`]
        /// - [`Format::A2b10g10r10_Uint_Pack32`]
        /// - [`Format::R16g16_Uint`]
        /// - [`Format::R8g8_Uint`]
        /// - [`Format::R16_Uint`]
        /// - [`Format::R8_Uint`]
        shader_storage_image_extended_formats,
        /// Specifies whether multisampled storage images are supported.
        shader_storage_image_multisample,
        /// Specifies whether storage images and storage texel buffers require a format qualifier to be
        /// specified when reading.
        shader_storage_image_read_without_format,
        /// Specifies whether storage images and storage texel buffers require a format qualifier to be
        /// specified when writing.
        shader_storage_image_write_without_format,
        /// Specifies whether arrays of uniform buffers **can** be indexed by integer expressions that are
        /// dynamically uniform within either the subgroup or the invocation group in shader code..
        shader_uniform_buffer_array_dynamic_indexing,
        /// Specifies whether arrays of samplers or sampled images **can** be indexed by integer
        /// expressions that are dynamically uniform within either the subgroup or the invocation group
        /// in shader code.
        shader_sampled_image_array_dynamic_indexing,
        /// Specifies whether arrays of storage buffers **can** be indexed by integer expressions that are
        /// dynamically uniform within either the subgroup or the invocation group in shader code.
        shader_storage_buffer_array_dynamic_indexing,
        /// Specifies whether arrays of storage images **can** be indexed by integer expressions that are
        /// dynamically uniform within either the subgroup or the invocation group in shader code.
        shader_storage_image_array_dynamic_indexing,
        /// Specifies whether clip distances are supported in shader code.
        shader_clip_distance,
        /// Specifies whether cull distances are supported in shader code.
        shader_cull_distance,
        /// Specifies whether 64-bit floats are supported in shader code.
        shader_float64,
        /// Specifies whether 64-bit integers (signed and unsigned) are supported in shader code.
        shader_int64,
        /// Specifies whether 16-bit integers (signed and unsigned) are supported in shader code.
        shader_int16,
        /// Specifies whether image operations that return resource residency information are supported
        /// in shader code.
        shader_resource_residency,
        /// Specifies whether image operations specifying the minimum resource LOD are supported in
        /// shader code.
        shader_resource_min_lod,
        /// Specifies whether resource memory **can** be managed at opaque sparse block level instead of at
        /// the object level.
        sparse_binding,
        /// Specifies whether the device **can** access partially resident buffers.
        sparse_residency_buffer,
        /// Specifies whether the device **can** access partially resident 2D images with 1 sample per
        /// pixel.
        sparse_residency_image2_d as sparse_residency_image_2d,
        /// Specifies whether the device **can** access partially resident 3D images.
        sparse_residency_image3_d as sparse_residency_image_3d,
        /// Specifies whether the physical device **can** access partially resident 2D images with 2
        /// samples per pixel.
        sparse_residency2_samples as sparse_residency_2_samples,
        /// Specifies whether the physical device **can** access partially resident 2D images with 4
        /// samples per pixel.
        sparse_residency4_samples as sparse_residency_4_samples,
        /// Specifies whether the physical device **can** access partially resident 2D images with 8
        /// samples per pixel.
        sparse_residency8_samples as sparse_residency_8_samples,
        /// Specifies whether the physical device **can** access partially resident 2D images with 16
        /// samples per pixel.
        sparse_residency16_samples as sparse_residency_16_samples,
        /// Specifies whether the physical device **can** correctly access data aliased into multiple
        /// locations.
        sparse_residency_aliased,
        /// Specifies whether pipelines in a subpass with no attachments **must** have the same
        /// [`sample count`][1].
        ///
        /// [1]: SampleShadingInfo::samples
        variable_multisample_rate,
        /// Specifies whether a secondary command buffer **may** be executed while a query is active.
        inherited_queries,
    );
}

/// Core Vulkan 1.1 features and [`attributes`][Attribute].
pub mod vulkan_11 {

    use super::*;

    leimu_mem::smallbox!(
        struct Inner: Fn[(bool, &mut vk::PhysicalDeviceVulkan11Features) -> Result<()>]
    );

    pub(super) struct Feature(Inner<8>);

    impl Feature {

        #[inline]
        pub fn query_support(
            &self,
            features: &vk::PhysicalDeviceVulkan11Features,
        ) -> Result<()> {
            (*self.0)(true, &mut features.clone())
        }

        pub fn enable(
            &self,
            features: &mut vk::PhysicalDeviceVulkan11Features,
        ) {
            (*self.0)(false, features).ok();
        }
    }

    #[inline]
    fn feature(
        inner: Inner<8>,
        name: ConstName,
        enable: bool,
    ) -> DeviceFeature {
        DeviceFeature {
            inner: features::Inner::Vulkan11(Feature(inner)),
            name,
            enable,
        }
    } 

    core_features!(
        /// Specifies whether SPIR-V objects in the `StorageBuffer`, `ShaderRecordBufferKHR`, or
        /// `PhysicalStorageBuffer` storage class with the `Block` decoration **can** have 16-bit
        /// integer and floating-point members.
        storage_buffer16_bit_access as storage_buffer_16_bit_access,
        /// Specifies whether SPIR-V objects in the `Uniform` storage class with the `Block` decoration
        /// **can** have 16-bit integer and floating-point members.
        uniform_and_storage_buffer16_bit_access as uniform_and_storage_buffer_16_bit_access,
        /// Spcifies whether SPIR-V objects in the `PushConstant` storage class **can** have 16-bit
        /// integer and floating-point members.
        storage_push_constant16 as storage_push_constant_16,
        /// Specifies whether SPIR-V objects in the `Input` and `Output` storage classes **can** have
        /// 16-bit integer and floating-point members.
        storage_input_output16 as storage_input_output_16,
        /// Specifies whether multiview rendering is supported. If thius feature is not enabled, the
        /// [`view_mask`][1] member of [`RenderingInfo`] **must** be zero.
        ///
        /// [1]: RenderingInfo::view_mask
        multiview,
        /// Specifies whether multiview rendering is supported with [`geometry shaders`][1].
        ///
        /// [1]: ShaderStage::Geometry
        multiview_geometry_shader,
        /// Specifies whether multiview rendering is supported with tessellation shaders.
        multiview_tessellation_shader,
        /// Specifies whether SPIR-V `VariablePointersStorageBuffer` capability is supported.
        variable_pointers_storage_buffer,
        /// Specifies whether SPIR-V `VariablePointers` capability is supported.
        variable_pointers,
        /// Specifies whether protected memory is supported.
        protected_memory,
        /// Specifies whether sampler Ycbcr conversion is supported.
        sampler_ycbcr_conversion,
        /// Specifies whether SPIR-V `DrawParameters` capability is supported.
        shader_draw_parameters,
    );
}

/// Core Vulkan 1.2 features.
///
/// The timeline_semaphore and separate_depth_stencil_layouts features are always enabled.
pub mod vulkan_12 {

    use super::*;
    
    leimu_mem::smallbox!(
        struct Inner: Fn[(bool, &mut vk::PhysicalDeviceVulkan12Features) -> Result<()>]
    );

    pub(super) struct Feature(Inner<8>);

    impl Feature {

        #[inline]
        pub fn query_support(
            &self,
            features: &vk::PhysicalDeviceVulkan12Features,
        ) -> Result<()> {
            (*self.0)(true, &mut features.clone())
        }

        pub fn enable(
            &self,
            features: &mut vk::PhysicalDeviceVulkan12Features,
        ) {
            (*self.0)(false, features).ok();
        }
    }

    #[inline]
    fn feature(
        inner: Inner<8>,
        name: ConstName,
        enable: bool,
    ) -> DeviceFeature {
        DeviceFeature {
            inner: features::Inner::Vulkan12(Feature(inner)),
            name,
            enable,
        }
    }

    core_features!(
        /// Allows the use of the [`MirrorClampToEdge`][1] [`SamplerAddressMode`].
        ///
        /// [1]: SamplerAddressMode::MirrorClampToEdge
        sampler_mirror_clamp_to_edge,
        /// Allows the use of `vkCmdDrawIndirectCount` and `vkCmdDrawIndexedIndirectCount`.
        draw_indirect_count,
        /// Specifies whether SPIR-V objects in the `StorageBuffer` `ShaderRecordBufferKHR`, or
        /// `PhysicalStorageBuffer` storage class with the `Block` decoration **can** have 8-bit
        /// integer members.
        storage_buffer8_bit_access as storage_buffer_8_bit_access,
        /// Specifies whether SPIR-V objects in the Uniform storage class with the `Block`
        /// decoration **can** have 8-bit integer members.
        uniform_and_storage_buffer8_bit_access as uniform_and_storage_buffer_8_bit_access,
        /// Specifies whether objects in the `PushConstant` storage class **can** have 8-bit integer
        /// members.
        storage_push_constant8 as storage_push_constant_8,
        /// Specifies whether shaders **can** perform 64-bit unsigned and signed integer atomic
        /// operations on buffers.
        shader_buffer_int64_atomics,
        /// Specifies whether shaders **can** perform 64-bit unsigned and signed integer atomic
        /// operations on shared and payload memory.
        shader_shared_int64_atomics,
        /// Specifies whether 16-bit floats are supported in shader code.
        shader_float16,
        /// Specifies whether 8-bit unsigned and signed integers are supported in shader code.
        shader_int8,
        /// Specifies whether the the minimum set of descriptor indexing features are supported.
        ///
        /// The minimum set includes:
        /// - [`shader_sampled_image_array_dynamic_indexing`][1]
        /// - [`shader_storage_buffer_array_dynamic_indexing`][2]
        /// - [`shader_uniform_texel_buffer_array_dynamic_indexing`]
        /// - [`shader_storage_texel_buffer_array_dynamic_indexing`]
        /// - [`shader_sampled_image_array_non_uniform_indexing`]
        /// - [`shader_storage_buffer_array_non_uniform_indexing`]
        /// - [`shader_uniform_texel_buffer_array_non_uniform_indexing`]
        /// - [`descriptor_binding_sampled_image_update_after_bind`]
        /// - [`descriptor_binding_storage_image_update_after_bind`]
        /// - [`descriptor_binding_storage_buffer_update_after_bind`]
        /// - [`descriptor_binding_uniform_texel_buffer_update_after_bind`]
        /// - [`descriptor_binding_storage_texel_buffer_update_after_bind`]
        /// - [`descriptor_binding_update_unused_while_pending`]
        /// - [`descriptor_binding_partially_bound`]
        /// - [`runtime_descriptor_array`]
        ///
        /// [1]: vulkan_10::shader_sampled_image_array_dynamic_indexing
        /// [2]: vulkan_10::shader_storage_buffer_array_dynamic_indexing
        descriptor_indexing,
        /// Specifies whether arrays of input attachments **can** be indexed by integer expressions
        /// that are dynamically uniform within either the subgroup or the invocation group in
        /// shader code.
        shader_input_attachment_array_dynamic_indexing,
        /// Specifies whether arrays of uniform texel buffers **can** be indexed by integer expressions
        /// that are dynamically uniform within either the subgroup or the invocation group in
        /// shader code.
        shader_uniform_texel_buffer_array_dynamic_indexing,
        /// Specifies whether arrays of storage texel buffers **can** be indexed by integer expressions
        /// that are dynamically uniform within either the subgroup or the invocation group in
        /// shader code.
        shader_storage_texel_buffer_array_dynamic_indexing,
        /// Specifies whether arrays of uniform buffers **can** be indexed by
        /// non-uniform integer expressions in shader code.
        shader_uniform_buffer_array_non_uniform_indexing,
        /// Specifies whether arrays of samplers or sampled images **can** be indexed by
        /// non-uniform integer expressions in shader code.
        shader_sampled_image_array_non_uniform_indexing,
        /// Specifies whether arrays of storage buffers **can** be indexed by non-uniform
        /// integer expressions in shader code.
        shader_storage_buffer_array_non_uniform_indexing,
        /// Specifies whether arrays of storage images **can** be indexed by non-uniform integer
        /// expressions in shaader code.
        shader_storage_image_array_non_uniform_indexing,
        /// Specifies whether arrays of input attachments **can** be indexed by non-uniform integer
        /// expressions in shader code.
        shader_input_attachment_array_non_uniform_indexing,
        /// Specifies whether arrays of uniform texel buffers can be indexed by non-uniform integer
        /// expressions in shader code.
        shader_uniform_texel_buffer_array_non_uniform_indexing,
        /// Specifies whether arrays of storage texel buffers can be indexed by non-uniform integer
        /// expressions in shader code.
        shader_storage_texel_buffer_array_non_uniform_indexing,
        /// Specifies whether the uniform buffer descriptors **can** be updated after a set is bound.
        descriptor_binding_uniform_buffer_update_after_bind,
        /// Specifies whether the sampled image descriptors **can** be updated after a set is bound.
        descriptor_binding_sampled_image_update_after_bind,
        /// Specifies whether the storage image descriptors **can** be updated after a set is bound.
        descriptor_binding_storage_image_update_after_bind,
        /// Specifies whether the storage buffer descriptors **can** be updated after a set is bound.
        descriptor_binding_storage_buffer_update_after_bind,
        /// Specifies whether the uniform texel buffer descriptors **can** be updated after a set is bound.
        descriptor_binding_uniform_texel_buffer_update_after_bind,
        /// Specifies whether the storage texel buffer descriptors **can** be updated after a set is bound.
        descriptor_binding_storage_texel_buffer_update_after_bind,
        /// Specifies whether descriptors can be updated while the set is in use.
        descriptor_binding_update_unused_while_pending,
        /// Specifies whether statically using a descriptor set binding in which some descriptors
        /// are not valid is supported.
        descriptor_binding_partially_bound,
        /// Specifies whether descriptor sets with a variable-sized last binding is supported.
        descriptor_binding_variable_descriptor_count,
        /// Specifies whether SPIR-V `RunTimeDescriptorArray` capability is supported.
        runtime_descriptor_array,
        /// Specifies whether a minimum set of required formats supporting min/max filtering is
        /// supported.
        ///
        /// The minimum set includes:
        /// - [`Format::R8_Unorm`]
        /// - [`Format::R8_Snorm`]
        /// - [`Format::R16_Unorm`]
        /// - [`Format::R16_Snorm`]
        /// - [`Format::R16_Sfloat`]
        /// - [`Format::R32_Sfloat`]
        /// - [`Format::D16_Unorm`]
        /// - [`Format::X8_D24_Unorm_Pack32`]
        /// - [`Format::D32_Sfloat`]
        /// - [`Format::D16_Unorm_S8_Uint`]
        /// - [`Format::D24_Unorm_S8_Uint`]
        /// - [`Format::D32_Sfloat_S8_Uint`]
        sampler_filter_minmax,
        /// Specifies whether layout of resource blocks in shaders using scalar alginment is
        /// supported.
        scalar_block_layout,
        /// Specifies whether uniform buffers support the same layouts as storage and other kinds of
        /// buffers.
        uniform_buffer_standard_layout,
        /// Specifies whether subgroup operations **can** use 8-bit integer, 16-bit integer, 64-bit
        /// integer, 16-bit floating-point, and vectors of these types in group operations with
        /// subgroup scope, if the implementation supports the types.
        shader_subgroup_extended_types,
        /// Specifies whether resetting queries from the host with `vkResetQueryPool` is supported.
        host_query_reset,
        /// Specifies a [`device address`][DeviceAddress] **can** be queried with
        /// [`get_buffer_device_address`][1].
        ///
        /// [1]: Gpu::get_buffer_device_address
        buffer_device_address,
        /// Specifies whether shader modules **can** declare the `VulkanMemoryModel` capability.
        vulkan_memory_model,
        /// Specifies whether `VulkanMemoryModel` can use `Device` scope synchronization.
        vulkan_memory_model_device_scope,
        /// Specifies whether the Vulkan Memory Model can use availability and visibility chains with
        /// more than one element.
        vulkan_memory_model_availability_visibility_chains,
        /// Specifies whether the `ShaderViewportIndex` SPIR-V capability is supported, enabling
        /// variables decorated with the `ViewportIndex` built-in to be exported from mesh, vertex
        /// or tessellation evaluation shaders.
        shader_output_viewport_index,
        /// Specifies whether the `ShaderLayer` SPIR-V capability is supported, enabling variables
        /// decorated with the `Layer` built-in to be exported from mesh, vertex or tessellation
        /// evaluation shaders.
        shader_output_layer,
        /// Specifies whether the "Id" operand of `OpGroupNonUniformBroadcast` **can** be
        /// dynamically uniform within a subgroup, and the "Index" operand of
        /// `OpGroupNonUniformQuadBroadcast` **can** be dynamically uniform within the derivative
        /// group.
        subgroup_broadcast_dynamic_id,
    );

    #[inline]
    pub(crate) fn timeline_semaphore() -> DeviceFeature {
        feature(
            Inner::new(|query_support, features| if query_support {
                if features.timeline_semaphore == 0 {
                    return Err(Error::just_context(
                        "missing required feature timeline_semaphore"
                    ))
                } else {
                    Ok(())
                }
            } else {
                *features = features.timeline_semaphore(true);
                Ok(())
            }),
            ConstName::new("timeline_semaphore"),
            true,
        )
    }

    #[inline]
    pub(crate) fn separate_depth_stencil_layouts() -> DeviceFeature {
        feature(
            Inner::new(|query_support, features| if query_support {
                if features.separate_depth_stencil_layouts == 0 {
                    return Err(Error::just_context(
                        "missing required feature separate_depth_stencil_layouts"
                    ))
                } else {
                    Ok(())
                }
            } else {
                *features = features.separate_depth_stencil_layouts(true);
                Ok(())
            }),
            ConstName::new("separate_depth_stencil_layouts"),
            true,
        )
    }
}

/// Core Vulkan 1.3 features.
pub mod vulkan_13 {

    use super::*;

    /// Specifies support for Robust Image Access guarantees for shader image accesses.
    pub mod robust_image_access {

        use super::*;
    
        /// The name of this feature.
        pub const NAME: ConstName = ConstName::new(stringify!(robust_image_access));
    }

    /// Specifies support for Robust Image Access guarantees for shader image accesses.
    pub fn robust_image_access(enable: bool) -> DeviceFeature {
        use tuhka::ext;
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(ext::image_robustness::NAME)?;
                }
                let mut features = vk::PhysicalDeviceImageRobustnessFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.robust_image_access == 0 {
                    Err(Error::just_context("robust_image_access not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceImageRobustnessFeatures|
                            features.robust_image_access(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.robust_image_access(enable)
                    );
                }
            },
            robust_image_access::NAME,
            enable,
        ) }
    }

    /// Specifies that [`inline uniform block descriptors`][1] **may** be used.
    ///
    /// [1]: ShaderSetAttributes::with_inline_uniform_block 
    pub mod inline_uniform_block {

        use super::*;

        /// The name of this feature.
        pub const NAME: ConstName = ConstName::new(stringify!(inline_uniform_block));
   
        /// [`inline_uniform_block`] properties.
        #[derive(Clone, Copy)]
        pub struct Properties {
            /// The maximum size in bytes of a single inline uniform block binding.
            pub max_inline_uniform_block_size: u32,
            /// The maximum number of inline uniform blocks that **can** be accessible to a single
            /// shader stage.
            pub max_per_stage_descriptor_inline_uniform_blocks: u32,
            /// The maximum number of inline uniform blocks that **can** be accessible to a single
            /// shader stage with or without the `UPDATE_AFTER_BIND_POOL` bit set.
            pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
            /// The maximum number of inline uniform block bindings that **can** be included in
            /// descriptor bindings of a [`ShaderSet`] across all pipeline shader stages and
            /// descriptor set numbers.
            pub max_descriptor_set_inline_uniform_blocks: u32,
            /// The maximum number of inline uniform block bindings that **can** be included in
            /// descriptor bindings of a [`ShaderSet`] across all pipeline shader stages and
            /// descriptor set numbers with or without the `UPDATE_AFTER_BIND_POOL` bit set.
            pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
            /// The maximum total size in bytes of all inline uniform block bindings across all
            /// pipeline shader stages and descriptor set numbers that **can** be included in a
            /// [`ShaderSet`].
            pub max_inline_uniform_total_size: u32,
        }

        /// [`Properties`] [`device attribute`][1].
        ///
        /// [1]: Gpu::get_device_attribute
        pub const PROPERTIES: AttributeName<Properties> = AttributeName::new("inline_uniform_block Properties");
    }

    /// Specifies that [`inline uniform block descriptors`][1] **may** be used.
    ///
    /// [1]: ShaderSetAttributes::with_inline_uniform_block 
    pub fn inline_uniform_block(enable: bool) -> DeviceFeature {
        use tuhka::ext;
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(ext::inline_uniform_block::NAME)?;
                }
                let mut features = vk::PhysicalDeviceInlineUniformBlockFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.inline_uniform_block == 0 {
                    Err(Error::just_context("inline_uniform_block not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceInlineUniformBlockFeatures|
                            features.inline_uniform_block(enable)
                    );
                    let mut properties = vk::PhysicalDeviceInlineUniformBlockProperties
                        ::default();
                    ctx.get_physical_device_properties(&mut properties);
                    ctx.add_device_attribute(Attribute::new(
                        inline_uniform_block::PROPERTIES,
                        inline_uniform_block::Properties {
                            max_inline_uniform_block_size:
                                properties.max_inline_uniform_block_size,
                            max_per_stage_descriptor_inline_uniform_blocks:
                                properties.max_per_stage_descriptor_inline_uniform_blocks,
                            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks:
                                properties.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks,
                            max_descriptor_set_inline_uniform_blocks:
                                properties.max_descriptor_set_inline_uniform_blocks,
                            max_descriptor_set_update_after_bind_inline_uniform_blocks:
                                properties.max_descriptor_set_update_after_bind_inline_uniform_blocks,
                            max_inline_uniform_total_size: u32::MAX,
                        },
                    ));
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.inline_uniform_block(enable)
                    );
                    let mut properties = vk::PhysicalDeviceVulkan13Properties
                        ::default();
                    ctx.get_physical_device_properties(&mut properties);
                    ctx.add_device_attribute(Attribute::new(
                        inline_uniform_block::PROPERTIES,
                        inline_uniform_block::Properties {
                            max_inline_uniform_block_size:
                                properties.max_inline_uniform_block_size,
                            max_per_stage_descriptor_inline_uniform_blocks:
                                properties.max_per_stage_descriptor_inline_uniform_blocks,
                            max_per_stage_descriptor_update_after_bind_inline_uniform_blocks:
                                properties.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks,
                            max_descriptor_set_inline_uniform_blocks:
                                properties.max_descriptor_set_inline_uniform_blocks,
                            max_descriptor_set_update_after_bind_inline_uniform_blocks:
                                properties.max_descriptor_set_update_after_bind_inline_uniform_blocks,
                            max_inline_uniform_total_size:
                                properties.max_inline_uniform_total_size,
                        },
                    ));
                }
            },
            inline_uniform_block::NAME,
            enable
        ) }
    }

    /// Specifies that inline uniform block descriptors **can** be updated after a set is bound.
    pub mod descriptor_binding_inline_uniform_block_update_after_bind {

        use super::*;

        /// The name of this feature.
        pub const NAME: ConstName = ConstName::new(stringify!(descriptor_binding_inline_uniform_block_update_after_bind));
    }

    /// Specifies that inline uniform block descriptors **can** be updated after a set is bound.
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(enable: bool) -> DeviceFeature {
        use tuhka::ext;
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(ext::inline_uniform_block::NAME)?;
                }
                let mut features = vk::PhysicalDeviceInlineUniformBlockFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.descriptor_binding_inline_uniform_block_update_after_bind == 0 {
                    Err(Error::just_context("descriptor_binding_inline_uniform_block_update_after_bind not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceInlineUniformBlockFeatures|
                            features.descriptor_binding_inline_uniform_block_update_after_bind(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.descriptor_binding_inline_uniform_block_update_after_bind(enable)
                    );
                }
            },
            inline_uniform_block::NAME,
            enable,
        ) }
    }
    /// Specifies that the `DemoteToHelperInvocationEXT` SPIR-V capability **can** be used.
    pub mod shader_demote_to_helper_invocation {
       
        use super::*;

        /// The name of this feature.
        pub const NAME: ConstName = ConstName::new(stringify!(shader_demote_to_helper_invocation));
    }

    /// Specifies that the `DemoteToHelperInvocationEXT` SPIR-V capability **can** be used.
    pub fn shader_demote_to_helper_invocation(enable: bool) -> DeviceFeature {
        use tuhka::ext;
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(ext::shader_demote_to_helper_invocation::NAME)?;
                }
                let mut features = vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.shader_demote_to_helper_invocation == 0 {
                    Err(Error::just_context("shader_demote_to_helper_invocation not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures|
                            features.shader_demote_to_helper_invocation(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.shader_demote_to_helper_invocation(enable)
                    );
                }
            },
            shader_demote_to_helper_invocation::NAME,
            enable,
        ) }
    }

    /// Specifies that the SPIR-V extension `SPV_KHR_terminate_invocation` is supported.
    pub mod shader_terminate_invocation {

        use super::*;

        /// The name of this feature.
        pub const NAME: ConstName = ConstName::new(stringify!(shader_terminate_invocation));
    }

    /// Specifies that the SPIR-V extension `SPV_KHR_terminate_invocation` is supported.
    pub fn shader_terminate_invocation(enable: bool) -> DeviceFeature {
        use tuhka::khr;
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(khr::shader_terminate_invocation::NAME)?;
                }
                let mut features = vk::PhysicalDeviceShaderTerminateInvocationFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.shader_terminate_invocation == 0 {
                    Err(Error::just_context("shader_terminate_invocation not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceShaderTerminateInvocationFeatures|
                            features.shader_terminate_invocation(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.shader_terminate_invocation(enable)
                    );
                }
            },
            shader_terminate_invocation::NAME,
            enable
        ) }
    }

    /// Allows the use of [`varying subgroup size`][1] and the specification of
    /// [`required_subgroup_size`][2] when creating a pipeline [`shader stage`][3].
    ///
    /// [1]: PipelineShaderStageCreateFlags::ALLOW_VARYING_SUBGROUP_SIZE
    /// [2]: PipelineShaderStageCreateInfo::required_subgroup_size
    /// [3]: PipelineShaderStageCreate|nfo
    pub mod subgroup_size_control {

        use super::*;

        /// The name of this feature.
        pub const NAME: ConstName = ConstName::new(stringify!(subgroup_size_control));

        /// [`subgroup_size_control`] properties.
        #[derive(Clone, Copy)]
        pub struct Properties {
            /// The minimum subgroup size supported by this device.
            pub min_subgroup_size: u32,
            /// The maximum subgroup size supported by this device.
            pub max_subgroup_size: u32,
            /// The maximum number of subgroups supported by the implementation within a workgroup.
            pub max_compute_workgroup_subgroups: u32,
            /// A bitmask of [`ShaderStageFlags`] specifying which shader stages support having
            /// [`required subgroup size`][1] specified.
            ///
            /// [1]: PipelineShaderStageCreateInfo::required_subgroup_size
            pub required_subgroup_size_stages: ShaderStageFlags,
        }

        /// [`Properties`] [`device attribute`][1].
        ///
        /// [1]: Gpu::get_device_attribute
        pub const PROPERTIES: AttributeName<Properties> = AttributeName::new("subgroup_size_control Properties");
    }

    /// Allows the use of [`varying subgroup size`][1] and the specification of
    /// [`required_subgroup_size`][2] when creating a pipeline [`shader stage`][3].
    ///
    /// [1]: PipelineShaderStageCreateFlags::ALLOW_VARYING_SUBGROUP_SIZE
    /// [2]: PipelineShaderStageCreateInfo::required_subgroup_size
    /// [3]: PipelineShaderStageCreate|nfo
    pub fn subgroup_size_control(enable: bool) -> DeviceFeature {
        use tuhka::ext;
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(ext::subgroup_size_control::NAME)?;
                }
                let mut features = vk::PhysicalDeviceSubgroupSizeControlFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.subgroup_size_control == 0 {
                    return Err(Error::just_context("subgroup_size_control not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceSubgroupSizeControlFeatures|
                            features.subgroup_size_control(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.subgroup_size_control(enable)
                    );
                }
            },
            subgroup_size_control::NAME,
            enable,
        ) }
    }

    /// Allows the use of [`full subgroups`][1] when creating a pipeline [`shader stage`][2]
    ///
    /// [1]: PipelineShaderStageCreateFlags::REQUIRE_FULL_SUBGROUPS
    /// [2]: PipelineShaderStageCreateInfo
    pub mod compute_full_subgroups {

        use super::*;

        /// The name of this feature.
        pub const NAME: ConstName = ConstName::new(stringify!(compute_full_subgroups));
    }

    /// Allows the use of [`full subgroups`][1] when creating a pipeline [`shader stage`][2]
    ///
    /// [1]: PipelineShaderStageCreateFlags::REQUIRE_FULL_SUBGROUPS
    /// [2]: PipelineShaderStageCreateInfo
    pub fn compute_full_subgroups(enable: bool) -> DeviceFeature {
        use tuhka::ext;
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(ext::subgroup_size_control::NAME)?;
                }
                let mut features = vk::PhysicalDeviceSubgroupSizeControlFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.compute_full_subgroups == 0 {
                    return Err(Error::just_context("compute_full_subgroups not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceSubgroupSizeControlFeatures|
                            features.compute_full_subgroups(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.compute_full_subgroups(enable)
                    );
                }
            },
            subgroup_size_control::NAME,
            enable,
        ) }
    }

    pub(crate) fn synchronization2() -> DeviceFeature {
        use tuhka::khr;
        const NAME: ConstName = ConstName::new("synchronization2");
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(khr::synchronization2::NAME)?;
                }
                let mut features = vk::PhysicalDeviceSynchronization2Features
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.synchronization2 == 0 {
                    return Err(Error::just_context("required feature synchronization2 not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceSynchronization2Features|
                            features.synchronization2(true)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.synchronization2(true)
                    );
                }
            },
            NAME,
            true,
        ) }
    }

    /// Specifies that all of the ASTC HDR compressed texture formats are supported.
    pub mod texture_compression_astc_hdr {

        use super::*;

        /// The name of this feature.
        pub const NAME: ConstName = ConstName::new(stringify!(texture_compression_astc__hdr));
    }

    /// Specifies that all of the ASTC HDR compressed texture formats are supported.
    pub fn texture_compression_astc_hdr(enable: bool) -> DeviceFeature {
        use tuhka::ext;
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(ext::texture_compression_astc_hdr::NAME)?;
                }
                let mut features = vk::PhysicalDeviceTextureCompressionASTCHDRFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.texture_compression_astc_hdr == 0 {
                    return Err(Error::just_context("texture_compression_astc_hdr not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceTextureCompressionASTCHDRFeatures|
                            features.texture_compression_astc_hdr(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.texture_compression_astc_hdr(enable)
                    );
                }
            },
            texture_compression_astc_hdr::NAME,
            enable,
        ) }
    }

    /// Specifies whether initializing a variable in Workgroup storage class is supported.
    pub mod shader_zero_initialize_workgroup_memory {

        use super::*;

        /// The name of this feature.
        pub const NAME: ConstName = ConstName::new(stringify!(shader_zero_initialize_workgroup_memory));
    }

    /// Specifies whether initializing a variable in Workgroup storage class is supported.
    pub fn shader_zero_initialize_workgroup_memory(enable: bool) -> DeviceFeature {
        use tuhka::khr;
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(khr::zero_initialize_workgroup_memory::NAME)?;
                }
                let mut features = vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.shader_zero_initialize_workgroup_memory == 0 {
                    return Err(Error::just_context("shader_zero_initialize_workgroup_memory not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures|
                            features.shader_zero_initialize_workgroup_memory(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.shader_zero_initialize_workgroup_memory(enable)
                    );
                }
            },
            shader_zero_initialize_workgroup_memory::NAME,
            enable,
        ) }
    }

    pub(crate) fn dynamic_rendering() -> DeviceFeature {
        use tuhka::khr;
        const NAME: ConstName = ConstName::new("dynamic_rendering");
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(khr::dynamic_rendering::NAME)?;
                }
                let mut features = vk::PhysicalDeviceDynamicRenderingFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.dynamic_rendering == 0 {
                    return Err(Error::just_context("required feature dynamic_rendering not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceDynamicRenderingFeatures|
                            features.dynamic_rendering(true)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.dynamic_rendering(true)
                    );
                }
            },
            NAME,
            true
        ) }
    }

    /// Specifies whether the `DotProductInputAllKHR`, `DotProductInput4x8BitKHR`,
    /// `DotProductInput4x8BitPackedKHR` and `DotProductKHR` capabilities are enabled.
    pub mod shader_integer_dot_product {

        use super::*;

        /// The name of this feature.
        pub const NAME: ConstName = ConstName::new(stringify!(shader_integer_dot_product));
    }

    pub fn shader_integer_dot_product(enable: bool) -> DeviceFeature {
        use tuhka::khr;
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(khr::shader_integer_dot_product::NAME)?;
                }
                let mut features = vk::PhysicalDeviceShaderIntegerDotProductFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.shader_integer_dot_product == 0 {
                    Err(Error::just_context("shader_integer_dot_product not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceShaderIntegerDotProductFeatures|
                            features.shader_integer_dot_product(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.shader_integer_dot_product(enable)
                    );
                }
            },
            shader_integer_dot_product::NAME,
            enable
        ) }
    }

    pub(crate) fn maintenance4() -> DeviceFeature {
        use tuhka::khr;
        const NAME: ConstName = ConstName::new("maintenance4");
        unsafe { feature_ext(
            move |ctx| -> Result<()> {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.add_required_extension(khr::maintenance4::NAME)?;
                }
                let mut features = vk::PhysicalDeviceMaintenance4Features
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.maintenance4 == 0 {
                    Err(Error::just_context("required feature maintenance4 not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_3 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceMaintenance4Features|
                            features.maintenance4(true)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan13Features|
                            features.maintenance4(true)
                    );
                }
            },
            NAME,
            true
        ) }
    }
}

/// Core Vulkan 1.4 features.
pub mod vulkan_14 {

    use tuhka::ext::vertex_attribute_robustness;

    use super::*;

    /// Allows the use of the SPIR-V `GroupNonUniformRotateKHR` capability.
    pub mod shader_subgroup_rotate {

        use super::*;

        /// The name of the feature.
        pub const NAME: ConstName = ConstName::new(stringify!(shader_subgroup_rotate));
    }

    /// Allows the use of the SPIR-V `GroupNonUniformRotateKHR` capability.
    pub fn shader_subgroup_rotate(enable: bool) -> DeviceFeature {
        use tuhka::khr;
        unsafe { feature_ext(
            |ctx| {
                if ctx.api_version() < API_VERSION_1_4 {
                    ctx.add_required_extension(khr::shader_subgroup_rotate::NAME)?;
                }
                let mut features = vk::PhysicalDeviceShaderSubgroupRotateFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.shader_subgroup_rotate == 0 {
                    Err(Error::just_context("shader_subgroup_rotate not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_4 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceShaderSubgroupRotateFeatures|
                            features.shader_subgroup_rotate(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan14Features|
                            features.shader_subgroup_rotate(enable)
                    );
                }
            },
            shader_subgroup_rotate::NAME,
            enable
        ) }
    }

    /// Allows the use of the `ClusterSize` operand to `GroupNonUniformRotateKHR`.
    pub mod shader_subgroup_rotate_clustered {

        use super::*;

        /// The name of the feature.
        pub const NAME: ConstName = ConstName::new(stringify!(shader_subgroup_rotate_clustered));
    }

    /// Allows the use of the `ClusterSize` operand to `GroupNonUniformRotateKHR`.
    pub fn shader_subgroup_rotate_clustered(enable: bool) -> DeviceFeature {
        use tuhka::khr;
        unsafe { feature_ext(
            |ctx| {
                if ctx.api_version() < API_VERSION_1_4 {
                    ctx.add_required_extension(khr::shader_subgroup_rotate::NAME)?;
                }
                let mut features = vk::PhysicalDeviceShaderSubgroupRotateFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.shader_subgroup_rotate_clustered == 0 {
                    Err(Error::just_context("shader_subgroup_rotate_clustered not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_4 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceShaderSubgroupRotateFeatures|
                            features.shader_subgroup_rotate_clustered(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan14Features|
                            features.shader_subgroup_rotate_clustered(enable)
                    );
                }
            },
            shader_subgroup_rotate_clustered::NAME,
            enable
        ) }
    }

    /// Allows the use of the `FloatControls2` SPIR-V capability.
    pub mod shader_float_controls2 {

        use super::*;

        /// The name of the feature.
        pub const NAME: ConstName = ConstName::new(stringify!(shader_float_controls2));
    }

    /// Allows the use of the `FloatControls2` SPIR-V capability.
    pub fn shader_float_controls2(enable: bool) -> DeviceFeature {
        use tuhka::khr;
        unsafe { feature_ext(
            |ctx| {
                if ctx.api_version() < API_VERSION_1_4 {
                    ctx.add_required_extension(khr::shader_float_controls2::NAME)?;
                }
                let mut features = vk::PhysicalDeviceShaderFloatControls2Features
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.shader_float_controls2 == 0 {
                    Err(Error::just_context("shader_float_controls2 not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_4 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceShaderFloatControls2Features|
                            features.shader_float_controls2(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan14Features|
                            features.shader_float_controls2(enable)
                    );
                }
            },
            shader_float_controls2::NAME,
            enable
        ) }
    }

    /// Allows the use of the `ExpectAssumeKHR` SPIR-V capability.
    pub mod shader_expect_assume {

        use super::*;

        /// The name of the feature.
        pub const NAME: ConstName = ConstName::new(stringify!(shader_expect_assume));
    }

    /// Allows the use of the `ExpectAssumeKHR` SPIR-V capability.
    pub fn shader_expect_assume(enable: bool) -> DeviceFeature {
        use tuhka::khr;
        unsafe { feature_ext(
            |ctx| {
                if ctx.api_version() < API_VERSION_1_4 {
                    ctx.add_required_extension(khr::shader_expect_assume::NAME)?;
                }
                let mut features = vk::PhysicalDeviceShaderExpectAssumeFeatures
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.shader_expect_assume == 0 {
                    Err(Error::just_context("shader_expect_assume not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                if ctx.api_version() < API_VERSION_1_4 {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceShaderExpectAssumeFeatures|
                            features.shader_expect_assume(enable)
                    );
                } else {
                    ctx.set_features(
                        |features: vk::PhysicalDeviceVulkan14Features|
                            features.shader_expect_assume(enable)
                    );
                }
            },
            shader_expect_assume::NAME,
            enable
        ) }
    }

    /// Specifies whether vertex attribute fetching may be repeated in instanced rendering.
    pub mod vertex_attribute_instance_rate_divisor {

        use super::*;

        /// The name of the feature.
        pub const NAME: ConstName = ConstName::new(stringify!(vertex_attribute_instance_rate_divisor));
    
        /// [`vertex_attribute_instance_rate_divisor`] properties
        #[derive(Clone, Copy)]
        pub struct Properties {
            /// The maximum value of the number of instances that **can** repeat the same vertex
            /// attribute data.
            pub max_vertex_attrib_divisor: u32,
        }
    }

    /// Specifies whether vertex attribute fetching may be repeated in instanced rendering.
    pub fn vertex_attribute_instance_rate_divisor(enable: bool) -> DeviceFeature {
        use tuhka::ext;
        unsafe {
            feature_ext(
                |ctx| {
                    if ctx.api_version() < API_VERSION_1_4 {
                        ctx.add_required_extension(ext::vertex_attribute_divisor::NAME)?;
                    }
                    let mut features = vk::PhysicalDeviceVertexAttributeDivisorFeatures
                        ::default();
                    ctx.get_physical_device_features(&mut features);
                    if features.vertex_attribute_instance_rate_divisor == 0 {
                        Err(Error::just_context("vertex_attribute_instance_rate_divisor not supported"))
                    } else {
                        Ok(())
                    }
                },
                move |ctx| {
                    if ctx.api_version() < API_VERSION_1_4 {
                        ctx.set_features(
                            |features: vk::PhysicalDeviceVertexAttributeDivisorFeatures|
                                features.vertex_attribute_instance_rate_divisor(enable)
                        );
                    } else {
                        ctx.set_features(
                            |features: vk::PhysicalDeviceVulkan14Features|
                                features.vertex_attribute_instance_rate_divisor(enable)
                        );
                    }
                },
                vertex_attribute_instance_rate_divisor::NAME,
                enable
            )
        }
    }

    /// Specifies whether a zero value **can** be used as the [`divisor`][1] in
    /// [`VertexInputBindingDivisor`].
    ///
    /// [1]: VertexInputBindingDivisor::divisor
    pub mod vertex_attribute_instance_rate_zero_divisor {

        use super::*;

        /// The name of the feature.
        pub const NAME: ConstName = ConstName::new(stringify!(vertex_attribute_instance_rate_zero_divisor));
    }

    /// Specifies whether a zero value **can** be used as the [`divisor`][1] in
    /// [`VertexInputBindingDivisor`].
    ///
    /// [1]: VertexInputBindingDivisor::divisor
    pub fn vertex_attribute_instance_rate_zero_divisor(enable: bool) -> DeviceFeature {
        use tuhka::ext;
        unsafe {
            feature_ext(
                |ctx| {
                    if ctx.api_version() < API_VERSION_1_4 {
                        ctx.add_required_extension(ext::vertex_attribute_divisor::NAME)?;
                    }
                    let mut features = vk::PhysicalDeviceVertexAttributeDivisorFeatures
                        ::default();
                    ctx.get_physical_device_features(&mut features);
                    if features.vertex_attribute_instance_rate_zero_divisor == 0 {
                        Err(Error::just_context("vertex_attribute_instance_rate_zero_divisor not supported"))
                    } else {
                        Ok(())
                    }
                },
                move |ctx| {
                    if ctx.api_version() < API_VERSION_1_4 {
                        ctx.set_features(
                            |features: vk::PhysicalDeviceVertexAttributeDivisorFeatures|
                                features.vertex_attribute_instance_rate_zero_divisor(enable)
                        );
                    } else {
                        ctx.set_features(
                            |features: vk::PhysicalDeviceVulkan14Features|
                                features.vertex_attribute_instance_rate_zero_divisor(enable)
                        );
                    }
                },
                vertex_attribute_instance_rate_zero_divisor::NAME,
                enable
            )
        }
    }
}

pub mod ext {

    use super::*;

    /// *Provided by VK_EXT_primitive_topology_list_restart*
    ///
    /// Indicates that list type primitives [`PointList`][1], [`LineList`][2], [`TriangleList`][3],
    /// [`LineListWithAdjacency`][4] and [`TriangleListWithAdjacency`][5] **can** use
    /// the primitive restart index value in index buffers.
    ///
    /// [1]: PrimitiveTopology::PointList
    /// [2]: PrimitiveTopology::LineList
    /// [3]: PrimitiveTopology::TriangleList
    /// [4]: PrimitiveTopology::LineListWithAdjacency
    /// [5]: PrimitiveTopology::TriangleListWithAdjacency
    pub mod primitive_topology_list_restart {

        use super::*;

        pub const NAME: ConstName = ConstName::new(stringify!(primitive_topology_list_restart));
    }

    /// *Provided by VK_EXT_primitive_topology_list_restart*
    ///
    /// Specifies that list type primitives [`PointList`][1], [`LineList`][2], [`TriangleList`][3],
    /// [`LineListWithAdjacency`][4] and [`TriangleListWithAdjacency`][5] **can** use
    /// the primitive restart index value in index buffers.
    ///
    /// [1]: PrimitiveTopology::PointList
    /// [2]: PrimitiveTopology::LineList
    /// [3]: PrimitiveTopology::TriangleList
    /// [4]: PrimitiveTopology::LineListWithAdjacency
    /// [5]: PrimitiveTopology::TriangleListWithAdjacency
    pub fn primitive_topology_list_restart(enable: bool) -> DeviceFeature {
        unsafe { feature_ext(
            |ctx| {
                let mut features = vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.primitive_topology_list_restart == 0 {
                    Err(Error::just_context("primitive_topology_list_restart not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                ctx.set_features(|features: vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT| {
                    features.primitive_topology_list_restart(enable)
                });
            },
            primitive_topology_list_restart::NAME,
            enable,
        ) }
    }

    /// *Provided by VK_EXT_primitive_topology_list_restart*
    ///
    /// Specifies that [`PatchList`][1] topology **can** use the primitive restart index value in
    /// index buffers.
    ///
    /// [1]: PrimitiveTopology::PatchList
    pub mod primitive_topology_patch_list_restart {

        use super::*;

        pub const NAME: ConstName = ConstName::new(stringify!(primitive_topology_patch_list_restart));
    }

    /// *Provided by VK_EXT_primitive_topology_list_restart*
    ///
    /// Specifies that [`PatchList`][1] topology **can** use the primitive restart index value in
    /// index buffers.
    ///
    /// [1]: PrimitiveTopology::PatchList
    pub fn primitive_topology_patch_list_restart(enable: bool) -> DeviceFeature {
        unsafe { feature_ext(
            |ctx| {
                let mut features = vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT
                    ::default();
                ctx.get_physical_device_features(&mut features);
                if features.primitive_topology_patch_list_restart == 0 {
                    Err(Error::just_context("primitive_topology_patch_list_restart not supported"))
                } else {
                    Ok(())
                }
            },
            move |ctx| {
                ctx.set_features(|features: vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT| {
                    features.primitive_topology_patch_list_restart(enable)
                });
            },
            primitive_topology_patch_list_restart::NAME,
            enable,
        ) }
    }
}

#[inline]
pub(crate) fn core_features() -> Vec32<DeviceFeature> {
    vec32![
        vulkan_12::timeline_semaphore(),
        vulkan_12::separate_depth_stencil_layouts(),
        vulkan_13::synchronization2(),
        vulkan_13::dynamic_rendering(),
        vulkan_13::maintenance4(),
    ]
}

pub mod prelude {

    pub use super::{
        vulkan_10, vulkan_11, vulkan_12, vulkan_13, vulkan_14,
        ext
    };
}
