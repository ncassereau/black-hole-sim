use std::env;
use std::process::Command;

fn main() {
    println!("cargo::rerun-if-changed=src/cuda/kernels");
    println!("cargo::rerun-if-changed=build.rs");
    let path = env::current_dir()
        .unwrap()
        .join("src")
        .join("cuda")
        .join("kernels")
        .join("main.cu");

    let out_dir = env::var("OUT_DIR").unwrap();
    let ptx_output = format!("{}/kernels.ptx", out_dir);
    let command = Command::new("nvcc")
        .args(&["-ptx", "-o", &ptx_output])
        .arg(path)
        .output()
        .unwrap_or_else(|e| panic!("{e}"));

    if !command.status.success() {
        panic!("nvcc failed: {}", String::from_utf8_lossy(&command.stderr));
    }
    println!("cargo::rustc-env=KERNELS_PTX_PATH={}", ptx_output);
}
