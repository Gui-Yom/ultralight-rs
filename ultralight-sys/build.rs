use std::env;

use bindgen::EnumVariation;

fn main() {
    println!("cargo:rustc-link-lib=AppCore");
    println!("cargo:rustc-link-lib=Ultralight");
    println!("cargo:rustc-link-lib=UltralightCore");
    println!("cargo:rustc-link-lib=WebCore");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .impl_debug(true)
        .impl_partialeq(true)
        .generate_comments(true)
        .generate_inline_functions(true)
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: false,
        })
        .bitfield_enum("ULWindowFlags|JSPropertyAttributes")
        .whitelist_var("^UL.*|JS.*|ul.*|WK.*")
        .whitelist_type("^UL.*|JS.*|ul.*|WK.*")
        .whitelist_function("^UL.*|JS.*|ul.*|WK.*")
        .clang_arg(format!("-I{}/include", env::var("ULTRALIGHT_SDK").unwrap()))
        .clang_arg("-fretain-comments-from-system-headers")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = std::path::PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
