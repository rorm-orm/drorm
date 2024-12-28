use std::path::Path;
use std::str::FromStr;
use std::{env, fs};

use datatest_stable::{harness, Utf8Path};
use proc_macro2::TokenStream;

harness! {
    { test = test_model, root = "tests/derive_model/inputs/", pattern = r"^[^/]+\.rs$" },
    { test = test_patch, root = "tests/derive_patch/inputs/", pattern = r"^[^/]+\.rs$" },
    { test = test_db_enum, root = "tests/derive_db_enum/inputs/", pattern = r"^[^/]+\.rs$" },
}

fn test_model(path: &Utf8Path, input: String) -> datatest_stable::Result<()> {
    test_macro(path, input, rorm_macro_impl::derive_model)
}

fn test_patch(path: &Utf8Path, input: String) -> datatest_stable::Result<()> {
    test_macro(path, input, rorm_macro_impl::derive_patch)
}

fn test_db_enum(path: &Utf8Path, input: String) -> datatest_stable::Result<()> {
    test_macro(path, input, rorm_macro_impl::derive_db_enum)
}

fn test_macro(
    input_file: &Utf8Path,
    input: String,
    macro_fn: impl FnOnce(TokenStream) -> TokenStream,
) -> datatest_stable::Result<()> {
    let inputs_dir = input_file.parent().unwrap();
    assert_eq!(inputs_dir.file_name(), Some("inputs"));
    let io_dir = inputs_dir.parent().unwrap();
    let output_file = io_dir.join("outputs").join(input_file.file_name().unwrap());

    let input =
        TokenStream::from_str(&input).map_err(|e| format!("Failed to tokenize input: {e}"))?;

    let expected_output = if output_file.exists() {
        TokenStream::from_str(
            &fs::read_to_string(&output_file).map_err(|e| format!("Failed to read output: {e}"))?,
        )
        .map_err(|e| format!("Failed to tokenize expected output: {e}"))?
    } else {
        TokenStream::new()
    };

    let actual_output = macro_fn(input);

    let expected_output = prettyplease::unparse(
        &syn::parse2(expected_output)
            .map_err(|e| format!("Failed to parse expected output: {e}"))?,
    );
    let actual_output = prettyplease::unparse(
        &syn::parse2(actual_output).map_err(|e| format!("Failed to parse actual output: {e}"))?,
    );

    if actual_output == expected_output {
        Ok(())
    } else {
        if env::var_os("RORM_WRITE_OUTPUT").is_some() {
            fs::write(output_file, actual_output)?;
        }
        Err("Output doesn't match expectation".into())
    }
}
