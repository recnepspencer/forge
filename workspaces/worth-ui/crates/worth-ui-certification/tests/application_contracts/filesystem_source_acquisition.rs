use std::fs;

use worth_ui::facade::graph::UiGraphWorldDifferenceKind;
use worth_ui::facade::source::{
    WorthUiFilesystemSourceAcquisitionDenial, WorthUiFilesystemSourceProvider,
};
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

#[test]
fn production_filesystem_reader_freezes_real_bytes_before_public_application_preparation() {
    let scenario = FilesystemApplicationLifecycleScenario::new("source-acquisition");
    let workspace = FilesystemContractWorkspace::new("source-acquisition");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::current_source_text(),
    );
    workspace.write("app/editor-backup.txt", "component invalid.backup {");
    let filesystem = WorthUiFilesystemSourceProvider::new(workspace.root());
    let snapshot = filesystem.read().expect("real source bytes should freeze");
    assert_eq!(
        snapshot.source_revision().provider_id(),
        fs::canonicalize(workspace.root())
            .expect("source root should canonicalize")
            .to_string_lossy(),
        "filesystem identity must use the normalized source root"
    );
    let current_revision = snapshot.source_revision().final_package_digest();

    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::candidate_source_text(),
    );
    let capabilities = scenario.capability_application();
    let filesystem_submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        snapshot,
        capabilities.capabilities(),
    );
    let rust_submission = FilesystemApplicationLifecycleScenario::current_rust_submission(
        capabilities.capabilities(),
    );
    let filesystem_app = scenario.prepare_application(filesystem_submission);
    let rust_app = scenario.prepare_application(rust_submission);

    assert_eq!(
        filesystem_app.generation_identity(),
        rust_app.generation_identity()
    );
    assert_eq!(
        filesystem_app.graph().compare_to(rust_app.graph()).kind(),
        UiGraphWorldDifferenceKind::SameWorldEquivalent
    );
    let active = filesystem_app
        .launch()
        .expect("real filesystem application should launch publicly");
    assert_eq!(active.generation_identity(), rust_app.generation_identity());
    let next_snapshot = filesystem
        .read()
        .expect("replacement bytes should produce a later stable snapshot");
    assert_ne!(
        current_revision,
        next_snapshot.source_revision().final_package_digest(),
        "the first snapshot must retain the bytes read before the external rewrite"
    );
    let _ = active.shutdown();
    workspace.close();
}

#[test]
fn filesystem_reader_rejects_a_file_masquerading_as_a_workspace_root() {
    let workspace = FilesystemContractWorkspace::new("source-root-denial");
    workspace.write(
        "not-a-workspace.wui",
        &FilesystemApplicationLifecycleScenario::current_source_text(),
    );
    let path = workspace.path("not-a-workspace.wui");

    let denial = WorthUiFilesystemSourceProvider::new(&path)
        .read()
        .expect_err("a source root must be a real directory");

    assert_eq!(
        denial,
        WorthUiFilesystemSourceAcquisitionDenial::RootNotDirectory(path)
    );
    workspace.close();
}
