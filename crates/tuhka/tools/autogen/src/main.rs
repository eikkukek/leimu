use std::fs::File;
use std::io::{BufReader, Write};
use std::collections::{HashSet, HashMap};
use std::process;
use core::{
    hash::{self, Hash},
    str::FromStr,
};
use indexmap::{IndexMap, IndexSet};

use quote::quote;
use proc_macro2::{Span, TokenStream};

use syn::{Ident, punctuated::Punctuated, Token};
use xml::reader::{EventReader, XmlEvent};

#[derive(Default)]
struct Bitmask {
    requires: Option<String>,
    parsing_type: bool,
    ty: String,
    parsing_name: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum FlagType {
    U32,
    U64,
}

fn doc_link(name: &str) -> Option<String> {
    Some(format!("<https://docs.vulkan.org/refpages/latest/refpages/source/{name}.html>"))
}

fn upper_snake_case(string: &str) -> String
{
    let mut res = string.to_string();
    for (idx, ch) in string.char_indices().rev() {
        if idx == 0 { break }
        if ch.is_uppercase() {
            res.insert(idx, '_');
        }
    }
    res.to_uppercase()
}

fn lower_snake_case(string: &str) -> String {
    let mut res = string.to_string();
    let mut iter = string.char_indices().rev();
    let mut next_ch = iter.next().unwrap().1;
    for (idx, ch) in iter {
        if !ch.is_uppercase() &&
            ch != '_' &&
            next_ch.is_uppercase()
        {
            let idx = idx + 1;
            res.remove(idx);
            res.insert(idx, next_ch.to_lowercase().next().unwrap());
            res.insert(idx, '_');
        }
        next_ch = ch;
    }
    res.to_lowercase()
}

fn parse_punctuated(string: &str) -> Vec<String> {
    string
        .split(",")
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.to_string()
        })
        .collect()
}

struct Bit {
    name: String,
    bit: u64,
    doc: Option<String>,
    alias: Option<String>,
    deprecated: Option<String>,
}

impl Hash for Bit {

    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Bit {

    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Bit {}

struct BitmaskBits {
    bits: IndexSet<Bit>,
    flag_type: FlagType,
}

struct Variant {
    name: String,
    value: syn::LitInt,
    _doc: Option<String>,
    alias: Option<String>,
    deprecated: Option<String>
}

impl PartialEq for Variant {

    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Variant {}

impl Hash for Variant {

    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

struct EnumVariants {
    variants: IndexSet<Variant>,
}

impl EnumVariants {

    fn add(
        &mut self,
        name: String,
        value: syn::LitInt,
        _doc: Option<String>
    ) {
        self.variants.insert(Variant {
            name, value, _doc, alias: None,
            deprecated: None,
        });
    }

    fn add_ext(
        &mut self,
        name: String,
        offset: u32,
        extnumber: u32,
        dir: Option<&String>,
        _doc: Option<String>,
    ) {
        let mut value = format!("1{:06}{:03}", extnumber - 1, offset);
        if let Some(dir) = dir {
            value.insert_str(0, dir);
        }
        self.variants.insert(Variant {
            name,
            value: syn::LitInt::new(
                &value,
                Span::call_site()
            ),
            _doc,
            alias: None,
            deprecated: None,
        });
    }
    
    fn add_alias(
        &mut self,
        name: String,
        alias: String,
        deprecated: Option<String>,
    ) {
        self.variants.insert(Variant {
            name,
            value: syn::LitInt::new("0", Span::call_site()),
            _doc: None,
            alias: Some(alias),
            deprecated,
        });
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum StructParsing {
    None,
    #[default]
    Const,
    Type,
    Pointer,
    ArrayLen,
    Name,
    Comment,
}

#[derive(Default, Clone)]
struct StructMember {
    value: Option<String>,
    parsing: StructParsing,
    ty: Option<Ident>,
    is_const_ptr: bool,
    ptr_type: PtrType,
    array_len: Option<String>,
    name: String,
    comment: Option<String>,
    deprecated: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
enum packed_u9_u9_u6_u4_u4 {
    A(String),
    B(String),
    C(String),
    D(String),
    E(String),
}

#[derive(Default, Clone)]
struct Struct {
    name: Option<Ident>,
    extends_unparsed: Option<String>,
    extends: Vec<Ident>,
    members: Vec<StructMember>,
    packed_u24_u8: Option<String>,
    packed_u9_u9_u6_u4_u4: Option<packed_u9_u9_u6_u4_u4>,
    parsing_member: Option<StructMember>,
    needs_lifetime: bool,
    is_union: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum HandleType {
    NonDispatchable,
    Dispatchable,
}

#[derive(Clone)]
struct Handle {
    type_enum: String,
    parsing_ty: bool,
    ty: HandleType,
    parsing_name: bool,
}

#[derive(Default, Clone)]
enum FuncParsing {
    #[default]
    Const,
    Ty,
    Pointer,
    Name,
    Array,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum PtrType {
    #[default]
    None,
    ConstPtr,
    MutPtr,
    ConstPtrConstPtr,
    MutPtrMutPtr,
}

impl quote::ToTokens for PtrType {

    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            Self::None => {},
            Self::ConstPtr => quote! { *const }.to_tokens(tokens),
            Self::MutPtr => quote! { *mut }.to_tokens(tokens),
            Self::ConstPtrConstPtr => quote! { *const *const }.to_tokens(tokens),
            Self::MutPtrMutPtr => quote! { *mut *mut }.to_tokens(tokens),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FuncParamPtrType {
    PtrType(PtrType),
    Array(usize),
}

impl Default for FuncParamPtrType {

    fn default() -> Self {
        Self::PtrType(Default::default())
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum ParamOptional {
    #[default]
    False,
    True,
    FalseTrue,
}

#[derive(Default, Clone)]
enum FuncpointerParsing {
    #[default]
    None,
    ImplicitParams,
    Proto(FuncParsing),
    Param {
        optional: ParamOptional,
        has_len: Option<Option<u32>>,
        parsing: FuncParsing,
        is_const: bool,
        ty: String,
        ptr: FuncParamPtrType,
        name: String,
    },
}

#[derive(Default, Clone)]
struct Funcparam {
    optional: ParamOptional,
    has_len: Option<Option<u32>>,
    ty: String,
    ptr: FuncParamPtrType,
    name: String,
}

struct Func {
    ret_ty: String,
    ret_ptr: PtrType,
    params: Vec<Funcparam>,
}

#[derive(Default, Clone)]
struct Funcpointer {
    success_codes: Vec<String>,
    parsing: FuncpointerParsing,
    ret_is_const: bool,
    ret_ty: String,
    ret_ptr: PtrType,
    params: Vec<Funcparam>,
}

enum Type {
    Handle(Handle),
    Bitmask(Bitmask),
    Struct(Struct),
    Funcpointer(Funcpointer),
}

enum Enums {
    Constants,
    Enum(String),
}

enum CommandBody {
    Alias(String),
    Func(Func)
}

struct UnresolvedCmd {
    original_name: Ident,
    pfn_name: Ident,
    pfn_def: TokenStream,
    pfn_default: TokenStream,
    rs_def: TokenStream,
    rs_def_ext: Option<TokenStream>,
    rs_def_get_data: Option<TokenStream>,
    num_arguments: u32,
}
struct ResolvedCmd {
    original_name: Ident,
    pfn_def: TokenStream,
    pfn_default: TokenStream,
    rs_def: TokenStream,
    rs_def_ext: Option<TokenStream>,
    rs_def_get_data: Option<TokenStream>,
}
impl UnresolvedCmd {

    fn resolve(&self,  mut name_rs: Ident) -> ResolvedCmd {
        let Self {
            original_name,
            pfn_def,
            pfn_name: _,
            pfn_default,
            rs_def,
            rs_def_ext,
            rs_def_get_data,
            num_arguments,
        } = self;
        let allow_too_many_arguments = (*num_arguments > 7).then(|| {
            quote! {
                #[allow(clippy::too_many_arguments)]
            }
        });
        let rs_def_get_data =
            if let Some(rs_get_data)  = rs_def_get_data {
                let get_data = quote! {
                    #allow_too_many_arguments
                    pub unsafe fn #name_rs #rs_get_data
                };
                name_rs = Ident::new(&format!("{name_rs}_len"), Span::call_site());
                Some(get_data)
            } else { None };
        ResolvedCmd {
            original_name: original_name.clone(),
            pfn_def: pfn_def.clone(),
            pfn_default: pfn_default.clone(),
            rs_def: quote! {
                #allow_too_many_arguments
                pub unsafe fn #name_rs #rs_def
            },
            rs_def_ext: rs_def_ext.as_ref().map(|def| quote! {
                #allow_too_many_arguments
                pub unsafe fn #name_rs #def
            }),
            rs_def_get_data,
        }
    }
}

struct Command {
    body: CommandBody,
    success_codes: Vec<String>,
    version: Option<String>,
    unresolved: Option<UnresolvedCmd>,
    deprecated: Option<Option<String>>,
    name_rs: Ident,
}
#[derive(Clone)]
struct FeatureCmdCounterPart {
    ext_cmd: String,
    ext_name: String,
    depends_on: Vec<String>,
}

#[derive(Clone)]
struct Feature {
    commands: IndexMap<String, Option<FeatureCmdCounterPart>>,
}

#[derive(Clone)]
enum ExtConstant {
    U32(u32),
    String(String),
}

#[derive(Clone)]
struct ExtCommand {
    name: String,
    depends_on: Vec<String>,
}

#[derive(Clone)]
struct Extension {
    ty: String,
    new_constants: Vec<(String, ExtConstant)>,
    commands: Vec<ExtCommand>,
    number: u32,
    promoted_to: Option<String>,
}

struct ExtParsingRequires {
    depth: u32,
    is_sc: bool,
    depends_on: Vec<String>,
}

struct ExtParsing {
    name: String,
    ext: Extension,
    is_disabled: bool,
    parsing_requires: Option<ExtParsingRequires>,
}

enum ReaderElement {
    Inactive,
    Type(Type),
    Enums(Enums),
    Feature {
        version: String,
        feature: Feature,
        parsing_requires: Option<u32>,
        parsing_deprecated: Option<u32>,
        parsing_remove: bool,
        is_sc: bool,
    },
    Extensions {
        parsing_extension: Option<ExtParsing>,
    },
}

struct Reader {
    depth: u32,
    element: ReaderElement,
    element_name: Option<String>,
}

#[derive(Default, Clone, Copy)]
enum DepthType {
    #[default]
    Unknown,
    Types,
    Enums,
    Commands,
    Feature,
    Extensions,
}

#[derive(Default, Clone, Copy)]
struct Depth {
    depth: u32,
    vulkan_sc: Option<u32>,
    ty: DepthType,
}

fn main() -> std::io::Result<()> {
    let xml = BufReader::new(File::open("vk.xml")?);
    let parser = EventReader::new(xml);
    let mut reader = Reader {
        depth: 0,
        element: ReaderElement::Inactive,
        element_name: None,
    };
    let mut depth = Depth::default();
    let mut unique_categories = HashSet::new();
    let mut handles = File::create("../../src/vk/handles.rs")?;
    write!(handles, "use super::*; use crate::define_non_dispatchable_handle; use crate::define_handle;")?;
    let mut enums = File::create("../../src/vk/enums.rs")?;
    write!(enums, "use super::*; use crate::bitflags;")?;
    let mut structs = File::create("../../src/vk/structs.rs")?;
    write!(structs, "use super::*; use vk_video::*;")?;
    let mut unions = File::create("../../src/vk/unions.rs")?;
    write!(unions, "use super::*;")?;
    let mut type_defs = File::create("../../src/vk/type_defs.rs")?;
    write!(type_defs, "use super::*;")?;
    let mut type_aliases = IndexMap::<String, String>::new();
    let mut bitmask_bits = IndexMap::<String, BitmaskBits>::new();
    let mut enum_variants = IndexMap::<String, EnumVariants>::new();
    let mut struct_extends = IndexSet::<Ident>::new();
    let mut struct_defs = vec![];
    let mut funcpointers = vec![];
    let mut commands = IndexMap::<String, Command>::new();
    let mut features = IndexMap::<String, Feature>::new();
    let mut extensions = IndexMap::<String, Extension>::new();
    let mut type_exclude_list = HashSet::new();
    let mut dispatchable_handles = HashSet::new();
    let mut non_eq_types = HashMap::new();
    non_eq_types.insert(Ident::new("f32", Span::call_site()), true);
    non_eq_types.insert(Ident::new("f64", Span::call_site()), true);
    let opaque_handles: HashSet<_> = [
        Ident::new("Display", Span::call_site()),
        Ident::new("wl_display", Span::call_site()),
        Ident::new("wl_surface", Span::call_site()),
        Ident::new("ubm_device", Span::call_site()),
        Ident::new("ubm_surface", Span::call_site()),
        Ident::new("HANDLE", Span::call_site()),
        Ident::new("SECURITY_ATTRIBUTES", Span::call_site()),
        Ident::new("xcb_connection_t", Span::call_site()),
        Ident::new("xcb_visualid_t", Span::call_site()),
        Ident::new("xcb_window_t", Span::call_site()),
        Ident::new("IDirectFB", Span::call_site()),
        Ident::new("IDirectFBSurface", Span::call_site()),
        Ident::new("_screen_context", Span::call_site()),
        Ident::new("_screen_window", Span::call_site()),
        Ident::new("_screen_buffer", Span::call_site()),
        Ident::new("__IOSurface", Span::call_site()),
        Ident::new("ANativeWindow", Span::call_site()),
        Ident::new("AHardwareBuffer", Span::call_site()),
        Ident::new("CAMetalLayer", Span::call_site()),
        Ident::new("OHNativeWindow", Span::call_site()),
        Ident::new("OHBufferHandle", Span::call_site()),
        Ident::new("OH_NativeBuffer", Span::call_site()),
    ].into_iter().collect();
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let find_attribute = |name: &str| -> Option<&xml::attribute::OwnedAttribute> {
                    attributes
                        .iter()
                        .find(|attr|
                            attr.name.local_name == name
                        )
                };
                let current_depth = reader.depth;
                reader.depth += 1;
                if let Some(api) = find_attribute("api") &&
                    api.value == "vulkansc"
                {
                    depth.vulkan_sc = Some(current_depth);
                }
                if depth.vulkan_sc.is_some() &&
                    (!matches!(depth.ty, DepthType::Feature | DepthType::Extensions)) &&
                    name.local_name != "feature"
                {
                    continue
                }
                match depth.ty {
                    DepthType::Unknown => {
                        if name.local_name == "types" {
                            depth.ty = DepthType::Types;
                            depth.depth = current_depth;
                        } else if name.local_name == "enums" {
                            depth.ty = DepthType::Enums;
                            depth.depth = current_depth;
                            let ty = find_attribute("type").unwrap();
                            if ty.value == "constants" {
                                reader.element = ReaderElement::Enums(Enums::Constants);
                            } else if ty.value == "enum" || ty.value == "bitmask" {
                                let name = find_attribute("name").unwrap();
                                reader.element = ReaderElement::Enums(Enums::Enum(name.value.clone()));
                            }
                        } else if name.local_name == "commands" {
                            depth.ty = DepthType::Commands;
                            depth.depth = current_depth;
                        } else if name.local_name == "feature" {
                            depth.ty = DepthType::Feature;
                            depth.depth = current_depth;
                            let version = find_attribute("number")
                                .unwrap().value.clone();
                            reader.element = ReaderElement::Feature {
                                version,
                                feature: Feature { commands: IndexMap::new() },
                                parsing_requires: None,
                                parsing_deprecated: None,
                                parsing_remove: false,
                                is_sc: depth.vulkan_sc.is_some(),
                            }
                        } else if name.local_name == "extensions" {
                            depth.ty = DepthType::Extensions;
                            depth.depth = current_depth;
                            reader.element = ReaderElement::Extensions {
                                parsing_extension: None,
                            };
                        }
                    },
                    DepthType::Types => {
                        if current_depth == depth.depth + 1 &&
                            name.local_name == "type"
                        {
                            let Some(category) = find_attribute("category")
                            else {
                                continue
                            };
                            unique_categories.insert(category.value.clone());
                            let name = find_attribute("name");
                            if let Some(alias) = find_attribute("alias")
                            {
                                type_aliases.insert(
                                    name
                                        .map(|name|
                                            name.value.clone()
                                    ).unwrap(),
                                    alias.value.clone()
                                );
                                continue
                            }
                            if category.value == "bitmask" {
                                let bitmask = Bitmask {
                                    requires : find_attribute("requires")
                                        .or_else(|| find_attribute("bitvalues"))
                                        .map(|attr| attr.value.clone()),
                                    ..Default::default()
                                };
                                reader.element = ReaderElement::Type(Type::Bitmask(bitmask));
                                reader.element_name = name.map(|name| name.value.clone());
                            } else if category.value == "enum" {
                                enum_variants.insert(name.as_ref().unwrap().value.clone(), EnumVariants {
                                    variants: IndexSet::new(),
                                });
                            } else if category.value == "struct" || category.value == "union" {
                                let s = Struct {
                                    extends_unparsed: find_attribute("structextends")
                                        .map(|attr| attr.value.clone()),
                                    is_union: category.value == "union",
                                    ..Default::default()
                                };
                                reader.element = ReaderElement::Type(Type::Struct(s));
                                reader.element_name = Some(name.map(|name| name.value.clone()).unwrap());
                            } else if category.value == "handle" {
                                let handle = Handle {
                                    type_enum: find_attribute("objtypeenum")
                                        .unwrap().value.clone(),
                                    ty: HandleType::Dispatchable,
                                    parsing_ty: false,
                                    parsing_name: false,
                                };
                                reader.element = ReaderElement::Type(Type::Handle(handle));
                                reader.element_name = None;
                            } else if category.value == "funcpointer" ||
                                category.value == "command"
                            {
                                let funcpointer = Funcpointer::default();
                                reader.element = ReaderElement::Type(Type::Funcpointer(funcpointer));
                                reader.element_name = None;
                            }
                        } else {
                            let ReaderElement::Type(ty) = &mut reader.element else {
                                continue
                            };
                            match ty {
                                Type::Bitmask(bitmask) => {
                                    if name.local_name == "type" {
                                        bitmask.parsing_type = true;
                                    } else if name.local_name == "name" {
                                        bitmask.parsing_name = true;
                                    } 
                                },
                                Type::Struct(structure) => {
                                    if let Some(parsing_member) = &mut structure.parsing_member {
                                        if name.local_name == "type" {
                                            parsing_member.parsing = StructParsing::Type;
                                        } else if name.local_name == "name" {
                                            parsing_member.parsing = StructParsing::Name;
                                        } else if name.local_name == "enum" {
                                            assert!(parsing_member.parsing == StructParsing::ArrayLen);
                                        } else if name.local_name == "comment" {
                                            parsing_member.parsing = StructParsing::Comment;
                                        }
                                    } else if name.local_name == "member" {
                                        let member = StructMember {
                                            value: find_attribute("values")
                                                .map(|attr| attr.value.clone()),
                                            deprecated: find_attribute("deprecated")
                                                .map(|attr| attr.value.clone()),
                                            ..Default::default()
                                        };
                                        structure.parsing_member = Some(member);
                                    }
                                },
                                Type::Handle(handle) => {
                                    if name.local_name == "type" {
                                        handle.parsing_ty = true;
                                    } else if name.local_name == "name" {
                                        handle.parsing_name = true;
                                    }
                                },
                                Type::Funcpointer(funcpointer) => {
                                    if let FuncpointerParsing::Proto (parsing) =
                                    &mut funcpointer.parsing {
                                        if name.local_name == "type" {
                                            *parsing = FuncParsing::Ty;
                                        } else if name.local_name == "name" {
                                            *parsing = FuncParsing::Name;
                                        }
                                    } else if let FuncpointerParsing::Param {
                                        parsing, ..
                                    } = &mut funcpointer.parsing {
                                        if name.local_name == "type" {
                                            *parsing = FuncParsing::Ty;
                                        } else if name.local_name == "name" {
                                            *parsing = FuncParsing::Name;
                                        }
                                    } else if name.local_name == "proto" {
                                        funcpointer.parsing = FuncpointerParsing::Proto(
                                            FuncParsing::Const
                                        );
                                    } else if name.local_name == "param" {
                                        funcpointer.parsing = FuncpointerParsing::Param {
                                            parsing: FuncParsing::Const,
                                            optional: find_attribute("optional")
                                                .map(|attr| {
                                                    match attr.value.as_str() {
                                                        "false" => ParamOptional::False,
                                                        "true" => ParamOptional::True,
                                                        "false,true" => ParamOptional::FalseTrue,
                                                        _ => ParamOptional::False,
                                                    }
                                                }).unwrap_or_default(),
                                            has_len: find_attribute("len")
                                                .map(|attr| {
                                                    funcpointer.params
                                                        .iter()
                                                        .enumerate()
                                                        .find_map(|(idx, param)| {
                                                            (param.name == attr.value)
                                                            .then_some(idx as u32)
                                                        })
                                                }),
                                            is_const: false,
                                            ptr: Default::default(),
                                            ty: Default::default(),
                                            name: Default::default(),
                                        };
                                    } else if name.local_name == "implicitexternsyncparams" {
                                        funcpointer.parsing = FuncpointerParsing::ImplicitParams;
                                    }
                                },
                            };
                        }
                    },
                    DepthType::Enums => {
                        let ReaderElement::Enums(e) = &reader.element else {
                            unreachable!()
                        };
                        match e {
                            Enums::Constants => {
                                let (mut ty, mut value, mut name) = Default::default();
                                for attr in &attributes {
                                    if attr.name.local_name == "type" {
                                        ty = Some(attr.value.as_str());
                                    } else if attr.name.local_name == "value" {
                                        value = Some(&attr.value);
                                    } else if attr.name.local_name == "name" {
                                        name = Some(&attr.value);
                                    }
                                }
                                let (ty, value, name) = (ty.unwrap(), value.unwrap(), name.unwrap());
                                let (ty, value) = match ty {
                                    "uint32_t" | "uint64_t" => {
                                        let mut value = value.to_string();
                                        let mut not = None;
                                        if let Some(a) = value.find("~") {
                                            let number = value.split_at(a + 1).1;
                                            let number = number.split_at(1).0;
                                            value = number.to_string();
                                            not = Some(quote! {!});
                                        }
                                        let number = syn::LitInt::new(&value, Span::call_site());
                                        (
                                            syn::Ident::new(
                                                if ty == "uint32_t" { "u32" } else {
                                                    "u64"
                                                },
                                                Span::call_site()
                                            ),
                                            quote! {
                                                #not #number
                                            }
                                        )
                                    },
                                    "float" => {
                                        let number = syn::LitFloat::new(
                                            value
                                                .trim_end_matches("f")
                                                .trim_end_matches("F"),
                                            Span::call_site()
                                        );
                                        (
                                            syn::Ident::new("f32", Span::call_site()),
                                            quote! {#number},
                                        )
                                    },
                                    _ => unreachable!(),
                                };
                                let name = Ident::new(name.trim_start_matches("VK_"), Span::call_site());
                                let def = quote! {
                                    pub const #name: #ty = #value;
                                };
                                write!(enums, "{}", def)?;
                            },
                            Enums::Enum(n) => {
                                if name.local_name != "enum" {
                                    continue
                                }
                                let name = n;
                                let variant_name = find_attribute("name")
                                    .unwrap().value.clone();
                                let doc = find_attribute("comment")
                                    .map(|attr| attr.value.clone());
                                if let Some(alias) = find_attribute("alias")
                                {
                                    if name.find("FlagBits").is_some() {
                                        let bits = &mut bitmask_bits.get_mut(name).unwrap();
                                        bits.bits.insert(Bit {
                                            name: variant_name,
                                            bit: 0,
                                            doc,
                                            alias: Some(alias.value.clone()),
                                            deprecated: find_attribute("deprecated")
                                                .map(|attr| attr.value.clone()),
                                        });
                                    } else {
                                        let variants = &mut enum_variants.get_mut(name).unwrap();
                                        variants.add_alias(
                                            variant_name,
                                            alias.value.clone(),
                                            find_attribute("deprecated")
                                                .map(|attr| attr.value.clone())
                                        );
                                    }
                                    continue
                                }
                                if name.find("FlagBits").is_some()
                                {
                                    let bits = &mut bitmask_bits.get_mut(name).unwrap();
                                    if let Some(bitpos) = find_attribute("bitpos")
                                    {
                                        let bitpos: u32 = str::parse(&bitpos.value).unwrap();
                                        bits.bits.insert(Bit {
                                            name: variant_name,
                                            bit: 1 << bitpos,
                                            doc,
                                            alias: None,
                                            deprecated: None,
                                        });
                                    } else {
                                        let value = find_attribute("value")
                                            .unwrap().value
                                            .clone();
                                        let value =
                                            if value.starts_with("0x") {
                                                u64::from_str_radix(value.trim_start_matches("0x"), 16).unwrap()
                                            } else {
                                                str::parse(&value).unwrap()
                                            };
                                        bits.bits.insert(Bit {
                                            name: variant_name,
                                            bit: value,
                                            doc,
                                            alias: None,
                                            deprecated: None,
                                        });
                                    }
                                } else {
                                    let variants = &mut enum_variants.get_mut(name).unwrap();
                                    let value = find_attribute("value")
                                        .unwrap().value.clone();
                                    variants.add(variant_name, syn::LitInt::new(&value, Span::call_site()), doc);
                                }
                            },
                        }
                    },
                    DepthType::Commands => {
                        if name.local_name == "command" {
                            let success_codes = parse_punctuated(
                                find_attribute("successcodes")
                                .map(|attr| attr.value.as_str())
                                .unwrap_or_default()
                            );
                            if let Some(alias) = find_attribute("alias")
                            {
                                let name = find_attribute("name")
                                    .unwrap().value.clone();
                                let alias = alias.value.clone();
                                let pfn_name = format!("PFN_{name}");
                                let pfn_alias = format!("PFN_{alias}");
                                let name_rs = Ident::new(&lower_snake_case(
                                    name.trim_start_matches("vk")
                                ), Span::call_site());
                                commands.insert(name, Command {
                                    body: CommandBody::Alias(alias),
                                    success_codes,
                                    deprecated: None,
                                    version: None,
                                    unresolved: None,
                                    name_rs,
                                });
                                type_aliases.insert(
                                    pfn_name,
                                    pfn_alias,
                                );
                                continue
                            }
                            let funcpointer = Funcpointer {
                                success_codes,
                                ..Default::default()
                            };
                            reader.element = ReaderElement::Type(Type::Funcpointer(funcpointer));
                            reader.element_name = None;
                        } else {
                            let ReaderElement::Type(ty) = &mut reader.element else {
                                continue
                            };
                            if let Type::Funcpointer(funcpointer) = ty
                            {
                                if let FuncpointerParsing::Proto(parsing)
                                = &mut funcpointer.parsing {
                                    if name.local_name == "type" {
                                        *parsing = FuncParsing::Ty;
                                    } else if name.local_name == "name" {
                                        *parsing = FuncParsing::Name;
                                    }
                                } else if let FuncpointerParsing::Param {
                                    parsing, ..
                                } = &mut funcpointer.parsing {
                                    if name.local_name == "type" {
                                        *parsing = FuncParsing::Ty;
                                    } else if name.local_name == "name" {
                                        *parsing = FuncParsing::Name;
                                    }
                                } else if let FuncpointerParsing::ImplicitParams = funcpointer.parsing {
                                } else if name.local_name == "proto" {
                                    funcpointer.parsing = FuncpointerParsing::Proto(
                                        FuncParsing::Const
                                    );
                                } else if name.local_name == "param" {
                                    funcpointer.parsing = FuncpointerParsing::Param {
                                        parsing: FuncParsing::Const,
                                        is_const: false,
                                        ptr: Default::default(),
                                        ty: Default::default(),
                                        name: Default::default(),
                                        optional: find_attribute("optional")
                                            .map(|attr| {
                                                match attr.value.as_str() {
                                                    "false" => ParamOptional::False,
                                                    "true" | "true,true" => ParamOptional::True,
                                                    "false,true" => ParamOptional::FalseTrue,
                                                    _ => unreachable!(),
                                                }
                                            }).unwrap_or_default(),
                                        has_len: find_attribute("len")
                                                .map(|attr| {
                                                    funcpointer.params
                                                        .iter()
                                                        .enumerate()
                                                        .find_map(|(idx, param)| {
                                                            (param.name == attr.value)
                                                            .then_some(idx as u32)
                                                        })
                                                }),
                                    };
                                } else if name.local_name == "implicitexternsyncparams" {
                                    funcpointer.parsing = FuncpointerParsing::ImplicitParams;
                                }
                            }
                        }
                    },
                    DepthType::Feature => {
                        let ReaderElement::Feature {
                            version, feature, parsing_requires,
                            parsing_deprecated, parsing_remove,
                            ..
                        } = &mut reader.element else { unreachable!() };
                        if depth.vulkan_sc.is_none() {
                            if name.local_name == "require" {
                                *parsing_requires = Some(reader.depth);
                            } else if parsing_requires.is_some() &&
                                name.local_name == "command"
                            {
                                let name = find_attribute("name")
                                    .unwrap().value.clone();
                                let cmd = commands.get_mut(&name).unwrap();
                                cmd.version = Some(version.clone());
                                feature.commands.insert(name, None);
                            } else if name.local_name == "enum" &&
                                let Some(extends) = find_attribute("extends")
                                    .map(|extends| &extends.value)
                            {
                                let name = find_attribute("name")
                                    .unwrap().value.clone();
                                if let Some(offset) = find_attribute("offset")
                                {
                                    let offset = str::parse(&offset.value).unwrap();
                                    let extnumber = find_attribute("extnumber").unwrap();
                                    let extnumber = str::parse(&extnumber.value).unwrap();
                                    let doc = find_attribute("comment")
                                        .map(|attr| attr.value.clone());
                                    let variants = enum_variants
                                        .get_mut(extends)
                                        .unwrap();
                                    let dir = find_attribute("dir")
                                        .map(|attr| &attr.value);
                                    variants.add_ext(name, offset, extnumber, dir, doc);
                                } else if let Some(bitpos) = find_attribute("bitpos")
                                {
                                    let bitpos: u32 = str::parse(&bitpos.value).unwrap();
                                    let bits = bitmask_bits
                                        .get_mut(extends)
                                        .unwrap();
                                    let doc = find_attribute("comment")
                                        .map(|attr| attr.value.clone());
                                    bits.bits.insert(Bit {
                                        name,
                                        bit: 1 << bitpos,
                                        doc,
                                        alias: None,
                                        deprecated: None,
                                    });
                                } else if let Some(value) = find_attribute("value")
                                {
                                    let variants = enum_variants
                                        .get_mut(extends)
                                        .unwrap();
                                    let doc = find_attribute("comment")
                                        .map(|attr| attr.value.clone());
                                    variants.add(
                                        name,
                                        syn::LitInt::new(&value.value, Span::call_site()),
                                        doc
                                    );
                                }
                            } else if name.local_name == "deprecate" {
                                *parsing_deprecated = Some(reader.depth);
                            } else if parsing_deprecated.is_some() &&
                                name.local_name == "command"
                            {
                                let name = &find_attribute("name")
                                    .unwrap().value;
                                let deprecated_by = find_attribute("supersededby")
                                    .map(|attr| attr.value.clone());
                                let command = commands.get_mut(name)
                                    .unwrap();
                                command.deprecated = Some(deprecated_by);
                            }
                        } else {
                            if name.local_name == "remove" {
                                *parsing_remove = true;
                            } else if !*parsing_remove &&
                                let Some(name) = find_attribute("name")
                            {
                                type_exclude_list.insert(
                                    Ident::new(name.value.trim_start_matches("Vk"), Span::call_site())
                                );
                            }
                        }
                    },
                    DepthType::Extensions => {
                        let ReaderElement::Extensions {
                            parsing_extension,
                        } = &mut reader.element else {
                            unreachable!()
                        };
                        if let Some(ext) = parsing_extension {
                            if let Some(parsing) = &ext.parsing_requires {
                                if !parsing.is_sc && !ext.is_disabled {
                                    if name.local_name == "command" {
                                        let name = find_attribute("name")
                                            .unwrap().value.clone();
                                        ext.ext.commands.push(ExtCommand {
                                            name,
                                            depends_on: parsing.depends_on.clone(),
                                        });
                                    } else if name.local_name == "enum" {
                                        let name = find_attribute("name")
                                            .unwrap().value.clone();
                                        if let Some(value) = find_attribute("value")
                                            .map(|attr| &attr.value)
                                        {
                                            if let Some(extends) = &find_attribute("extends") {
                                                let variant = enum_variants
                                                    .get_mut(&extends.value)
                                                    .unwrap();
                                                let doc = find_attribute("comment")
                                                    .map(|attr| attr.value.clone());
                                                variant.add(
                                                    name,
                                                    syn::LitInt::new(value, Span::call_site()),
                                                    doc,
                                                );
                                            } else {
                                                let constant = if value.starts_with("\"") {
                                                    ExtConstant::String(value
                                                        .trim_start_matches("\"")
                                                        .trim_end_matches("\"")
                                                        .to_string()
                                                    )
                                                } else {
                                                    ExtConstant::U32(str::parse(value).unwrap())
                                                };
                                                ext.ext.new_constants.push((name, constant));
                                            }
                                        } else if let Some(offset) = find_attribute("offset")
                                        {
                                            let offset = str::parse(&offset.value).unwrap();
                                            let extends = &find_attribute("extends")
                                                .unwrap().value;
                                            let extnumber = find_attribute("extnumber")
                                                .map(|attr| str::parse(&attr.value).unwrap())
                                                .unwrap_or(ext.ext.number);
                                            let variant = enum_variants
                                                .get_mut(extends)
                                                .unwrap();
                                            let dir = find_attribute("dir")
                                                .map(|attr| &attr.value);
                                            let doc = find_attribute("comment")
                                                .map(|attr| attr.value.clone());
                                            variant.add_ext(
                                                name,
                                                offset,
                                                extnumber,
                                                dir,
                                                doc,
                                            );
                                        } else if let Some(bitpos) = find_attribute("bitpos")
                                        {
                                            let bitpos: u32 = str::parse(&bitpos.value).unwrap();
                                            let extends = &find_attribute("extends")
                                                .unwrap().value;
                                            let bits = bitmask_bits
                                                .get_mut(extends)
                                                .unwrap();
                                            let doc = find_attribute("comment")
                                                .map(|attr| attr.value.clone());
                                            bits.bits.insert(Bit {
                                                name,
                                                bit: 1 << bitpos,
                                                doc,
                                                alias: None,
                                                deprecated: None,
                                            });
                                        } else if let Some(alias) = find_attribute("alias") &&
                                            let Some(extends) = find_attribute("extends")
                                                .map(|attr| &attr.value)
                                        {
                                            let deprecated = find_attribute("deprecated")
                                                .map(|attr| attr.value.clone());
                                            if let Some(variants) = enum_variants.get_mut(extends) {
                                                variants.add_alias(name, alias.value.clone(), deprecated);
                                            } else if let Some(bits) = bitmask_bits.get_mut(extends)
                                            {
                                                bits.bits.insert(Bit {
                                                    name,
                                                    bit: 0,
                                                    doc: None,
                                                    alias: Some(alias.value.clone()),
                                                    deprecated: find_attribute("deprecated")
                                                        .map(|attr| attr.value.clone()),
                                                });
                                            }
                                        }
                                    }
                                } else {
                                    if let Some(name) = find_attribute("name") {
                                        type_exclude_list.insert(
                                            Ident::new(name.value
                                                .trim_start_matches("Vk"),
                                                Span::call_site(),
                                            )
                                        );
                                    }
                                }
                            } else if name.local_name == "require" {
                                let depends_on = find_attribute("depends")
                                    .map(|attr| {
                                        parse_punctuated(&attr.value)
                                    }).unwrap_or_default();
                                ext.parsing_requires = Some(ExtParsingRequires {
                                    depth: current_depth,
                                    is_sc: depth.vulkan_sc.is_some(),
                                    depends_on,
                                });
                            }
                        } else if name.local_name == "extension" &&
                            find_attribute("author").is_some()
                        {
                            *parsing_extension = Some(ExtParsing {
                                name: find_attribute("name")
                                    .unwrap()
                                    .value.clone(),
                                is_disabled:
                                    if let Some(supported) = find_attribute("supported") &&
                                        supported.value == "disabled"
                                    {
                                        true
                                    } else { false },
                                ext: Extension {
                                    ty: find_attribute("type")
                                        .map(|attr| attr.value.clone())
                                        .unwrap_or_default(),
                                    number: str::parse(&find_attribute("number")
                                        .unwrap()
                                        .value
                                    ).unwrap(),
                                    promoted_to: find_attribute("promotedto")
                                        .map(|attr| attr.value.clone()),
                                    new_constants: vec![],
                                    commands: vec![],
                                },
                                parsing_requires: None,
                            })
                        }
                    },
                };
            },
            Ok(XmlEvent::Characters(string)) => {
                if depth.vulkan_sc.is_some() { continue }
                if let ReaderElement::Type(ty) = &mut reader.element {
                    match ty {
                        Type::Bitmask(bitmask) => {
                            if bitmask.parsing_type {
                                bitmask.ty = string;
                                bitmask.parsing_type = false;
                            } else if bitmask.parsing_name {
                                reader.element_name = Some(string);
                                bitmask.parsing_name = false;
                            }
                        },
                        Type::Struct(structure) => {
                            if let Some(parsing_member) = &mut structure.parsing_member {
                                match parsing_member.parsing {
                                    StructParsing::Const => {
                                        if string.starts_with("const") {
                                            parsing_member.is_const_ptr = true;
                                        }
                                        parsing_member.parsing = StructParsing::None;
                                    },
                                    StructParsing::Type => {
                                        parsing_member.ty = Some(Ident::new(string
                                            .trim_start_matches("Vk"),
                                            Span::call_site()
                                        ));
                                        parsing_member.parsing = StructParsing::Pointer;
                                    },
                                    StructParsing::Pointer => {
                                        if string.starts_with("**") {
                                            assert!(!parsing_member.is_const_ptr);
                                            parsing_member.ptr_type = PtrType::MutPtrMutPtr;
                                        } else if string.starts_with("* const*") ||
                                            string.starts_with("* const *")
                                        {
                                            assert!(parsing_member.is_const_ptr);
                                            parsing_member.ptr_type = PtrType::ConstPtrConstPtr;
                                        } else if string.starts_with("*  ") ||
                                            string == "*" || string == "* "
                                        {
                                            if parsing_member.is_const_ptr {
                                                parsing_member.ptr_type = PtrType::ConstPtr
                                            } else { parsing_member.ptr_type = PtrType::MutPtr }
                                        } else {
                                            assert!(!string.starts_with("*"), "{string}")
                                        };
                                        parsing_member.parsing = StructParsing::None;
                                    },
                                    StructParsing::Name => {
                                        let name = lower_snake_case(&string);
                                        if name == "p_next" {
                                            structure.needs_lifetime = true;
                                        }
                                        parsing_member.name = name;
                                        parsing_member.parsing = StructParsing::ArrayLen;
                                    },
                                    StructParsing::ArrayLen => {
                                        if string.contains("[") {
                                            let split = string.split_at(1).1;
                                            if split.starts_with(char::is_numeric) {
                                                let idx = split.find("]").unwrap();
                                                let split = split.split_at(idx).0;
                                                parsing_member.array_len = Some(split.to_string());
                                                parsing_member.parsing = StructParsing::None;
                                            }
                                            continue
                                        } else if string.contains(":24") {
                                            structure.packed_u24_u8 = Some(parsing_member.name.clone());
                                        } else if string.contains(":5") {
                                            let packed_u24_u8 = structure.packed_u24_u8.as_mut().unwrap();
                                            *packed_u24_u8 = format!("{packed_u24_u8}_and_{}", parsing_member.name);
                                        } else if string.contains(":8") || string.contains(":3")
                                        {
                                            let packed_u24_u8 = structure.packed_u24_u8.take().unwrap();
                                            parsing_member.name = format!("{packed_u24_u8}_and_{}", parsing_member.name);
                                            parsing_member.ty = Some(Ident::new("packed_u24_u8", Span::call_site()));
                                        } else if let Some(packed) = structure.packed_u9_u9_u6_u4_u4.take() {
                                            use packed_u9_u9_u6_u4_u4 as E;
                                            structure.packed_u9_u9_u6_u4_u4 = match packed {
                                                E::A(name) => Some(E::B(format!("{name}_and_{}_", parsing_member.name))),
                                                E::B(name) => Some(E::C(format!("{name}_and_{}_", parsing_member.name))),
                                                E::C(name) => Some(E::D(format!("{name}_and_{}_", parsing_member.name))),
                                                E::D(name) => Some(E::E(format!("{name}_and_{}_", parsing_member.name))),
                                                E::E(name) => {
                                                    parsing_member.name = format!("{name}_and_{}", parsing_member.name);
                                                    parsing_member.ty = Some(
                                                        Ident::new("packed_u9_u9_u6_u4_u4",
                                                        Span::call_site()
                                                    ));
                                                    None
                                                },
                                            }
                                        } else if string.contains(":9") {
                                            structure.packed_u9_u9_u6_u4_u4 = Some(
                                                packed_u9_u9_u6_u4_u4::A(parsing_member.name.clone())
                                            );
                                        } else {
                                            parsing_member.array_len = Some(string
                                                .trim_start_matches("VK_")
                                                .to_string()
                                            );
                                            parsing_member.parsing = StructParsing::None;
                                        }
                                    },
                                    StructParsing::Comment => {
                                        parsing_member.comment = Some(string);
                                        parsing_member.parsing = StructParsing::None;
                                    },
                                    StructParsing::None => {},
                                }
                            }
                        },
                        Type::Handle(handle) => {
                            if handle.parsing_ty {
                                if string == "VK_DEFINE_HANDLE" {
                                    handle.ty = HandleType::Dispatchable;
                                } else if string == "VK_DEFINE_NON_DISPATCHABLE_HANDLE" {
                                    handle.ty = HandleType::NonDispatchable;
                                } else { unreachable!() }
                                handle.parsing_ty = false;
                            } else if handle.parsing_name {
                                reader.element_name = Some(string.trim_start_matches("Vk").to_string());
                                handle.parsing_name = false;
                            }
                        },
                        Type::Funcpointer(funcpointer) => {
                            if let FuncpointerParsing::Proto(parsing)
                                = &mut funcpointer.parsing 
                            {
                                match parsing {
                                    FuncParsing::Const => {
                                        assert!(string == "const", "{string}");
                                        funcpointer.ret_is_const = true;
                                        *parsing = FuncParsing::Ty;
                                    },
                                    FuncParsing::Ty => {
                                        funcpointer.ret_ty = string;
                                        *parsing = FuncParsing::Pointer;
                                    },
                                    FuncParsing::Pointer => {
                                        if string.starts_with("**") {
                                            assert!(!funcpointer.ret_is_const);
                                            funcpointer.ret_ptr = PtrType::MutPtrMutPtr;
                                        } else if string.starts_with("* const*") {
                                            assert!(funcpointer.ret_is_const);
                                            funcpointer.ret_ptr = PtrType::ConstPtrConstPtr;
                                        } else if string.starts_with("*  ") ||
                                            string == "*" || string == "* "
                                        {
                                            if funcpointer.ret_is_const {
                                                funcpointer.ret_ptr = PtrType::ConstPtr
                                            } else { funcpointer.ret_ptr = PtrType::MutPtr }
                                        } else {
                                            assert!(!string.starts_with("*"), "{string}");
                                        };
                                        *parsing = FuncParsing::Name;
                                    },
                                    FuncParsing::Name => {
                                        reader.element_name = Some(string);
                                        funcpointer.parsing = FuncpointerParsing::None;
                                    },
                                    FuncParsing::Array => unreachable!(),
                                }
                            } else if let FuncpointerParsing::Param {
                                parsing,
                                is_const,
                                ty,
                                ptr,
                                name,
                                ..
                            } = &mut funcpointer.parsing {
                                match parsing {
                                    FuncParsing::Const => {
                                        *parsing = FuncParsing::Ty;
                                        if string.starts_with("const") {
                                            *is_const = true;
                                        }
                                    },
                                    FuncParsing::Ty => {
                                        *ty = string;
                                        *parsing = FuncParsing::Pointer;
                                    },
                                    FuncParsing::Pointer => {
                                        if string.starts_with("**") {
                                            assert!(!*is_const);
                                            *ptr = FuncParamPtrType::PtrType(PtrType::MutPtrMutPtr);
                                        } else if string.starts_with("* const*") {
                                            assert!(*is_const);
                                            *ptr = FuncParamPtrType::PtrType(PtrType::ConstPtrConstPtr);
                                        } else if string.starts_with("*  ") ||
                                            string == "*" || string == "* "
                                        {
                                            if *is_const {
                                                *ptr = FuncParamPtrType::PtrType(PtrType::ConstPtr);
                                            } else {
                                                *ptr = FuncParamPtrType::PtrType(PtrType::MutPtr);
                                            }
                                        } else {
                                            assert!(!string.starts_with("*"), "{string}");
                                        };
                                        *parsing = FuncParsing::Name;
                                    },
                                    FuncParsing::Name => {
                                        *name = string;
                                        *parsing = FuncParsing::Array;
                                    },
                                    FuncParsing::Array => {
                                        assert!(matches!(*ptr, FuncParamPtrType::PtrType(PtrType::None)));
                                        if let Some(idx) = string.find("[") {
                                            let str = string.split_at(idx + 1).1;
                                            let idx = str.find("]").unwrap();
                                            let str = str.split_at(idx).0;
                                            *ptr = FuncParamPtrType::Array(str::parse(str).unwrap());
                                        }
                                    },
                                }
                            }
                        },
                    }
                }
            },
            Ok(XmlEvent::EndElement { name }) => {
                reader.depth -= 1;
                if let Some(d) = depth.vulkan_sc 
                {
                    if d == reader.depth {
                        depth.vulkan_sc = None;
                    }
                    if !matches!(depth.ty, DepthType::Feature | DepthType::Extensions) {
                        continue
                    }
                }
                match depth.ty {
                    DepthType::Types => {
                        if name.local_name == "types" {
                            depth.ty = DepthType::Unknown;
                            reader.element = ReaderElement::Inactive;
                            reader.element_name = None;
                        } else if depth.depth + 1 == reader.depth {
                            if let ReaderElement::Type(ty) = &mut reader.element
                            {
                                match ty {
                                    Type::Bitmask(bitmask) => {
                                        bitmask_bits
                                            .insert(
                                                bitmask.requires
                                                    .clone()
                                                    .unwrap_or_else(||
                                                        reader.element_name
                                                            .as_ref()
                                                            .unwrap()
                                                            .replace("Flags", "FlagBits")
                                                    ),
                                                BitmaskBits {
                                                    bits: IndexSet::new(),
                                                    flag_type:
                                                        if bitmask.ty == "VkFlags" {
                                                            FlagType::U32
                                                        } else if bitmask.ty == "VkFlags64" {
                                                            FlagType::U64
                                                        } else { panic!("{:?}", reader.element_name) }
                                                }
                                            );
                                    },
                                    Type::Struct(structure) => {
                                        structure.name = Some(Ident::new(reader.element_name
                                            .as_ref()
                                            .unwrap()
                                            .trim_start_matches("Vk"),
                                            Span::call_site()
                                        ));
                                        if let Some(extends) = &structure.extends_unparsed {
                                            for item in parse_punctuated(extends) {
                                                let item = item.trim_start_matches("Vk");
                                                struct_extends.insert(Ident::new(item, Span::call_site()));
                                                structure.extends.push(Ident::new(
                                                    &format!("Extends{}", item),
                                                    Span::call_site()
                                                ));
                                            }
                                        }
                                        struct_defs.push(structure.clone());
                                    },
                                    Type::Handle(handle) => {
                                        let type_enum = Ident::new(
                                            handle.type_enum.trim_start_matches("VK_OBJECT_TYPE_"),
                                            Span::call_site(),
                                        );
                                        let name_str = reader.element_name.take().unwrap();
                                        let doc = doc_link(&name_str).unwrap_or_default();
                                        let name = Ident::new(&name_str, Span::call_site());
                                        let def = match handle.ty {
                                            HandleType::NonDispatchable => {
                                                non_eq_types.insert(name.clone(), false);
                                                quote! {
                                                    define_non_dispatchable_handle!(
                                                        #name, #type_enum, #doc,
                                                    );
                                                }
                                            },
                                            HandleType::Dispatchable => {
                                                dispatchable_handles.insert(name_str.clone());
                                                quote! {
                                                    define_handle!(
                                                        #name, #type_enum, #doc,
                                                    );
                                                }
                                            },
                                        };
                                        write!(handles, "{def}")?;
                                    },
                                    Type::Funcpointer(funcpointer) => {
                                        funcpointers.push((reader.element_name.take().unwrap(), Func {
                                            ret_ty: funcpointer.ret_ty.clone(),
                                            ret_ptr: funcpointer.ret_ptr,
                                            params: funcpointer.params.clone(),
                                        }));
                                    },
                                }
                            }
                            reader.element = ReaderElement::Inactive;
                            reader.element_name = None;
                        } else if 
                            let ReaderElement::Type(ty) = &mut reader.element &&
                            let Type::Struct(structure) = ty &&
                            name.local_name == "member"
                        {
                            let member = structure.parsing_member.take().unwrap();
                            if structure.packed_u24_u8.is_none() && structure.packed_u9_u9_u6_u4_u4.is_none() {
                                structure.members.push(member);
                            }
                        } else if
                            let ReaderElement::Type(ty) = &mut reader.element &&
                            let Type::Funcpointer(funcpointer) = ty
                        {
                            if name.local_name == "proto" {
                                funcpointer.parsing = FuncpointerParsing::None;
                            } else if name.local_name == "param" {
                                let FuncpointerParsing::Param {
                                    ty, ptr, name, optional, has_len, ..
                                } = funcpointer.parsing.clone() else { unreachable!() };
                                funcpointer.params.push(Funcparam {
                                    optional, has_len, name,
                                    ty, ptr,
                                });
                                funcpointer.parsing = FuncpointerParsing::None;
                            }
                        }
                    },
                    DepthType::Enums => {
                        if name.local_name == "enums" {
                            depth.ty = DepthType::Unknown;
                            reader.element = ReaderElement::Inactive;
                            reader.element_name = None;
                        }
                    },
                    DepthType::Commands => {
                        if name.local_name == "commands" {
                            depth.ty = DepthType::Unknown;
                            reader.element = ReaderElement::Inactive;
                            reader.element_name = None;
                        } else if name.local_name == "command" &&
                            let ReaderElement::Type(ty) = &mut reader.element &&
                            let Type::Funcpointer(funcpointer) = ty
                        {
                            let name = reader.element_name.clone().unwrap();
                            let name_rs = Ident::new(&lower_snake_case(
                                name.trim_start_matches("vk")
                            ), Span::call_site());
                            commands.insert(
                                reader.element_name.clone().unwrap(),
                                Command {
                                    body: CommandBody::Func(Func {
                                        ret_ty: funcpointer.ret_ty.clone(),
                                        ret_ptr: funcpointer.ret_ptr,
                                        params: funcpointer.params.clone(),
                                    }),
                                    name_rs,
                                    success_codes: funcpointer.success_codes.clone(),
                                    deprecated: None,
                                    version: None,
                                    unresolved: None,
                                }
                            );
                        } else if
                            let ReaderElement::Type(ty) = &mut reader.element &&
                            let Type::Funcpointer(funcpointer) = ty
                        {
                            if name.local_name == "proto" {
                                funcpointer.parsing = FuncpointerParsing::None;
                            } else if name.local_name == "param" &&
                                !matches!(funcpointer.parsing, FuncpointerParsing::ImplicitParams)
                            {
                                let FuncpointerParsing::Param {
                                    ty, ptr, name, optional, has_len, ..
                                } = funcpointer.parsing.clone() else { unreachable!() };
                                funcpointer.params.push(Funcparam {
                                    ty, ptr, name, optional, has_len
                                });
                                funcpointer.parsing = FuncpointerParsing::None;
                            } else if name.local_name == "implicitexternsyncparams" {
                                funcpointer.parsing = FuncpointerParsing::None;
                            }
                        }
                    },
                    DepthType::Feature => {
                        if name.local_name == "require" &&
                            let ReaderElement::Feature {
                                parsing_requires, ..
                            } = &mut reader.element
                        {
                            *parsing_requires = None;
                        } else if name.local_name == "deprecate" &&
                            let ReaderElement::Feature {
                                parsing_deprecated, ..
                            } = &mut reader.element
                        {
                            *parsing_deprecated = None;
                        } else if name.local_name == "feature" &&
                            depth.depth == reader.depth
                        {
                            let ReaderElement::Feature {
                                version, feature, is_sc, ..
                            } = &mut reader.element else { unreachable!() };
                            if !*is_sc {
                                let mut version = version.replace('.', "_");
                                version.insert_str(0, "VK_VERSION_");
                                features.entry(version)
                                    .and_modify(|f| {
                                        f.commands.append(&mut feature.commands);
                                    }).or_insert_with(|| {
                                        feature.clone()
                                    });
                            }
                            reader.element = ReaderElement::Inactive;
                            depth.ty = DepthType::Unknown;
                        }
                    },
                    DepthType::Extensions => {
                        if name.local_name == "extensions" {
                            reader.element = ReaderElement::Inactive;
                            depth.ty = DepthType::Unknown;
                        } else if let ReaderElement::Extensions {
                            parsing_extension,
                        } = &mut reader.element
                        {
                            if name.local_name == "extension" {
                                let Some(ext) = parsing_extension.take() else {
                                    continue
                                };
                                if !ext.is_disabled {
                                    extensions.insert(
                                        ext.name,
                                        ext.ext,
                                    );
                                }
                            } else if let Some(ext) = parsing_extension &&
                                name.local_name == "require" &&
                                let Some(parsing) = ext.parsing_requires.take()
                            {
                                assert!(parsing.depth == reader.depth);
                            }
                        }
                    },
                    DepthType::Unknown => {},
                };
            },
            Err(e) => {
                eprintln!("error: {e}");
                break
            }
            _ => {},
        }
    }
    let fn_safety = quote! {
        /// # Safety
        /// All raw Vulkan calls are unsafe as there is no validation of input or usage.
    };
    println!("{:?}", unique_categories);
    for (name_str, variants) in enum_variants {
        if name_str.find("FlagBits").is_some() { continue }
        let doc = doc_link(&name_str)
            .map(|link| quote! { #[doc = #link] });
        let name = Ident::new(name_str.trim_start_matches("Vk"), Span::call_site());
        let def = quote! {
           
            #doc
            #[repr(transparent)]
            #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
            pub struct #name(i32);

            impl #name {
                #[inline]
                pub const fn from_raw(x: i32) -> Self {
                    Self(x)
                }
                #[inline]
                pub const fn as_raw(self) -> i32 {
                    self.0
                }
            }
        };
        let mut display = vec![];
        let var1 = variants.variants.first().map(|var| var.name.as_str()).unwrap_or_default();
        let sc = name_str.trim_end_matches("EXT")
            .trim_end_matches("KHR")
            .trim_end_matches("NV");
        let mut sc = upper_snake_case(sc);
        if !sc.ends_with('_') {
            sc.push('_');
        }
        let mut i = 0;
        while var1.chars().nth(i) == sc.chars().nth(i) { i += 1; }
        let replace = var1.split_at(i).0;
        let mut var_names = vec![];
        let variants = variants
            .variants
            .iter().map(|Variant { name, value, _doc, alias, deprecated, }| {
                let mut name_str = name
                    .trim_start_matches(replace)
                    .trim_start_matches("VK_")
                    .to_uppercase();
                if name_str.starts_with(char::is_numeric) {
                    name_str.insert_str(0, "TYPE_")
                }
                let name = Ident::new(&name_str, Span::call_site());
                //let doc = doc.as_ref().map(|doc| quote! { #[doc = #doc] });
                var_names.push(name.clone());
                let deprecated = deprecated
                    .as_ref()
                    .map(|reason| quote! { #[deprecated = #reason] });
                let value = if let Some(alias) = alias {
                    let mut alias_str = alias
                        .trim_start_matches(replace)
                        .to_uppercase();
                    if alias_str.starts_with(char::is_numeric) {
                        alias_str.insert_str(0, "TYPE_")
                    }
                    let alias = Ident::new(&alias_str, Span::call_site());
                    quote! { Self::#alias }
                } else {
                    display.push(quote! {
                        Self::#name => write!(f, #name_str),
                    });
                    quote! { Self(#value) }
                };
                quote! {
                    #deprecated
                    pub const #name: Self = #value;
                }
            });
        let variants = quote! {
            impl #name {
                #(#variants)*
            }
        };
        let body = if var_names.is_empty() {
            quote! { write!(f, "{}", self.0) }
        } else if var_names.len() == 1 {
            let var = var_names.first().unwrap();
            let fmt = var.to_string();
            quote! {
                if *self == Self::#var {
                    write!(f, #fmt)
                } else {
                    write!(f, "{}", self.0)
                }
            }
        } else {
            quote! {
                match *self {
                    #(#display)*
                    x => write!(f, "{x}"),
                }
            }
        };
        let display = quote! {
            impl fmt::Display for #name {
                
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    #body
                }
            }
        };
        write!(enums, "{def}{variants}{display}")?;
    }
    for (requires, bits) in &bitmask_bits {
        let requires = requires.trim_start_matches("Vk");
        let name = requires.replace("FlagBits", "Flags");
        let doc = doc_link(&name)
            .map(|link| quote! { #[doc = #link] });
        let name = Ident::new(&name, Span::call_site());
        let ty = match bits.flag_type {
            FlagType::U32 => Ident::new("Flags", Span::call_site()),
            FlagType::U64 => Ident::new("Flags64", Span::call_site())
        };
        let def = quote! {
            #doc
            #[repr(transparent)]
            #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct #name(#ty);
            bitflags!(#name, #ty); 
        };
        let requires_sc = upper_snake_case(requires);
        let idx = requires_sc.find("FLAG_BITS").unwrap_or_default();
        let mut requires_sc = requires_sc
            .split_at(idx).0
            .trim_start_matches("_")
            .to_string();
        if requires.ends_with("2")
        {
            requires_sc.push_str("2_");
        } else if requires.ends_with("3") ||
            requires.ends_with("3KHR")
        {
            requires_sc.push_str("3_");
        }
        let mut variants = vec![];
        let mut display = vec![];
        let as_u64 = (bits.flag_type == FlagType::U32).then(||
            quote! { as u64 }
        );
        let mut duplicates = HashSet::new();
        for Bit { name, bit, doc, alias, deprecated }  in &bits.bits {
            let mut name_str = name
                .trim_start_matches("VK_")
                .replace("_BIT", "")
                .replace(&requires_sc, "");
            if !duplicates.insert(name_str.clone()) {
                continue
            }
            if name_str.starts_with(char::is_numeric) {
                name_str.insert_str(0, "TYPE_");
            }
            let name = Ident::new(&name_str, Span::call_site());
            let doc = doc.as_ref().map(|doc| quote! { #[doc = #doc] });
            let deprecated = deprecated
                .as_ref()
                .map(|reason| quote! { #[deprecated = #reason] });
            let value = if let Some(alias) = alias {
                let mut alias_str = alias
                    .trim_start_matches("VK_")
                    .replace("_BIT", "")
                    .replace(&requires_sc, "");
                if alias_str.starts_with(char::is_numeric) {
                    alias_str.insert_str(0, "TYPE_");
                }
                let alias = Ident::new(&alias_str, Span::call_site());
                quote! {
                    Self::#alias
                }
            } else {
                if *bit != 0 {
                    display.push(quote! {
                        (#bit, #name_str),
                    });
                }
                let bit = syn::LitInt::new(&format!("0x{:x}", bit), Span::call_site());
                quote! {
                    Self(#bit)
                }
            };
            variants.push( quote! {
                #doc
                #deprecated
                pub const #name: Self = #value;
            });
        }
        let variants = quote! {
            impl #name {
                #(#variants)*
            }
        };
        let display = quote! {

            impl fmt::Display for #name {

                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    if self.is_empty() {
                        write!(f, "NONE")?;
                    }
                    const BITS: &[(u64, &str)] = &[
                        #(#display)*
                    ];
                    flag_display(
                        self.0 #as_u64,
                        BITS,
                        f,
                    )
                }
            }
        };
        write!(enums, "{def}{variants}{display}")?;
    }
    let not_send_sync_types: HashSet<Ident> =
        [
            Ident::new("LPCWSTR", Span::call_site()),
            Ident::new("HANDLE", Span::call_site()),
            Ident::new("NvSciSyncAttrList", Span::call_site()),
            Ident::new("NvSciSyncObj", Span::call_site()),
            Ident::new("NvSciBufAttrList", Span::call_site()),
            Ident::new("NvSciBufObj", Span::call_site()),
            Ident::new("MTLDevice_id", Span::call_site()),
            Ident::new("MTLCommandQueue_id", Span::call_site()),
            Ident::new("MTLBuffer_id", Span::call_site()),
            Ident::new("MTLTexture_id", Span::call_site()),
            Ident::new("MTLSharedEvent_id", Span::call_site()),
            Ident::new("IOSurfaceRef", Span::call_site()),
            Ident::new("RemoteAddressNV", Span::call_site()),
        ].into_iter().collect();
    let mut type_needs_lifetime: IndexSet<Ident> =
        struct_defs
            .iter()
            .filter(|def| def.needs_lifetime)
            .map(|def| def.name.clone().unwrap())
            .collect(); 
    let mut depth = 0;
    loop {
        depth += 1;
        let new: Vec<_> = struct_defs
            .iter()
            .filter(|def| {
                if type_needs_lifetime.contains(def.name.as_ref().unwrap()) {
                    return false
                }
                for member in &def.members {
                    if type_needs_lifetime.contains(member.ty.as_ref().unwrap()) {
                        return true;
                    }
                }
                false
            }).map(|def| def.name.clone().unwrap()).collect();
        if new.is_empty() {
            break
        }
        type_needs_lifetime.extend(new);
    }
    let resolve_ty = |ty: &str, prefix_handle_vk: bool| -> Option<Punctuated<Ident, Token![::]>> {
        let mut punct = Punctuated::new();
        if ty.starts_with("uint") {
            let ident = syn::Ident::new(
                &format!("u{}", ty
                    .trim_start_matches("uint")
                    .trim_end_matches("_t")
                ),
                Span::call_site()
            );
            punct.push_value(ident);
        } else if ty.starts_with("int") {
            if ty == "int" {
                punct.push_value(Ident::new("ffi", Span::call_site()));
                punct.push(Ident::new("c_int", Span::call_site()));
            } else {
                let ident = syn::Ident::new(
                    &format!("i{}", ty
                        .trim_start_matches("int")
                        .trim_end_matches("_t")
                    ),
                    Span::call_site()
                );
                punct.push_value(ident);
            }
        } else if ty == "size_t" {
            punct.push_value(Ident::new("usize", Span::call_site()));
        } else if ty == "float" {
            punct.push_value(Ident::new("f32", Span::call_site()));
        }  else if ty == "double" {
            punct.push_value(Ident::new("f64", Span::call_site()));
        } else if ty == "void" {
            punct.push_value(Ident::new("ffi", Span::call_site()));
            punct.push(Ident::new("c_void", Span::call_site()));
        } else if ty == "char" {
            punct.push_value(Ident::new("ffi", Span::call_site()));
            punct.push(Ident::new("c_char", Span::call_site()));
        } else {
            let ty = ty.replace("FlagBits", "Flags");
            let ty = ty
                .trim_start_matches("Vk");
            let ident = syn::Ident::new(
                ty,
                Span::call_site()
            );
            if type_exclude_list.contains(&ident) {
                return None
            }
            if prefix_handle_vk &&
                ty == "Device" ||
                ty == "Instance"
            {
                punct.push_value(Ident::new("crate", Span::call_site()));
                punct.push(Ident::new("vk", Span::call_site()));
                punct.push(ident);
            } else {
                punct.push_value(ident);
            }
        }
        Some(punct)
    };
    println!("lifetime resolve depth: {depth}");
    depth = 0;
    loop {
        depth += 1;
        let new: Vec<_> = struct_defs
            .iter()
            .filter_map(|def| {
                let name = def.name.as_ref().unwrap();
                if def.is_union {
                    return Some((name, false))
                }
                let (mut derive_eq, mut derive_partial_eq) =
                    if let Some(partial) = non_eq_types.get(name) {
                        if !partial {
                            return None
                        }
                        (false, true)
                    } else {
                        (true, true)
                    };
                for member in &def.members {
                    let ty_str = member.ty.as_ref().unwrap().to_string();
                    if ty_str.starts_with("PFN") {
                        return Some((name, false))
                    }
                    let resolved_ty = resolve_ty(&ty_str, false)?;
                    let member_ty = resolved_ty.iter().next().unwrap();
                    if let Some(partial) = non_eq_types.get(member_ty) {
                        derive_eq = false;
                        if !partial {
                            derive_partial_eq = false;
                        }
                    }
                    if !derive_partial_eq {
                        break;
                    }
                }
                if derive_eq {
                    return None
                }
                Some((name, derive_partial_eq))
            }).collect();
        let mut changed = 0;
        for (name, partial) in new {
            non_eq_types.entry(name.clone())
                .and_modify(|p| {
                    if !partial {
                        if *p {
                            changed += 1;
                        }
                        *p = false;
                    }
                }).or_insert_with(|| {
                    changed += 1;
                    partial
                });
        }
        if changed == 0 {
            break
        }
    }
    println!("eq resolve depth: {depth}");
    let mut defined_aliases = HashSet::new();  
    for def in struct_defs {
        let name = def.name.as_ref().unwrap();
        if type_exclude_list.contains(name) {
            continue
        }
        let mut field_defs = vec![];
        let mut field_defaults = vec![];
        let mut field_setters = vec![];
        let mut field_getters = vec![];
        let mut prev_field = None;
        let mut has_pointer = false;
        let mut p_next_mutability = None;
        let mut needs_lifetime = def.needs_lifetime;
        let mut s_type = None;
        let mut derive_default = true;
        let (derive_eq, derive_partial_eq) =
            non_eq_types.get(name).map(|&partial| (false, partial))
            .unwrap_or((true, true));
        for member in &def.members {
            let ty_str = member.ty.as_ref().unwrap().to_string();
            let resolved_ty = resolve_ty(&ty_str, false).unwrap();
            let member_ty = resolved_ty.iter().next().unwrap();
            if not_send_sync_types.contains(member_ty) {
                has_pointer = true;
            }
            let pointer = member.ptr_type;
            if let Some(prev_field) = prev_field.take() &&
                prev_field == member.name
            {
                continue
            }
            prev_field = Some(member.name.clone());
            if member.name == "p_next" {
                p_next_mutability = match member.ptr_type {
                    PtrType::ConstPtr => Some(false),
                    PtrType::MutPtr => Some(true),
                    _ => unreachable!(),
                };
            }
            let lifetime = (type_needs_lifetime.contains(member_ty)).then(||
            {
                needs_lifetime = true;
                quote! { <'a> }
            });
            let array;
            let ty =
                if let Some(len) = &member.array_len {
                    let len =
                        if len.starts_with(char::is_numeric) {
                            let len: usize = str::parse(len).unwrap();
                            quote! { #len }
                        } else {
                            let len = Ident::new(len, Span::call_site());
                            quote! { #len as _ }
                        };
                    let q = quote! {
                        [#pointer #resolved_ty #lifetime; #len]
                    };
                    array = Some(len);
                    q
                } else {
                    array = None;
                    quote! {
                        #pointer #resolved_ty #lifetime
                    }
                };
            let has_s_type =
                if let Some(value) = &member.value {
                    let ident = Ident::new(
                        value.trim_start_matches("VK_STRUCTURE_TYPE_"),
                        Span::call_site()
                    );
                    s_type = Some(ident);
                    true
                } else {
                    false
                };
            let doc = member.comment
                .as_ref()
                .and_then(|comment| {
                    (!comment.contains("{")).then_some(comment)
                })
                .map(|comment| {
                    quote! {#[doc = #comment]}
                });
            let deprecated = member.deprecated
                .as_ref()
                .map(|reason| {
                    quote! {#[deprecated = #reason]}
                });
            let name = if member.name == "type" {
                "ty"
            } else { &member.name };
            let name = Ident::new(name, Span::call_site());
            field_defs.push(
                quote! {
                    #doc
                    #deprecated
                    pub #name: #ty,
                }
            );
            if !def.is_union {
                if deprecated.is_none() {
                    field_setters.push(
                        if member_ty == "Bool32" &&
                            member.ptr_type == PtrType::None
                        {
                            quote! {
                                #[inline]
                                pub const fn #name(mut self, value: bool) -> Self {
                                    self.#name = value as Bool32;
                                    self
                                }
                            }
                        } else if resolved_ty.iter().any(|ty| ty == "c_char") &&
                            member.array_len.is_some()
                        {
                            let getter_name = Ident::new(
                                &format!("{name}_as_cstr"),
                                Span::call_site(),
                            );
                            field_getters.push(quote! {
                                #[inline]
                                pub fn #getter_name(
                                    &self
                                ) -> ::core::result::Result<&ffi::CStr, ffi::FromBytesUntilNulError>
                                {
                                    c_char_slice_until_nul(&self.#name)
                                }
                            });
                            quote! {
                                #[inline]
                                pub fn #name(
                                    mut self,
                                    value: &ffi::CStr
                                ) -> ::core::result::Result<Self, WriteCStrToArrayError> {
                                    write_c_str_to_c_char_slice(
                                        value,
                                        &mut self.#name,
                                    )?;
                                    Ok(self)
                                }
                            }
                        } else {
                            quote! {
                                #[inline]
                                pub const fn #name(mut self, value: #ty) -> Self {
                                    self.#name = value;
                                    self
                                }
                            }
                        }
                    );
                }
                field_defaults.push(
                    if has_s_type {
                        derive_default = false;
                        quote! {
                            #name: Self::S_TYPE,
                        }
                    } else if let Some(array) = array {
                        derive_default = false;
                        quote! {
                            #name: [#resolved_ty::default(); #array],
                        }
                    } else {
                        match member.ptr_type {
                            PtrType::None => {
                                if ty_str.starts_with("PFN") {
                                    derive_default = false;
                                }
                                quote! {
                                    #name: #resolved_ty::default(),
                                }
                            },
                            PtrType::MutPtr | PtrType::MutPtrMutPtr => {
                                has_pointer = true;
                                quote! {
                                    #name: ::core::ptr::null_mut(),
                                }
                            },
                            PtrType::ConstPtr | PtrType::ConstPtrConstPtr => {
                                has_pointer = true;
                                quote! {
                                    #name: ::core::ptr::null(),
                                }
                            },
                        }
                    }
                );
            }
        }
        if !derive_eq {
            non_eq_types.insert(name.clone(), derive_partial_eq);
        }
        let doc = doc_link(&format!("Vk{name}"))
            .map(|link| quote! { #[doc = #link] });
        if !def.is_union {
            let mut extends_trait = None;
            let push_next = struct_extends
                .contains(name)
                .then(|| {
                    let p_next_mutability = if p_next_mutability.unwrap() {
                        quote! { mut }
                    } else { quote! { const } };
                    let trait_name = Ident::new(
                        &format!("Extends{name}"),
                        Span::call_site(),
                    );
                    let doc = format!("A trait for structures that extend {name}.");
                    let doc = quote! {
                        #[doc = #doc]
                        ///
                        /// # Safety
                        /// A struct implementing this trait *must* adhere to the memory layout of
                        /// [`BaseOutStructure`].
                    };
                    extends_trait = Some(quote! {
                        #doc
                        pub unsafe trait #trait_name: Chainable {}
                    });
                    quote! {

                        /// Pushes an item to the [`p_next`][1] chain of this structure.
                        ///
                        /// [1]: Self::p_next
                        #[inline]
                        pub fn push_next<T>(mut self, next: &'a mut T) -> Self
                            where T: ?Sized + #trait_name,
                        {
                            let p_next: *#p_next_mutability ffi::c_void = <*mut T>::cast(next);
                            let last = chain_out_iter(next).last().unwrap();
                            last.p_next = self.p_next as *mut BaseOutStructure<'a>;
                            self.p_next = p_next;
                            self
                        }
                    }
                });
            let lifetime = needs_lifetime.then(|| {
                field_defs.push(quote! {
                    pub _marker: PhantomData<&'a ()>,
                });
                field_defaults.push(quote! {
                    _marker: PhantomData,
                });
                quote! { <'a> }
            });
            let impl_send_sync = has_pointer.then(|| quote! {
                unsafe impl #lifetime Send for #name #lifetime {}
                unsafe impl #lifetime Sync for #name #lifetime {}
            });
            let impl_chainable = (s_type.is_some() && p_next_mutability.is_some()).then(||
                quote! {
                    unsafe impl<'a> Chainable for #name<'a> {

                        #[inline]
                        fn base_in(&self) -> &BaseInStructure<'_> {
                            unsafe {
                                &*<*const Self>::cast::<BaseInStructure>(self)
                            }
                        }
                    }
                }
            );
            let s_type = s_type.map(|s_type| quote! {
                pub const S_TYPE: StructureType = StructureType::#s_type;
            });
            let (derive_default, impl_default) = if derive_default {
                (quote! { Default, }, quote! {})
            } else {
                (quote! {}, quote! {

                    impl #lifetime Default for #name #lifetime {
                            
                        #[allow(deprecated)]
                        fn default() -> Self {
                            Self {
                                #(#field_defaults)*
                            }
                        }
                    }
                })
            };
            let impl_extends = def.extends
                .iter()
                .map(|extends| quote! {
                    unsafe impl<'a> #extends for #name<'a> {}
                });
            let derive_partial_eq = derive_partial_eq.then(|| quote! {
                PartialEq,
            });
            let derive_eq = derive_eq.then(|| quote! {
                Eq,
            });
            let def = quote! {
                #doc
                #[repr(C)]
                #[derive(#derive_default Clone, Copy, #derive_partial_eq #derive_eq)]
                pub struct #name #lifetime {
                    #(#field_defs)*
                }

                #extends_trait

                impl #lifetime #name #lifetime {
                    #s_type
                    #(#field_setters)*
                    #(#field_getters)*
                    #push_next
                }

                #impl_default

                #impl_chainable

                #(#impl_extends)*

                #impl_send_sync
            };
            write!(structs, "{def}")?;
        } else {
            let lifetime = needs_lifetime.then(|| {
                quote! { <'a> }
            });
            non_eq_types.insert(name.clone(), false);
            let def = quote! {
                #doc
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub union #name #lifetime {
                    #(#field_defs)*
                }

                impl #lifetime Default for #name #lifetime {

                    #[inline]
                    fn default() -> Self {
                        unsafe { ::core::mem::zeroed() }
                    }
                }
            };
            write!(unions, "{def}")?;
        }
    }
    let mut pfn_duplicates = HashSet::new();
    for (name, func) in funcpointers {
        let ret_ptr = func.ret_ptr;
        let ty = if ret_ptr == PtrType::None && func.ret_ty == "void" {
            quote! {}
        } else {
            let Some(ty) = resolve_ty(&func.ret_ty, false) else {
                continue
            };
            quote! {
                ->  #ret_ptr #ty
            }
        };
        let name = Ident::new(&name, Span::call_site());
        let mut skip_sc = false;
        let mut field_tys = vec![];
        let mut params_ignored = vec![];
        let params: Vec<_> = func.params
            .iter()
            .map(|param| {
                let Some(ty) = resolve_ty(&param.ty, false) else {
                    skip_sc = true;
                    return quote! {}
                };
                let name = lower_snake_case(&param.name);
                let name = Ident::new(&name, Span::call_site());
                let ty = match param.ptr {
                    FuncParamPtrType::PtrType(ptr) => quote! { #ptr #ty },
                    FuncParamPtrType::Array(len) => quote! { *const [#ty; #len] },
                };
                field_tys.push(ty.to_string());
                let ignored_name = Ident::new(&format!("_{name}"), Span::call_site());
                params_ignored.push(
                    quote! {
                        #ignored_name: #ty,
                    }
                );
                quote! {
                    #name: #ty,
                }
            }).collect();
        let pfn_default = pfn_duplicates.insert(field_tys).then(|| quote! {
            impl PFN_Default for #name {

                #[inline]
                fn default() -> Self {
                    unsafe extern "system" fn f(
                        #(#params_ignored)*
                    ) #ty {
                        panic!(concat!(
                            stringify!(#name),
                            " not loaded"
                        ))
                    }
                    f
                }
            }
        });
        let def = quote! {
            #[allow(non_camel_case_types)]
            pub type #name = unsafe extern "system" fn(
                #(#params)*
            ) #ty; 

            #pfn_default
        };
        if skip_sc {
            continue
        }
        write!(type_defs, "{def}")?;
    }
    let mut core_commands = IndexSet::new();
    let mut instance_commands = IndexSet::new();
    let mut device_commands = IndexSet::new(); 
    for (name, command) in &mut commands {
        println!("processing function {name}");
        let extension =
            extensions.iter()
                .find(|(_, ext)| 
                    ext.commands
                    .iter()
                    .any(|cmd| cmd.name == *name)
                );
        let mut fp = Ident::new("fp", Span::call_site());
        let doc = if let Some(version) = &command.version {
            let doc = format!("Provided by Vulkan version {version}.");
            fp = Ident::new(&format!("fp_v{}", version.replace(".", "")), Span::call_site());
            quote! {#[doc = #doc]}
        } else if let Some((ext, _)) = &extension {
            let doc = format!("Provided by {ext}.");
            quote! {#[doc = #doc]}
        } else { quote! {} };
        let CommandBody::Func(func) = &mut command.body else {
            continue
        };
        let extension_type =
            extension
                .as_ref()
                .map(|(_, ext)| {
                    ext.ty.as_str()
                }).unwrap_or_default();
        let ret_ptr = func.ret_ptr;
        let ret_ty = if ret_ptr == PtrType::None && func.ret_ty == "void" {
            None
        } else {
            let Some(ty) = resolve_ty(&func.ret_ty, true) else {
                continue
            };
            Some(ty)
        };
        let pfn_name = format!("PFN_{name}");
        let is_aliased = type_aliases
            .values()
            .any(|name| *name == pfn_name);
        let pfn_name = Ident::new(&pfn_name, Span::call_site());
        let mut skip = false;
        enum DispatchableType {
            Instance {
                optional: bool,
            },
            Device,
        }
        let mut dispatchable_handle = None;
        let mut params_ignored = vec![];
        enum RsTy {
            Value(Punctuated<Ident, Token![::]>),
            ConstRef(Punctuated<Ident, Token![::]>, bool),
            QueryLen(Punctuated<Ident, Token![::]>, bool),
            MutRef(Punctuated<Ident, Token![::]>, bool),
            MutPtr(Punctuated<Ident, Token![::]>),
            ReturnType(PtrType, Punctuated<Ident, Token![::]>),
            ConstSliceConstRef(Punctuated<Ident, Token![::]>, bool),
            Array(Punctuated<Ident, Token![::]>, usize, bool),
            CStr,
        }
        let success_filter = (func.ret_ty == "VkResult")
            .then(|| {
                if command.success_codes.is_empty() {
                    println!("no success codes for {name}");
                    skip = true;
                }
                let filter = command.success_codes
                    .iter()
                    .map(|code|
                        Ident::new(code.trim_start_matches("VK_"), Span::call_site())
                    );
                quote! {
                    &[
                        #(crate::vk::Result::#filter),*
                    ]
                }
            });
        let mut param_names_and_tys = vec![];
        let params: Vec<_> = func.params
            .iter_mut()
            .enumerate()
            .map(|(i, param)| {
                if dispatchable_handles.contains(param.ty.trim_start_matches("Vk")) &&
                    i == 0
                {
                    if param.ty == "VkInstance" || param.ty == "VkPhysicalDevice" {
                        dispatchable_handle = Some(DispatchableType::Instance {
                            optional: matches!(param.optional, ParamOptional::True),
                        });
                    } else {
                        if matches!(param.optional, ParamOptional::True) {
                            println!("optional {} for {name}", param.ty);
                        }
                        dispatchable_handle = Some(DispatchableType::Device);
                    }
                }
                let Some(ty) = resolve_ty(&param.ty, true) else {
                    skip = true;
                    return quote! {}
                };
                let mut name = lower_snake_case(&param.name);
                if name == "type" {
                    name = "ty".to_string();
                }
                let name = Ident::new(name.trim_start_matches("p_"), Span::call_site());
                let has_lifetime = ty.iter().any(|ident| type_needs_lifetime.contains(ident));
                let (ty, rs_ty) = match param.ptr {
                    FuncParamPtrType::PtrType(ptr) =>
                        (quote! { #ptr #ty },
                            match ptr {
                                PtrType::None => RsTy::Value(ty),
                                PtrType::ConstPtr =>
                                    if ty.iter().any(|ident| ident == "c_char") {
                                        RsTy::CStr
                                    } else {
                                        RsTy::ConstRef(ty, has_lifetime)
                                    },
                                PtrType::MutPtr =>
                                    if opaque_handles.contains(ty.iter().next().unwrap()) {
                                        RsTy::MutPtr(ty)
                                    } else if param.optional == ParamOptional::True {
                                        if param.has_len.is_some()
                                        {
                                            RsTy::QueryLen(ty, has_lifetime)
                                        } else {
                                            RsTy::MutRef(ty, has_lifetime)
                                        }
                                    } else if param.name == "pMetricValues" {
                                        param.has_len = Some(Some(3));
                                        RsTy::QueryLen(ty, has_lifetime)
                                    } else if has_lifetime ||
                                        (success_filter.is_none() && param.optional == ParamOptional::True) ||
                                        param.has_len.is_some()
                                    {
                                        RsTy::MutRef(ty, has_lifetime)
                                    } else {
                                        RsTy::ReturnType(PtrType::None, ty)
                                    },
                                PtrType::ConstPtrConstPtr => RsTy::ConstSliceConstRef(ty, has_lifetime),
                                PtrType::MutPtrMutPtr => 
                                    RsTy::ReturnType(PtrType::MutPtr, ty),
                            }
                        ),
                    FuncParamPtrType::Array(len) =>
                        (quote! { *const [#ty; #len] }, RsTy::Array(ty, len, has_lifetime)),
                };
                let ignored_name = Ident::new(&format!("_{name}"), Span::call_site());
                params_ignored.push(
                    quote! {
                        #ignored_name: #ty,
                    }
                );
                let res = quote! {
                    #name: #ty,
                };
                param_names_and_tys.push((name, rs_ty));
                res
            }).collect();
        if let Some(dispatchable_handle) = &dispatchable_handle {
            match dispatchable_handle {
                DispatchableType::Instance { optional } => {
                    instance_commands.insert(name.clone());
                    if *optional {
                        core_commands.insert(name.clone());
                    }
                },
                DispatchableType::Device => {
                    device_commands.insert(name.clone());
                },
            }
        } else {
            core_commands.insert(name.clone());
        }
        if skip {
            println!("skipping {name}");
            continue
        }
        let pfn_ret = ret_ty.as_ref().map(|ty| quote! {
            -> #ret_ptr #ty
        });
        let name_rs = &command.name_rs;
        let def = quote! {
            #doc
            #[allow(non_camel_case_types)]
            pub type #pfn_name = unsafe extern "system" fn(
                #(#params)*
            ) #pfn_ret;
        };
        let pfn_default = quote! {
            unsafe extern "system" fn #name_rs (
                #(#params_ignored)*
            ) #pfn_ret {
                panic!(concat!(
                    "failed to load ",
                    stringify!(#name_rs),
                ))
            }
        };
        let pfn_def = quote! {
            pub #name_rs: #pfn_name
        };
        let mut fn_params = vec![Some(quote! { &self, })];
        let mut fn_maps = vec![];
        let mut get_lens = vec![];
        let mut rs_def_get_data = None;
        let mut ret =
            if success_filter.is_none() {
                pfn_ret.clone()
            } else {
                None
            };
        let mut with_result = None;
        let mut query_len = None;
        for (idx, param) in func.params.iter().enumerate() {
            let (name, ty) = &param_names_and_tys[idx];
            match ty {
                RsTy::Value(ty) => {
                    if (name == "instance" &&
                        (extension_type == "instance" || extension_type.is_empty())) || 
                        (name == "device" && (extension_type == "device" || extension_type.is_empty()))
                    {
                        fn_params.push(None);
                        fn_maps.push(quote! {
                            self.handle,
                        });
                    } else {
                        if ty.iter().next().unwrap() == "Bool32" {
                            fn_params.push(Some(quote! {
                                #name: bool,
                            }));
                            fn_maps.push(quote! {
                                #name as Bool32,
                            });
                        } else {
                            fn_params.push(Some(quote! {
                                #name: #ty,
                            }));
                            fn_maps.push(quote! {
                                #name,
                            });
                        }
                    }
                },
                RsTy::ConstRef(ty, has_lifetime) => {
                    let lifetime = has_lifetime.then(|| quote! { <'_> });
                    match param.optional {
                        ParamOptional::False | ParamOptional::FalseTrue => {
                            if let Some(len) = param.has_len {
                                if let Some(len) = len {
                                    let (ty, cast_ptr) = if ty.iter().any(|ident| ident == "c_void") {
                                        (quote! { u8 }, Some(quote! { .cast() }))
                                    } else {
                                        (quote! { #ty }, None)
                                    };
                                    if get_lens.contains(&len) {
                                        fn_params.push(Some(quote! {
                                            #name: &[#ty #lifetime],
                                        }));
                                        fn_maps.push(quote! {
                                            if !#name.is_empty() {
                                                #name.as_ptr()#cast_ptr
                                            } else { ::core::ptr::null() },
                                        });
                                    } else {
                                        get_lens.push(len);
                                        fn_params[len as usize + 1] = None;
                                        fn_params.push(Some(quote! {
                                            #name: &[#ty #lifetime],
                                        }));
                                        fn_maps[len as usize] = quote! {
                                            #name.len() as _,
                                        };
                                        fn_maps.push(quote! {
                                            if !#name.is_empty() {
                                                #name.as_ptr()#cast_ptr
                                            } else { ::core::ptr::null() },
                                        });
                                    }
                                } else {
                                    fn_params.push(Some(quote! {
                                        #name: &[#ty #lifetime],
                                    }));
                                    fn_maps.push(quote! {
                                        #name.as_ptr(),
                                    });
                                }
                            } else {
                                fn_params.push(Some(quote! {
                                    #name: &#ty #lifetime,
                                }));
                                fn_maps.push(quote! {
                                    #name,
                                });
                            }
                        },
                        ParamOptional::True => {
                            if param.has_len.is_some() {
                                fn_params.push(Some(quote! {
                                    #name: Option<&[#ty #lifetime]>,
                                }));
                                fn_maps.push(quote! {
                                    #name.and_then(|s|
                                        (!s.is_empty()).then_some(s.as_ptr())
                                    ).unwrap_or_default(),
                                });
                            } else {
                                fn_params.push(Some(quote! {
                                    #name: Option<&#ty #lifetime>,
                                }));
                                fn_maps.push(quote! {
                                    #name.as_ptr(),
                                });
                            }
                        },
                    };
                },
                RsTy::QueryLen(ty, has_lifetime) => {
                    if idx != func.params.len() - 1 {
                        assert!(idx == func.params.len() - 2);
                        assert!(query_len.is_none());
                        query_len = Some(idx);
                    } else {
                        let lifetime = has_lifetime.then(|| quote! { <'_> });
                        let len = param.has_len.unwrap().unwrap();
                        let mut fn_params2 = fn_params.clone();
                        let mut fn_maps2 = fn_maps.clone();
                        let mut out_name = Ident::new("out", Span::call_site());
                        let mut get_len_name = out_name.clone();
                        if let Some(idx) = query_len {
                            let first = &param_names_and_tys[idx];
                            let name1 = Ident::new(&format!("out_{}", first.0), Span::call_site());
                            get_len_name = name1.clone();
                            let RsTy::QueryLen(ty1, has_lifetime1) = &first.1 else {
                                unreachable!()
                            };
                            let lifetime1 = has_lifetime1.then(|| quote! { <'_> });
                            out_name = Ident::new(&format!("out_{name}"), Span::call_site());
                            fn_params2[len as usize + 1] = Some(quote! {
                                #name1: &mut [#ty1 #lifetime1],
                            });
                            fn_params2.push(Some(quote! {
                                #out_name: &mut [#ty #lifetime],
                            }));
                            fn_maps2[len as usize] = quote! { &mut len, };
                            fn_maps2[idx] = quote! {
                                #name1.as_mut_ptr(),
                            };
                            fn_maps2.push(quote! {
                                #out_name.as_mut_ptr(),
                            });
                        } else {
                            let (ty, cast) = if ty.iter().any(|ident| ident == "c_void") {
                                (quote! { u8 }, Some(quote! { .cast() }))
                            } else {
                                (quote! { #ty }, None)
                            };
                            fn_params2[len as usize + 1] = Some(quote! {
                                out: &mut [#ty #lifetime],
                            });
                            fn_maps2[len as usize] = quote! { &mut len, };
                            fn_maps2.push(quote! {
                                out.as_mut_ptr()#cast,
                            });
                        }
                        let mut ret = pfn_ret.clone();
                        let with_result = if let Some(filter) = &success_filter {
                            ret = Some(quote! {
                                -> crate::VkResult<()>
                            });
                            Some(quote! {
                                .result(#filter)
                            })
                        } else { None };
                        rs_def_get_data = Some(quote! {
                            (#(#fn_params2)*) #ret {
                                unsafe {
                                    let mut len = #get_len_name.len() as _;
                                    (self.#fp.#name_rs)(
                                        #(#fn_maps2)*
                                    )#with_result
                                }
                            }
                        });
                    }
                    fn_maps.push(quote! {
                        core::ptr::null_mut(),
                    });
                },
                RsTy::MutRef(ty, has_lifetime) => {
                    assert!(param.optional != ParamOptional::FalseTrue);
                    let lifetime = has_lifetime.then(|| quote! { <'_> });
                    if param.has_len.is_some() {
                        if param.optional == ParamOptional::False {
                            fn_params.push(Some(quote! {
                                #name: &mut [#ty #lifetime],
                            }));
                            fn_maps.push(quote! {
                                #name.as_mut_ptr(),
                            });
                        } else {
                            unreachable!()
                        }
                    } else {
                        if param.optional == ParamOptional::False {
                            fn_params.push(Some(quote! {
                                #name: &mut #ty #lifetime,
                            }));
                            fn_maps.push(quote! {
                                #name,
                            });
                        } else {
                            fn_params.push(Some(quote! {
                                #name: Option<&mut #ty #lifetime>,
                            }));
                            fn_maps.push(quote! {
                                #name.as_ptr(),
                            });
                        }
                    } 
                },
                RsTy::ReturnType(ptr, ty) => {
                    assert!(ret.is_none());
                    (ret, with_result) = if let Some(filter) = &success_filter {
                        if ty.iter().next().unwrap() == "Bool32" {
                            assert!(matches!(ptr, PtrType::None));
                            let ret = quote! {
                                -> crate::VkResult<bool>
                            };
                            let with_result = (name.clone(), quote! {
                                .result_with(#filter, || {
                                    #name.assume_init() != 0
                                })
                            });
                            (Some(ret), Some(with_result))
                        } else {
                            let ret = quote! {
                                -> crate::VkResult<#ptr #ty>
                            };
                            let with_result = (name.clone(), quote! {
                                .result_with_assume_init(#filter, #name)
                            });
                            (Some(ret), Some(with_result))
                        }
                    } else {
                        let ret = quote! {
                            -> #ptr #ty
                        };
                        (Some(ret), Some((name.clone(), quote! { ; #name.assume_init() })))
                    };
                    fn_params.push(None);
                    fn_maps.push(quote! {
                        #name.as_mut_ptr(),
                    });
                },
                RsTy::ConstSliceConstRef(ty, has_lifetime) => {
                    assert!(param.optional == ParamOptional::False);
                    let lifetime = has_lifetime.then(|| quote! {
                        <'_>
                    });
                    let len = param.has_len.unwrap().unwrap();
                    let name = Ident::new(
                        name.to_string().trim_start_matches("pp_"),
                        Span::call_site(),
                    );
                    if get_lens.contains(&len) {
                        fn_params.push(Some(quote! {
                            #name: &[&#ty #lifetime],
                        }));
                        fn_maps.push(quote! {
                            #name.as_ptr().cast(),
                        });
                    } else {
                        get_lens.push(len);
                        fn_params[len as usize + 1] = None;
                        fn_params.push(Some(quote! {
                            #name: &[&#ty #lifetime],
                        }));
                        fn_maps[len as usize] = quote! {
                            #name.len() as _,
                        };
                        fn_maps.push(quote! {
                            #name.as_ptr().cast(),
                        });
                    }
                },
                RsTy::Array(ty, len, has_lifetime) => {
                    assert!(param.optional == ParamOptional::False);
                    let lifetime = has_lifetime.then(|| quote! {
                        <'_>
                    });
                    fn_params.push(Some(quote! {
                        #name: &[#ty #lifetime; #len],
                    }));
                    fn_maps.push(quote! {
                        #name,
                    });
                },
                RsTy::CStr => {
                    match param.optional {
                        ParamOptional::False => {
                            fn_params.push(Some(quote! {
                                #name: &ffi::CStr,
                            }));
                            fn_maps.push(quote! {
                                #name.as_ptr(),
                            });
                        },
                        ParamOptional::True => {
                            fn_params.push(Some(quote! {
                                #name: Option<&ffi::CStr>,
                            }));
                            fn_maps.push(quote! {
                                 #name
                                    .map(|#name| #name.as_ptr())
                                    .unwrap_or_default(),
                            });
                        },
                        _ => unreachable!(),
                    }
                },
                RsTy::MutPtr(ty) => {
                    fn_params.push(Some(quote! {
                        #name: *mut #ty,
                    }));
                    fn_maps.push(quote! {
                        #name,
                    });
                },
            }
        }
        let (uninit, mut with_result) =
            if let Some((name, with_result)) = with_result {
                (
                    Some(quote! {
                        let mut #name = ::core::mem::MaybeUninit::uninit();
                    }),
                    Some(with_result),
                )
            } else {
                (None, None)
            };
        if ret.is_none() && let Some(filter) = &success_filter {
            ret = Some(quote! {
                -> crate::VkResult<()>
            });
            with_result = Some(quote! {
                .result(#filter)
            });
        }
        let rs_def = quote! {
            (#(#fn_params)*) #ret {
                unsafe {
                    #uninit
                    (self.#fp.#name_rs)(
                        #(#fn_maps)*
                    )#with_result
                }
            }
        };
        let rs_def_ext = is_aliased.then(|| quote! {
            (#(#fn_params)*) #ret {
                unsafe {
                    #uninit
                    (self.fp.#name_rs)(
                        #(#fn_maps)*
                    )#with_result
                }
            }
        });
        let num_arguments = fn_params
            .iter()
            .filter(|arg| arg.is_some())
            .count() as u32 + 1;
        command.unresolved = Some(UnresolvedCmd {
            pfn_name,
            original_name: name_rs.clone(),
            pfn_def,
            pfn_default,
            rs_def,
            rs_def_ext,
            rs_def_get_data,
            num_arguments,
        });
        write!(type_defs, "{def}")?;
    }
    let type_aliases = type_aliases
        .iter()
        .filter_map(|(name, alias)| {
            let doc = doc_link(name)
                .map(|link| quote! { #[doc = #link] });
            let name_str = name.trim_start_matches("Vk").replace("FlagBits", "Flags");
            if defined_aliases.contains(&name_str) {
                return None
            }
            let non_camel = name_str.contains('_').then(|| quote! {
                #[allow(non_camel_case_types)]
            });
            let name = Ident::new(&name_str, Span::call_site());
            defined_aliases.insert(name_str);
            let alias_str = &alias.trim_start_matches("Vk").replace("FlagBits", "Flags");
            let alias = Ident::new(alias_str, Span::call_site());
            if type_needs_lifetime.contains(&alias) {
                type_needs_lifetime.insert(alias.clone());
                Some(quote! {
                    #doc
                    pub type #name<'a> = #alias<'a>;
                })
            } else {
                Some(quote! {
                    #doc
                    #non_camel
                    pub type #name = #alias;
                })
            }
        });
    write!(type_defs, "{}", quote! {
        #(#type_aliases)*
    })?;
    #[derive(Debug)]
    struct MissingFnInfo {
        name: String,
        promoted_to_core: String,
    }
    let mut missing_fns = vec![];
    let mut found_core_tables = HashSet::new(); 
    let mut ext_fns = HashSet::new();
    let mut extensions_by_prefix: IndexMap<&str, String> = IndexMap::new();
    extensions.get_mut("VK_KHR_swapchain").unwrap()
        .commands
        .retain(|cmd| cmd.name != "vkGetPhysicalDevicePresentRectanglesKHR");
    extensions.get_mut("VK_KHR_surface").unwrap()
        .commands
        .push(ExtCommand {
            name: "vkGetPhysicalDevicePresentRectanglesKHR".to_string(),
            depends_on: vec!["VK_KHR_swapchain".to_string()],
        });
    for (name, ext) in &extensions {
        let prefix = name.trim_start_matches("VK_");
        let idx = prefix.find("_").unwrap();
        let (prefix, mut mod_name) = prefix.split_at(idx);
        let tmp = mod_name.trim_start_matches("_");
        if !tmp.starts_with(char::is_numeric) {
            mod_name = tmp;
        }
        let buffer = extensions_by_prefix.entry(prefix)
            .or_insert_with(||
                format!("pub mod {} {{ use super::*;", prefix.to_lowercase())
            );
        buffer.push_str(&format!("pub mod {mod_name} {{"));
        let promoted_to_core =
            if let Some(promoted_to) = &ext.promoted_to &&
                let Some(feature) = features.get_mut(promoted_to)
            {
                println!("promoting ext {} to core {}", name, promoted_to);
                let mut missing = vec![];
                for command in &ext.commands {
                    let core_cmd = command.name.replace(prefix, "");
                    if let Some(counter_part) = feature.commands
                        .get_mut(&core_cmd)
                    {
                        found_core_tables.insert(&command.name);
                        if let Some(counter_part) = counter_part
                        {
                            if counter_part.depends_on
                                .contains(name)
                            {
                                *counter_part = FeatureCmdCounterPart {
                                    ext_cmd: command.name.clone(),
                                    ext_name: name.clone(),
                                    depends_on: command.depends_on.clone(),
                                };
                            }
                        } else {
                            *counter_part = Some(FeatureCmdCounterPart {
                                ext_cmd: command.name.clone(),
                                ext_name: name.clone(),
                                depends_on: command.depends_on.clone(),
                            })
                        }
                    } else {
                        missing.push(command.clone());
                    }
                }
                for command in &missing {
                    missing_fns.push(MissingFnInfo {
                        name: command.name.clone(),
                        promoted_to_core: promoted_to.clone(),
                    });
                }
                true
            } else {
                false
            };
        let mut replace1 = name.to_uppercase();
        let mut replace2 = replace1.clone();
        if replace2.ends_with(char::is_numeric) {
            if replace2.ends_with("10") {
                replace2 = replace2.replace("10", "_10");
            } else {
                replace2.insert(replace2.len() - 1, '_');
            }
        }
        replace2 = replace2.replace("COLORSPACE", "COLOR_SPACE");
        replace1 = format!("{}_", replace1);
        replace2 = format!("{}_", replace2);
        let write_constants = |buffer: &mut String|  {
            for (name, constant) in &ext.new_constants {
                let name = name
                    .replace(&replace1, "")
                    .replace(&replace2, "")
                    .replace("EXTENSION_NAME", "NAME");
                let name = Ident::new(name.trim_start_matches("VK_"), Span::call_site());
                let def = match constant {
                    ExtConstant::U32(value) => quote!{
                        pub const #name: u32 = #value;
                    },
                    ExtConstant::String(value) => {
                        let literal = std::ffi::CString
                            ::from_str(value)
                            .unwrap();
                        quote! {
                            pub const #name: &ffi::CStr = #literal;
                        }
                    },
                };
                buffer.push_str(&format!("{def}"));
            }
            std::io::Result::Ok(())
        };
        match ext.ty.as_str() {
            "instance" => {
                buffer.push_str(&format!("//! {} instance extension.\n", name));
                if promoted_to_core {
                    buffer.push_str(&format!("//!\n//! Promoted to core {}.\n",
                        ext.promoted_to.as_ref().unwrap(),
                    ));
                    if !ext.commands.is_empty() {
                        buffer.push_str(
                            "//!\n//! See [`Instance`][1] for any commands included in this extension.\n",
                        );
                        buffer.push_str("//!\n//! [1]: crate::Instance\n");
                    }
                }
                buffer.push_str("use super::*;");
                write_constants(buffer)?;
                if !promoted_to_core {
                    let mut pfn_defs = vec![];
                    let mut load_fns = vec![];
                    let mut rs_defs = vec![];
                    for command in &ext.commands {
                        ext_fns.insert(&command.name);
                        let name_rs = Ident::new(&lower_snake_case(
                            command.name.trim_start_matches("vk")
                            .trim_end_matches(&prefix)
                        ), Span::call_site());
                        let cname = std::ffi::CString::from_str(&command.name).unwrap();
                        let link = doc_link(&command.name)
                            .map(|link| quote! {
                                /// # Vulkan docs
                                #[doc = #link]
                            });
                        let cmd = commands
                            .get(&command.name)
                            .unwrap();
                        let mut success_doc = vec![];
                        let mut success_doc_links = vec![];
                        for (i, code) in cmd.success_codes.iter().enumerate() {
                            let code = code.trim_start_matches("VK_");
                            success_doc.push(format!("* [`{code}`][{i}]"));
                            success_doc_links.push(format!("[{i}]: Result::{code}"));
                        }
                        let success_doc = if !success_doc.is_empty() {
                            Some(quote! {
                                #[doc = "# Success codes"]
                                #(#[doc = #success_doc])*
                            })
                        } else { None };
                        let doc = if !command.depends_on.is_empty() {
                            let dep = command.depends_on
                                .iter()
                                .enumerate()
                                .map(|(i, dep)| {
                                    if i != command.depends_on.len() - 1 {
                                        format!("* {dep} or")
                                    } else {
                                        format!("* {dep}")
                                    }
                                });
                            quote! {
                                /// # Depends on
                                #(#[doc = #dep])*
                                #success_doc
                                #fn_safety
                                #link
                                ///
                                #(#[doc = #success_doc_links])*
                            }
                        } else {
                            quote! {
                                #success_doc
                                #fn_safety
                                #link
                                ///
                                #(#[doc = #success_doc_links])*
                            }
                        };
                        let command = cmd;
                        let command = match &command.body {
                            CommandBody::Func(_) => command,
                            CommandBody::Alias(name) => {
                                let command = commands
                                    .get(name)
                                    .unwrap();
                                assert!(matches!(command.body, CommandBody::Func(_)));
                                command
                            }
                        };
                        let unresolved = command.unresolved
                            .as_ref().unwrap();
                        let ResolvedCmd {
                            pfn_def, pfn_default, rs_def, rs_def_ext, rs_def_get_data,
                            original_name,
                        } = unresolved
                            .resolve(name_rs.clone());
                        pfn_defs.push(pfn_def);
                        let rs_def = rs_def_ext.unwrap_or(rs_def);
                        let pfn = &unresolved.pfn_name;
                        let load_fn = quote! {
                            #original_name: unsafe {
                                #pfn_default
                                let f = f(#cname);
                                if f.is_null() {
                                    #original_name
                                } else {
                                    ::core::mem::transmute::<
                                        *const ffi::c_void,
                                        #pfn,
                                    >(f)
                                }
                            },
                        };
                        load_fns.push(load_fn);
                        let rs_def1 = quote! {
                            #doc
                            #rs_def
                        };
                        let rs_def2 = rs_def_get_data
                            .map(|def| quote! {
                                #doc
                                #def
                            });
                        rs_defs.push(quote! {
                            #rs_def1
                            #rs_def2
                        });
                    }
                    if !ext.commands.is_empty() {
                        let def = quote! {
                            #[derive(Clone, Copy)]
                            pub struct InstanceFp {
                                #(#pfn_defs),*
                            }
                            unsafe impl Send for InstanceFp {}
                            unsafe impl Sync for InstanceFp {}
                            impl InstanceFp {

                                pub fn load(
                                    f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void,
                                ) -> Self {
                                    Self {
                                        #(#load_fns)*
                                    }
                                }
                            }
                            #[derive(Clone)]
                            pub struct Instance<Ext = nop::Instance> {
                                handle: crate::vk::Instance,
                                fp: InstanceFp,
                                _ext: Ext,
                            }
                            impl Instance {
                                #[inline]
                                pub fn new<Ext>(instance: &crate::Instance<Ext>) -> Self {
                                    Self {
                                        handle: instance.handle(),
                                        fp: InstanceFp::load(
                                            &mut |cname| unsafe {
                                                instance.get_instance_proc_addr(cname)
                                                    as *const ffi::c_void
                                            },
                                        ),
                                        _ext: nop::Instance,
                                    }
                                }
                            }
                            impl<Ext> Instance<Ext> { 
                                #[inline]
                                pub fn handle(&self) -> crate::vk::Instance {
                                    self.handle
                                }
                                #[inline]
                                pub fn fp(&self) -> &InstanceFp {
                                    &self.fp
                                }
                                #(#rs_defs)*
                            }
                            #[cfg(feature = "ext-load-with")]
                            impl<Ext> LoadWith for Instance<Ext>
                                where Ext: LoadWith<Handle = crate::vk::Instance>
                            {
                                type Handle = crate::vk::Instance;
                                unsafe fn load_with(
                                    f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void,
                                    handle: Self::Handle
                                ) -> Self {
                                    Self {
                                        handle,
                                        fp: InstanceFp::load(f),
                                        _ext: unsafe {
                                            Ext::load_with(f, handle)
                                        },
                                    }
                                }
                            }
                            #[cfg(feature = "ext-load-with")]
                            impl<Ext> Deref for Instance<Ext> {
                                type Target = Ext;
                                fn deref(&self) -> &Ext {
                                    &self._ext
                                }
                            }
                        };
                        buffer.push_str(&def.to_string());
                    }
                } 
            },
            "device" => {
                buffer.push_str(&format!("//! {} device extension.\n", name));
                if promoted_to_core {
                    buffer.push_str(&format!("//!\n//! Promoted to core {}\n",
                        ext.promoted_to.as_ref().unwrap(),
                    ));
                    if !ext.commands.is_empty() {
                        buffer.push_str("//!\n//!See [`Device`][1] for any commands included in this extension.\n");
                        buffer.push_str("//!\n//![1]: crate::Device\n");
                    }
                }
                buffer.push_str("use super::*;");
                write_constants(buffer)?;
                if !promoted_to_core {
                    let mut pfn_defs = vec![];
                    let mut load_fns = vec![];
                    let mut rs_defs = vec![];
                    for command in &ext.commands {
                        ext_fns.insert(&command.name);
                        let name_rs = Ident::new(&lower_snake_case(
                            command.name.trim_start_matches("vk")
                            .trim_end_matches(&prefix)
                        ), Span::call_site());
                        let cname = std::ffi::CString::from_str(&command.name).unwrap();
                        let link = doc_link(&command.name)
                            .map(|link| quote! {
                                /// # Vulkan docs
                                #[doc = #link]
                            });
                        let cmd = commands
                            .get(&command.name)
                            .unwrap();
                        let mut success_doc = vec![];
                        let mut success_doc_links = vec![];
                        for (i, code) in cmd.success_codes.iter().enumerate() {
                            let code = code.trim_start_matches("VK_");
                            success_doc.push(format!("* [`{code}`][{i}]"));
                            success_doc_links.push(format!("[{i}]: Result::{code}"));
                        }
                        let success_doc = if !success_doc.is_empty() {
                            Some(quote! {
                                #[doc = "# Success codes"]
                                #(#[doc = #success_doc])*
                                ///
                                #(#[doc = #success_doc_links])*
                            })
                        } else { None };
                        let doc = if !command.depends_on.is_empty() {
                            let dep = command.depends_on
                                .iter()
                                .enumerate()
                                .map(|(i, dep)| {
                                    if i != command.depends_on.len() - 1 {
                                        format!("* {dep} or")
                                    } else {
                                        format!("* {dep}")
                                    }
                                });
                            quote! {
                                /// # Depends on
                                #(#[doc = #dep])*
                                #success_doc
                                #fn_safety
                                #link
                                ///
                                #(#[doc = #success_doc_links])*
                            }
                        } else {
                            quote! {
                                #success_doc
                                #fn_safety
                                #link
                                ///
                                #(#[doc = #success_doc_links])*
                            }
                        };
                        let command = cmd;
                        let command = match &command.body {
                            CommandBody::Func(_) => command,
                            CommandBody::Alias(name) => {
                                let command = commands
                                    .get(name)
                                    .unwrap();
                                assert!(matches!(command.body, CommandBody::Func(_)));
                                command
                            }
                        };
                        let unresolved = command.unresolved
                            .as_ref().unwrap();
                        let ResolvedCmd { 
                            pfn_def, pfn_default, rs_def, rs_def_ext, rs_def_get_data,
                            original_name,
                        } = unresolved
                            .resolve(name_rs.clone());
                        let rs_def = rs_def_ext.unwrap_or(rs_def);
                        pfn_defs.push(pfn_def);
                        let pfn = &unresolved.pfn_name;
                        let load_fn = quote! {
                            #original_name: unsafe {
                                #pfn_default
                                let f = f(#cname);
                                if f.is_null() {
                                    #original_name
                                } else {
                                    ::core::mem::transmute::<
                                        *const ffi::c_void,
                                        #pfn,
                                    >(f)
                                }
                            },
                        };
                        load_fns.push(load_fn);
                        let rs_def1 = quote! {
                            #doc
                            #rs_def
                        };
                        let rs_def2 = rs_def_get_data
                            .map(|def| quote! {
                                #doc
                                #def
                            });
                        rs_defs.push(quote! {
                            #rs_def1
                            #rs_def2
                        });
                    }
                    if !ext.commands.is_empty() {
                        let def = quote! {
                            #[derive(Clone, Copy)]
                            pub struct DeviceFp {
                                #(#pfn_defs),*
                            }
                            unsafe impl Send for DeviceFp {}
                            unsafe impl Sync for DeviceFp {}
                            impl DeviceFp {

                                pub fn load(
                                    f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void,
                                ) -> Self {
                                    Self {
                                        #(#load_fns)*
                                    }
                                }
                            }
                            #[derive(Clone)]
                            pub struct Device<Ext = nop::Device> {
                                handle: crate::vk::Device,
                                fp: DeviceFp,
                                _ext: Ext,
                            }
                            impl Device {
                                #[inline]
                                pub fn new<Ext>(device: &crate::Device<Ext>) -> Self {
                                    Self {
                                        handle: device.handle(),
                                        fp: DeviceFp::load(
                                            &mut |cname| unsafe {
                                                device.get_device_proc_addr(cname)
                                                    as *const ffi::c_void
                                            },
                                        ),
                                        _ext: nop::Device,
                                    }
                                }
                            }
                            impl<Ext> Device<Ext> {
                                #[inline]
                                pub fn handle(&self) -> crate::vk::Device {
                                    self.handle
                                }
                                #[inline]
                                pub fn fp(&self) -> &DeviceFp {
                                    &self.fp
                                }
                                #(#rs_defs)*
                            }
                            #[cfg(feature = "ext-load-with")]
                            impl<Ext> LoadWith for Device<Ext>
                                where Ext: LoadWith<Handle = crate::vk::Device>
                            {
                                type Handle = crate::vk::Device;
                                unsafe fn load_with(
                                    f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void,
                                    handle: Self::Handle
                                ) -> Self {
                                    Self {
                                        handle,
                                        fp: DeviceFp::load(f),
                                        _ext: unsafe {
                                            Ext::load_with(f, handle)
                                        },
                                    }
                                }
                            }
                            #[cfg(feature = "ext-load-with")]
                            impl<Ext> Deref for Device<Ext> {
                                type Target = Ext;
                                fn deref(&self) -> &Ext {
                                    &self._ext
                                }
                            }
                        };
                        buffer.push_str(&def.to_string());
                    }
                }
            },
            _ => unreachable!(),
        }
        buffer.push('}');
    }
    let mut extension_gen = File::create("../../src/extension_gen.rs")?;
    write!(extension_gen,
"use core::ffi; use crate::vk::*;
use crate::{{nop, PtrOption}};
#[cfg(feature = \"ext-load-with\")]
use {{
    core::ops::Deref,
    crate::LoadWith,
}};
")?;
    for buffer in extensions_by_prefix.values() {
        write!(extension_gen, "{buffer} }}")?;
    }
    missing_fns.retain(|f|
        !ext_fns.contains(&f.name) &&
        !found_core_tables.contains(&f.name)
    );
    for f in missing_fns {
        println!("adding {} to core {}", f.name, f.promoted_to_core);
    }
    let mut core_gen = File::create("../../src/core_gen.rs")?;
    writeln!(core_gen, "use core::ffi; use crate::vk::*; use crate::PtrOption;")?;
    struct CoreFn {
        name_rs: Ident,
        core: ResolvedCmd,
        load_fn: TokenStream,
        ext_counter_part: Option<TokenStream>,
    }
    instance_commands.insert("vkGetDeviceProcAddr".to_string());
    for (cmd, table, handle, filter, filter_rs, has_ext) in [
        (core_commands, "LibraryFp", quote!{ Library },
            ["vkGetInstanceProcAddr"].as_slice(),
            ["create_instance", "destroy_instance", "enumerate_instance_version"].as_slice(),
            false,
        ),
        (instance_commands, "InstanceFp", quote!{ Instance },
            [].as_slice(),
            ["create_device", "destroy_instance", "get_device_proc_addr"].as_slice(),
            true,
        ),
        (device_commands, "DeviceFp", quote!{ Device },
            [].as_slice(),
            ["destroy_device"].as_slice(),
            true,
        ),
    ] {
        let mut version_tables: Vec<(u32, Vec<CoreFn>)> = vec![];
        for name in &cmd {
            if filter.contains(&name.as_str()) {
                continue
            }
            let command = commands
                .get(name)
                .unwrap();
            if let Some(version) = command.version
                .as_ref()
            {
                let version_minor: u32 = str::parse(version.split_at(2).1).unwrap();
                let version = format!("VK_VERSION_{}", version.replace('.', "_"));
                let feature = features
                    .get(&version).unwrap();
                let name_rs = &command.name_rs;
                let unresolved = command.unresolved
                    .as_ref()
                    .unwrap();
                let ResolvedCmd { 
                    pfn_def, pfn_default, mut rs_def, mut rs_def_get_data,
                    original_name, rs_def_ext,
                } = unresolved
                    .resolve(name_rs.clone());
                let counter_part = feature.commands
                    .get(name)
                    .unwrap();
                let doc_link = doc_link(name)
                    .map(|link| quote! { #[doc =  #link]});
                let mut success_doc = vec![];
                let mut success_doc_links = vec![];
                for (i, code) in command.success_codes.iter().enumerate() {
                    let code = code.trim_start_matches("VK_");
                    success_doc.push(format!("* [`{code}`][{i}]"));
                    success_doc_links.push(format!("[{i}]: Result::{code}"));
                }
                let success_doc = if !success_doc.is_empty() {
                    Some(quote! {
                        #[doc = "# Success codes"]
                        #(#[doc = #success_doc])*
                    })
                } else { None };
                let doc = if version != "VK_VERSION_1_0" {
                    let mut doc = format!("Requires Vulkan version 1.{version_minor}");
                    if let Some(counter_part) = counter_part {
                        doc = format!("{doc}, otherwise provided by {}.\n", counter_part.ext_name);
                        if !counter_part.depends_on.is_empty() {
                            doc += &format!("{} depends on either:", counter_part.ext_name);
                            for (i, dep) in counter_part.depends_on.iter().enumerate() {
                                doc += &if i == counter_part.depends_on.len() -1 {
                                    format!("* {dep}")
                                } else {
                                    format!("* {dep} or")
                                };
                            }
                        }
                    }
                    quote! {
                        #[doc = #doc]
                        ///
                        #success_doc
                        #fn_safety
                        /// # Vulkan docs
                        #doc_link
                        ///
                        #(#[doc = #success_doc_links])*
                    }
                } else {
                    quote! {
                        #success_doc
                        #fn_safety
                        /// # Vulkan docs
                        #doc_link
                        ///
                        #(#[doc = #success_doc_links])*
                    }
                };
                let deprecated =
                    if let Some(dep) = &command.deprecated {
                        if let Some(by) = dep {
                            let by = format!("superseded by {by}");
                            Some(quote! {
                                #[deprecated = #by]
                            })
                        } else {
                            Some(quote! {
                                #[deprecated]
                            })
                        }
                    } else { None };
                rs_def = quote! {
                    #doc
                    #deprecated
                    #rs_def
                };
                rs_def_get_data = rs_def_get_data.map(|def| {
                    quote! {
                        #doc
                        #deprecated
                        #def
                    }
                });
                let pfn = &unresolved.pfn_name;
                let cname = std::ffi::CString::from_str(name).unwrap();
                let load_fn = quote! {
                    #original_name: unsafe {
                        #pfn_default
                        let f = f(#cname);
                        if f.is_null() {
                            #original_name
                        } else {
                            ::core::mem::transmute::<
                                *const ffi::c_void,
                                #pfn,
                            >(f)
                        }
                    },
                };
                let counter_part = counter_part.as_ref().map(|cmd| {
                    let pfn = Ident::new(&format!("PFN_{}", cmd.ext_cmd), Span::call_site());
                    let cname = std::ffi::CString::from_str(&cmd.ext_cmd).unwrap();
                    quote! {
                        #name_rs: unsafe {
                            #pfn_default
                            let f = f(#cname);
                            if f.is_null() {
                                #name_rs
                            } else {
                                ::core::mem::transmute::<
                                    *const ffi::c_void,
                                    #pfn,
                                >(f)
                            }
                        },
                    }
                });
                let f = CoreFn {
                    core: ResolvedCmd { 
                        pfn_def, pfn_default, rs_def, rs_def_get_data,
                        original_name, rs_def_ext,
                    },
                    load_fn,
                    ext_counter_part: counter_part,
                    name_rs: name_rs.clone(),
                };
                if let Some(idx) = version_tables
                    .iter()
                    .enumerate()
                    .find(|(_, t)| t.0 == version_minor)
                    .map(|(i, _)| i)
                {
                    version_tables[idx].1.push(f);
                } else {
                    let mut new = (version_minor, vec![]);
                    new.1.push(f);
                    version_tables.push(new);
                }
            } 
        }
        version_tables.sort_unstable_by_key(|t| t.0);
        for (version_minor, commands) in version_tables {
            let name = Ident::new(&format!("{table}V1{version_minor}"), Span::call_site());
            let vk_version = Ident::new(&format!("API_VERSION_1_{version_minor}"), Span::call_site());
            let mut pfns = vec![];
            let mut load_fns = vec![];
            let mut load_fns_alt = vec![];
            let mut non_alt_count = 0;
            let mut rs_defs = vec![];
            for cmd in commands {
                pfns.push(cmd.core.pfn_def);
                load_fns_alt.push(cmd.ext_counter_part
                    .unwrap_or_else(|| { non_alt_count += 1; cmd.load_fn.clone() })
                );
                load_fns.push(cmd.load_fn);
                let rs1 = cmd.core.rs_def;
                let rs2 = cmd.core.rs_def_get_data;
                if !filter_rs.iter().any(|f| cmd.name_rs == f) {
                    rs_defs.push(quote! {
                        #rs1
                        #rs2
                    });
                }
            }
            let load_body = if non_alt_count == load_fns.len() {
                quote! {
                    Self {
                        #(#load_fns)*
                    }
                }
            } else {
                quote! {
                    if version >= #vk_version {
                        Self {
                            #(#load_fns)*
                        }
                    } else {
                        Self {
                            #(#load_fns_alt)*
                        }
                    }
                }
            };
            let ext = has_ext.then(|| quote! { <Ext> });
            let def = quote! {
                #[derive(Clone, Copy)]
                pub struct #name {
                    #(#pfns),*
                }
                unsafe impl Send for #name {}
                unsafe impl Sync for #name {}
                impl #name {
                    #[allow(unused_variables)]
                    pub fn load(
                        version: u32,
                        f: &mut dyn FnMut(&ffi::CStr) -> *const ffi::c_void,
                    ) -> Self {
                        #load_body
                    }
                }

                impl #ext crate::#handle #ext {
                    #(#rs_defs)*
                }
            };
            write!(core_gen, "{def}")?;
        }
    }
    drop(handles);
    drop(enums);
    drop(structs);
    drop(unions);
    drop(type_defs);
    drop(core_gen);
    drop(extension_gen);
    process::Command::new("rustfmt")
        .arg("../../src/vk/handles.rs")
        .arg("../../src/vk/enums.rs")
        .arg("../../src/vk/structs.rs")
        .arg("../../src/vk/unions.rs")
        .arg("../../src/vk/type_defs.rs")
        .arg("../../src/core_gen.rs")
        .arg("../../src/extension_gen.rs")
        .arg("--edition")
        .arg("2024")
        .output()?;
    Ok(())
}
