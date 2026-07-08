use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_root() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    std::env::temp_dir().join(format!("worth-schema-core-facade-{unique}"))
}

fn write_consumer(root: &Path) {
    fs::create_dir_all(root.join("src")).expect("create consumer crate");
    fs::write(
        root.join("Cargo.toml"),
        format!(
            "[package]\nname = \"facade-boundary-consumer\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nworth-schema-core = {{ path = {:?} }}\n",
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        ),
    )
    .expect("write consumer manifest");
    fs::write(
        root.join("src/main.rs"),
        "use worth_schema_core::identity::Identity;\n\nfn main() { let _ = Identity::Anonymous; }\n",
    )
    .expect("write consumer source");
}

#[test]
fn public_deep_import_past_facade_is_rejected() {
    let root = temp_root();
    write_consumer(&root);
    let output = Command::new("cargo")
        .arg("check")
        .current_dir(&root)
        .output()
        .expect("run cargo check");
    assert!(
        !output.status.success(),
        "deep import unexpectedly compiled"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("module `identity` is private"), "{stderr}");
}
