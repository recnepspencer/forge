#[test]
fn s45_entry_boundary_rejects_lower_authority_callers_at_compile_time() {
    for fixture in s45_entry_boundary_fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct S45EntryBoundaryFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

fn s45_entry_boundary_fixtures() -> Vec<S45EntryBoundaryFixture> {
    vec![
        S45EntryBoundaryFixture {
            name: "copied_report_cannot_enter.rs",
            expected_stderr: &["RecoveryPhysicsCertificationBundle", "&str"],
        },
        S45EntryBoundaryFixture {
            name: "raw_inventory_surface_cannot_be_minted.rs",
            expected_stderr: &["S45ExistingHarnessSurface", "private"],
        },
        S45EntryBoundaryFixture {
            name: "entry_identity_cannot_be_minted.rs",
            expected_stderr: &["S45SimulationHarnessEntryIdentity", "private"],
        },
        S45EntryBoundaryFixture {
            name: "entry_struct_literal_cannot_be_minted.rs",
            expected_stderr: &["S45SimulationHarnessEntry", "private"],
        },
    ]
}

fn assert_compile_fails(fixture: S45EntryBoundaryFixture) {
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
    let fixture_path = fixture_path(&manifest_dir, fixture_name);
    let case_dir = compile_fail_case_dir(fixture_name);
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

fn fixture_path(manifest_dir: &std::path::Path, fixture_name: &str) -> std::path::PathBuf {
    manifest_dir
        .join("tests")
        .join("ui")
        .join("s4_5_entry_boundary")
        .join(fixture_name)
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-s45-entry-boundary-ui")
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
        "[package]\nname = \"s45_entry_boundary_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nforge-store-physical-certification = {{ path = \"{}\" }}\n",
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("forge-store")
                .join("crates")
                .join("forge-store-physical-certification")
        ),
    )
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
