#[test]
fn observer_oracle_boundary_rejects_forbidden_sources_at_compile_time() {
    for fixture in observer_oracle_boundary_fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct ObserverOracleBoundaryFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

fn observer_oracle_boundary_fixtures() -> Vec<ObserverOracleBoundaryFixture> {
    vec![
        ObserverOracleBoundaryFixture {
            name: "test_support_oracle_cannot_implement_physical_oracle.rs",
            expected_stderr: &["CertificationOwnedOracle", "TestSupportOracle"],
        },
        ObserverOracleBoundaryFixture {
            name: "log_text_cannot_be_oracle.rs",
            expected_stderr: &["PhysicalProofOracle", "&str"],
        },
        ObserverOracleBoundaryFixture {
            name: "expected_error_text_cannot_be_verdict.rs",
            expected_stderr: &["PhysicalProofOracleVerdict", "&str"],
        },
        ObserverOracleBoundaryFixture {
            name: "same_run_self_comparison_cannot_be_oracle.rs",
            expected_stderr: &["PhysicalProofOracle", "SameRunSelfComparison"],
        },
        ObserverOracleBoundaryFixture {
            name: "fixture_label_cannot_be_oracle.rs",
            expected_stderr: &["PhysicalProofOracle", "FixtureLabel"],
        },
        ObserverOracleBoundaryFixture {
            name: "oracle_verdict_basis_cannot_be_struct_literal.rs",
            expected_stderr: &["OracleVerdictBasis", "private"],
        },
        ObserverOracleBoundaryFixture {
            name: "oracle_verdict_cannot_be_struct_literal.rs",
            expected_stderr: &["PhysicalProofOracleVerdict", "private"],
        },
    ]
}

fn assert_compile_fails(fixture: ObserverOracleBoundaryFixture) {
    let case_dir = prepare_compile_fail_case(fixture.name);
    let output = run_compile_fail_case(&case_dir);

    assert!(
        !output.status.success(),
        "{} unexpectedly compiled successfully",
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
        fixture_path(&manifest_dir, fixture_name),
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

fn fixture_path(manifest_dir: &std::path::Path, fixture_name: &str) -> std::path::PathBuf {
    manifest_dir
        .join("tests")
        .join("compile_fail")
        .join("recovery")
        .join("observer_oracle_boundary")
        .join(fixture_name)
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-s45-observer-oracle-ui")
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
        "[package]\nname = \"s45_observer_oracle_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nforge-store-physical-certification = {{ path = \"{}\" }}\nforge-store-test-support = {{ path = \"{}\" }}\n",
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("forge-store")
                .join("crates")
                .join("forge-store-physical-certification")
        ),
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("forge-store")
                .join("crates")
                .join("forge-store-test-support")
        ),
    )
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
