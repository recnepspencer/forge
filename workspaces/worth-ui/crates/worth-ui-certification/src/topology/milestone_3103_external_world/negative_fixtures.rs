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
fn runner_rejects_direct_product_runtime_authority() {
    let inventory = WorkspaceSourceInventory::capture(workspace_root());
    let source = inventory
        .source("apps/platform-pulse/tests/executable_world.rs")
        .expect("runner entry");
    assert!(!source.text().contains("worth_ui_runtime"));
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates")
        .parent()
        .expect("workspace")
        .to_path_buf()
}
