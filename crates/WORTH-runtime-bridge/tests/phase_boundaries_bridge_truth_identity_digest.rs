use sha2::{Digest, Sha256};

const EXPECTED_BRIDGE_DIGEST: &str =
    "8370fa72b67cbb6c44ed8d8e40b6448a1fe10fa71b86b973e6cd9fc72d88bf24";
const EXPECTED_QUERY_RECEIPT_DIGEST: &str =
    "bfdf28c66b2916e00d1db7900db80ef7feb4110fce6e4209c535bbe1b5156d7b";
const EXPECTED_ADAPTER_DIGEST: &str =
    "f5f9d49a04fbd8b389373fbd2b60faf15a1f8a4373c8166a928f46cf8ee402eb";
const EXPECTED_WORKSPACE_RED_EXPOSURE_DIGEST: &str =
    "0b5396e93bc743a90ad78d1c0253a9ec5d5fca3b1aa85f28b59c5b9237e7b48d";
const EXPECTED_COLLAPSE_MATRIX_DIGEST: &str =
    "3fcec17b4b308dd60ba9584a3e091e1b47b488220853627cffc67499fcb3c071";

#[test]
fn bridge_truth_identity_compile_fail_boundary_digest_is_stable() {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let files = [
        "tests/phase_boundaries_bridge_truth_identity_compile_fail.rs",
        "tests/ui/bridge_truth_identity/projection_cannot_satisfy_truth_authority.rs",
        "tests/ui/bridge_truth_identity/projection_cannot_satisfy_truth_authority.stderr",
        "tests/ui/bridge_truth_identity/digest_cannot_satisfy_truth_authority.rs",
        "tests/ui/bridge_truth_identity/digest_cannot_satisfy_truth_authority.stderr",
        "tests/ui/bridge_truth_identity/external_token_cannot_satisfy_truth_authority.rs",
        "tests/ui/bridge_truth_identity/external_token_cannot_satisfy_truth_authority.stderr",
        "tests/ui/bridge_truth_identity/bridged_cannot_satisfy_current_truth_authority.rs",
        "tests/ui/bridge_truth_identity/bridged_cannot_satisfy_current_truth_authority.stderr",
        "tests/ui/bridge_truth_identity/wrong_kind_cannot_satisfy_truth_family.rs",
        "tests/ui/bridge_truth_identity/wrong_kind_cannot_satisfy_truth_family.stderr",
        "tests/ui/bridge_truth_identity/raw_text_cannot_satisfy_truth_authority.rs",
        "tests/ui/bridge_truth_identity/raw_text_cannot_satisfy_truth_authority.stderr",
        "tests/ui/bridge_truth_identity/retained_evidence_cannot_rebuild_from_text.rs",
        "tests/ui/bridge_truth_identity/retained_evidence_cannot_rebuild_from_text.stderr",
        "tests/ui/bridge_truth_identity/truth_commit_identity_string_facade_private.rs",
        "tests/ui/bridge_truth_identity/truth_commit_identity_string_facade_private.stderr",
    ];
    let mut hasher = Sha256::new();
    for file in files {
        hasher.update(file.as_bytes());
        hasher.update(b"\n");
        hasher.update(std::fs::read(manifest_dir.join(file)).expect(file));
        hasher.update(b"\n");
    }

    let digest = format!("{:x}", hasher.finalize());

    assert_eq!(digest, EXPECTED_BRIDGE_DIGEST);
}

#[test]
fn query_receipt_string_field_compile_fail_boundary_digest_is_stable() {
    let workspace = workspace_root();
    let files = [
        "crates/worth-query/tests/phase_boundaries_bridge_truth_identity_compile_fail.rs",
        "crates/worth-query/tests/ui/bridge_truth_identity/mutation_receipt_string_literal_fields_private.rs",
        "crates/worth-query/tests/ui/bridge_truth_identity/mutation_receipt_string_literal_fields_private.stderr",
        "crates/worth-query/tests/ui/bridge_truth_identity/live_patch_string_literal_fields_private.rs",
        "crates/worth-query/tests/ui/bridge_truth_identity/live_patch_string_literal_fields_private.stderr",
    ];

    let digest = combined_workspace_digest(&workspace, &files);

    assert_eq!(digest, EXPECTED_QUERY_RECEIPT_DIGEST);
}

#[test]
fn adapter_snapshot_token_compile_fail_boundary_digest_is_stable() {
    let workspace = workspace_root();
    let files = [
        "crates/worth-query/tests/ui/bridge_truth_identity/runtime_source_adapter_snapshot_token_removed.rs",
        "crates/worth-query/tests/ui/bridge_truth_identity/runtime_source_adapter_snapshot_token_removed.stderr",
        "crates/worth-query/tests/ui/bridge_truth_identity/runtime_backend_snapshot_token_removed.rs",
        "crates/worth-query/tests/ui/bridge_truth_identity/runtime_backend_snapshot_token_removed.stderr",
        "crates/worth-query/tests/ui/bridge_truth_identity/declaration_initialization_snapshot_str_removed.rs",
        "crates/worth-query/tests/ui/bridge_truth_identity/declaration_initialization_snapshot_str_removed.stderr",
    ];

    let digest = combined_workspace_digest(&workspace, &files);

    assert_eq!(digest, EXPECTED_ADAPTER_DIGEST);
}

#[test]
fn workspace_red_exposure_digest_is_stable() {
    let workspace = workspace_root();
    let digest = file_digest(
        &workspace
            .join("_docs/worth-query/fixtures/bridge_truth_identity_phase2_keepgoing_errors.txt"),
    );

    assert_eq!(digest, EXPECTED_WORKSPACE_RED_EXPOSURE_DIGEST);
}

#[test]
fn collapse_matrix_cross_check_digest_is_stable() {
    let workspace = workspace_root();
    let digest = file_digest(
        &workspace.join("_docs/worth-query/milestone-9.6-bridge-truth-identity-lowering.md"),
    );

    assert_eq!(digest, EXPECTED_COLLAPSE_MATRIX_DIGEST);
}

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates directory")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn combined_workspace_digest(workspace: &std::path::Path, files: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for file in files {
        hasher.update(file.as_bytes());
        hasher.update(b"\n");
        hasher.update(std::fs::read(workspace.join(file)).expect(file));
        hasher.update(b"\n");
    }
    format!("{:x}", hasher.finalize())
}

fn file_digest(path: &std::path::Path) -> String {
    let mut hasher = Sha256::new();
    hasher.update(std::fs::read(path).expect("digest input"));
    format!("{:x}", hasher.finalize())
}
