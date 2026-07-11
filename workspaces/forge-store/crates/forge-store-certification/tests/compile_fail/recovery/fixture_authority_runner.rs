#[test]
fn fixture_authority_rejects_hand_filled_and_skipped_progression_at_compile_time() {
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
            name: "fixture_receipt_cannot_be_struct_literal.rs",
            expected_stderr: &["FixtureAuthorityReceipt", "private"],
        },
        CompileFailFixture {
            name: "production_fixture_cannot_be_struct_literal.rs",
            expected_stderr: &["ProductionBackedPhysicalFixture", "private"],
        },
        CompileFailFixture {
            name: "raw_persisted_layout_cannot_satisfy_fixture_materialization.rs",
            expected_stderr: &[
                "ProductionBackedFixtureMaterialization",
                "PersistedPhysicalLayout",
            ],
        },
        CompileFailFixture {
            name: "fixture_label_cannot_satisfy_fixture_materialization.rs",
            expected_stderr: &["ProductionBackedFixtureMaterialization", "&str"],
        },
        CompileFailFixture {
            name: "synthetic_in_memory_store_cannot_satisfy_fixture_materialization.rs",
            expected_stderr: &[
                "ProductionBackedFixtureMaterialization",
                "SyntheticInMemoryStore",
            ],
        },
        CompileFailFixture {
            name: "fixture_builder_cannot_reopen_without_boundary.rs",
            expected_stderr: &["and_reopen_through_physical_authority"],
        },
        CompileFailFixture {
            name: "raw_mutation_boundary_cannot_satisfy_capability_declaration.rs",
            expected_stderr: &["FixtureNeedsBoundary", "mutation_boundary"],
        },
        CompileFailFixture {
            name: "fixture_receipt_cannot_be_cloned.rs",
            expected_stderr: &["clone"],
        },
        CompileFailFixture {
            name: "production_fixture_cannot_be_cloned.rs",
            expected_stderr: &["clone"],
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
        .expect("certification crate lives under workspaces/forge-store/crates");
    let case_dir = compile_fail_case_dir(fixture_name);
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("compile_fail")
            .join("recovery")
            .join("fixture_authority")
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
        .join("forge-store-s45-fixture-authority-ui")
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
        "[package]\nname = \"s45_fixture_authority_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nforge-store-physical-certification = {{ path = \"{}\" }}\nforge-store-physical-format = {{ path = \"{}\" }}\n",
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
                .join("forge-store-physical-format")
        ),
    )
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
