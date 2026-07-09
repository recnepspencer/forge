#[test]
fn s6_later_readiness_handoff_public_boundary_rejects_substitution() {
    for case in compile_fail_cases() {
        assert_compile_fails(case);
    }
}

#[derive(Clone, Copy)]
struct CompileFailCase {
    fixture: &'static str,
    expected: &'static str,
}

fn compile_fail_cases() -> [CompileFailCase; 31] {
    [
        CompileFailCase {
            fixture: "compaction_cannot_enter_placement.rs",
            expected: "expected `S7PlacementIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "placement_cannot_enter_compaction.rs",
            expected: "expected `S10CompactionIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "compaction_cannot_enter_operator.rs",
            expected: "expected `S11OperatorIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "backup_cannot_enter_compaction.rs",
            expected: "expected `S10CompactionIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "placement_cannot_enter_backup.rs",
            expected: "expected `S10BackupExportIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "placement_cannot_enter_repair.rs",
            expected: "expected `S10RepairScanIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "placement_cannot_enter_operator.rs",
            expected: "expected `S11OperatorIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "backup_cannot_enter_placement.rs",
            expected: "expected `S7PlacementIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "backup_cannot_enter_operator.rs",
            expected: "expected `S11OperatorIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "repair_cannot_enter_backup.rs",
            expected: "expected `S10BackupExportIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "repair_cannot_enter_placement.rs",
            expected: "expected `S7PlacementIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "repair_cannot_enter_compaction.rs",
            expected: "expected `S10CompactionIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "backup_cannot_enter_repair.rs",
            expected: "expected `S10RepairScanIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "repair_cannot_enter_operator.rs",
            expected: "expected `S11OperatorIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "operator_cannot_enter_backup.rs",
            expected: "expected `S10BackupExportIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "operator_cannot_enter_placement.rs",
            expected: "expected `S7PlacementIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "operator_cannot_enter_compaction.rs",
            expected: "expected `S10CompactionIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "operator_cannot_enter_repair.rs",
            expected: "expected `S10RepairScanIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "external_code_cannot_mint_placement_handoff.rs",
            expected: "private",
        },
        CompileFailCase {
            fixture: "external_code_cannot_mint_compaction_handoff.rs",
            expected: "private",
        },
        CompileFailCase {
            fixture: "external_code_cannot_mint_backup_handoff.rs",
            expected: "private",
        },
        CompileFailCase {
            fixture: "external_code_cannot_mint_repair_handoff.rs",
            expected: "private",
        },
        CompileFailCase {
            fixture: "external_code_cannot_mint_operator_handoff.rs",
            expected: "private",
        },
        CompileFailCase {
            fixture: "external_code_cannot_mint_compaction_pacing.rs",
            expected: "private",
        },
        CompileFailCase {
            fixture: "external_code_cannot_mint_backup_pacing.rs",
            expected: "private",
        },
        CompileFailCase {
            fixture: "external_code_cannot_mint_repair_pacing.rs",
            expected: "private",
        },
        CompileFailCase {
            fixture: "certification_operator_evidence_cannot_enter_operator.rs",
            expected: "expected `S11OperatorIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "certification_placement_evidence_cannot_enter_placement.rs",
            expected: "expected `S7PlacementIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "certification_compaction_evidence_cannot_enter_compaction.rs",
            expected: "expected `S10CompactionIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "certification_backup_evidence_cannot_enter_backup.rs",
            expected: "expected `S10BackupExportIoReadinessHandoff`",
        },
        CompileFailCase {
            fixture: "certification_repair_evidence_cannot_enter_repair.rs",
            expected: "expected `S10RepairScanIoReadinessHandoff`",
        },
    ]
}

fn assert_compile_fails(case: CompileFailCase) {
    let case_dir = prepare_compile_fail_case(case.fixture);
    let output = run_compile_fail_case(&case_dir);
    assert!(
        !output.status.success(),
        "{} unexpectedly compiled",
        case.fixture
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(case.expected),
        "{} failed for the wrong reason; expected {:?}, stderr:\n{}",
        case.fixture,
        case.expected,
        stderr
    );
}

fn prepare_compile_fail_case(fixture_name: &str) -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let case_dir = manifest_dir
        .join("target")
        .join("s6-later-readiness-handoff-compile-fail")
        .join(fixture_name.trim_end_matches(".rs"));
    let _ = std::fs::remove_dir_all(&case_dir);
    std::fs::create_dir_all(case_dir.join("src")).expect("compile-fail case directory");
    std::fs::write(
        case_dir.join("Cargo.toml"),
        compile_fail_manifest(&manifest_dir),
    )
    .expect("compile-fail manifest");
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("ui")
            .join("s6_later_readiness_handoff")
            .join(fixture_name),
        case_dir.join("src").join("main.rs"),
    )
    .expect("compile-fail fixture copy");
    case_dir
}

fn compile_fail_manifest(manifest_dir: &std::path::Path) -> String {
    let crates_root = manifest_dir.parent().expect("store crates root");
    format!(
        r#"[package]
name = "s6_later_readiness_handoff_compile_fail"
edition = "2021"
version = "0.0.0"

[dependencies]
worth-store-io-scheduler = {{ path = "{}" }}
worth-store-operations = {{ path = "{}" }}
worth-store-tiering = {{ path = "{}" }}
worth-store-certification = {{ path = "{}" }}

[workspace]
"#,
        toml_path(manifest_dir),
        toml_path(&crates_root.join("worth-store-operations")),
        toml_path(&crates_root.join("worth-store-tiering")),
        toml_path(&crates_root.join("worth-store-certification")),
    )
}

fn toml_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

fn run_compile_fail_case(case_dir: &std::path::Path) -> std::process::Output {
    std::process::Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .env("CARGO_TARGET_DIR", compile_fail_target_dir(case_dir))
        .env("CARGO_BUILD_JOBS", "1")
        .current_dir(case_dir)
        .output()
        .expect("compile-fail cargo check")
}

fn compile_fail_target_dir(case_dir: &std::path::Path) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("WORTH_s6_later_handoff_cf")
        .join(case_dir.file_name().expect("compile-fail case name"))
}
