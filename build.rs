

fn main() {
    println!("cargo:rustc-link=mud");
    println!("cargo:rustc-link-lib=crypt");
    println!("cargo:rustc-link-search={}", std::env::var("CARGO_MANIFEST_DIR").unwrap());
}
