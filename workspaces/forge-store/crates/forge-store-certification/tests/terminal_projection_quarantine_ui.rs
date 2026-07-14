#[path = "compile_fail/cargo_artifacts.rs"]
mod cargo_artifacts;

const TEST_TARGET: &str = "terminal_projection_quarantine_ui";

#[test]
fn terminal_projection_quarantine_denies_neutral_public_callers() {
    cargo_artifacts::discover(TEST_TARGET);
    for fixture in terminal_projection_quarantine_fixtures() {
        assert_compile_fails(fixture);
    }
}

struct CompileFailFixture {
    fixture_name: &'static str,
    expected_stderr: &'static str,
}

fn terminal_projection_quarantine_fixtures() -> &'static [CompileFailFixture] {
    &[
        CompileFailFixture {
            fixture_name: "store_identity_does_not_implement_display.rs",
            expected_stderr: "doesn't implement `std::fmt::Display`",
        },
        CompileFailFixture {
            fixture_name: "store_identity_has_no_neutral_string_accessor.rs",
            expected_stderr: "no method named `as_str`",
        },
        CompileFailFixture {
            fixture_name: "store_locator_has_no_neutral_string_accessor.rs",
            expected_stderr: "no method named `as_str`",
        },
        CompileFailFixture {
            fixture_name: "terminal_json_projection_cannot_satisfy_boundary_fact.rs",
            expected_stderr: "expected `StoreAspectBoundaryFact`",
        },
        CompileFailFixture {
            fixture_name: "terminal_json_projection_document_is_not_public.rs",
            expected_stderr: "no method named `terminal_projection_document`",
        },
        CompileFailFixture {
            fixture_name: "terminal_json_projection_has_no_public_document_constructor.rs",
            expected_stderr: "associated function `from_terminal_projection_document` is private",
        },
        CompileFailFixture {
            fixture_name: "terminal_json_document_checksum_cannot_satisfy_digest_authority.rs",
            expected_stderr: "found `StoreTerminalDocumentChecksum`",
        },
        CompileFailFixture {
            fixture_name: "terminal_projection_text_cannot_satisfy_identity.rs",
            expected_stderr: "expected `StoreAspectIdentity`",
        },
    ]
}

fn assert_compile_fails(fixture: &CompileFailFixture) {
    let output = compile_fail_fixture(fixture.fixture_name);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "{} unexpectedly compiled successfully",
        fixture.fixture_name
    );
    assert!(
        stderr.contains(fixture.expected_stderr),
        "{} failed for the wrong reason; expected stderr to contain {:?}, stderr was:\n{}",
        fixture.fixture_name,
        fixture.expected_stderr,
        stderr
    );
}

fn compile_fail_fixture(fixture_name: &str) -> std::process::Output {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture_path = terminal_projection_quarantine_fixture_path(&manifest_dir, fixture_name);
    let output_dir = terminal_projection_quarantine_rustc_metadata_dir(fixture_name);
    std::fs::create_dir_all(&output_dir).unwrap();
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let workspace_deps_dir = workspace_debug_deps_dir(&manifest_dir);
    let aspect_native_rlib = newest_aspect_native_rlib(&workspace_deps_dir);

    std::process::Command::new(rustc)
        .arg("--edition=2021")
        .arg("--crate-name")
        .arg(fixture_name.trim_end_matches(".rs"))
        .arg("--crate-type=bin")
        .arg("--emit=metadata")
        .arg("--out-dir")
        .arg(&output_dir)
        .arg("-L")
        .arg(format!("dependency={}", workspace_deps_dir.display()))
        .arg("--extern")
        .arg(format!(
            "forge_store_aspect_native={}",
            aspect_native_rlib.display()
        ))
        .arg(&fixture_path)
        .output()
        .unwrap()
}

fn terminal_projection_quarantine_fixture_path(
    manifest_dir: &std::path::Path,
    fixture_name: &str,
) -> std::path::PathBuf {
    manifest_dir
        .join("tests")
        .join("ui")
        .join("terminal_projection_quarantine")
        .join(fixture_name)
}

fn terminal_projection_quarantine_rustc_metadata_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-terminal-projection-quarantine-ui")
        .join("terminal_projection_quarantine_ui")
        .join(fixture_name.trim_end_matches(".rs"))
}

fn workspace_debug_deps_dir(manifest_dir: &std::path::Path) -> std::path::PathBuf {
    let _ = manifest_dir;
    cargo_artifacts::dependency_dir()
}

fn newest_aspect_native_rlib(deps_dir: &std::path::Path) -> std::path::PathBuf {
    let _ = deps_dir;
    cargo_artifacts::compiled_extern(TEST_TARGET, "forge_store_aspect_native")
}
