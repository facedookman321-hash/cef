fn main() {
    // DX_SDK should point to "<DirectX SDK June 2010>/Lib/x86".
    let dx = std::env::var("DX_SDK").unwrap_or_else(|_| {
        "C:\\Program Files (x86)\\Microsoft DirectX SDK (June 2010)\\Lib\\x86".to_string()
    });
    println!("cargo:rerun-if-env-changed=DX_SDK");
    println!("cargo:rustc-link-search=native={}", dx);
    println!("cargo:rustc-link-lib=static=d3dx9");
}
