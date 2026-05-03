use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::Literal;
use syn::{parse_macro_input, Data, DeriveInput};
use quote::quote;

fn parse_inputs(
    name: &syn::Ident,
    data: &syn::DataStruct,
) -> syn::Result<Vec<TokenStream2>> {
    let mut inputs = vec![];
    for (i, field) in data.fields.iter().enumerate() {
        let offset_of = field.ident
            .as_ref()
            .map(|ident| {
                quote! {
                    ::core::mem::offset_of!(#name, #ident)
                }
            }).unwrap_or_else(|| {
                let f = Literal::usize_unsuffixed(i);
                quote! {
                    ::core::mem::offset_of!(#name, #f)
                }
            });
        let i = i as u32;
        let Some(attr) = field
            .attrs.iter()
            .find(|attr|
                attr.path().is_ident("format")
            )
        else {
            return Err(
                syn::Error::new_spanned(field, "missing format attribute for field")
            )
        };
        let format = attr.parse_args::<syn::Expr>()?;
        inputs.push(quote! {
            leimu::gpu::VertexInputAttribute {
                location: first_location + #i,
                format: #format,
                offset: #offset_of as u32,
            }
        });
    }
    Ok(inputs)
}

pub fn vertex_input(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let mut repr_c = false;
    for attr in &input.attrs {
        if attr.path().is_ident("repr") &&
            let Ok(ident) = attr.parse_args::<syn::Ident>() &&
            ident == "C"
        {
            repr_c = true;
        }
    }
    if !repr_c {
        let err = syn::Error::new_spanned(&input, "VertexInput must be repr(C)");
        return err.to_compile_error().into()
    }
    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input, "VertexInput must be a struct")
            .to_compile_error()
            .into()
    };
    let name = &input.ident; 
    let inputs = match parse_inputs(name, data) {
        Ok(inputs) => inputs,
        Err(err) => {
            return err
                .to_compile_error()
                .into()

        }
    };
    let n = inputs.len();
    quote! {
        impl<> leimu::gpu::VertexInput<#n> for #name {

            fn get_attributes(
                first_location: u32,
            ) -> [leimu::gpu::VertexInputAttribute; #n] {
                [#(
                    #inputs
                ),*]
            }
        }
    }.into()
}
