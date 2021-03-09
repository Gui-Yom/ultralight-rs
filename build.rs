use std::env;

fn main() {
    println!(
        "cargo:rustc-link-search={}/lib",
        env::var("ULTRALIGHT_SDK").unwrap()
    );
}
