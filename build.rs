use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=webpack.config.js");
    println!("cargo:rerun-if-changed=src/main.svelte");
    println!("cargo:warning=Building FE");

    // Run FE build
    Command::new("./node_modules/.bin/webpack-cli")
        .arg("build")
        .output()
        .expect("Failed to build Svelte");
}
