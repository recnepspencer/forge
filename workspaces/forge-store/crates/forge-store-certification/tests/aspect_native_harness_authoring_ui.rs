#[test]
fn aspect_native_harness_public_facade_rejects_json_shortcuts() {
    for fixture in aspect_native_harness_public_facade_compile_fail_fixtures() {
        assert_public_facade_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct AspectNativeHarnessPublicFacadeCompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

fn aspect_native_harness_public_facade_compile_fail_fixtures(
) -> Vec<AspectNativeHarnessPublicFacadeCompileFailFixture> {
    vec![
        AspectNativeHarnessPublicFacadeCompileFailFixture {
            name: "terminal_json_fixture_cannot_satisfy_native_fixture.rs",
            expected_stderr: &[
                "NativeStoreAspectFixture",
                "StoreTerminalProjectionJsonFixture",
            ],
        },
        AspectNativeHarnessPublicFacadeCompileFailFixture {
            name: "hostile_json_fixture_cannot_satisfy_native_fixture.rs",
            expected_stderr: &[
                "NativeStoreAspectFixture",
                "StoreHostileReadmissionJsonFixture",
            ],
        },
        AspectNativeHarnessPublicFacadeCompileFailFixture {
            name: "terminal_json_projection_requires_terminal_suite_witness.rs",
            expected_stderr: &[
                "projection",
                "StoreTerminalProjectionJsonFixtureBoundaryWitness",
            ],
        },
        AspectNativeHarnessPublicFacadeCompileFailFixture {
            name: "hostile_json_payload_requires_hostile_suite_witness.rs",
            expected_stderr: &[
                "into_attacker_document",
                "StoreHostileReadmissionJsonFixtureBoundaryWitness",
            ],
        },
        AspectNativeHarnessPublicFacadeCompileFailFixture {
            name: "json_suite_boundary_cannot_be_self_declared.rs",
            expected_stderr: &["StoreJsonFixtureSuiteBoundary"],
        },
        AspectNativeHarnessPublicFacadeCompileFailFixture {
            name: "ordinary_prelude_does_not_export_json_macro.rs",
            expected_stderr: &["no `json` in the root"],
        },
        AspectNativeHarnessPublicFacadeCompileFailFixture {
            name: "ordinary_prelude_does_not_export_value.rs",
            expected_stderr: &["no `Value` in the root"],
        },
        AspectNativeHarnessPublicFacadeCompileFailFixture {
            name: "terminal_json_boundary_witness_cannot_be_constructed.rs",
            expected_stderr: &[
                "StoreTerminalProjectionJsonFixtureBoundaryWitness",
                "private",
            ],
        },
        AspectNativeHarnessPublicFacadeCompileFailFixture {
            name: "hostile_readmission_boundary_witness_cannot_be_constructed.rs",
            expected_stderr: &[
                "StoreHostileReadmissionJsonFixtureBoundaryWitness",
                "private",
            ],
        },
    ]
}

fn assert_public_facade_compile_fails(fixture: AspectNativeHarnessPublicFacadeCompileFailFixture) {
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
    let fixture_path = aspect_native_harness_authoring_fixture_path(&manifest_dir, fixture_name);
    let case_dir = aspect_native_harness_authoring_compile_fail_case_dir(fixture_name);
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

fn aspect_native_harness_authoring_fixture_path(
    manifest_dir: &std::path::Path,
    fixture_name: &str,
) -> std::path::PathBuf {
    manifest_dir
        .join("tests")
        .join("ui")
        .join("aspect_native_harness_authoring")
        .join(fixture_name)
}

fn aspect_native_harness_authoring_compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-native-harness-ui")
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
        .join("aspect_native_harness_authoring_ui")
        .join("target")
}

fn fixture_manifest(repo_root: &std::path::Path) -> String {
    format!(
        "[package]\nname = \"aspect_native_harness_authoring_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nforge-store-test-support = {{ path = \"{}\" }}\n",
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
