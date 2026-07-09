#[test]
fn coverage_and_readiness_authority_cannot_be_hand_filled() {
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
            name: "coverage_row_cannot_be_struct_literal.rs",
            expected_stderr: &["PhysicalCoverageMatrixRow", "private"],
        },
        CompileFailFixture {
            name: "generated_matrix_cannot_be_struct_literal.rs",
            expected_stderr: &["GeneratedCoverageMatrix", "private"],
        },
        CompileFailFixture {
            name: "s5_readiness_cannot_be_struct_literal.rs",
            expected_stderr: &["S5SimulationHarnessReadiness", "private"],
        },
        CompileFailFixture {
            name: "s5_dependency_evidence_cannot_be_struct_literal.rs",
            expected_stderr: &["S5HarnessMaturityDependencyEvidence", "private"],
        },
        CompileFailFixture {
            name: "s5_readiness_from_generated_maturity_is_private.rs",
            expected_stderr: &["from_generated_maturity", "private"],
        },
        CompileFailFixture {
            name: "terminal_json_cannot_satisfy_coverage.rs",
            expected_stderr: &["GeneratedCoverageMatrix", "Value"],
        },
        CompileFailFixture {
            name: "mutation_coverage_cannot_be_label_minted.rs",
            expected_stderr: &["admitted_expected_failure"],
        },
        CompileFailFixture {
            name: "mutation_coverage_cannot_be_plan_only.rs",
            expected_stderr: &["from_private_mutation_denial"],
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
        .expect("certification crate lives under workspaces/worth-store/crates");
    let case_dir = compile_fail_case_dir(fixture_name);
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("ui")
            .join("s4_5")
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
        .join("worth-store-s45-coverage-ui")
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
        "[package]\nname = \"s45_coverage_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nworth-store-physical-certification = {{ path = \"{}\" }}\nworth-store-readiness = {{ path = \"{}\" }}\nserde_json = \"1\"\n",
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("worth-store")
                .join("crates")
                .join("worth-store-physical-certification")
        ),
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("worth-store")
                .join("crates")
                .join("worth-store-readiness")
        ),
    )
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
