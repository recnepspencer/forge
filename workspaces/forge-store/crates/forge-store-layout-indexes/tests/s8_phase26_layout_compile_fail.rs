#[test]
fn phase26_layout_surfaces_reject_forgeable_budget_and_interference_shortcuts() {
    for fixture in fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 8] {
    [
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_maintenance_queue_layout.rs",
            expected_stderr: &["AdmittedMaintenanceQueueLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_scheduler_reservation_layout.rs",
            expected_stderr: &["AdmittedSchedulerReservationLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_tier_placement_layout.rs",
            expected_stderr: &["AdmittedTierPlacementLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_cold_recall_layout.rs",
            expected_stderr: &["AdmittedColdRecallLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_recall_amplification_layout.rs",
            expected_stderr: &["AdmittedRecallAmplificationLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_background_pacing_layout.rs",
            expected_stderr: &["AdmittedBackgroundPacingLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_foreground_interference_layout.rs",
            expected_stderr: &["AdmittedForegroundInterferenceLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "admitted_background_pacing_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &[
                "AdmittedBackgroundPacingLayoutRule",
                "private associated function",
            ],
        },
    ]
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let case_root = std::env::temp_dir()
        .join("forge-store-phase26-ui")
        .join(fixture.name.replace('.', "_"));
    if case_root.exists() {
        std::fs::remove_dir_all(&case_root).unwrap();
    }
    let src_dir = case_root.join("src");
    std::fs::create_dir_all(&src_dir).unwrap();
    std::fs::write(case_root.join("Cargo.toml"), compile_fail_manifest()).unwrap();
    std::fs::copy(fixture_path(fixture.name), src_dir.join("main.rs")).unwrap();

    let output = std::process::Command::new("cargo")
        .arg("check")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(case_root.join("Cargo.toml"))
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "{} unexpectedly compiled",
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

fn fixture_path(fixture_name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("ui")
        .join("phase26")
        .join(fixture_name)
}

fn compile_fail_manifest() -> String {
    format!(
        "[package]\nname = \"phase26-ui-case\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nforge-store-layout-indexes = {{ path = {:?} }}\nforge-store-maintenance = {{ path = {:?} }}\nforge-store-tiering = {{ path = {:?} }}\nforge-store-io-scheduler = {{ path = {:?} }}\n",
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-layout-indexes"),
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-maintenance"),
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-tiering"),
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-io-scheduler"),
    )
}

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("layout-indexes crate lives under forge/workspaces/forge-store/crates")
        .to_path_buf()
}
