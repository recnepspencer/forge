#[test]
fn courtroom_test_authority_cannot_satisfy_production_surfaces() {
    for fixture in compile_fail_fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
    include_physical_certification: bool,
}

fn compile_fail_fixtures() -> [CompileFailFixture; 4] {
    [
        CompileFailFixture {
            name: "harness_reference_is_not_physical_reference.rs",
            expected_stderr: &["PhysicalReference", "HarnessPhysicalReference"],
            include_physical_certification: true,
        },
        CompileFailFixture {
            name: "harness_reference_cannot_expose_private_reference_lane.rs",
            expected_stderr: &["no method named `as_physical_reference`"],
            include_physical_certification: true,
        },
        CompileFailFixture {
            name: "platform_physical_runtime_receipt_cannot_be_minted.rs",
            expected_stderr: &["PlatformPhysicalRuntimeReceipt", "private fields"],
            include_physical_certification: true,
        },
        CompileFailFixture {
            name: "platform_physical_runtime_denial_receipt_requires_certification_authority.rs",
            expected_stderr: &["from_append_hidden_scan_denial", "not found"],
            include_physical_certification: false,
        },
    ]
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let case_dir = prepare_compile_fail_case(fixture);
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

fn prepare_compile_fail_case(fixture: CompileFailFixture) -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .ancestors()
        .nth(4)
        .expect("test-support crate lives under workspaces/forge-store/crates");
    let case_dir = compile_fail_case_dir(fixture.name);
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::copy(
        fixture_path(&manifest_dir, fixture.name),
        source_dir.join("main.rs"),
    )
    .unwrap();
    std::fs::write(
        case_dir.join("Cargo.toml"),
        fixture_manifest(repo_root, fixture.include_physical_certification),
    )
    .unwrap();
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
        .join("ui")
        .join("harness_authority")
        .join(fixture_name)
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-test-support-harness-authority-ui")
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

fn fixture_manifest(repo_root: &std::path::Path, include_physical_certification: bool) -> String {
    let mut manifest = format!(
        "[package]\nname = \"forge_store_test_support_harness_authority_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nforge-store-contracts = {{ path = \"{}\" }}\nforge-store-physical-format = {{ path = \"{}\" }}\nforge-store-test-support = {{ path = \"{}\" }}\nforge-store-physical-certification = {{ path = \"{}\" }}\n",
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("forge-store")
                .join("crates")
                .join("forge-store-contracts")
        ),
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("forge-store")
                .join("crates")
                .join("forge-store-physical-format")
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
                .join("forge-store-physical-certification")
        ),
    );
    if !include_physical_certification {
        manifest = manifest.replace(
            &format!(
                "forge-store-test-support = {{ path = \"{}\" }}\n",
                manifest_path(
                    &repo_root
                        .join("workspaces")
                        .join("forge-store")
                        .join("crates")
                        .join("forge-store-test-support")
                )
            ),
            "",
        );
        manifest = manifest.replace(
            &format!(
                "forge-store-physical-certification = {{ path = \"{}\" }}\n",
                manifest_path(
                    &repo_root
                        .join("workspaces")
                        .join("forge-store")
                        .join("crates")
                        .join("forge-store-physical-certification")
                )
            ),
            "",
        );
    }
    manifest
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
