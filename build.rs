use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=webpack.config.js");
    println!("cargo:rerun-if-changed=src/main.ts");
    println!("cargo:rerun-if-changed=src/crypto.ts");
    println!("cargo:rerun-if-changed=src/components/notepad.svelte");
    println!("cargo:rerun-if-changed=src/components/status.svelte");
    println!("cargo:warning=Building FE");

    // Run FE build
    let output = Command::new("./node_modules/.bin/webpack-cli")
        .arg("build")
        .output()
        .expect("Failed to build Svelte");

    println!("cargo:warning={:?}", output);
}
