use forge_store_physical_isolation as _;

#[test]
fn stable_read_execution_misuse_does_not_compile() {
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
            name: "plan_cannot_start_execution.rs",
            expected_stderr: &["from_plan", "StablePhysicalReadExecution"],
        },
        CompileFailFixture {
            name: "raw_bytes_cannot_be_read_by_execution.rs",
            expected_stderr: &["read_guarded_bytes", "PhysicalByteGuard"],
        },
        CompileFailFixture {
            name: "reachability_barrier_is_not_byte_guard.rs",
            expected_stderr: &["PhysicalByteGuard", "PhysicalReadReachabilityBarrier"],
        },
        CompileFailFixture {
            name: "byte_guard_does_not_expose_raw_bytes.rs",
            expected_stderr: &["no method named", "as_bytes"],
        },
        CompileFailFixture {
            name: "raw_vec_cannot_mint_owned_read_buffer_guard.rs",
            expected_stderr: &["from_owned_read_buffer"],
        },
        CompileFailFixture {
            name: "guarded_bytes_cannot_outlive_execution_completion.rs",
            expected_stderr: &["cannot move out of", "borrowed"],
        },
        CompileFailFixture {
            name: "root_witness_cannot_satisfy_logical_decode_scope.rs",
            expected_stderr: &["LogicalDecodeSecurityScopeEntry", "CurrentPhysicalRoot"],
        },
        CompileFailFixture {
            name: "raw_bytes_cannot_enter_scoped_logical_decode.rs",
            expected_stderr: &["PhysicalByteGuard", "[u8"],
        },
        CompileFailFixture {
            name: "logical_decode_scope_entry_cannot_be_constructed.rs",
            expected_stderr: &["from_observed_scope", "private"],
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
            .join("ui")
            .join("s5_stable_read_execution")
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
        .join("forge-store-s5-read-execution-ui")
        .join(std::process::id().to_string())
        .join(fixture_name.trim_end_matches(".rs"))
}

fn write_compile_fail_manifest(case_dir: &std::path::Path) {
    std::fs::write(
        case_dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"s5-read-execution-ui\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\nforge-store-physical-isolation = {{ path = {:?} }}\n",
            physical_isolation_crate_dir(),
        ),
    )
    .unwrap();
}

fn physical_isolation_crate_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("forge-store-physical-isolation")
}
