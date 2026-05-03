//! Contains everything reflection relevant.

#![warn(missing_docs)]

mod error;

use core::ffi::CStr;

use crate::{
    core::*,
    op,
    module::*,
};

pub use error::*;

/// An Id to an instruction, which has a [`result`][1].
///
/// [1]: op::IdResult
pub type Id = op::IdRef;

/// Represents an constant value.
#[derive(Clone, Copy, Debug)]
pub enum Constant<'a> {
    /// A non-specialization constant boolean.
    Bool(bool),
    /// A non-specialization constant.
    Constant(Literal),
    /// A non-specialization constant composite.
    Composite {
        /// The type [`Id`] of the composite type.
        ty: Id,
        /// The constituents of composite value.
        constituents: &'a [Id],
    },
    /// A constant sampler.
    Sampler {
        /// The addressing mode of the sampler.
        addressing_mode: op::SamplerAddressingMode,
        /// Whether the sampler uses normalized coordinates.
        is_normalized: bool,
        /// The filter mode of the sampler.
        filter_mode: op::SamplerFilterMode,
    },
    /// A null value.
    Null {
        /// The type [`Id`] of the null value.
        ty: Id,
    },
    /// A specialization constant boolean.
    SpecBool(bool),
    /// A specialization constant.
    SpecConstant(Literal),
    /// A specialization constant composite.
    SpecConstantComposite {
        /// The type [`Id`] of the composite type.
        ty: Id,
        /// The constituents of the composite value.
        constituents: &'a [Id],
    },
    /// A specialization constant operation.
    SpecConstantOp {
        /// The resultant type [`Id`] of the operation.
        ty: Id,
        /// The [`code`][1] specifying the operation.
        ///
        /// [1]: op::Code
        opcode: op::Code,
        /// The operands of the operation.
        operands: &'a [Id]
    },
}

/// Represents a scalar type.
#[derive(Clone, Copy, Debug)]
pub enum ScalarType {
    /// A boolean type.
    Bool,
    /// An integer type.
    Int {
        /// The width of the integer in bits.
        width: u32,
        /// Whether the integer is signed.
        is_signed: bool,
    },
    /// A floating point type.
    Float {
        /// The width of the floating point type in bits.
        width: u32,
    },
}

/// Represents a specialization constant
#[derive(Clone, Copy, Debug)]
pub struct SpecConstant {
    /// The scalar type of the constant.
    pub ty: ScalarType,
    /// The constant id assigned to the constant.
    pub constant_id: u32,
    /// The [`Id`] of the constant.
    pub id: Id,
}

/// A type specifying, which resources to [`reflect`][1].
///
/// [1]: Reflector::resources_for_type
#[derive(Clone, Copy, Debug)]
pub enum ResourceType {
    /// A uniform buffer resource.
    UniformBuffer,
    /// A storage buffer resource.
    StorageBuffer,
    /// A push constant resource.
    PushConstant,
    /// An atomic counter resource.
    AtomicCounter,
    /// An input attachment resource.
    InputAttachment,
    /// A storage image resource.
    StorageImage,
    /// A sampled image containing both a sampler and an image.
    CombinedImageSampler,
    /// A sampled image without a sampler.
    SeparateImage,
    /// An sampler without an image.
    SeparateSampler,
    /// A uniform buffer interpreted as an image.
    UniformTexelBuffer,
    /// A storage buffer interpreted as an image.
    StorageTexelBuffer,
}

/// A hole created from the declaration of a runtime array.
///
/// Can be resolved by giving the number of elements in the array.
#[derive(Clone, Debug)]
pub struct RuntimeArrayHole {
    stride: usize,
}

impl RuntimeArrayHole {

    /// Gets the declared size of runtime array, which is just its stride.
    #[inline]
    pub fn declared(&self) -> usize {
        self.stride
    }

    /// Resolves the hole with `count`.
    #[inline]
    pub fn resolve(&self, count: usize) -> usize {
        self.stride * count
    }
}

/// A hole created from the declaration of an array without the [`array stride decoration`][1].
///
/// Can be resolved, by resolving the element type [`hint`][2].
///
/// [1]: op::Decoration::ArrayStride
/// [2]: TypeSizeHint
#[derive(Clone, Debug)]
pub struct UnknownArrayStrideHole {
    element: Box<TypeSizeHint>,
    count: usize,
}

impl UnknownArrayStrideHole {

    /// Gets the declared size of the hole.
    #[inline]
    pub fn declared(&self) -> usize {
        self.element.declared() * self.count
    }

    /// Resolves the hole by resolving the element type [`hint`][1].
    ///
    /// [1]: TypeSizeHint
    #[inline]
    pub fn resolve<F>(&self, f: F) -> usize
        where F: FnOnce(&TypeSizeHint) -> usize
    {
        f(&self.element) * self.count
    }
}

/// A hole created from the usage of a matrix type without the [`matrix stride decoration`][1].
///
/// Can be resolved by specifying the stride and whether the matrix is row-major.
///
/// [1]: op::Decoration::MatrixStride
#[derive(Clone, Debug)]
pub struct MatrixStrideHole {
    columns: u32,
    rows: u32,
    declared: usize
}

impl MatrixStrideHole {

    /// Gets the declared size of the hole.
    #[inline]
    pub fn declared(&self) -> usize {
        self.declared
    }

    /// Resolves the hole with a stride and whether the matrix is row-major.
    #[inline]
    pub fn resolve(&self, stride: usize, is_row_major: bool) -> usize {
        if is_row_major {
            stride * self.rows as usize
        } else {
            stride * self.columns as usize
        }
    }
}

/// A hole created from a struct, which has a runtime array as the last member.
///
/// Can be resolved by giving the number of elements in the runtime array.
#[derive(Clone, Debug)]
pub struct StructHole {
    last_offset: usize,
    hole: RuntimeArrayHole,
}

impl StructHole {

    /// Gets the declared size of the hole.
    #[inline]
    pub fn declared(&self) -> usize {
        self.last_offset + self.hole.declared()
    }

    /// Resolves the hole by giving the number of elements in the runtime array.
    #[inline]
    pub fn resolve(&self, count: usize) -> usize {
        self.last_offset + self.hole.resolve(count)
    }
}

/// A hint of the size of a [`Type`].
///
/// It can be a statically known size, or it can have a hole which can be resolved.
#[derive(Clone, Debug)]
pub enum TypeSizeHint {
    /// A statically known size.
    Static(usize),
    /// A runtime array hole.
    RuntimeArray(RuntimeArrayHole),
    /// A matrix stride hole.
    Matrix(MatrixStrideHole),
    /// An unknown array stride hole.
    UnknownArrayStride(UnknownArrayStrideHole),
    /// A hole caused by a runtime array at the end of a struct.
    Struct(StructHole),
}

impl TypeSizeHint {

    /// Gets the declared minimum size of the type.
    #[inline]
    pub fn declared(&self) -> usize {
        match self {
            &Self::Static(size) => size,
            Self::RuntimeArray(hole) => hole.declared(),
            Self::Matrix(hole) => hole.declared(),
            Self::UnknownArrayStride(hole) => hole.declared(),
            Self::Struct(hole) => hole.declared(),
        }
    } 
}

/// Represents resource count in [`Resource`].
#[derive(Clone, Copy, Debug)]
pub enum ResourceCount {
    /// The size is known statically.
    Static(usize),
    /// The resource is in a runtime array.
    Runtime {
        /// The declared minimum count of resources.
        declared: usize,
    },
}

impl ResourceCount {

    /// Gets the declared count.
    #[inline]
    pub fn declared(self) -> usize {
        match self {
            Self::Static(size) => size,
            Self::Runtime { declared } => declared,
        }
    }
}

/// Represents a shader resource.
#[derive(Clone, Copy)]
pub struct Resource<'a> {
    /// The outer type of the resource.
    pub type_id: Id,
    /// The inner type of the resource.
    pub base_type_id: Id,
    /// The variable id of the resource.
    pub variable_id: Id,
    /// The variable name string of the resource.
    pub name: Option<CompilerStr<'a>>,
    /// The number of [`base type`][1] values in an array.
    ///
    /// [1]: Self::base_type_id
    pub count: ResourceCount,
    /// Offset of the first member in a struct used for [`push constants`][1].
    ///
    /// [1]: ResourceType::PushConstant
    pub offset: Option<u32>,
}

/// Describes a type.
pub struct Type<'a> {
    /// The [`Id`] of the type.
    pub id: Id,
    /// The name of the type.
    pub name: Option<CompilerStr<'a>>,
    /// A [`hint`][1] of the size of the type.
    ///
    /// [1]: TypeSizeHint
    pub size_hint: TypeSizeHint,
}

#[derive(Clone, Copy)]
struct Name<'a> {
    target: Id,
    member: Option<u32>,
    name: CompilerStr<'a>,
}

/// A decoration added to an instruction, or a member of a struct.
#[derive(Clone, Copy)]
pub struct Decorate<'a> {
    /// The target [`Id`] of the decoration.
    pub target: Id,
    /// The member index within [`target`][1], if this decorates a member of a struct.
    ///
    /// [1]: Self::target
    pub member: Option<u32>,
    /// The [`Decoration`][1] assigned to the target.
    ///
    /// [1]: op::Decoration
    pub decoration: op::Decoration<'a>,
}

/// Represents a variable.
#[derive(Clone, Copy)]
pub struct Variable {
    /// The resultant type [`Id`] of the variable.
    pub result_type: Id,
    /// The [`Id`] of the variable.
    pub id_result: Id,
    /// The storage class of the variable.
    pub storage_class: op::StorageClass,
}

#[derive(Clone, Copy)]
struct EntryPoint<'a> {
    _execution_model: op::ExecutionModel,
    _name: CompilerStr<'a>,
    interface: &'a [Id],
}

/// Reflects SPIR-V code through a [`module`][1].
///
/// # Example
/// ``` rust
/// use leimu_spirv::op;
/// use leimu_spirv::Module;
/// use leimu_spirv::reflect::{Reflector, ResourceType};
/// 
/// let spirv: &[u32] = ...;
/// let module = Module::new(spirv).unwrap();
/// let mut reflector = Reflector::new(module).unwrap();
/// reflector.set_entry_point(c"main", op::ExecutionModel::FRAGMENT).unwrap();
/// for push_constant in reflector.resources_for_type(ResourceType::PushConstant).unwrap() {
///     let push_constant = push_constant.unwrap();
///     let size = reflector
///         .type_description(push_constant.base_type_id)
///         .unwrap().size_hint.declared();
///     println!("Push constant (size {size}): {}", push_constant.name.unwrap_or_default());
/// }
/// ```
///
/// [1]: Module
pub struct Reflector<'a> {
    module: Module<'a>,
    decorates: Vec<Decorate<'a>>,
    current_entry_point: Option<EntryPoint<'a>>,
}

impl<'a> Reflector<'a> {

    /// Creates a reflector from a [`module`][1].
    ///
    /// Automatically calls [`parse_full`][2].
    ///
    /// [1]: Module
    /// [2]: Module::parse_full
    pub fn new(
        mut module: Module<'a>,
    ) -> ReflectResult<Reflector<'a>>
    {
        module.parse_full()?;
        let mut decorates = vec![];
        for mut stream in module.all_instructions() {
            if matches!(
                stream.code(),
                op::Code::DECORATE | op::Code::DECORATE_ID | op::Code::DECORATE_STRING,
            ) {
                decorates.push(Decorate {
                    target: Id::parse_one(&mut stream)?,
                    member: None,
                    decoration: op::Decoration::parse_one(&mut stream)?,
                });
            } else if matches!(
                stream.code(),
                op::Code::MEMBER_DECORATE | op::Code::MEMBER_DECORATE_STRING,
            ) {
                decorates.push(Decorate {
                    target: Id::parse_one(&mut stream)?,
                    member: Some(stream.read()?),
                    decoration: op::Decoration::parse_one(&mut stream)?,
                });
            }
        }
        Ok(Self {
            decorates,
            module,
            current_entry_point: None,
        })
    }

    /// Returns an iterator over all variables.
    #[inline]
    pub fn variables(&self) -> impl Iterator<Item = ReflectResult<Variable>>
    {
        self.module
            .results()
            .map(|(_, mut stream)| {
                if stream.code() == op::Code::VARIABLE {
                    let result_type = Id::parse_one(&mut stream)?;
                    let id_result = Id::parse_one(&mut stream)?;
                    let storage_class = op::StorageClass::parse_one(&mut stream)?;
                    return Ok(Some(Variable {
                        result_type,
                        id_result,
                        storage_class,
                    }))
                }
                ReflectResult::Ok(None)
            }).filter_map(|variable| {
                match variable {
                    Ok(var) => var.map(Ok),
                    Err(err) => Some(Err(err)),
                }
            })
    }

    /// Sets the entry point to reflect.
    ///
    /// This *must* be set before [`reflecting any resources`][1].
    ///
    /// [1]: Self::resources_for_type
    #[inline]
    pub fn set_entry_point(
        &mut self,
        name: &CStr,
        execution_model: op::ExecutionModel,
    ) -> ReflectResult<()>
    {
        for mut stream in self.module.all_instructions() {
            if stream.code() == op::Code::ENTRY_POINT {
                let exec_model = op::ExecutionModel::parse_one(&mut stream)?;
                if exec_model == execution_model {
                    stream.advance(1)?;
                    let cname = stream.read_string()?;
                    if name == cname.to_cstr()? {
                        self.current_entry_point = Some(EntryPoint {
                            _execution_model: exec_model,
                            _name: cname,
                            interface: Id::parse_eos(&mut stream)?,
                        });
                        return Ok(())
                    }
                }
            }
        }
        Err(ReflectError::EntryPointNotFound(name.to_owned(), execution_model))
    }

    fn names(&self) -> impl Iterator<Item = ReflectResult<Name<'a>>> {
        self.module
            .all_instructions()
            .filter(|stream| matches!(stream.code(), op::Code::NAME | op::Code::MEMBER_NAME))
            .map(|mut stream| {
                if stream.code() == op::Code::NAME {
                    Ok(Name {
                        target: Id::parse_one(&mut stream)?,
                        member: None,
                        name: stream.read_string()?,
                    })
                } else if stream.code() == op::Code::MEMBER_NAME {
                    Ok(Name {
                        target: Id::parse_one(&mut stream)?,
                        member: Some(stream.read()?),
                        name: stream.read_string()?,
                    })
                } else { unreachable!() }
            })
        }

    /// Returns an iterator over the statically known values and constant ids of all
    /// specialization constants.
    #[inline]
    pub fn spec_constants(&self) -> impl Iterator<Item = ReflectResult<SpecConstant>> {
        self.module
            .results()
            .filter_map(|(id, stream)| 
                if matches!(
                    stream.code(),
                    op::Code::SPEC_CONSTANT_TRUE
                    | op::Code::SPEC_CONSTANT_FALSE
                    | op::Code::SPEC_CONSTANT
                ) {
                    self.decorates
                        .iter()
                        .find_map(|&decorate|
                            if decorate.target == id &&
                                let op::Decoration::SpecId { specialization_constant_id } =
                                decorate.decoration
                            {
                                Some(specialization_constant_id)
                            } else { None }
                        ).map(|constant_id| (constant_id, id, stream))
                } else { None }
            )
            .map(|(constant_id, id, mut stream)| {
                match stream.code() {
                    op::Code::SPEC_CONSTANT_TRUE | op::Code::SPEC_CONSTANT_FALSE => {
                        Ok(SpecConstant {
                            ty: ScalarType::Bool,
                            constant_id,
                            id,
                        })
                    },
                    op::Code::SPEC_CONSTANT => {
                        let result_type = op::IdResultType::parse_one(&mut stream)?;
                        stream.advance(1)?;
                        let ctx = &mut ParseContext {
                            result_type: Some(result_type)
                        };
                        let value = Literal::parse_one(&self.module, &mut stream, ctx)?;
                        let ty = match value {
                            Literal::F16(_) => ScalarType::Float { width: 2, },
                            Literal::F32(_) => ScalarType::Float { width: 4, },
                            Literal::F64(_) => ScalarType::Float { width: 8, },
                            Literal::I8(_) => ScalarType::Int { width: 1, is_signed: true, },
                            Literal::I16(_) => ScalarType::Int { width: 2, is_signed: true, },
                            Literal::I32(_) => ScalarType::Int { width: 4, is_signed: true, },
                            Literal::I64(_) => ScalarType::Int { width: 8, is_signed: true, },
                            Literal::U8(_) => ScalarType::Int { width: 1, is_signed: false, },
                            Literal::U16(_) => ScalarType::Int { width: 2, is_signed: false, },
                            Literal::U32(_) => ScalarType::Int { width: 4, is_signed: false, },
                            Literal::U64(_) => ScalarType::Int { width: 8, is_signed: false, },
                        };
                        Ok(SpecConstant {
                            ty,
                            constant_id,
                            id,
                        })
                    },
                    _ => unreachable!(),
                }
            })
    }

    /// Gets a specific constant with `id`.
    pub fn constant(
        &self,
        id: Id,
    ) -> ReflectResult<Constant<'a>>
    {
        let mut stream = self.module.get_result(id).ok_or(ReflectError::InvalidConstantId(id))?;
        match stream.code() {
            op::Code::CONSTANT_TRUE => {
                Ok(Constant::Bool(true))
            },
            op::Code::CONSTANT_FALSE => {
                Ok(Constant::Bool(false))
            },
            op::Code::CONSTANT => {
                let result_type = op::IdResultType::parse_one(&mut stream)?;
                stream.advance(1)?;
                let ctx = &mut ParseContext {
                    result_type: Some(result_type)
                };
                let value = Literal::parse_one(&self.module, &mut stream, ctx)?;
                Ok(Constant::Constant(value))
            },
            op::Code::CONSTANT_COMPOSITE => {
                let ty = Id::parse_one(&mut stream)?;
                stream.advance(1)?;
                Ok(Constant::Composite {
                    ty,
                    constituents: Id::parse_eos(&mut stream)?,
                })
            },
            op::Code::CONSTANT_SAMPLER => {
                stream.advance(2)?;
                let addressing_mode = op::SamplerAddressingMode::parse_one(&mut stream)?;
                let is_normalized = stream.read()? == 1;
                let filter_mode = op::SamplerFilterMode::parse_one(&mut stream)?;
                Ok(Constant::Sampler { addressing_mode, is_normalized, filter_mode })
            },
            op::Code::CONSTANT_NULL => {
                let ty = Id::parse_one(&mut stream)?;
                Ok(Constant::Null { ty })
            },
            op::Code::SPEC_CONSTANT_TRUE => {
                Ok(Constant::SpecBool(true))
            },
            op::Code::SPEC_CONSTANT_FALSE => {
                Ok(Constant::SpecBool(false))
            },
            op::Code::SPEC_CONSTANT => {
                let result_type = op::IdResultType::parse_one(&mut stream)?;
                stream.advance(1)?;
                let ctx = &mut ParseContext {
                    result_type: Some(result_type)
                };
                let value = Literal::parse_one(&self.module, &mut stream, ctx)?;
                Ok(Constant::SpecConstant(value))
            },
            op::Code::SPEC_CONSTANT_COMPOSITE => {
                let ty = Id::parse_one(&mut stream)?;
                stream.advance(1)?;
                Ok(Constant::SpecConstantComposite {
                    ty,
                    constituents: Id::parse_eos(&mut stream)?,
                })
            },
            op::Code::SPEC_CONSTANT_OP => {
                let ty = Id::parse_one(&mut stream)?;
                stream.advance(1)?;
                let code = op::LiteralSpecConstantOpInteger::parse_one(&mut stream)?;
                Ok(Constant::SpecConstantOp { ty, opcode: code.code, operands: code.operands })
            },
            _ => Err(ReflectError::InvalidConstantId(id))
        }
    }

    #[inline]
    /// Returns a name assigned to an [`Id`], or optionally to a struct member of a struct.
    pub fn name(&self, id: Id, struct_member: Option<u32>) -> ReflectResult<Option<CompilerStr<'a>>> {
        for name in self.names() {
            let name = name?;
            if name.target == id &&
                name.member == struct_member
            {
                return Ok(Some(name.name))
            }
        }
        Ok(None)
    }

    /// Returns an iterator over all resources of [`type`][1].
    ///
    /// Entry point *must* be [`set`][2] before calling this function.
    ///
    /// If the SPIR-V version is >= 1.4, the resources are filtered to only include ones used by
    /// the current entry point.
    ///
    /// [1]: ResourceType
    /// [2]: Self::set_entry_point
    pub fn resources_for_type(&self, ty: ResourceType) -> ReflectResult<
        impl Iterator<Item = ReflectResult<Resource<'a>>>
    >
    {
        let Some(entry_point) = self.current_entry_point else {
            return Err(ReflectError::EntryPointNotSet)
        };
        let resource_class = match ty {
            ResourceType::UniformBuffer => op::StorageClass::UNIFORM,
            ResourceType::StorageBuffer => op::StorageClass::STORAGE_BUFFER,
            ResourceType::PushConstant => op::StorageClass::PUSH_CONSTANT,
            ResourceType::AtomicCounter => op::StorageClass::ATOMIC_COUNTER,
            ResourceType::InputAttachment
            | ResourceType::StorageImage
            | ResourceType::CombinedImageSampler
            | ResourceType::SeparateImage
            | ResourceType::SeparateSampler
            | ResourceType::UniformTexelBuffer
            | ResourceType::StorageTexelBuffer => op::StorageClass::UNIFORM_CONSTANT,
        };
        Ok(self.variables().map(move |op_variable| {
            let op_variable = op_variable?;
            let name = self.names().find_map(|op_name| {
                if let Ok(name) = op_name &&
                    name.target == op_variable.id_result
                {
                    Some(name.name)
                } else { None }
            });
            if self.module.version() >= VERSION_1_4 &&
                !entry_point.interface.contains(&op_variable.id_result)
            {
                return Ok(None)
            }
            let mut base_type = op_variable.result_type;
            let mut count = ResourceCount::Static(1);
            loop {
                let mut stream = self.module
                    .get_result(base_type)
                    .ok_or(ReflectError::InvalidTypeId(base_type))?;
                base_type = match stream.code()
                {
                    op::Code::TYPE_POINTER => {
                        stream.advance(1)?;
                        let storage_class = op::StorageClass::parse_one(&mut stream)?;
                        if storage_class != resource_class {
                            return Ok(None)
                        }
                        Id::parse_one(&mut stream)?
                    },
                    op::Code::TYPE_ARRAY => {
                        stream.advance(1)?;
                        let id = Id::parse_one(&mut stream)?;
                        let length = match self.constant(Id::parse_one(&mut stream)?)?
                        {
                            Constant::Constant(literal) => {
                                literal.as_usize()
                                    .ok_or(ReflectError::NonIntegerLiteral(literal))?
                            },
                            Constant::SpecConstant(literal) => {
                                literal.as_usize()
                                    .ok_or(ReflectError::NonIntegerLiteral(literal))?
                            },
                            x => return Err(ReflectError::ExpectedConstantLiteral {
                                found: format!("{x:?}")
                            })
                        };
                        count = match count {
                            ResourceCount::Static(count) =>
                                ResourceCount::Static(count * length),
                            ResourceCount::Runtime { declared } =>
                                ResourceCount::Runtime { declared: declared * length },
                        };
                        id
                    },
                    op::Code::TYPE_RUNTIME_ARRAY => {
                        stream.advance(1)?;
                        count = match count {
                            ResourceCount::Static(count) =>
                                ResourceCount::Runtime { declared: count, },
                            ResourceCount::Runtime { declared } =>
                                ResourceCount::Runtime { declared },

                        };
                        Id::parse_one(&mut stream)?
                    },
                    _ => {
                        break
                    },
                };
            } 
            if resource_class == op::StorageClass::UNIFORM_CONSTANT {
                match ty {
                    ResourceType::InputAttachment => {
                        let mut stream = self.module
                            .get_result(base_type)
                            .ok_or(ReflectError::InvalidTypeId(base_type))?;
                        if matches!(stream.code(), op::Code::TYPE_IMAGE) {
                            stream.advance(2)?;
                            let dim = op::Dim::parse_one(&mut stream)?;
                            if dim == op::Dim::SUBPASS_DATA {
                                return Ok(Some(Resource {
                                    type_id: op_variable.result_type,
                                    base_type_id: base_type,
                                    variable_id: op_variable.id_result,
                                    name,
                                    count,
                                    offset: None,
                                }))
                            }
                        }
                        Ok(None)
                    },
                    ResourceType::StorageImage => {
                        let mut stream = self.module
                            .get_result(base_type)
                            .ok_or(ReflectError::InvalidTypeId(base_type))?;
                        if matches!(stream.code(), op::Code::TYPE_IMAGE) {
                            stream.advance(2)?;
                            let dim = op::Dim::parse_one(&mut stream)?;
                            stream.advance(3)?;
                            let sampled = stream.read()?;
                            if sampled == 2 &&
                                matches!(
                                    dim,
                                    op::Dim::TYPE_1D
                                    | op::Dim::TYPE_2D
                                    | op::Dim::TYPE_3D
                                    | op::Dim::CUBE
                                    | op::Dim::RECT
                                )
                            {
                                return Ok(Some(Resource {
                                    type_id: op_variable.result_type,
                                    base_type_id: base_type,
                                    variable_id: op_variable.id_result,
                                    name,
                                    count,
                                    offset: None,
                                }))
                            }
                        }
                        Ok(None)
                    },
                    ResourceType::CombinedImageSampler => {
                        let stream = self.module
                            .get_result(base_type)
                            .ok_or(ReflectError::InvalidTypeId(base_type))?;
                        if matches!(stream.code(), op::Code::TYPE_SAMPLED_IMAGE) {
                            Ok(Some(Resource {
                                type_id: op_variable.result_type,
                                base_type_id: base_type,
                                variable_id: op_variable.id_result,
                                name,
                                count,
                                offset: None,
                            }))
                        } else { Ok(None) }
                    },
                    ResourceType::SeparateImage => {
                        let mut stream = self.module
                            .get_result(base_type)
                            .ok_or(ReflectError::InvalidTypeId(base_type))?;
                        if matches!(
                            stream.code(),
                            op::Code::TYPE_IMAGE,
                        ) {
                            stream.advance(2)?;
                            let dim = op::Dim::parse_one(&mut stream)?;
                            stream.advance(3)?;
                            let sampled = stream.read()?;
                            if sampled != 2 &&
                                matches!(
                                    dim,
                                    op::Dim::TYPE_1D
                                    | op::Dim::TYPE_2D
                                    | op::Dim::TYPE_3D
                                    | op::Dim::CUBE
                                    | op::Dim::RECT
                                )
                            {
                                return Ok(Some(Resource {
                                    type_id: op_variable.result_type,
                                    base_type_id: base_type,
                                    variable_id: op_variable.id_result,
                                    name,
                                    count,
                                    offset: None,
                                }))
                            }
                        } 
                        Ok(None)
                    },
                    ResourceType::SeparateSampler => {
                        let stream = self.module
                            .get_result(base_type)
                            .ok_or(ReflectError::InvalidTypeId(base_type))?;
                        if matches!(stream.code(), op::Code::TYPE_SAMPLER) {
                            Ok(Some(Resource {
                                type_id: op_variable.result_type,
                                base_type_id: base_type,
                                variable_id: op_variable.id_result,
                                name,
                                count,
                                offset: None,
                            }))
                        } else { Ok(None) }
                    },
                    ResourceType::UniformTexelBuffer => {
                        let mut stream = self.module
                            .get_result(base_type)
                            .ok_or(ReflectError::InvalidTypeId(base_type))?;
                        if matches!(stream.code(), op::Code::TYPE_IMAGE) {
                            stream.advance(2)?;
                            let dim = op::Dim::parse_one(&mut stream)?;
                            stream.advance(3)?;
                            let sampled = stream.read()?;
                            if sampled != 2 && dim == op::Dim::BUFFER {
                                return  Ok(Some(Resource {
                                    type_id: op_variable.result_type,
                                    base_type_id: base_type,
                                    variable_id: op_variable.id_result, 
                                    name,
                                    count,
                                    offset: None,
                                }))
                            }
                        }
                        Ok(None)
                    },
                    ResourceType::StorageTexelBuffer => {
                        let mut stream = self.module
                            .get_result(base_type)
                            .ok_or(ReflectError::InvalidTypeId(base_type))?;
                        if matches!(stream.code(), op::Code::TYPE_IMAGE) {
                            stream.advance(2)?;
                            let dim = op::Dim::parse_one(&mut stream)?;
                            stream.advance(3)?;
                            let sampled = stream.read()?;
                            if sampled == 2 && dim == op::Dim::BUFFER {
                                return  Ok(Some(Resource {
                                    type_id: op_variable.result_type,
                                    base_type_id: base_type,
                                    variable_id: op_variable.id_result, 
                                    name,
                                    count,
                                    offset: None,
                                }))
                            }
                        }
                        Ok(None)
                    },
                    _ => unreachable!()
                }
            } else {
                let offset = match ty {
                    ResourceType::PushConstant => {
                        self.decorations(base_type)
                            .filter_map(|dec| {
                                if let op::Decoration::Offset { byte_offset } = dec.decoration {
                                    Some(byte_offset)
                                } else { None }
                            }).min()
                    },
                    _ => None
                };
                Ok((op_variable.storage_class == resource_class).then_some(
                    Resource {
                        type_id: op_variable.result_type,
                        base_type_id: base_type,
                        variable_id: op_variable.id_result,
                        name,
                        count,
                        offset,
                    }
                ))
            }
        }).filter_map(|resource| {
            match resource {
                Ok(res) => res.map(Ok),
                Err(err) => Some(Err(err)),
            }
        }))
    } 

    /// Returns all [`decorations`][1] added to a given [`Id`].
    ///
    /// [1]: op::Decoration
    #[inline]
    pub fn decorations(
        &self,
        target_id: Id,
    ) -> impl Iterator<Item = Decorate<'a>>
    {
        self.decorates
            .iter()
            .filter_map(move |&dec|
                (dec.target == target_id).then_some(
                    dec
                )
            )
    }

    /// Gets a [`type description`][1] of a given [`Id`] pointing to a type.
    ///
    /// The description notably contains a [`hint`][2] of its size, which can be resolved to a
    /// known size when needed.
    ///
    /// [1]: Type
    /// [2]: TypeSizeHint
    pub fn type_description(&self, id: Id) -> ReflectResult<Type<'a>> {
        let mut stream = self.module
            .get_result(id)
            .ok_or(ReflectError::InvalidTypeId(id))?;
        let name = self.names().find_map(|name| {
            if let Ok(name) = name &&
                name.target == id
            {
                Some(name.name)
            } else { None }
        });
        match stream.code() {
            op::Code::TYPE_BOOL => Ok(Type {
                id,
                name,
                size_hint: TypeSizeHint::Static(4),
            }),
            op::Code::TYPE_INT | op::Code::TYPE_FLOAT => {
                stream.advance(1)?;
                let width = stream.read()?;
                Ok(Type {
                    id,
                    name,
                    size_hint: TypeSizeHint::Static(width as usize / 8),
                })
            },
            op::Code::TYPE_VECTOR => {
                stream.advance(1)?;
                let component_type = Id::parse_one(&mut stream)?;
                let component_count = stream.read()?;
                let mut stream = self.module
                    .get_result(component_type)
                    .ok_or(ReflectError::InvalidTypeId(id))?;
                let size = match stream.code() {
                    op::Code::TYPE_BOOL => 4 * component_count as usize,
                    op::Code::TYPE_INT | op::Code::TYPE_FLOAT => {
                        stream.advance(1)?;
                        let width = stream.read()?;
                        (width / 8 * component_count) as usize
                    },
                    x => return Err(ReflectError::ExpectedScalarType { found: x })
                };
                Ok(Type {
                    id,
                    name,
                    size_hint: TypeSizeHint::Static(size),
                })
            }
            op::Code::TYPE_MATRIX => {
                stream.advance(1)?;
                let column_type = Id::parse_one(&mut stream)?;
                let column_count = stream.read()?;
                let mut stream = self.module
                    .get_result(column_type)
                    .ok_or(ReflectError::InvalidTypeId(id))?;
                if !matches!(stream.code(), op::Code::TYPE_VECTOR) {
                    return Err(ReflectError::ExpectedVectorType { found: stream.code() })
                }
                stream.advance(1)?;
                let component_type = Id::parse_one(&mut stream)?;
                let component_count = stream.read()?;
                let mut stream = self.module
                    .get_result(component_type)
                    .ok_or(ReflectError::InvalidTypeId(id))?;
                let column_size = match stream.code() {
                    op::Code::TYPE_BOOL => 4,
                    op::Code::TYPE_INT | op::Code::TYPE_FLOAT => {
                        stream.advance(1)?;
                        stream.read()? as usize / 8 
                    },
                    x => return Err(ReflectError::ExpectedScalarType {
                        found: x,
                    })
                };
                Ok(Type {
                    id,
                    name,
                    size_hint: TypeSizeHint::Matrix(MatrixStrideHole {
                        columns: column_count,
                        rows: component_count,
                        declared: column_size * (column_count as usize),
                    }),
                })
            },
            op::Code::TYPE_ARRAY => {
                stream.advance(1)?;
                let element_type = Id::parse_one(&mut stream)?;
                let length = Id::parse_one(&mut stream)?;
                let count = {
                    let constant = self.constant(length)?;
                    match constant {
                        Constant::Constant(literal) =>
                            literal.as_usize().ok_or(ReflectError::NonIntegerLiteral(literal))?,
                        Constant::SpecConstant(literal) =>
                            literal.as_usize().ok_or(ReflectError::NonIntegerLiteral(literal))?,
                        x => return Err(ReflectError::ExpectedConstantLiteral {
                            found: format!("{x:?}"),
                        })
                    }
                };
                let size_hint = {
                    if let Some(stride) =
                        self.decorations(id)
                        .find_map(|dec| {
                            let op::Decoration::ArrayStride { array_stride } = dec.decoration else {
                                return None
                            };
                            Some(array_stride as usize)
                        })
                    {
                        TypeSizeHint::Static(count * stride)
                    } else {
                        let element_size_hint = self.type_description(element_type)?.size_hint;
                        match element_size_hint {
                            TypeSizeHint::Static(size) => TypeSizeHint::Static(count * size),
                            TypeSizeHint::Matrix(_) | TypeSizeHint::UnknownArrayStride(_)
                                => TypeSizeHint::UnknownArrayStride(
                                UnknownArrayStrideHole {
                                    element: Box::new(element_size_hint),
                                    count,
                                }),
                            TypeSizeHint::RuntimeArray(_) | TypeSizeHint::Struct(_)
                                => return Err(ReflectError::InvalidRuntimeArray),
                        }
                    }
                };
                Ok(Type {
                    id,
                    name,
                    size_hint
                })
            },
            op::Code::TYPE_RUNTIME_ARRAY => {
                let stride = self
                    .decorations(id)
                    .find_map(|dec| {
                        let op::Decoration::ArrayStride { array_stride } = dec.decoration else {
                            return None
                        };
                        Some(array_stride as usize)
                    }).ok_or(ReflectError::MissingRequiredDecoration("ArrayStride"))?;
                Ok(Type {
                    id,
                    name,
                    size_hint: TypeSizeHint::RuntimeArray(RuntimeArrayHole { stride }),
                })
            }
            op::Code::TYPE_STRUCT => {
                stream.advance(1)?;
                let member_types = Id::parse_eos(&mut stream)?;
                let mut min = u32::MAX;
                let (desc, member, offset) = self.decorations(id)
                    .filter_map(|dec| {
                        if let Some(member) = dec.member &&
                            let op::Decoration::Offset { byte_offset } = dec.decoration &&
                            let Some(&ty) = member_types.get(member as usize) &&
                            let Ok(desc) = self.type_description(ty)
                        {
                            Some((desc, member, byte_offset))
                        } else { None }
                    }).max_by_key(|(_, _, offset)| {
                        min = min.min(*offset);
                        *offset
                    }).ok_or(ReflectError::MissingRequiredDecoration("Offset"))?;
                let offset = (offset - min) as usize;
                let size_hint = match desc.size_hint {
                    TypeSizeHint::Static(size) => TypeSizeHint::Static(offset + size),
                    TypeSizeHint::RuntimeArray(hole) => TypeSizeHint::Struct(StructHole {
                        last_offset: offset,
                        hole,
                    }),
                    TypeSizeHint::Matrix(hole) => {
                        let mut stride = None;
                        let mut is_row_major = false;
                        for dec in self.decorations(id) {
                            if let Some(dec_member) = dec.member &&
                                dec_member == member
                            {
                                if let op::Decoration::MatrixStride { matrix_stride } = dec.decoration {
                                    stride = Some(matrix_stride)
                                } else if let op::Decoration::RowMajor = dec.decoration {
                                    is_row_major = true
                                }
                            }
                        }
                        TypeSizeHint::Static(offset + hole
                            .resolve(
                                stride.ok_or(
                                    ReflectError::MissingRequiredDecoration("MatrixStride")
                                )? as usize,
                                is_row_major
                            ))
                    },
                    TypeSizeHint::UnknownArrayStride(_) => return Err(
                        ReflectError::MissingRequiredDecoration("ArrayStride")
                    ),
                    TypeSizeHint::Struct(_) => return Err(
                        ReflectError::InvalidRuntimeArray
                    ),
                };
                Ok(Type {
                    id,
                    name,
                    size_hint,
                })
            },
            op::Code::TYPE_POINTER => {
                stream.advance(2)?;
                self.type_description(Id::parse_one(&mut stream)?)
            },
            _ => Ok(Type {
                id,
                name,
                size_hint: TypeSizeHint::Static(0),
            })
        }
    }
}
