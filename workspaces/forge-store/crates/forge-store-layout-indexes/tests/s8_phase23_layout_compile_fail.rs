#[test]
fn phase23_layout_surfaces_reject_forgeable_admission_shortcuts() {
    for fixture in fixtures() {
        assert_compile_fails(fixture);
    }
}

#[derive(Debug, Clone, Copy)]
struct CompileFailFixture {
    name: &'static str,
    expected_stderr: &'static [&'static str],
}

const fn fixtures() -> [CompileFailFixture; 10] {
    [
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_snapshot_layout.rs",
            expected_stderr: &["AdmittedSnapshotLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "admitted_snapshot_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["internal_phase23", "AdmittedSnapshotLayoutRule"],
        },
        CompileFailFixture {
            name: "caller_defined_rule_cannot_open_branch_delta_layout.rs",
            expected_stderr: &["AdmittedBranchDeltaLayoutRule", "private field"],
        },
        CompileFailFixture {
            name: "admitted_continuation_layout_rule_constructor_is_not_public.rs",
            expected_stderr: &["internal_phase23", "AdmittedContinuationLayoutRule"],
        },
        CompileFailFixture {
            name: "snapshot_layout_access_is_not_public.rs",
            expected_stderr: &[
                "SnapshotLayoutAccess",
                "no `SnapshotLayoutAccess` in the root",
            ],
        },
        CompileFailFixture {
            name: "branch_delta_layout_access_is_not_public.rs",
            expected_stderr: &[
                "BranchDeltaLayoutAccess",
                "no `BranchDeltaLayoutAccess` in the root",
            ],
        },
        CompileFailFixture {
            name: "published_snapshot_handle_constructor_is_not_public.rs",
            expected_stderr: &["PublishedSnapshotHandle", "new"],
        },
        CompileFailFixture {
            name: "same_branch_descendant_witness_constructor_is_not_public.rs",
            expected_stderr: &["SameBranchDescendantWitness", "new"],
        },
        CompileFailFixture {
            name: "stable_basis_read_plan_constructor_is_not_public.rs",
            expected_stderr: &["StableBasisReadPlan", "new"],
        },
        CompileFailFixture {
            name: "cursor_continuation_plan_constructor_is_not_public.rs",
            expected_stderr: &["CursorContinuationPlan", "new"],
        },
    ]
}

fn assert_compile_fails(fixture: CompileFailFixture) {
    let case_root = std::env::temp_dir()
        .join("forge-store-phase23-ui")
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
        .join("phase23")
        .join(fixture_name)
}

fn compile_fail_manifest() -> String {
    format!(
        "[package]\nname = \"phase23-ui-case\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nforge-store-layout-indexes = {{ path = {:?} }}\nforge-store-snapshots = {{ path = {:?} }}\nforge-store-branch-deltas = {{ path = {:?} }}\nforge-store-live-query = {{ path = {:?} }}\nforge-store-contracts = {{ path = {:?} }}\n",
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-layout-indexes"),
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-snapshots"),
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-branch-deltas"),
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-live-query"),
        workspace_root()
            .join("workspaces")
            .join("forge-store")
            .join("crates")
            .join("forge-store-contracts"),
    )
}

fn workspace_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("layout-indexes crate lives under forge/workspaces/forge-store/crates")
        .to_path_buf()
}
