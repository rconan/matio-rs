use std::path::PathBuf;

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }

    let out = cmake::Config::new("matio")
        .define("MATIO_SHARED", "OFF")
        .define("MATIO_MAT73", "OFF")
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        out.join("lib").display()
    );
    println!("cargo:rustc-link-lib=matio");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .clang_arg(&format!("-I{}", out.join("include").display()))
        .clang_arg("-Imatio/src")
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
