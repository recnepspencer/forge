#[test]
fn phase25_layout_surfaces_reject_forgeable_maintenance_admission_shortcuts() {
    for fixture in fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 6] {
    [
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_dedupe_layout.rs",
            expected_stderr: &["AdmittedDedupeLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_reclaim_layout.rs",
            expected_stderr: &["AdmittedReclaimLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_quarantine_layout.rs",
            expected_stderr: &["AdmittedQuarantineLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "admitted_compaction_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &[
                "AdmittedCompactionLayoutRule",
                "private associated function",
            ],
        },
        CompileFailFixture {
            name: "blob_chunks_layout_access_is_not_public_for_phase25.rs",
            expected_stderr: &["layout_access", "private module"],
        },
        CompileFailFixture {
            name: "retention_disposition_authority_is_not_public.rs",
            expected_stderr: &[
                "RetentionDispositionAuthority",
                "no `RetentionDispositionAuthority` in the root",
            ],
        },
    ]
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let case_root = std::env::temp_dir()
        .join("forge-store-phase25-ui")
        .join(fixture.name.replace('.', "_"));
    if case_root.exists() {
        std::fs::remove_dir_all(&case_root).unwrap();
    }
    let src_dir = case_root.join("src");
    std::fs::create_dir_all(&src_dir).unwrap();
    std::fs::write(case_root.join("Cargo.toml"), compile_fail_manifest()).unwrap();
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

fn fixture_path(fixture_name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ui")
        .join("phase25")
        .join(fixture_name)
}

fn compile_fail_manifest() -> String {
    format!(
        "[package]\nname = \"phase25-ui-case\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nforge-store-layout-indexes = {{ path = {:?} }}\nforge-store-blob-chunks = {{ path = {:?} }}\nforge-store-retention = {{ path = {:?} }}\n",
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-layout-indexes"),
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-blob-chunks"),
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-retention"),
    )
}

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("layout-indexes crate lives under forge/workspaces/forge-store/crates")
        .to_path_buf()
}
