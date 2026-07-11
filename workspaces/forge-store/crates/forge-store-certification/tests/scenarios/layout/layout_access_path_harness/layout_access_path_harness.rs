#[test]
fn scenario_metadata_alone_cannot_form_layout_index_layout_replay_bundle() {
    for fixture in compile_fail_fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

fn compile_fail_fixtures() -> [CompileFailFixture; 2] {
    [
        CompileFailFixture {
            name: "scenario_certificate_cannot_form_replay_bundle.rs",
            expected_stderr: &[
                "S8LayoutExecutedScenarioWitness",
                "S8LayoutScenarioDefinition",
            ],
        },
        CompileFailFixture {
            name: "direct_executed_witness_construction_is_not_public.rs",
            expected_stderr: &["admit_layout_index_layout_executed_scenario", "private"],
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
        .join("layout")
        .join("layout_access_path_harness")
        .join(fixture_name)
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-certification-new-layout-access-path")
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
        "[package]\nname = \"forge_store_certification_s8_layout_access_path\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nforge-store-certification = {{ path = \"{}\" }}\nforge-store-physical-certification = {{ path = \"{}\" }}\nforge-store-test-support = {{ path = \"{}\" }}\nforge-store-layout-indexes = {{ path = \"{}\" }}\n",
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("forge-store")
                .join("crates")
                .join("forge-store-certification")
        ),
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
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("forge-store")
                .join("crates")
                .join("forge-store-layout-indexes")
        ),
    )
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
