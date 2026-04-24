#[macro_use]
mod util;
mod display;
mod error;
mod build_structure;
mod vertex_input;

extern crate proc_macro;

use proc_macro::TokenStream;

/// Derive macro for [`Display`][1].
///
/// [1]: core::fmt::Display
#[proc_macro_derive(Display, attributes(display))]
pub fn display(item: TokenStream) -> TokenStream {
    display::display(item)
}

/// Derive macro for [`Error`][1].
///
/// [1]: core::error::Error
#[proc_macro_derive(Error, attributes(display, source, from))]
pub fn error(item: TokenStream) -> TokenStream {
    error::error(item)
}

/// Derive macro for [`VertexInput`].
#[proc_macro_derive(VertexInput)]
pub fn vertex_input(item: TokenStream) -> TokenStream {
    vertex_input::vertex_input(item)
}

#[proc_macro_derive(BuildStructure, attributes(by_mut, skip, default))]
pub fn build_structure(item: TokenStream) -> TokenStream {
    build_structure::build_structure(item)
}
