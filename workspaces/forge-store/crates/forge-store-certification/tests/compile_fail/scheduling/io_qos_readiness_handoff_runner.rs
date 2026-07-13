#[test]
fn io_qos_readiness_handoff_denies_public_raw_materialization_paths() {
    for fixture in io_qos_readiness_handoff_fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct S6ReadinessHandoffFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

fn io_qos_readiness_handoff_fixtures() -> Vec<S6ReadinessHandoffFixture> {
    vec![
        S6ReadinessHandoffFixture {
            name: "raw_counts_cannot_materialize_readiness.rs",
            expected_stderr: &["S5CertifiedStoreExecutionCloseout"],
        },
        S6ReadinessHandoffFixture {
            name: "raw_source_cannot_publish_readiness.rs",
            expected_stderr: &["publish_scheduler_isolation_capability"],
        },
        S6ReadinessHandoffFixture {
            name: "raw_request_cannot_be_built_without_handoff_evidence.rs",
            expected_stderr: &["SchedulerIsolationCapabilityRequest"],
        },
    ]
}

fn assert_compile_fails(fixture: S6ReadinessHandoffFixture) {
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
    let fixture_path = io_qos_readiness_handoff_fixture_path(&manifest_dir, fixture_name);
    let case_dir = io_qos_readiness_handoff_compile_fail_case_dir(fixture_name);
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

fn io_qos_readiness_handoff_fixture_path(
    manifest_dir: &std::path::Path,
    fixture_name: &str,
) -> std::path::PathBuf {
    manifest_dir
        .join("tests")
        .join("compile_fail")
        .join("scheduling")
        .join("io_qos_readiness_handoff")
        .join(fixture_name)
}

fn io_qos_readiness_handoff_compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-s6-readiness-handoff-ui")
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
        .join("s6_io_qos_readiness_handoff_ui")
        .join("target")
}

fn fixture_manifest(repo_root: &std::path::Path) -> String {
    format!(
        "[package]\nname = \"s6_io_qos_readiness_handoff_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nforge-store-authority = {{ path = \"{}\" }}\nforge-store-physical-isolation = {{ path = \"{}\", features = [\"certification-authority\"] }}\n",
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("forge-store")
                .join("crates")
                .join("forge-store-authority")
        ),
        manifest_path(&repo_root.join("workspaces").join("forge-store").join("crates").join(
            "forge-store-physical-isolation"
        )),
    )
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
