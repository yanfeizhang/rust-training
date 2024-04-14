use std::{env};
use quote::{format_ident, quote, ToTokens};
use syn::{
    Error, File, FnArg, Ident, Item, ItemTrait, Meta, Pat, Path, PathSegment, Result, ReturnType,
    Token, TraitItem, Type,
};
use proc_macro2::{Span, TokenStream};
use std::collections::HashMap;

#[macro_export]
macro_rules! serr {
    ($msg:expr) => {
        ::syn::Error::new(::proc_macro2::Span::call_site(), $msg)
    };
}

#[macro_export]
macro_rules! sbail {
    ($msg:expr) => {
        return Err(::syn::Error::new(::proc_macro2::Span::call_site(), $msg))
    };
}

fn main() {
    let input = "./src/input.rs";
    let output = "./src/output.rs";
    //let p = env::current_dir().unwrap();
    //println!("cwd: {:?}", p);

    // 使用std::fs读取input.rs
    let file_content = std::fs::read_to_string(&input).expect("Unable to read file");
    let syntax = syn::parse_file(&file_content).expect("Unable to parse file");

    let mut generated_code = quote! {
        #[repr(C)]
        pub struct MyStruct {
            pub field1: i32,
            pub field2: String,
        }
    };

    for item in syntax.items.iter() {
        match item {
            Item::Struct(s) => {
                let struct_name = s.ident.clone();
                let struct_name_ref = format_ident!("{}Ref", struct_name);
                let mut field_names = Vec::with_capacity(s.fields.len());
                let mut field_types = Vec::with_capacity(s.fields.len());
                for field in s.fields.iter() {
                    println!("{:?}", field);
                    let field_name = field
                        .clone()
                        .ident;
                    let field_type = ParamType::try_from(&field.ty);
                    field_names.push(field_name);
                    field_types.push(field_type);
                }
                /*generated_code.extend(quote! {
                    #[repr(C)]
                    pub struct #struct_name_ref {
                        #(pub #field_names: #field_types,)*
                    }
                });*/
                println!("{}", struct_name);
                println!("{}", struct_name_ref);
                println!("{:?}", field_names);
                //println!("{:?}", field_types);
            }
            _ => continue,
        }
    }

    println!("{}", generated_code.to_string());
}


pub struct ParamType {
    inner: ParamTypeInner,
    is_reference: bool,
}

pub enum ParamTypeInner {
    Primitive(Ident),
    Custom(Ident),
    List(Type),
}

impl ToTokens for ParamType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.is_reference {
            tokens.extend(quote! {&});
        }
        match &self.inner {
            ParamTypeInner::Primitive(ty) => ty.to_tokens(tokens),
            ParamTypeInner::Custom(ty) => ty.to_tokens(tokens),
            ParamTypeInner::List(ty) => ty.to_tokens(tokens),
        }
    }
}

impl TryFrom<&Type> for ParamType {
    type Error = Error;

    fn try_from(mut ty: &Type) -> Result<Self> {
        let mut is_reference = false;
        if let Type::Reference(r) = ty {
            is_reference = true;
            ty = &r.elem;
        }

        // TypePath -> ParamType
        let seg = type_to_segment(ty)?;
        let param_type_inner = match seg.ident.to_string().as_str() {
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "usize" | "isize"
            | "bool" | "char" | "f32" => {
                if !seg.arguments.is_none() {
                    sbail!("primitive types with arguments are not supported")
                }
                ParamTypeInner::Primitive(seg.ident.clone())
            }
            "Vec" => ParamTypeInner::List(ty.clone()),
            _ => {
                if !seg.arguments.is_none() {
                    sbail!("custom types with arguments are not supported")
                }
                ParamTypeInner::Custom(seg.ident.clone())
            }
        };
        Ok(ParamType {
            inner: param_type_inner,
            is_reference,
        })
    }
}

fn type_to_segment(ty: &Type) -> Result<&PathSegment> {
    let field_type = match ty {
        Type::Path(p) => p,
        _ => sbail!("only path types are supported"),
    };
    let path = &field_type.path;
    // Leading colon is not allow
    if path.leading_colon.is_some() {
        sbail!("types with leading colons are not supported");
    }
    // We only accept single-segment path
    if path.segments.len() != 1 {
        sbail!("types with multiple segments are not supported");
    }
    Ok(path.segments.first().unwrap())
}

impl ParamType {
    fn to_c(&self, with_struct: bool) -> String {
        let struct_ = if with_struct { "struct " } else { "" };
        match &self.inner {
            ParamTypeInner::Primitive(name) => match name.to_string().as_str() {
                "u8" => "uint8_t",
                "u16" => "uint16_t",
                "u32" => "uint32_t",
                "u64" => "uint64_t",
                "i8" => "int8_t",
                "i16" => "int16_t",
                "i32" => "int32_t",
                "i64" => "int64_t",
                "bool" => "bool",
                "char" => "uint32_t",
                "usize" => "uintptr_t",
                "isize" => "intptr_t",
                "f32" => "float",
                "f64" => "double",
                _ => panic!("unreconigzed rust primitive type {name}"),
            }
            .to_string(),
            ParamTypeInner::Custom(c) => format!("{struct_}{c}Ref"),
            ParamTypeInner::List(_) => format!("{struct_}ListRef"),
        }
    }

    fn to_go(&self) -> String {
        match &self.inner {
            ParamTypeInner::Primitive(name) => match name.to_string().as_str() {
                "u8" => "uint8",
                "u16" => "uint16",
                "u32" => "uint32",
                "u64" => "uint64",
                "i8" => "int8",
                "i16" => "int16",
                "i32" => "int32",
                "i64" => "int64",
                "bool" => "bool",
                "char" => "rune",
                "usize" => "uint",
                "isize" => "int",
                "f32" => "float32",
                "f64" => "float64",
                _ => panic!("unreconigzed rust primitive type {name}"),
            }
            .to_string(),
            ParamTypeInner::Custom(c) => {
                let s = c.to_string();
                match s.as_str() {
                    "String" => "string".to_string(),
                    _ => s,
                }
            }
            ParamTypeInner::List(inner) => {
                let seg = type_to_segment(inner).unwrap();
                let inside = match &seg.arguments {
                    syn::PathArguments::AngleBracketed(ga) => match ga.args.last().unwrap() {
                        syn::GenericArgument::Type(ty) => ty,
                        _ => panic!("list generic must be a type"),
                    },
                    _ => panic!("list type must have angle bracketed arguments"),
                };
                format!(
                    "[]{}",
                    ParamType::try_from(inside)
                        .expect("unable to convert list type")
                        .to_go()
                )
            }
        }
    }

    // f: StructRef -> Struct
    fn c_to_go_field_converter(&self, mapping: &HashMap<Ident, u8>) -> (String, u8) {
        match &self.inner {
            ParamTypeInner::Primitive(name) => (
                match name.to_string().as_str() {
                    "u8" => "newC_uint8_t",
                    "u16" => "newC_uint16_t",
                    "u32" => "newC_uint32_t",
                    "u64" => "newC_uint64_t",
                    "i8" => "newC_int8_t",
                    "i16" => "newC_int16_t",
                    "i32" => "newC_int32_t",
                    "i64" => "newC_int64_t",
                    "bool" => "newC_bool",
                    "usize" => "newC_uintptr_t",
                    "isize" => "newC_intptr_t",
                    "f32" => "newC_float",
                    "f64" => "newC_double",
                    _ => panic!("unrecognized rust primitive type {name}"),
                }
                .to_string(),
                0,
            ),
            ParamTypeInner::Custom(c) => (
                format!("new{}", c.to_string().as_str()),
                *mapping.get(c).unwrap(),
            ),
            ParamTypeInner::List(inner) => {
                let seg = type_to_segment(inner).unwrap();
                let inside = match &seg.arguments {
                    syn::PathArguments::AngleBracketed(ga) => match ga.args.last().unwrap() {
                        syn::GenericArgument::Type(ty) => ty,
                        _ => panic!("list generic must be a type"),
                    },
                    _ => panic!("list type must have angle bracketed arguments"),
                };
                let (inner, inner_level) = ParamType::try_from(inside)
                    .expect("unable to convert list type")
                    .c_to_go_field_converter(mapping);
                if inner_level == 0 {
                    (format!("new_list_mapper_primitive({inner})"), 1)
                } else {
                    (format!("new_list_mapper({inner})"), 2.min(inner_level + 1))
                }
            }
        }
    }

    fn go_to_c_field_counter(&self, mapping: &HashMap<Ident, u8>) -> (String, u8) {
        match &self.inner {
            ParamTypeInner::Primitive(name) => (
                match name.to_string().as_str() {
                    "u8" => "cntC_uint8_t",
                    "u16" => "cntC_uint16_t",
                    "u32" => "cntC_uint32_t",
                    "u64" => "cntC_uint64_t",
                    "i8" => "cntC_int8_t",
                    "i16" => "cntC_int16_t",
                    "i32" => "cntC_int32_t",
                    "i64" => "cntC_int64_t",
                    "bool" => "cntC_bool",
                    "usize" => "cntC_uintptr_t",
                    "isize" => "cntC_intptr_t",
                    "f32" => "cntC_float",
                    "f64" => "cntC_double",
                    _ => panic!("unrecognized rust primitive type {name}"),
                }
                .to_string(),
                0,
            ),
            ParamTypeInner::Custom(c) => (
                format!("cnt{}", c.to_string().as_str()),
                *mapping.get(c).unwrap(),
            ),
            ParamTypeInner::List(inner) => {
                let seg = type_to_segment(inner).unwrap();
                let inside = match &seg.arguments {
                    syn::PathArguments::AngleBracketed(ga) => match ga.args.last().unwrap() {
                        syn::GenericArgument::Type(ty) => ty,
                        _ => panic!("list generic must be a type"),
                    },
                    _ => panic!("list type must have angle bracketed arguments"),
                };
                let (inner, inner_level) = ParamType::try_from(inside)
                    .expect("unable to convert list type")
                    .go_to_c_field_counter(mapping);
                if inner_level == 0 {
                    (format!("cnt_list_mapper_primitive({inner})"), 1)
                } else {
                    (format!("cnt_list_mapper({inner})"), 2.min(inner_level + 1))
                }
            }
        }
    }

    // f: Struct -> StructRef
    fn go_to_c_field_converter(&self, mapping: &HashMap<Ident, u8>) -> (String, u8) {
        match &self.inner {
            ParamTypeInner::Primitive(name) => (
                match name.to_string().as_str() {
                    "u8" => "refC_uint8_t",
                    "u16" => "refC_uint16_t",
                    "u32" => "refC_uint32_t",
                    "u64" => "refC_uint64_t",
                    "i8" => "refC_int8_t",
                    "i16" => "refC_int16_t",
                    "i32" => "refC_int32_t",
                    "i64" => "refC_int64_t",
                    "bool" => "refC_bool",
                    "usize" => "refC_uintptr_t",
                    "isize" => "refC_intptr_t",
                    "f32" => "refC_float",
                    "f64" => "refC_double",
                    _ => panic!("unreconigzed rust primitive type {name}"),
                }
                .to_string(),
                0,
            ),
            ParamTypeInner::Custom(c) => (
                format!("ref{}", c.to_string().as_str()),
                *mapping.get(c).unwrap(),
            ),
            ParamTypeInner::List(inner) => {
                let seg = type_to_segment(inner).unwrap();
                let inside = match &seg.arguments {
                    syn::PathArguments::AngleBracketed(ga) => match ga.args.last().unwrap() {
                        syn::GenericArgument::Type(ty) => ty,
                        _ => panic!("list generic must be a type"),
                    },
                    _ => panic!("list type must have angle bracketed arguments"),
                };
                let (inner, inner_level) = ParamType::try_from(inside)
                    .expect("unable to convert list type")
                    .go_to_c_field_converter(mapping);
                if inner_level == 0 {
                    (format!("ref_list_mapper_primitive({inner})"), 1)
                } else {
                    (format!("ref_list_mapper({inner})"), 2.min(inner_level + 1))
                }
            }
        }
    }

    fn to_rust_ref(&self, prefix: Option<&TokenStream>) -> TokenStream {
        match &self.inner {
            ParamTypeInner::Primitive(name) => quote!(#name),
            ParamTypeInner::Custom(name) => {
                let ident = format_ident!("{}Ref", name);
                quote!(#prefix #ident)
            }
            ParamTypeInner::List(_) => {
                let ident = format_ident!("ListRef");
                quote!(#prefix #ident)
            }
        }
    }
}