use heck::CamelCase;
use quote::quote;
use std::path::PathBuf;
use std::{env, fs};

fn get_mod_children<'a>(item: &'a mut syn::Item) -> Option<&'a mut Vec<syn::Item>> {
    match item {
        syn::Item::Mod(syn::ItemMod {
            content: Some((_, children)),
            ..
        }) => Some(children),
        _ => None,
    }
}

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let bindings = bindgen::builder()
        .header("vendor/Include/LogitechLEDLib.h")
        .clang_arg("-xc++")
        .enable_cxx_namespaces()
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .raw_line("#![allow(dead_code)]")
        .generate()
        .expect("Unable to generate bindings");

    let mut file: syn::File = syn::parse_str(&bindings.to_string()).unwrap();

    // Fix casing in enums, since I'm making KeyName part of the API.
    for item in file
        .items
        .iter_mut()
        .filter_map(get_mod_children)
        .flat_map(|item| item.iter_mut())
        .filter_map(get_mod_children)
        .flat_map(|item| item.iter_mut())
    {
        match item {
            syn::Item::Enum(item) => {
                for variant in item.variants.iter_mut() {
                    variant.ident =
                        syn::parse_str(&variant.ident.to_string().to_camel_case()).unwrap();
                }
            }
            _ => {}
        }
    }

    fs::write(
        PathBuf::from(dir.as_str()).join("src/bindings.rs"),
        quote!(#file).to_string(),
    )
    .expect("Failed to write bindings");

    println!(
        "cargo:rustc-link-search={dir}/vendor/Lib/{arch}",
        dir = dir,
        arch = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "x86" => "x86",
            "x86_64" => "x64",
            arch => panic!("Unsupported architecture {}", arch),
        }
    );
    println!("cargo:rustc-link-lib=static=LogitechLEDLib")
}
