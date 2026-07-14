use worth_store_physical_isolation as _;

#[test]
fn stable_read_plan_admission_misuse_does_not_compile() {
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
            name: "handle_cannot_be_struct_literal.rs",
            expected_stderr: &["private", "StablePhysicalReadHandle"],
        },
        CompileFailFixture {
            name: "release_receipt_cannot_be_struct_literal.rs",
            expected_stderr: &["private", "footprint_basis"],
        },
        CompileFailFixture {
            name: "epoch_checked_plan_cannot_issue_handle.rs",
            expected_stderr: &["admit_stable_read_plan", "EpochCheckedStableReadPlan"],
        },
        CompileFailFixture {
            name: "raw_epoch_vector_cannot_observe_after_hazard.rs",
            expected_stderr: &["no method named", "observe_after_publication"],
        },
        CompileFailFixture {
            name: "post_hazard_root_observation_cannot_be_minted_without_hazard.rs",
            expected_stderr: &["PostHazardRootObservation"],
        },
        CompileFailFixture {
            name: "post_protection_observation_requires_published_hazard.rs",
            expected_stderr: &["from_authority_current_root"],
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
    let case_dir = compile_fail_case_dir(fixture_name);
    let source_dir = case_dir.join("src");
    if case_dir.exists() {
        std::fs::remove_dir_all(&case_dir).unwrap();
    }
    std::fs::create_dir_all(&source_dir).unwrap();
    write_compile_fail_manifest(&case_dir);
    std::fs::copy(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("compile_fail")
            .join("physical_isolation")
            .join("stable_read_plan_admission")
            .join(fixture_name),
        source_dir.join("main.rs"),
    )
    .unwrap();
    case_dir
}

fn run_compile_fail_case(case_dir: &std::path::Path) -> std::process::Output {
    std::process::Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(case_dir.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", case_dir.join("target"))
        .output()
        .unwrap()
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("worth-store-s5-read-plan-ui")
        .join(std::process::id().to_string())
        .join(fixture_name.trim_end_matches(".rs"))
}

fn write_compile_fail_manifest(case_dir: &std::path::Path) {
    std::fs::write(
        case_dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"s5-read-plan-ui\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nworth-store-physical-isolation = {{ path = {:?} }}\n",
            physical_isolation_crate_dir(),
        ),
    )
    .unwrap();
}

fn physical_isolation_crate_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("worth-store-physical-isolation")
}
