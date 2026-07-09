#[test]
fn driver_contract_boundary_rejects_receipt_minting_at_compile_time() {
    let output = compile_fail_fixture("yieldpoint_pause_receipt_cannot_be_minted.rs");

    assert!(!output.status.success(), "fixture unexpectedly compiled");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("YieldpointPauseReceipt") && stderr.contains("private"),
        "fixture failed for the wrong reason:\n{stderr}"
    );
}

#[test]
fn driver_contract_boundary_rejects_arbitrary_named_yieldpoint_authority() {
    let output = compile_fail_fixture("named_yieldpoint_authority_cannot_be_minted.rs");

    assert!(!output.status.success(), "fixture unexpectedly compiled");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("named_production_boundary"),
        "fixture failed for the wrong reason:\n{stderr}"
    );
}

#[test]
fn driver_contract_boundary_rejects_yieldpoint_struct_literal_minting() {
    assert_private_field_denial("physical_boundary_yieldpoint_fields_cannot_be_minted.rs");
}

#[test]
fn driver_contract_boundary_rejects_declaration_struct_literal_minting() {
    assert_private_field_denial("yieldpoint_declaration_cannot_be_minted.rs");
}

#[test]
fn driver_contract_boundary_rejects_schedule_binding_struct_literal_minting() {
    assert_private_field_denial("yieldpoint_schedule_binding_cannot_be_minted.rs");
}

#[test]
fn driver_contract_boundary_rejects_driver_struct_literal_minting() {
    assert_private_field_denial("physical_simulation_driver_cannot_be_minted.rs");
}

fn assert_private_field_denial(fixture_name: &str) {
    let output = compile_fail_fixture(fixture_name);

    assert!(!output.status.success(), "fixture unexpectedly compiled");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("private"),
        "fixture failed for the wrong reason:\n{stderr}"
    );
}

fn compile_fail_fixture(fixture_name: &str) -> std::process::Output {
    let case_dir = prepare_compile_fail_case(fixture_name);
    run_compile_fail_case(&case_dir)
}

fn prepare_compile_fail_case(fixture_name: &str) -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .ancestors()
        .nth(4)
        .expect("certification crate lives under workspaces/worth-store/crates");
    let case_dir = compile_fail_case_dir(fixture_name);
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("ui")
            .join("s4_5_driver_contract_boundary")
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
        .join("worth-store-s45-driver-contract-ui")
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
        "[package]\nname = \"s45_driver_contract_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nworth-store-physical-certification = {{ path = \"{}\" }}\n",
        repo_root
            .join("workspaces")
            .join("worth-store")
            .join("crates")
            .join("worth-store-physical-certification")
            .display()
            .to_string()
            .replace('\\', "/")
    )
}
