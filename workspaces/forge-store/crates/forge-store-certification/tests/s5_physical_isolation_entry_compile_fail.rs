#[test]
fn physical_isolation_physical_isolation_entry_authority_cannot_be_forged_at_compile_time() {
    for fixture in compile_fail_fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

fn compile_fail_fixtures() -> Vec<CompileFailFixture> {
    vec![
        CompileFailFixture {
            name: "entry_request_cannot_be_struct_literal.rs",
            expected_stderr: &["PhysicalIsolationEntryRequest", "private"],
        },
        CompileFailFixture {
            name: "entry_admission_cannot_be_struct_literal.rs",
            expected_stderr: &["PhysicalIsolationEntryAdmission", "private"],
        },
        CompileFailFixture {
            name: "entry_identity_cannot_be_struct_literal.rs",
            expected_stderr: &["PhysicalIsolationEntryIdentity", "private"],
        },
        CompileFailFixture {
            name: "root_epoch_basis_cannot_be_struct_literal.rs",
            expected_stderr: &["RootEpoch", "private"],
        },
        CompileFailFixture {
            name: "copied_recovery_fields_cannot_admit_entry.rs",
            expected_stderr: &["PhysicalIsolationEntryRequest"],
        },
        CompileFailFixture {
            name: "semantic_snapshot_cannot_admit_entry.rs",
            expected_stderr: &["PhysicalIsolationEntryRequest"],
        },
        CompileFailFixture {
            name: "foundational_evidence_cannot_admit_entry.rs",
            expected_stderr: &["PhysicalIsolationEntryRequest"],
        },
        CompileFailFixture {
            name: "proof_progression_cannot_admit_entry.rs",
            expected_stderr: &["PhysicalIsolationEntryRequest"],
        },
        CompileFailFixture {
            name: "physical_isolation_readiness_alone_cannot_register_lane.rs",
            expected_stderr: &["PhysicalIsolationHarnessReadinessReceipt"],
        },
        CompileFailFixture {
            name: "copied_s45_rows_cannot_register_lane.rs",
            expected_stderr: &["PhysicalIsolationHarnessReadinessReceipt"],
        },
        CompileFailFixture {
            name: "entry_evidence_cannot_register_lane.rs",
            expected_stderr: &["PhysicalIsolationHarnessReadinessReceipt"],
        },
        CompileFailFixture {
            name: "lane_registration_cannot_be_struct_literal.rs",
            expected_stderr: &["PhysicalIsolationCertificationLaneRegistration", "private"],
        },
    ]
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let case_dir = prepare_compile_fail_case(fixture.name);
    let output = run_compile_fail_case(&case_dir);
    assert!(
        !output.status.success(),
        "{} unexpectedly compiled",
        fixture.name
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    for expected in fixture.expected_stderr {
        assert!(
            stderr.contains(expected),
            "{} failed for the wrong reason; missing {expected:?}\nstderr:\n{stderr}",
            fixture.name
        );
    }
}

fn prepare_compile_fail_case(fixture_name: &str) -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .ancestors()
        .nth(4)
        .expect("certification crate lives under workspaces/forge-store/crates");
    let case_dir = compile_fail_case_dir(fixture_name);
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("ui")
            .join("physical_isolation_physical_isolation_entry")
            .join(fixture_name),
        source_dir.join("main.rs"),
    )
    .unwrap();
    std::fs::write(case_dir.join("Cargo.toml"), fixture_manifest(repo_root)).unwrap();
    case_dir
}

fn run_compile_fail_case(case_dir: &std::path::Path) -> std::process::Output {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    std::process::Command::new(cargo)
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(case_dir.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", compile_fail_case_target_dir(case_dir))
        .output()
        .unwrap()
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-s5-physical-isolation-entry-ui")
        .join(std::process::id().to_string())
        .join("cases")
        .join(fixture_name.trim_end_matches(".rs"))
}

fn compile_fail_case_target_dir(case_dir: &std::path::Path) -> std::path::PathBuf {
    case_dir
        .parent()
        .expect("compile-fail case lives under a cases directory")
        .parent()
        .expect("compile-fail cases directory lives under a process directory")
        .join("target")
}

fn fixture_manifest(repo_root: &std::path::Path) -> String {
    format!(
        "[package]\nname = \"physical_isolation_physical_isolation_entry_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nforge-store-physical-certification = {{ path = \"{}\" }}\nforge-store-physical-isolation = {{ path = \"{}\" }}\nforge-store-readiness = {{ path = \"{}\" }}\n",
        manifest_path(&repo_root.join("workspaces/forge-store/crates/forge-store-physical-certification")),
        manifest_path(&repo_root.join("workspaces/forge-store/crates/forge-store-physical-isolation")),
        manifest_path(&repo_root.join("workspaces/forge-store/crates/forge-store-readiness")),
    )
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
