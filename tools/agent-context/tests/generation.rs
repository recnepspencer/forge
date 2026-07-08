use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn test_root() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("agent-context-test-{unique}"));
    copy_dir(
        &workspace_root().join("tools/boundary-check/config"),
        &root.join("tools/boundary-check/config"),
    );
    copy_dir(
        &workspace_root().join("cad/workspaces/worth-contracts/crates/worth-schema-core"),
        &root.join("cad/workspaces/worth-contracts/crates/worth-schema-core"),
    );
    copy_dir(
        &workspace_root().join("cad/workspaces/worth-packs/crates/worth-pack-registry"),
        &root.join("cad/workspaces/worth-packs/crates/worth-pack-registry"),
    );
    root
}

fn write_discovered_unrouted_crate(root: &Path) {
    let crate_root = root.join("cad/workspaces/worth-derived/crates/worth-derived-shadow");
    fs::create_dir_all(crate_root.join("src")).expect("create discovered crate");
    fs::write(
        crate_root.join("Cargo.toml"),
        r#"[package]
name = "worth-derived-shadow"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
    )
    .expect("write discovered manifest");
    fs::write(
        crate_root.join("src/lib.rs"),
        "pub mod facade;\n\nmod projection;\n",
    )
    .expect("write discovered lib");
    fs::write(
        crate_root.join("src/facade.rs"),
        "pub use crate::projection::ShadowProjection;\n",
    )
    .expect("write discovered facade");
    fs::write(
        crate_root.join("src/projection.rs"),
        "pub struct ShadowProjection;\n",
    )
    .expect("write discovered module");
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tools directory")
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn copy_dir(source: &Path, target: &Path) {
    fs::create_dir_all(target).expect("create target directory");
    for entry in fs::read_dir(source).expect("read source directory") {
        let entry = entry.expect("read source entry");
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        if source_path.is_dir() {
            copy_dir(&source_path, &target_path);
        } else {
            fs::create_dir_all(target_path.parent().expect("target parent"))
                .expect("create parent");
            fs::copy(&source_path, &target_path).expect("copy file");
        }
    }
}

fn run_tool(root: &Path, mode: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_agent-context"))
        .arg(mode)
        .arg("--root")
        .arg(root)
        .arg("--config")
        .arg("tools/boundary-check/config/road1.toml")
        .output()
        .expect("run agent-context")
}

#[test]
fn generation_is_stable_and_check_passes() {
    let root = test_root();
    write_discovered_unrouted_crate(&root);
    let generate = run_tool(&root, "generate");
    assert!(
        generate.status.success(),
        "{}",
        String::from_utf8_lossy(&generate.stderr)
    );

    let schema_path =
        root.join("cad/workspaces/worth-contracts/crates/worth-schema-core/AGENT_CONTEXT.md");
    let first = fs::read_to_string(&schema_path).expect("read generated context");
    assert!(
        first.contains("Canonical machine constitution: `tools/boundary-check/config/road1.toml`")
    );
    assert!(first.contains("Constitutional class: `worth/schema`"));
    assert!(first.contains(
        "Road 1 exemplar role: Road 1 foundational identity / naming / tolerance specimen"
    ));
    assert!(first.contains(
        "`worth-entry-adoption` -> Query-native declaration/adoption facade (Milestone 3)"
    ));
    assert!(first.contains("Public surface: facade-only"));

    let discovered_path =
        root.join("cad/workspaces/worth-derived/crates/worth-derived-shadow/AGENT_CONTEXT.md");
    let discovered = fs::read_to_string(&discovered_path).expect("read discovered context");
    assert!(discovered.contains("# worth-derived-shadow"));
    assert!(discovered.contains("Road 1 exemplar role: No exemplar route assigned yet."));
    assert!(discovered.contains(
        "No seed-specific skeleton allowlist is declared for this born crate; general Road 1 boundary law still applies."
    ));

    let generate_again = run_tool(&root, "generate");
    assert!(
        generate_again.status.success(),
        "{}",
        String::from_utf8_lossy(&generate_again.stderr)
    );
    let second = fs::read_to_string(&schema_path).expect("read regenerated context");
    assert_eq!(first, second);

    let check = run_tool(&root, "check");
    assert!(
        check.status.success(),
        "{}",
        String::from_utf8_lossy(&check.stderr)
    );
}

#[test]
fn stale_hand_edit_is_rejected() {
    let root = test_root();
    let generate = run_tool(&root, "generate");
    assert!(
        generate.status.success(),
        "{}",
        String::from_utf8_lossy(&generate.stderr)
    );

    let schema_path =
        root.join("cad/workspaces/worth-contracts/crates/worth-schema-core/AGENT_CONTEXT.md");
    fs::write(&schema_path, "tampered\n").expect("overwrite generated context");

    let check = run_tool(&root, "check");
    assert!(!check.status.success(), "stale context unexpectedly passed");
    assert!(String::from_utf8_lossy(&check.stderr).contains("stale or hand-edited"));
}

#[test]
fn stale_hand_edited_agent_context_is_rejected() {
    stale_hand_edit_is_rejected();
}
