use std::path::PathBuf;

use crate::topology::WorkspaceSourceInventory;

use super::{manifest_contract, runner_contract};

#[test]
fn live_phase3_external_world_satisfies_the_contract() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    manifest_contract::audit(
        inventory.text("Cargo.toml"),
        inventory.text("apps/platform-pulse/Cargo.toml"),
    )
    .expect("live Phase 3 manifest");
    runner_contract::audit(&inventory).expect("live Phase 3 runner");
}

#[test]
fn manifest_rejects_a_non_windows_native_dependency() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let mutated = inventory.text("apps/platform-pulse/Cargo.toml").replace(
        "[target.'cfg(windows)'.dev-dependencies]",
        "[dev-dependencies]",
    );
    let error = manifest_contract::audit(inventory.text("Cargo.toml"), &mutated)
        .expect_err("native courtroom dependencies must stay Windows-only");
    assert!(error.contains("Windows courtroom"));
}

#[test]
fn manifest_rejects_missing_or_widened_wgc_capture_features() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let live = inventory.text("apps/platform-pulse/Cargo.toml");
    let missing = live.replace(
        r#"xcap = { workspace = true, features = ["wgc"] }"#,
        "xcap = { workspace = true }",
    );
    let error = manifest_contract::audit(inventory.text("Cargo.toml"), &missing)
        .expect_err("missing WGC feature must fail");
    assert!(error.contains("`xcap` application dependency contract drifted"));

    let widened = live.replace(
        r#"xcap = { workspace = true, features = ["wgc"] }"#,
        r#"xcap = { workspace = true, features = ["wgc", "shortcut"] }"#,
    );
    let error = manifest_contract::audit(inventory.text("Cargo.toml"), &widened)
        .expect_err("unplanned capture feature must fail");
    assert!(error.contains("`xcap` application dependency contract drifted"));
}

#[test]
fn runner_rejects_direct_product_runtime_authority() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let source = inventory
        .source("apps/platform-pulse/tests/executable_world.rs")
        .expect("runner entry");
    assert!(!source.text().contains("worth_ui_runtime"));
}

#[test]
fn runner_rejects_global_unexposed_and_resampled_native_capture() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let owner = native_owner(&inventory);
    let capture = inventory.text(
        "apps/platform-pulse/tests/executable_world/native_platform/windows/client_capture.rs",
    );

    let global = mutate_required(&owner, "win::EnumWindows(", "Window::all(");
    let error = runner_contract::audit_native_boundary(&global, capture)
        .expect_err("global native-window discovery must fail");
    assert!(error.contains("win::EnumWindows("), "{error}");

    let unexposed = mutate_required(&owner, "win::DwmFlush()", "Ok(())");
    let error = runner_contract::audit_native_boundary(&unexposed, capture)
        .expect_err("capture without a compositor exposure barrier must fail");
    assert!(error.contains("win::DwmFlush()"), "{error}");

    let resampled = format!("{capture}\nfn counterfeit() {{ imageops::resize(); }}\n");
    let error = runner_contract::audit_native_boundary(&owner, &resampled)
        .expect_err("resampled native capture must fail");
    assert!(error.contains("imageops::resize"), "{error}");
}

#[test]
fn runner_rejects_capture_without_exact_process_and_hwnd_identity() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let owner = native_owner(&inventory);
    let capture = inventory.text(
        "apps/platform-pulse/tests/executable_world/native_platform/windows/client_capture.rs",
    );
    for edge in [
        "window.pid().ok() == Some(process_id)",
        "window.id().ok() == Some(window_id)",
    ] {
        let identity_blind = mutate_required(capture, edge, "true");
        let error = runner_contract::audit_native_boundary(&owner, &identity_blind)
            .expect_err("capture without exact process and HWND identity must fail");
        assert!(error.contains(edge), "{error}");
    }
}

#[test]
fn runner_rejects_owner_drift_and_non_native_close() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let owner = native_owner(&inventory);
    let capture = inventory.text(
        "apps/platform-pulse/tests/executable_world/native_platform/windows/client_capture.rs",
    );

    let owner_blind = mutate_required(
        &owner,
        "owner_process_id != bound.observation.process_id()",
        "false",
    );
    let error = runner_contract::audit_native_boundary(&owner_blind, capture)
        .expect_err("capture without process-owner revalidation must fail");
    assert!(
        error.contains("owner_process_id != bound.observation.process_id()"),
        "{error}"
    );

    let forced = mutate_required(
        &owner,
        ".and_then(|pattern| pattern.close())",
        ".and_then(|_| process.kill())",
    );
    let error = runner_contract::audit_native_boundary(&forced, capture)
        .expect_err("forced process close must fail");
    assert!(error.contains("pattern.close()"), "{error}");
}

#[test]
fn runner_rejects_a_journey_detached_from_typed_native_close() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let courtroom = [
        inventory.text(
            "apps/platform-pulse/tests/executable_world/courtroom/platform_pulse_lifecycle.rs",
        ),
        inventory
            .text("apps/platform-pulse/tests/executable_world/courtroom/platform_pulse_journey.rs"),
        inventory.text(
            "apps/platform-pulse/tests/executable_world/courtroom/platform_pulse_journey/open.rs",
        ),
        inventory
            .text("apps/platform-pulse/tests/executable_world/courtroom/platform_pulse_cleanup.rs"),
    ]
    .join("\n");
    let detached = mutate_required(
        &courtroom,
        "close_recovered(self.recovered)",
        "drop(self.recovered)",
    );
    let error = runner_contract::audit_courtroom(&detached)
        .expect_err("the success journey must consume recovered state through native close");
    assert!(error.contains("close_recovered(self.recovered)"), "{error}");
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates")
        .parent()
        .expect("workspace")
        .to_path_buf()
}

fn native_owner(inventory: &WorkspaceSourceInventory) -> String {
    [
        inventory.text("apps/platform-pulse/tests/executable_world/native_platform/windows.rs"),
        inventory.text(
            "apps/platform-pulse/tests/executable_world/native_platform/windows/capture_region.rs",
        ),
    ]
    .join("\n")
}

fn mutate_required(source: &str, required: &str, replacement: &str) -> String {
    assert!(
        source.contains(required),
        "negative fixture cannot exercise absent edge `{required}`"
    );
    source.replacen(required, replacement, 1)
}
