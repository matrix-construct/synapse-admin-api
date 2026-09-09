fn main() {
    // Emitted unconditionally rather than from an environment variable: a
    // consumer building from a git source does not apply `.cargo/config.toml`.
    // ruma's request/response macros expand here, so this crate needs the cfg
    // set for its own types, not just ruma's.
    println!("cargo:rustc-cfg=ruma_unstable_exhaustive_types");
    println!("cargo:rerun-if-changed=build.rs");
}
