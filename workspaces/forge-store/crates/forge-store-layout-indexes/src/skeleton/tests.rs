use crate::skeleton::{
    S8CratePrimaryRole, S8CrateResponsibilityMap, S8CrossCrateAuthorityFlowReport,
    S8DomainSkeletonInventory, S8ForbiddenAuthoritySource, S8PhaseSkeletonObligation,
    S8ProjectionOutputPosture, S8SubsystemTopologyCloseout,
};

fn layout_indexes_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn store_workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("layout-indexes lives under workspaces/forge-store/crates")
        .to_path_buf()
}

#[test]
fn responsibility_map_covers_phase_zero_crates_and_shared_contracts() {
    let rows = S8CrateResponsibilityMap::current().rows();
    let expected = [
        "forge-store-contracts",
        "forge-store-layout-indexes",
        "forge-store-physical-format",
        "forge-store-wal",
        "forge-store-recovery-physics",
        "forge-store-buffer-pool",
        "forge-store-physical-integrity",
        "forge-store-physical-isolation",
        "forge-store-io-scheduler",
        "forge-store-blob-chunks",
        "forge-store-security",
        "forge-store-operations",
        "forge-store-maintenance",
        "forge-store-retention",
        "forge-store-tiering",
        "forge-store-snapshots",
        "forge-store-branch-deltas",
        "forge-store-compatibility",
        "forge-store-replication",
        "forge-store-bulk",
        "forge-store-live-query",
        "forge-store-offline-verifier",
        "forge-store-readiness",
        "forge-store-physical-certification",
        "forge-store-certification",
        "forge-store-test-support",
    ];

    for crate_name in expected {
        assert!(
            rows.iter().any(|row| row.crate_name() == crate_name),
            "missing responsibility row for {crate_name}",
        );
    }
}

#[test]
fn courtroom_and_terminal_lanes_are_never_production_boundary_authority() {
    for row in S8CrateResponsibilityMap::current().rows() {
        match row.primary_role() {
            S8CratePrimaryRole::PhysicalCertificationCourtroom
            | S8CratePrimaryRole::CertificationCloseoutCourtroom => {
                assert_eq!(
                    row.projection_outputs(),
                    S8ProjectionOutputPosture::CourtroomOnlyEvidence
                );
            }
            S8CratePrimaryRole::HonestTestFixtureSupport => {
                assert_eq!(
                    row.projection_outputs(),
                    S8ProjectionOutputPosture::NonAuthorityFixture
                );
            }
            S8CratePrimaryRole::TerminalOfflineObservation => {
                assert_eq!(
                    row.projection_outputs(),
                    S8ProjectionOutputPosture::TerminalObservation
                );
            }
            _ => {}
        }
    }
}

#[test]
fn phase_zero_obligation_exists_and_uses_external_compile_fail_boundary() {
    let obligations = S8PhaseSkeletonObligation::for_phase(0);
    assert_eq!(obligations.len(), 1);
    let obligation = obligations[0];
    assert_eq!(obligation.owning_crate(), "forge-store-layout-indexes");
    assert!(obligation.shortcut_proof().contains("external-crate UI"));
}

#[test]
fn inventory_and_map_stay_in_lockstep() {
    let inventory_rows = S8DomainSkeletonInventory::current().responsibility_rows();
    let map_rows = S8CrateResponsibilityMap::current().rows();
    assert_eq!(inventory_rows.len(), map_rows.len());
}

#[test]
fn topology_closeout_names_required_homes() {
    let closeout = S8SubsystemTopologyCloseout::current();
    assert!(closeout.layout_indexes_homes().contains(&"budget"));
    assert!(closeout.layout_indexes_homes().contains(&"materialization"));
    assert!(closeout.layout_indexes_homes().contains(&"handoff"));
    assert!(closeout
        .family_homes()
        .contains(&"forge-store-blob-chunks::layout_access"));
    assert!(closeout
        .courtroom_homes()
        .contains(&"forge-store-certification::s8_layout_closeout"));
}

#[test]
fn selected_phase_zero_files_exist_and_displaced_homes_are_gone() {
    let root = layout_indexes_root();
    let workspace_root = store_workspace_root();
    let actual_homes: std::collections::BTreeSet<_> = std::fs::read_dir(&root)
        .expect("layout-indexes src directory")
        .map(|entry| entry.expect("directory entry"))
        .filter(|entry| entry.file_type().expect("file type").is_dir())
        .map(|entry| {
            entry
                .file_name()
                .into_string()
                .expect("utf-8 directory name")
        })
        .collect();
    let expected_homes: std::collections::BTreeSet<_> = S8SubsystemTopologyCloseout::current()
        .layout_indexes_homes()
        .iter()
        .map(|home| home.split('/').next().expect("top-level home").to_owned())
        .collect();
    assert_eq!(
        actual_homes, expected_homes,
        "layout-indexes must expose exactly the selected Phase 0 top-level homes"
    );

    let required = [
        "access_shape/contract.rs",
        "access_shape/point.rs",
        "access_shape/range.rs",
        "access_shape/prefix.rs",
        "access_shape/scan.rs",
        "access_shape/streaming.rs",
        "access_shape/append.rs",
        "access_shape/verifier.rs",
        "access_shape/repair.rs",
        "access_shape/quarantine.rs",
        "access_shape/denial.rs",
        "planning/lowering_request.rs",
        "planning/alternative.rs",
        "planning/selection_policy.rs",
        "planning/plan_fingerprint.rs",
        "planning/selection_receipt.rs",
        "maintenance/rebuild.rs",
        "maintenance/parity.rs",
        "maintenance/mutation_plan.rs",
        "maintenance/maintenance_mode.rs",
        "maintenance/publication_protocol.rs",
        "maintenance/lag.rs",
        "maintenance/failure.rs",
        "migration/version.rs",
        "migration/compatibility.rs",
        "migration/migration_plan.rs",
        "migration/rollback_plan.rs",
        "migration/stale_rebind.rs",
        "strategy/invariant_suite.rs",
        "strategy/denial.rs",
        "facade/declarations.rs",
        "facade/key_domains.rs",
        "facade/maintenance.rs",
        "facade/migration.rs",
        "facade/readmission.rs",
        "facade/closeout.rs",
    ];

    for relative_path in required {
        assert!(
            root.join(relative_path).is_file(),
            "missing selected skeleton file {relative_path}"
        );
    }

    for displaced in [
        "rebuild/mod.rs",
        "closeout/mod.rs",
        "readmission/mod.rs",
        "facade/layout_closeout.rs",
    ] {
        assert!(
            !root.join(displaced).exists(),
            "displaced home still exists at {displaced}"
        );
    }

    let physical_certification_root_alias = workspace_root
        .join("crates")
        .join("forge-store-physical-certification")
        .join("src")
        .join("s8_layout_access.rs");
    assert!(
        !physical_certification_root_alias.exists(),
        "physical-certification still exposes a root S.8 alias outside the milestone home"
    );
}

#[test]
fn authority_flow_forbids_non_production_sources() {
    let report = S8CrossCrateAuthorityFlowReport::current();
    assert_eq!(report.required_edges().len(), 4);
    assert!(report
        .forbidden_sources()
        .contains(&S8ForbiddenAuthoritySource::FoundationalMaterializedReport));
    assert!(report
        .forbidden_sources()
        .contains(&S8ForbiddenAuthoritySource::TerminalProjection));
}

#[test]
fn family_and_courtroom_phase_zero_homes_physically_exist() {
    let workspace_root = store_workspace_root();
    let closeout = S8SubsystemTopologyCloseout::current();

    for relative_path in closeout.family_required_files() {
        let path = workspace_root.join(relative_path);
        assert!(
            path.is_file(),
            "missing required family skeleton file {}",
            path.display()
        );
    }

    for relative_path in closeout.courtroom_required_files() {
        let path = workspace_root.join(relative_path);
        assert!(
            path.is_file(),
            "missing required courtroom/test skeleton file {}",
            path.display()
        );
    }
}

#[test]
fn responsibility_rows_are_complete_and_point_to_real_phase_zero_homes() {
    let workspace_root = store_workspace_root();

    for row in S8CrateResponsibilityMap::current().rows() {
        assert!(
            !row.minted_authority().trim().is_empty(),
            "{} must declare minted authority",
            row.crate_name()
        );
        assert!(
            !row.consumed_authority().trim().is_empty(),
            "{} must declare consumed authority",
            row.crate_name()
        );
        assert!(
            !row.public_facade_home().trim().is_empty(),
            "{} must declare a facade home",
            row.crate_name()
        );
        assert!(
            !row.phase_obligations().is_empty(),
            "{} must carry S.8 phase obligations",
            row.crate_name()
        );
        assert!(
            row.phase_obligations().contains(&0),
            "{} must carry phase 0 obligations",
            row.crate_name()
        );

        if row.crate_name() == "forge-store-contracts" {
            assert_eq!(row.consumed_authority(), "none");
        } else {
            assert_ne!(
                row.consumed_authority(),
                "none",
                "{} must consume a named authority lane",
                row.crate_name()
            );
        }

        let crate_dir = workspace_root.join("crates").join(row.crate_name());
        assert!(
            crate_dir.is_dir(),
            "responsibility row points at missing crate directory {}",
            crate_dir.display()
        );
        assert!(
            crate_dir.join("src").join("lib.rs").is_file(),
            "{} must have a real crate root",
            row.crate_name()
        );

        match row.primary_role() {
            S8CratePrimaryRole::LayoutAccessGrammar => {
                assert_eq!(
                    row.public_facade_home(),
                    "forge_store_layout_indexes::{layout_declarations,strategy_admission,access_planning,layout_readmission,layout_closeout}"
                );
                assert!(crate_dir.join("src").join("facade").is_dir());
            }
            S8CratePrimaryRole::FamilyExecutionAuthority => {
                assert!(
                    row.public_facade_home().ends_with("::layout_access"),
                    "{} must expose its local layout_access home",
                    row.crate_name()
                );
                assert!(crate_dir.join("src").join("layout_access").is_dir());
            }
            S8CratePrimaryRole::PhysicalCertificationCourtroom => {
                assert_eq!(
                    row.public_facade_home(),
                    "forge_store_physical_certification::harness::by_milestone::s8_layout_access"
                );
                assert!(crate_dir
                    .join("src")
                    .join("harness")
                    .join("by_milestone")
                    .join("s8_layout_access")
                    .is_dir());
            }
            S8CratePrimaryRole::CertificationCloseoutCourtroom => {
                assert_eq!(
                    row.public_facade_home(),
                    "forge_store_certification::s8_layout_closeout"
                );
                assert!(crate_dir.join("src").join("s8_layout_closeout").is_dir());
            }
            S8CratePrimaryRole::HonestTestFixtureSupport => {
                assert_eq!(
                    row.public_facade_home(),
                    "forge_store_test_support::harness::milestone::s8_layout_access"
                );
                assert!(crate_dir
                    .join("src")
                    .join("harness")
                    .join("milestone")
                    .join("s8_layout_access")
                    .is_dir());
            }
            _ => {}
        }
    }
}
