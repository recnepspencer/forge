use forge_store_physical_isolation as _;

#[test]
fn latch_acquisition_misuse_does_not_compile() {
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
            name: "raw_root_epoch_cannot_construct_latch_key.rs",
            expected_stderr: &["RootEpoch", "integer"],
        },
        CompileFailFixture {
            name: "latch_key_cannot_be_struct_literal.rs",
            expected_stderr: &["private", "kind"],
        },
        CompileFailFixture {
            name: "raw_steps_cannot_execute_as_latch_plan.rs",
            expected_stderr: &["LatchAcquisitionPlan", "Vec"],
        },
        CompileFailFixture {
            name: "upgrade_authority_cannot_be_struct_literal.rs",
            expected_stderr: &["private", "LatchUpgradeAuthority"],
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
    let case_dir = compile_fail_case_dir(fixture_name);
    let source_dir = case_dir.join("src");
    if case_dir.exists() {
        std::fs::remove_dir_all(&case_dir).unwrap();
    }
    std::fs::create_dir_all(&source_dir).unwrap();
    write_compile_fail_manifest(&case_dir);
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("compile_fail")
            .join("physical_isolation")
            .join("latch_acquisition_order")
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
        .env("CARGO_TARGET_DIR", compile_fail_case_target_dir(case_dir))
        .output()
        .unwrap()
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-s5-latch-order-ui")
        .join(std::process::id().to_string())
        .join("cases")
        .join(fixture_name.trim_end_matches(".rs"))
}

fn write_compile_fail_manifest(case_dir: &std::path::Path) {
    std::fs::write(
        case_dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"s5-latch-order-ui\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nforge-store-physical-isolation = {{ path = {:?} }}\n",
            physical_isolation_crate_dir(),
        ),
    )
    .unwrap();
}

fn compile_fail_case_target_dir(case_dir: &std::path::Path) -> std::path::PathBuf {
    case_dir.join("target")
}

fn physical_isolation_crate_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("forge-store-physical-isolation")
}
