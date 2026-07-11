#[test]
fn forbidden_shortcut_authority_cannot_be_forged_at_compile_time() {
    for fixture in compile_fail_fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    directory: &'static str,
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

fn compile_fail_fixtures() -> Vec<CompileFailFixture> {
    vec![
        CompileFailFixture {
            directory: "s4_5_forbidden_shortcuts",
            name: "shortcut_report_cannot_be_struct_literal.rs",
            expected_stderr: &["SyntheticHarnessShortcutRejectionReport", "private"],
        },
        CompileFailFixture {
            directory: "s4_5_forbidden_shortcuts",
            name: "shortcut_receipt_cannot_be_struct_literal.rs",
            expected_stderr: &["SyntheticHarnessShortcutDenialReceipt", "private"],
        },
        CompileFailFixture {
            directory: "s4_5_forbidden_shortcuts",
            name: "raw_json_cannot_satisfy_certified_scenario.rs",
            expected_stderr: &["CertifiedPhysicalScenario", "Value"],
        },
        CompileFailFixture {
            directory: "s4_5_forbidden_shortcuts",
            name: "terminal_text_cannot_satisfy_evidence_bundle.rs",
            expected_stderr: &["PhysicalCertificationEvidenceBundle", "String"],
        },
        CompileFailFixture {
            directory: "s4_5_forbidden_shortcuts",
            name: "foundational_bundle_cannot_satisfy_store_shortcut_evidence.rs",
            expected_stderr: &[
                "PhysicalCertificationEvidenceBundle",
                "FoundationalPhysicalCertificationEvidenceBundle",
            ],
        },
        CompileFailFixture {
            directory: "s4_5_forbidden_shortcuts",
            name: "proof_recipe_cannot_satisfy_lowered_plan.rs",
            expected_stderr: &["PhysicalSimulationPlan", "Recipe"],
        },
        CompileFailFixture {
            directory: "s4_5_forbidden_shortcuts",
            name: "schedule_cannot_be_struct_literal.rs",
            expected_stderr: &["PhysicalInterleavingSchedule", "private"],
        },
        CompileFailFixture {
            directory: "s4_5_forbidden_shortcuts",
            name: "executed_transcript_parts_cannot_be_struct_literal.rs",
            expected_stderr: &["ExecutedTranscriptParts", "private"],
        },
        CompileFailFixture {
            directory: "s4_5_transcript_evidence",
            name: "transcript_cannot_be_struct_literal.rs",
            expected_stderr: &["PhysicalSimulationTranscript", "private"],
        },
        CompileFailFixture {
            directory: "s4_5_observer_oracle_boundary",
            name: "oracle_verdict_cannot_be_struct_literal.rs",
            expected_stderr: &["PhysicalProofOracleVerdict", "private"],
        },
        CompileFailFixture {
            directory: "s4_5_transcript_evidence",
            name: "copied_field_bag_cannot_construct_detached_replay_parts.rs",
            expected_stderr: &["DetachedSimulationReplayParts", "private"],
        },
        CompileFailFixture {
            directory: "s4_5",
            name: "physical_isolation_readiness_cannot_be_struct_literal.rs",
            expected_stderr: &["PhysicalIsolationHarnessReadiness", "private"],
        },
        CompileFailFixture {
            directory: "s4_5",
            name: "physical_isolation_readiness_from_generated_maturity_is_private.rs",
            expected_stderr: &["from_generated_maturity", "private"],
        },
        CompileFailFixture {
            directory: "s4_5_forbidden_shortcuts",
            name: "closeout_report_constructor_is_private.rs",
            expected_stderr: &["new", "private"],
        },
        CompileFailFixture {
            directory: "s4_5_forbidden_shortcuts",
            name: "executed_acceptance_suite_constructor_is_private.rs",
            expected_stderr: &[
                "entry_boundary_suite_run",
                "ExecutedSimulationHarnessAcceptanceSuiteEvidence",
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
    prepare_compile_fail_fixture(
        compile_fail_fixtures()
            .into_iter()
            .find(|fixture| fixture.name == fixture_name)
            .expect("fixture must be registered"),
    )
}

fn prepare_compile_fail_fixture(fixture: CompileFailFixture) -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let case_dir = compile_fail_case_dir(fixture.name);
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::copy(
        manifest_dir
            .join("tests")
            .join("ui")
            .join(fixture.directory)
            .join(fixture.name),
        source_dir.join("main.rs"),
    )
    .unwrap();
    case_dir
}

fn run_compile_fail_case(case_dir: &std::path::Path) -> std::process::Output {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let deps_dir = workspace_deps_dir();
    std::process::Command::new(rustc)
        .arg("--edition=2021")
        .arg("--crate-name")
        .arg("s45_shortcut_ui")
        .arg("--crate-type")
        .arg("bin")
        .arg(case_dir.join("src").join("main.rs"))
        .arg("-L")
        .arg(format!("dependency={}", deps_dir.display()))
        .arg("--extern")
        .arg(format!(
            "forge_store_physical_certification={}",
            latest_rlib(&deps_dir, "libforge_store_physical_certification").display()
        ))
        .arg("--extern")
        .arg(format!(
            "forge_proof={}",
            latest_rlib(&deps_dir, "libforge_proof").display()
        ))
        .arg("--extern")
        .arg(format!(
            "forge_store_readiness={}",
            latest_rlib(&deps_dir, "libforge_store_readiness").display()
        ))
        .arg("--extern")
        .arg(format!(
            "serde_json={}",
            latest_rlib(&deps_dir, "libserde_json").display()
        ))
        .arg("-o")
        .arg(case_dir.join("shortcut-ui.exe"))
        .output()
        .unwrap()
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("forge-store-s45-shortcut-ui")
        .join(std::process::id().to_string())
        .join("cases")
        .join(fixture_name.trim_end_matches(".rs"))
}

fn workspace_deps_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("certification crate lives under workspaces/forge-store/crates")
        .join("target")
        .join("debug")
        .join("deps")
}

fn latest_rlib(deps_dir: &std::path::Path, stem: &str) -> std::path::PathBuf {
    std::fs::read_dir(deps_dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "rlib")
                && path
                    .file_stem()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(stem))
        })
        .max_by_key(|path| {
            path.metadata()
                .and_then(|metadata| metadata.modified())
                .unwrap()
        })
        .unwrap_or_else(|| panic!("missing {stem} rlib in {}", deps_dir.display()))
}
