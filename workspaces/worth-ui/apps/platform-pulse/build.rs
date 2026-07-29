const WINDOWS_MSVC_STACK_RESERVE_BYTES: u64 = 8 * 1024 * 1024;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rustc-env=WORTH_UI_PLATFORM_PULSE_STACK_RESERVE_BYTES={WINDOWS_MSVC_STACK_RESERVE_BYTES}"
    );
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc")
    {
        println!(
            "cargo:rustc-link-arg-bin=worth-ui-platform-pulse=/STACK:{WINDOWS_MSVC_STACK_RESERVE_BYTES}"
        );
    }
}
