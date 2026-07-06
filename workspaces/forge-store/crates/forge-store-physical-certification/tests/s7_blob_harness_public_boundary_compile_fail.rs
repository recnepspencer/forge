use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn blob_harness_envelope_raw_constructor_is_not_public() {
    assert_compile_fails(CompileFailCase {
        name: "blob_harness_envelope_raw_constructor_is_private",
        source: include_str!(
            "ui/s7_blob_harness_public_boundary/blob_harness_envelope_raw_constructor_is_private.rs"
        ),
        stderr_fragments: &["new", "private"],
    });
}

struct CompileFailCase {
    name: &'static str,
    source: &'static str,
    stderr_fragments: &'static [&'static str],
}

fn assert_compile_fails(case: CompileFailCase) {
    let workspace = workspace_root();
    let case_dir = std::env::temp_dir().join(format!(
        "forge_store_physical_certification_{}_{}",
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
name = "s7_blob_harness_public_boundary_compile_fail"
version = "0.0.0"
edition = "2021"

[workspace]

[dependencies]
forge-store-budgets = {{ path = "{}/workspaces/forge-store/crates/forge-store-budgets" }}
"#,
        workspace
    )
}

fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(4)
        .expect("crate is under workspaces/forge-store/crates")
        .to_path_buf()
}
