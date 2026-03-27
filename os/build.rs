fn main() {
    println!("cargo:rerun-if-changed=src/arch/x86_64/boot.S");

    let output = std::process::Command::new("nasm")
        .args(&[
            "-f", "elf64",
            "-o", "src/arch/x86_64/boot.o",
            "src/arch/x86_64/boot.S"
        ])
        .output()
        .expect("Failed to assemble boot.S");

    if !output.status.success() {
        panic!("NASM assembly failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    println!("cargo:rustc-link-arg=src/arch/x86_64/boot.o");
}
