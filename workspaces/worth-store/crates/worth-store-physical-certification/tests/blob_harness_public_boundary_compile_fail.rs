use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn blob_harness_envelope_raw_constructor_is_not_public() {
    assert_compile_fails(CompileFailCase {
        name: "blob_harness_envelope_raw_constructor_is_private",
        source: include_str!(
            "ui/blob_harness/public_boundary/blob_harness_envelope_raw_constructor_is_private.rs"
        ),
        stderr_fragments: &["new", "private"],
        include_physical_certification: false,
        enable_blob_harness_certification_authority: false,
    });
}

#[test]
fn blob_harness_synthetic_replay_helpers_are_not_public_api() {
    assert_compile_fails(CompileFailCase {
        name: "blob_harness_replay_helpers_are_not_public",
        source: include_str!(
            "ui/blob_harness/public_boundary/blob_harness_replay_helpers_are_not_public.rs"
        ),
        stderr_fragments: &["replay_bundle_for_seed", "coverage_matrix_for_seed"],
        include_physical_certification: true,
        enable_blob_harness_certification_authority: false,
    });
}

#[test]
fn blob_harness_executed_witness_cannot_be_forged_from_raw_fields() {
    assert_compile_fails(CompileFailCase {
        name: "blob_harness_executed_witness_fields_are_private",
        source: include_str!(
            "ui/blob_harness/public_boundary/blob_harness_executed_witness_fields_are_private.rs"
        ),
        stderr_fragments: &["BlobHarnessExecutedWitness", "private"],
        include_physical_certification: false,
        enable_blob_harness_certification_authority: true,
    });
}

#[test]
fn blob_harness_execution_authority_is_not_public_on_default_surface() {
    assert_compile_fails(CompileFailCase {
        name: "blob_harness_execution_authority_is_not_public",
        source: include_str!(
            "ui/blob_harness/public_boundary/blob_harness_execution_authority_is_not_public.rs"
        ),
        stderr_fragments: &["execute_blob_harness", "BlobHarnessExecutionInput"],
        include_physical_certification: false,
        enable_blob_harness_certification_authority: false,
    });
}

struct CompileFailCase {
    name: &'static str,
    source: &'static str,
    stderr_fragments: &'static [&'static str],
    include_physical_certification: bool,
    enable_blob_harness_certification_authority: bool,
}

fn assert_compile_fails(case: CompileFailCase) {
    let workspace = workspace_root();
    let case_dir = std::env::temp_dir().join(format!(
        "worth_store_physical_certification_{}_{}",
        case.name,
        std::process::id()
    ));
    if case_dir.exists() {
        fs::remove_dir_all(&case_dir).unwrap();
    }
    fs::create_dir_all(case_dir.join("src")).unwrap();
    fs::write(case_dir.join("src/main.rs"), case.source).unwrap();
    fs::write(
        case_dir.join("Cargo.toml"),
        manifest(
            &workspace,
            case.include_physical_certification,
            case.enable_blob_harness_certification_authority,
        ),
    )
    .unwrap();

    let output = Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(case_dir.join("Cargo.toml"))
        .output()
        .unwrap();
    fs::remove_dir_all(&case_dir).unwrap();

    assert!(
        !output.status.success(),
        "{} unexpectedly compiled",
        case.name
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    for fragment in case.stderr_fragments {
        assert!(
            stderr.contains(fragment),
            "{} stderr missing {fragment:?}:\n{stderr}",
            case.name
        );
    }
}

fn manifest(
    workspace: &Path,
    include_physical_certification: bool,
    enable_blob_harness_certification_authority: bool,
) -> String {
    let workspace = workspace.display().to_string().replace('\\', "/");
    let physical_certification = if include_physical_certification {
        format!(
            "worth-store-physical-certification = {{ path = \"{}/workspaces/worth-store/crates/worth-store-physical-certification\" }}\n",
            workspace
        )
    } else {
        String::new()
    };
    let blob_chunk_features = if enable_blob_harness_certification_authority {
        ", features = [\"certification-test-authority\"]"
    } else {
        ""
    };
    format!(
        r#"[package]
name = "blob_harness_public_boundary_compile_fail"
version = "0.0.0"
edition = "2021"

[workspace]

[dependencies]
worth-store-blob-chunks = {{ path = "{}/workspaces/worth-store/crates/worth-store-blob-chunks"{} }}
worth-store-budgets = {{ path = "{}/workspaces/worth-store/crates/worth-store-budgets" }}
{}
"#,
        workspace, blob_chunk_features, workspace, physical_certification
    )
}

fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(4)
        .expect("crate is under workspaces/worth-store/crates")
        .to_path_buf()
}
