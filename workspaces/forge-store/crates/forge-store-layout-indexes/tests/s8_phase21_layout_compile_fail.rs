#[test]
fn phase21_layout_surfaces_reject_forgeable_admission_shortcuts() {
    for fixture in fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 4] {
    [
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_wal_layout.rs",
            expected_stderr: &["durable_mutation_layout", "no method named"],
        },
        CompileFailFixture {
            name: "admitted_wal_append_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["phase21", "AdmittedWalAppendLayoutRule"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_wal_tail_layout.rs",
            expected_stderr: &["replay_tail_layout", "no method named"],
        },
        CompileFailFixture {
            name: "admitted_checkpoint_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["phase21", "AdmittedCheckpointLayoutRule"],
        },
    ]
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let case_root = prepare_case_root(fixture.name);
    let manifest_path = case_root.join("Cargo.toml");
    let src_dir = case_root.join("src");
    std::fs::create_dir_all(&src_dir).unwrap();
    std::fs::write(manifest_path, compile_fail_manifest()).unwrap();
    std::fs::copy(fixture_path(fixture.name), src_dir.join("main.rs")).unwrap();

    let output = std::process::Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(case_root.join("Cargo.toml"))
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "{} unexpectedly compiled",
        fixture.name
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    for expected in fixture.expected_stderr {
        assert!(
            stderr.contains(expected),
            "{} failed for the wrong reason; missing stderr fragment {expected:?}\nstderr:\n{stderr}",
            fixture.name
        );
    }
}

fn prepare_case_root(fixture_name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir()
        .join("forge-store-phase21-ui")
        .join(fixture_name.replace('.', "_"));
    if root.exists() {
        std::fs::remove_dir_all(&root).unwrap();
    }
    root
}

fn fixture_path(fixture_name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ui")
        .join("phase21")
        .join(fixture_name)
}

fn compile_fail_manifest() -> String {
    format!(
        "[package]\nname = \"phase21-ui-case\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nforge-store-wal = {{ path = {:?} }}\n",
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-wal"),
    )
}

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("layout-indexes crate lives under forge/workspaces/forge-store/crates")
        .to_path_buf()
}
