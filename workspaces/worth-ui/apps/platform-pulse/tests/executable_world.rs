#[path = "executable_world/adjudication/mod.rs"]
mod adjudication;
#[path = "executable_world/courtroom/mod.rs"]
mod courtroom;
#[path = "executable_world/external_observation/mod.rs"]
mod external_observation;
#[cfg(target_os = "windows")]
#[path = "executable_world/failure_teardown/mod.rs"]
mod failure_teardown;
#[path = "executable_world/installation/mod.rs"]
mod installation;
#[path = "executable_world/native_platform/mod.rs"]
mod native_platform;
#[path = "executable_world/product_process/mod.rs"]
mod product_process;
#[path = "executable_world/source_delta/mod.rs"]
mod source_delta;

#[cfg(not(target_os = "windows"))]
#[test]
fn executable_world_is_explicitly_compile_only_off_windows() {
    assert_eq!(
        native_platform::current_platform_posture(),
        native_platform::NativePlatformPosture::CompileOnly
    );
}
