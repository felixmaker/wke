fn main() {
    let ul = std::env::var("ULTRALIGHT").unwrap();

    println!("cargo::rustc-link-search={}/lib", ul);

    println!("cargo::rustc-link-lib=dylib=Ultralight");
    println!("cargo::rustc-link-lib=dylib=UltralightCore");
    println!("cargo::rustc-link-lib=dylib=WebCore");
    println!("cargo::rustc-link-lib=dylib=AppCore");
}
