//! This crate tries to follow the base layout proposed by a [ferrous-systems.com](https://ferrous-systems.com/blog/testing-proc-macros/#the-pipeline) blog post.

use proc_macro2::TokenStream;

use crate::analyze::model::analyze_model;
use crate::generate::db_enum::generate_db_enum;
use crate::generate::model::generate_model;
use crate::generate::patch::generate_patch;
use crate::parse::db_enum::parse_db_enum;
use crate::parse::model::parse_model;
use crate::parse::patch::parse_patch;

mod analyze;
mod generate;
mod parse;
mod utils;

pub fn derive_db_enum(input: TokenStream) -> TokenStream {
    match parse_db_enum(input) {
        Ok(model) => generate_db_enum(&model),
        Err(error) => error.write_errors(),
    }
}

pub fn derive_model(input: TokenStream) -> TokenStream {
    match parse_model(input).and_then(analyze_model) {
        Ok(model) => generate_model(&model),
        Err(error) => error.write_errors(),
    }
}

pub fn derive_patch(input: TokenStream) -> TokenStream {
    match parse_patch(input) {
        Ok(patch) => generate_patch(&patch),
        Err(error) => error.write_errors(),
    }
}
