use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn copied_operation_cannot_satisfy_runtime_matrix() {
    let workspace = workspace_root();
    let case_dir = std::env::temp_dir().join(format!(
        "forge_store_layout_runtime_authority_{}",
        std::process::id()
    ));
    if case_dir.exists() {
        fs::remove_dir_all(&case_dir).unwrap();
    }
    fs::create_dir_all(case_dir.join("src")).unwrap();
    fs::write(
        case_dir.join("src/main.rs"),
        include_str!(
            "compile_fail/layout/runtime_authority/copied_runtime_operation_cannot_satisfy_matrix.rs"
        ),
    )
    .unwrap();
    fs::write(case_dir.join("Cargo.toml"), manifest(&workspace)).unwrap();

    let output = Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(case_dir.join("Cargo.toml"))
        .output()
        .unwrap();
    fs::remove_dir_all(&case_dir).unwrap();

    assert!(!output.status.success(), "fixture unexpectedly compiled");
    let stderr = String::from_utf8_lossy(&output.stderr);
    for fragment in ["LayoutRuntimeEvidence", "PlatformPhysicalRuntimeOperation"] {
        assert!(
            stderr.contains(fragment),
            "stderr missing {fragment:?}:\n{stderr}"
        );
    }
}

fn manifest(workspace: &Path) -> String {
    let workspace = workspace.display().to_string().replace('\\', "/");
    format!(
        r#"[package]
name = "layout_runtime_authority_compile_fail"
version = "0.0.0"
edition = "2021"

[workspace]

[dependencies]
forge-store-physical-certification = {{ path = "{0}/workspaces/forge-store/crates/forge-store-physical-certification" }}
forge-store-physical-format = {{ path = "{0}/workspaces/forge-store/crates/forge-store-physical-format" }}
"#,
        workspace
    )
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("crate is under workspaces/forge-store/crates")
        .to_path_buf()
}
