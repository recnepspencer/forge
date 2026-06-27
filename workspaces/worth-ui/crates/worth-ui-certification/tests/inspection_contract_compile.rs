#[path = "trybuild_support.rs"]
mod trybuild_support;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn inspection_contract_enforces_shared_callers_and_sealed_receipts() {
    let tests = trybuild_support::new_test_cases();
    tests.pass("tests/ui/inspection/ai_and_human_callers_share_inspection_contract.rs");
    tests.compile_fail("tests/ui/inspection/external_callers_cannot_mint_inspection_receipts.rs");
    tests.compile_fail(
        "tests/ui/inspection/exhaustive_matching_over_public_inspection_contract_enums_is_forbidden.rs",
    );
}

#[test]
fn ordinary_callers_cannot_import_receipt_projection_helper() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_root = isolated_helper_import_fixture(&manifest_dir);
    let fixture_manifest = fixture_root.join("Cargo.toml");
    let target_dir = fixture_root.join("target");
    let output = Command::new("cargo")
        .arg("check")
        .arg("--manifest-path")
        .arg(&fixture_manifest)
        .env("CARGO_TARGET_DIR", &target_dir)
        .current_dir(&fixture_root)
        .output()
        .expect("fixture cargo check should run");
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "ordinary callers unexpectedly imported the receipt projection helper:\n{}",
        stderr
    );
    assert!(
        stderr.contains("no `project_receipt_from_support_report` in the root"),
        "fixture failed for the wrong reason:\n{}",
        stderr
    );
}

fn isolated_helper_import_fixture(manifest_dir: &Path) -> PathBuf {
    let template_root = manifest_dir.join("tests/ui/inspection/helper_root_import_contract");
    let inspection_manifest = manifest_dir
        .parent()
        .expect("certification tests should have crate parent")
        .join("worth-ui-inspection");
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after unix epoch")
        .as_nanos();
    let fixture_root = std::env::temp_dir().join(format!(
        "worth-ui-inspection-helper-import-contract-{unique}"
    ));

    if fixture_root.exists() {
        fs::remove_dir_all(&fixture_root).expect("stale fixture directory should be removable");
    }

    fs::create_dir_all(fixture_root.join("src"))
        .expect("fixture source directory should be creatable");
    let manifest_template =
        fs::read_to_string(template_root.join("Cargo.toml")).expect("fixture manifest should read");
    let inspection_manifest = inspection_manifest.to_string_lossy().replace('\\', "/");
    let manifest_contents = manifest_template.replace(
        "../../../../../worth-ui-inspection",
        &inspection_manifest,
    );
    fs::write(fixture_root.join("Cargo.toml"), manifest_contents)
        .expect("fixture manifest should write");
    fs::copy(template_root.join("src/main.rs"), fixture_root.join("src/main.rs"))
        .expect("fixture main should copy");

    fixture_root
}

