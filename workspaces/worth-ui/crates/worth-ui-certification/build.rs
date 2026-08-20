const WINDOWS_MSVC_STACK_RESERVE_BYTES: u64 = 8 * 1024 * 1024;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc")
    {
        println!(
            "cargo:rustc-link-arg-bin=worth-ui-phase5-locality-matrix=/STACK:{WINDOWS_MSVC_STACK_RESERVE_BYTES}"
        );
    }
}
