#[test]
fn aspect_native_authority_denies_raw_public_callers() {
    for fixture in aspect_native_authority_fixtures() {
        assert_compile_fails(fixture);
    }
}

fn aspect_native_authority_fixtures() -> Vec<&'static str> {
    vec![
        "local_diagnostic_payload_cannot_satisfy_store_evidence.rs",
        "local_performance_claim_cannot_satisfy_store_evidence.rs",
        "raw_aspect_value_cannot_satisfy_boundary_fact.rs",
        "raw_string_cannot_satisfy_store_identity.rs",
        "raw_struct_cannot_satisfy_authority_input.rs",
        "terminal_projection_text_cannot_satisfy_locator.rs",
    ]
}

fn assert_compile_fails(fixture_name: &str) {
    let case_dir = prepare_compile_fail_case(fixture_name);
    let output = run_compile_fail_case(&case_dir);

    assert!(
        !output.status.success(),
        "{fixture_name} unexpectedly compiled successfully"
    );
}

fn prepare_compile_fail_case(fixture_name: &str) -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .ancestors()
        .nth(4)
        .expect("certification crate lives under workspaces/forge-store/crates");
    let fixture_path = aspect_native_authority_fixture_path(&manifest_dir, fixture_name);
    let case_dir = aspect_native_authority_compile_fail_case_dir(&manifest_dir, fixture_name);
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
        .env("CARGO_TARGET_DIR", case_dir.join("target"))
        .output()
        .unwrap()
}

fn aspect_native_authority_fixture_path(
    manifest_dir: &std::path::Path,
    fixture_name: &str,
) -> std::path::PathBuf {
    manifest_dir
        .join("tests")
        .join("ui")
        .join("aspect_native_authority")
        .join(fixture_name)
}

fn aspect_native_authority_compile_fail_case_dir(
    manifest_dir: &std::path::Path,
    fixture_name: &str,
) -> std::path::PathBuf {
    manifest_dir
        .join("target")
        .join("aspect_native_authority_ui")
        .join(fixture_name.trim_end_matches(".rs"))
}

fn fixture_manifest(repo_root: &std::path::Path) -> String {
    format!(
        "[package]\nname = \"aspect_native_authority_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nforge-foundational = {{ path = \"{}\" }}\nforge-store-aspect-native = {{ path = \"{}\" }}\n",
        repo_root.join("crates").join("forge-foundational").display(),
        repo_root
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-aspect-native")
            .display(),
    )
}
