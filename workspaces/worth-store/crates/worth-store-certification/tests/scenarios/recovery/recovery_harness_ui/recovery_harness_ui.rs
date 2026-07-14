#[test]
fn recovery_harness_public_facade_rejects_shortcut_authority() {
    for fixture in recovery_harness_public_facade_compile_fail_fixtures() {
        assert_public_facade_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct S4RecoveryHarnessCompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

fn recovery_harness_public_facade_compile_fail_fixtures() -> Vec<S4RecoveryHarnessCompileFailFixture>
{
    vec![
        S4RecoveryHarnessCompileFailFixture {
            name: "direct_private_mutation_cannot_certify.rs",
            expected_stderr: &["RecoveryPhysicsMutationSuiteLaneEvidence", "private"],
        },
        S4RecoveryHarnessCompileFailFixture {
            name: "same_run_self_comparison_cannot_certify.rs",
            expected_stderr: &["denied", "private"],
        },
        S4RecoveryHarnessCompileFailFixture {
            name: "foundational_bundle_cannot_satisfy_recovered_state.rs",
            expected_stderr: &[
                "RecoveredPhysicalState",
                "FoundationalRecoveryEvidenceBundle",
            ],
        },
        S4RecoveryHarnessCompileFailFixture {
            name: "proof_trace_cannot_satisfy_redo_plan.rs",
            expected_stderr: &["RecoveryRedoPlan", "ProofProgressionRecoveryTrace"],
        },
        S4RecoveryHarnessCompileFailFixture {
            name: "performance_receipt_cannot_satisfy_durable_ack.rs",
            expected_stderr: &["DurableAckReceipt", "RecoveryCounterPerformanceReceipt"],
        },
    ]
}

fn assert_public_facade_compile_fails(fixture: S4RecoveryHarnessCompileFailFixture) {
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
        .expect("certification crate lives under workspaces/worth-store/crates");
    let fixture_path = recovery_harness_fixture_path(&manifest_dir, fixture_name);
    let case_dir = recovery_harness_compile_fail_case_dir(fixture_name);
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::copy(&fixture_path, source_dir.join("main.rs")).unwrap();
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

fn recovery_harness_fixture_path(
    manifest_dir: &std::path::Path,
    fixture_name: &str,
) -> std::path::PathBuf {
    manifest_dir
        .join("tests")
        .join("compile_fail")
        .join("recovery")
        .join("recovery_harness")
        .join(fixture_name)
}

fn recovery_harness_compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("worth-store-s4-recovery-harness-ui")
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
        .join("s4_recovery_harness_ui")
        .join("target")
}

fn fixture_manifest(repo_root: &std::path::Path) -> String {
    format!(
        "[package]\nname = \"s4_recovery_harness_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nworth-store-certification = {{ path = \"{}\" }}\nworth-store-recovery-physics = {{ path = \"{}\" }}\nworth-store-physical-backend = {{ path = \"{}\" }}\n",
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("worth-store")
                .join("crates")
                .join("worth-store-certification")
        ),
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("worth-store")
                .join("crates")
                .join("worth-store-recovery-physics")
        ),
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("worth-store")
                .join("crates")
                .join("worth-store-physical-backend")
        ),
    )
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
