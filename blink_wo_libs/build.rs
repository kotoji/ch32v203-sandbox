fn main() {
    println!("cargo:rustc-link-arg-bins=-Tlink.x");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = std::path::PathBuf::from(out_dir);
    println!("cargo:rustc-link-search={}", out_dir.display());
}
