use super::super::repository_document;

#[test]
fn phase_three_world_accepts_only_semantic_program_input_through_the_ordinary_driver() {
    let manifest = repository_document("workspaces/worth-ui/apps/platform-pulse/Cargo.toml");
    for forbidden in ["worth-ui-runtime =", "worth-ui-host-contract ="] {
        assert!(
            !manifest.contains(forbidden),
            "the product world gained a mechanics dependency: {forbidden}"
        );
    }

    let application = repository_document(
        "workspaces/worth-ui/apps/platform-pulse/src/native_phase3_application.rs",
    );
    for required in [
        "UiNativeApplicationProgram",
        "UiNativeApplicationFrame",
        "UiNativeComponentPresenceChange",
        "install_frame_program",
    ] {
        assert!(
            application.contains(required),
            "semantic program omits {required}"
        );
    }
    for forbidden in [
        "UiMounted",
        "from_inert_mechanics",
        "certification_support",
        "UiNativeApplicationDriver",
        "worth_ui_host_contract",
    ] {
        assert!(
            !application.contains(forbidden),
            "application can inject presentation mechanics through {forbidden}"
        );
    }

    let platform = repository_document(
        "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/platform.rs",
    );
    let driver = repository_document(
        "workspaces/worth-ui/crates/worth-ui-runtime/src/native_platform/application_driver.rs",
    );
    for forbidden in ["phase3", "certification", "cfg(feature"] {
        assert!(
            !platform.contains(forbidden) && !driver.contains(forbidden),
            "ordinary platform progression contains alternate-driver residue: {forbidden}"
        );
    }
}
