use forge_foundational as _;
use forge_relational as _;
use forge_store_physical_isolation as _;

#[test]
fn semantic_visibility_cannot_satisfy_physical_read_stability_authority() {
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
            name: "transaction_id_cannot_satisfy_physical_authority.rs",
            expected_stderr: &["PhysicalReadStabilityAuthority", "TransactionId"],
        },
        CompileFailFixture {
            name: "branch_id_cannot_satisfy_physical_authority.rs",
            expected_stderr: &["PhysicalReadStabilityAuthority", "BranchId"],
        },
        CompileFailFixture {
            name: "snapshot_handle_cannot_satisfy_physical_authority.rs",
            expected_stderr: &["PhysicalReadStabilityAuthority", "SnapshotHandle"],
        },
        CompileFailFixture {
            name: "semantic_snapshot_scalar_cannot_admit_stable_read_plan.rs",
            expected_stderr: &["PhysicalEpoch", "unresolved import"],
        },
        CompileFailFixture {
            name: "semantic_reference_cannot_satisfy_physical_authority.rs",
            expected_stderr: &[
                "PhysicalReadStabilityAuthority",
                "SemanticVisibilityReference",
            ],
        },
        CompileFailFixture {
            name: "correlation_cannot_satisfy_physical_authority.rs",
            expected_stderr: &[
                "PhysicalReadStabilityAuthority",
                "PhysicalSnapshotCorrelation",
            ],
        },
        CompileFailFixture {
            name: "derived_role_claim_cannot_satisfy_physical_authority.rs",
            expected_stderr: &[
                "PhysicalReadStabilityAuthority",
                "FoundationalBoundaryRoleClaim",
            ],
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
    let case_dir = compile_fail_case_dir(fixture_name);
    let source_dir = case_dir.join("src");
    if case_dir.exists() {
        std::fs::remove_dir_all(&case_dir).unwrap();
    }
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("ui")
            .join("s5_physical_semantic_isolation")
            .join(fixture_name),
        source_dir.join("main.rs"),
    )
    .unwrap();
    case_dir
}

fn run_compile_fail_case(case_dir: &std::path::Path) -> std::process::Output {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let deps_dir = store_target_deps_dir();
    std::process::Command::new(rustc)
        .arg("--edition=2021")
        .arg(case_dir.join("src").join("main.rs"))
        .arg("--crate-name")
        .arg("s5_physical_semantic_isolation_ui")
        .arg("--crate-type")
        .arg("bin")
        .arg("-L")
        .arg(format!("dependency={}", deps_dir.display()))
        .arg("--extern")
        .arg(format!(
            "forge_foundational={}",
            newest_extern_rlib(&deps_dir, "forge_foundational").display()
        ))
        .arg("--extern")
        .arg(format!(
            "forge_relational={}",
            newest_extern_rlib(&deps_dir, "forge_relational").display()
        ))
        .arg("--extern")
        .arg(format!(
            "forge_store_physical_isolation={}",
            newest_extern_rlib(&deps_dir, "forge_store_physical_isolation").display()
        ))
        .output()
        .unwrap()
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-s5-physical-semantic-isolation-ui")
        .join(std::process::id().to_string())
        .join("cases")
        .join(fixture_name.trim_end_matches(".rs"))
}

fn store_target_deps_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under nested Store workspace")
        .join("target")
        .join("debug")
        .join("deps")
}

fn newest_extern_rlib(deps_dir: &std::path::Path, crate_name: &str) -> std::path::PathBuf {
    let prefix = format!("lib{crate_name}-");
    std::fs::read_dir(deps_dir)
        .expect("target dependency directory is readable")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(&prefix) && name.ends_with(".rlib"))
        })
        .max_by_key(|path| {
            path.metadata()
                .and_then(|metadata| metadata.modified())
                .ok()
        })
        .unwrap_or_else(|| panic!("missing compiled rlib for {crate_name}"))
}
