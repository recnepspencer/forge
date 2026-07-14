#[test]
fn simulation_plan_boundary_rejects_lower_authority_callers_at_compile_time() {
    for fixture in simulation_plan_boundary_fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct SimulationPlanBoundaryFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

fn simulation_plan_boundary_fixtures() -> Vec<SimulationPlanBoundaryFixture> {
    vec![
        SimulationPlanBoundaryFixture {
            name: "certified_scenario_cannot_satisfy_lowered_plan.rs",
            expected_stderr: &["PhysicalSimulationPlan", "CertifiedPhysicalScenario"],
        },
        SimulationPlanBoundaryFixture {
            name: "lowered_plan_cannot_expose_source_scenario.rs",
            expected_stderr: &["scenario"],
        },
        SimulationPlanBoundaryFixture {
            name: "lowered_plan_cannot_expose_source_definition.rs",
            expected_stderr: &["definition"],
        },
        SimulationPlanBoundaryFixture {
            name: "lowered_plan_cannot_expose_source_family.rs",
            expected_stderr: &["family"],
        },
        SimulationPlanBoundaryFixture {
            name: "lowered_plan_cannot_expose_source_expectation.rs",
            expected_stderr: &["expectation"],
        },
        SimulationPlanBoundaryFixture {
            name: "lowered_plan_cannot_expose_source_fault.rs",
            expected_stderr: &["fault"],
        },
        SimulationPlanBoundaryFixture {
            name: "lowered_plan_cannot_expose_source_schedule.rs",
            expected_stderr: &["schedule"],
        },
        SimulationPlanBoundaryFixture {
            name: "lowered_plan_cannot_expose_scenario_definition.rs",
            expected_stderr: &["scenario_definition"],
        },
        SimulationPlanBoundaryFixture {
            name: "plan_struct_literal_cannot_be_minted.rs",
            expected_stderr: &["PhysicalSimulationPlan", "private"],
        },
        SimulationPlanBoundaryFixture {
            name: "copied_plan_digest_cannot_be_identity.rs",
            expected_stderr: &["PhysicalSimulationPlanIdentity", "&str"],
        },
        SimulationPlanBoundaryFixture {
            name: "json_value_cannot_satisfy_lowered_plan.rs",
            expected_stderr: &["PhysicalSimulationPlan", "Value"],
        },
        SimulationPlanBoundaryFixture {
            name: "fixture_label_cannot_satisfy_forbidden_shortcut_set.rs",
            expected_stderr: &["ForbiddenShortcutSet", "&str"],
        },
    ]
}

fn assert_compile_fails(fixture: SimulationPlanBoundaryFixture) {
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
        .expect("certification crate lives under workspaces/worth-store/crates");
    let case_dir = compile_fail_case_dir(fixture_name);
    let source_dir = case_dir.join("src");
    std::fs::create_dir_all(&source_dir).unwrap();
    std::fs::copy(
        fixture_path(&manifest_dir, fixture_name),
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

fn fixture_path(manifest_dir: &std::path::Path, fixture_name: &str) -> std::path::PathBuf {
    manifest_dir
        .join("tests")
        .join("compile_fail")
        .join("recovery")
        .join("simulation_plan_boundary")
        .join(fixture_name)
}

fn compile_fail_case_dir(fixture_name: &str) -> std::path::PathBuf {
    std::env::temp_dir()
        .join("worth-store-s45-simulation-plan-ui")
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
        "[package]\nname = \"s45_simulation_plan_ui\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nworth-store-physical-certification = {{ path = \"{}\" }}\nserde_json = \"1\"\n",
        manifest_path(
            &repo_root
                .join("workspaces")
                .join("worth-store")
                .join("crates")
                .join("worth-store-physical-certification")
        ),
    )
}

fn manifest_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}
