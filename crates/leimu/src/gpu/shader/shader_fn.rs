use tuhka::vk;
use crate::gpu::Version;

#[inline]
pub fn glsl_to_spirv(
    input: &str,
    input_name: &str,
    shader_kind: shaderc::ShaderKind,
    vulkan_version: Version,
) -> shaderc::Result<shaderc::CompilationArtifact>
{
    let compiler = shaderc::Compiler::new()?;
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.set_target_env(
        shaderc::TargetEnv::Vulkan,
        vk::make_api_version(
            0,
            vulkan_version.major(),
            vulkan_version.minor(),
            0,
        ),
    );
    options.set_source_language(shaderc::SourceLanguage::GLSL);
    options.set_optimization_level(shaderc::OptimizationLevel::Performance);
    options.set_generate_debug_info();
    compiler.compile_into_spirv(
        input,
        shader_kind,
        input_name,
        "main",
        Some(&options)
    )
}
