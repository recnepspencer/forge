#[test]
fn phase22_layout_surfaces_reject_forgeable_admission_shortcuts() {
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
            name: "caller_defined_rule_cannot_open_replay_layout.rs",
            expected_stderr: &["AdmittedReplayIndexLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "admitted_replay_index_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["phase22", "AdmittedReplayIndexLayoutRule"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_crash_boundary_layout.rs",
            expected_stderr: &["AdmittedCrashBoundaryLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "admitted_readmission_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["phase22", "AdmittedReadmissionLayoutRule"],
        },
        CompileFailFixture {
            name: "recovery_source_precedence_graph_is_not_public_without_certification.rs",
            expected_stderr: &[
                "RecoverySourcePrecedenceGraph",
                "no `RecoverySourcePrecedenceGraph`",
            ],
        },
        CompileFailFixture {
            name: "partial_publication_classification_is_not_public_without_certification.rs",
            expected_stderr: &[
                "PartialPublicationClassification",
                "no `PartialPublicationClassification`",
            ],
        },
    ]
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let case_root = std::env::temp_dir()
        .join("forge-store-phase22-ui")
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
        .join("phase22")
        .join(fixture_name)
}

fn compile_fail_manifest() -> String {
    format!(
        "[package]\nname = \"phase22-ui-case\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nforge-store-recovery-physics = {{ path = {:?} }}\n",
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-recovery-physics"),
    )
}

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("layout-indexes crate lives under forge/workspaces/forge-store/crates")
        .to_path_buf()
}
