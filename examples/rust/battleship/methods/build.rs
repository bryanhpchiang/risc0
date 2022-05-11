use std::{env, path::Path, process::Command};

fn build() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let args = vec![
        "build",
        "--release",
        "--target",
        "../../../../bazel/rules/rust/riscv32im-unknown-none-elf.json",
        "-Z",
        "build-std=alloc,core",
        "--target-dir",
        &out_dir,
    ];
    Command::new("cargo").args(args).status().unwrap();
}

fn make_id(name: &str, key: &str) {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let src_path = Path::new(&out_dir)
        .join("riscv32im-unknown-none-elf")
        .join("release")
        .join(name);

    println!("cargo:rustc-env={}={}", key, src_path.display());

    let mut dest_path = Path::new(&out_dir).join(name);
    dest_path.set_extension("id");

    Command::new("make-id")
        .args([src_path, dest_path])
        .status()
        .unwrap();
}

fn main() {
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap() != "riscv32" {
        build();
        make_id("init", "RISC0_INIT_ELF_PATH");
        make_id("turn", "RISC0_TURN_ELF_PATH");
    }
}
