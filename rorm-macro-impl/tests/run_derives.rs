use std::fs::DirEntry;
use std::path::Path;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{fs, io};

use clap::Parser;
use proc_macro2::TokenStream;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    overwrite: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let test_failed: AtomicBool = AtomicBool::new(false);

    let in_dir = Path::new("./tests/derive_model/inputs/");
    let out_dir = Path::new("./tests/derive_model/outputs/");

    let mut errors = fs::read_dir(in_dir)?
        .par_bridge()
        .map(|result: io::Result<DirEntry>| -> io::Result<()> {
            let file_name = result?.file_name();
            let in_file = in_dir.join(&file_name);
            let out_file = out_dir.join(&file_name);

            let input = TokenStream::from_str(&fs::read_to_string(&in_file)?).map_err(|e| {
                io::Error::other(format!("Failed to parse {}: {e}", in_file.display()))
            })?;

            let expected_output = if out_file.exists() {
                TokenStream::from_str(&fs::read_to_string(&out_file)?).map_err(|e| {
                    io::Error::other(format!("Failed to parse {}: {e}", out_file.display()))
                })?
            } else {
                TokenStream::new()
            };

            let actual_output = rorm_macro_impl::derive_model(input);

            let expected_output =
                prettyplease::unparse(&syn::parse2(expected_output).map_err(io::Error::other)?);
            let actual_output =
                prettyplease::unparse(&syn::parse2(actual_output).map_err(io::Error::other)?);

            if actual_output.as_str() != expected_output.as_str() {
                eprintln!("{} failed", file_name.to_string_lossy());

                let mut mismatch = None;
                for (index, (actual, expected)) in actual_output
                    .bytes()
                    .zip(expected_output.bytes())
                    .enumerate()
                {
                    if actual != expected {
                        mismatch = Some(index);
                    }
                }
                let mismatch = mismatch.unwrap();
                let view = mismatch.saturating_sub(8)..;

                eprintln!("Expected:");
                eprintln!("{}", &expected_output[view.clone()]);
                eprintln!();
                eprintln!("Got:");
                eprintln!("{}", &actual_output[view]);
                test_failed.store(true, Ordering::Relaxed);
                if args.overwrite {
                    fs::write(out_file, actual_output)?;
                }
            }

            Ok(())
        })
        .filter_map(Result::err)
        .collect::<Vec<_>>();

    match errors.len() {
        0 => {
            if test_failed.into_inner() {
                Err(io::Error::other("Some tests failed"))
            } else {
                Ok(())
            }
        }
        1 => Err(errors.pop().unwrap()),
        2.. => {
            eprintln!("Multiple errors occured:");
            for error in errors {
                eprintln!("- {}", error);
            }
            Err(io::Error::other(""))
        }
    }
}
