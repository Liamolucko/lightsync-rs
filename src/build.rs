use std::env;

fn main() {
    let proj_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!(
        "cargo:rustc-link-search={dir}/vendor/Lib/{arch}",
        dir = proj_dir,
        arch = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
            "x86" => "x86",
            "x86_64" => "x64",
            arch => panic!("Unsupported architecture {}", arch),
        }
    );
    println!("cargo:rustc-link-lib=static=LogitechLEDLib")
}
