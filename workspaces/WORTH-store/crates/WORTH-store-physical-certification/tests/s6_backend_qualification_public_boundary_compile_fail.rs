use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn copied_rows_and_proof_authority_cannot_enter_public_publication_boundary() {
    for case in compile_fail_cases() {
        assert_compile_fails(case);
    }
}

struct CompileFailCase {
    name: &'static str,
    source: &'static str,
    stderr_fragments: &'static [&'static str],
}

fn compile_fail_cases() -> [CompileFailCase; 4] {
    [
        CompileFailCase {
            name: "copied_row_cannot_publish",
            source: include_str!(
                "ui/s6_backend_qualification_public_boundary/copied_row_cannot_publish.rs"
            ),
            stderr_fragments: &["with_row", "private"],
        },
        CompileFailCase {
            name: "proof_authority_constructor_is_private",
            source: include_str!(
                "ui/s6_backend_qualification_public_boundary/proof_authority_constructor_is_private.rs"
            ),
            stderr_fragments: &["private", "_private"],
        },
        CompileFailCase {
            name: "proof_authority_factory_is_private",
            source: include_str!(
                "ui/s6_backend_qualification_public_boundary/proof_authority_factory_is_private.rs"
            ),
            stderr_fragments: &["from_executed_store_evidence", "private"],
        },
        CompileFailCase {
            name: "row_proof_constructor_is_private",
            source: include_str!(
                "ui/s6_backend_qualification_public_boundary/row_proof_constructor_is_private.rs"
            ),
            stderr_fragments: &["from_admitted_backend_witness_with_proof", "private"],
        },
    ]
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
    fs::write(case_dir.join("Cargo.toml"), manifest(&workspace)).unwrap();

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

fn manifest(workspace: &Path) -> String {
    let workspace = workspace.display().to_string().replace('\\', "/");
    format!(
        r#"[package]
name = "qualification_boundary_compile_fail"
version = "0.0.0"
edition = "2021"

[workspace]

[dependencies]
worth-store-physical-backend = {{ path = "{}/workspaces/worth-store/crates/worth-store-physical-backend", features = ["certification-test-authority"] }}
worth-store-physical-certification = {{ path = "{}/workspaces/worth-store/crates/worth-store-physical-certification" }}
"#,
        workspace, workspace
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
