use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};

pub fn to_db_name(name: String) -> String {
    let mut name = name;
    name.make_ascii_lowercase();
    name
}

/// Create the expression for creating a `Option<Source>` instance from a span
pub fn get_source(span: Span) -> TokenStream {
    quote! {None}
    /*quote_spanned! {span=>
        Some(::rorm::internal::hmr::Source {
            file: ::std::file!(),
            line: ::std::line!() as usize,
            column: ::std::column!() as usize,
        })
    }*/
}
