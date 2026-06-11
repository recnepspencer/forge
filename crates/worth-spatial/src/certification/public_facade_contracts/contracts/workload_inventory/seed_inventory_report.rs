use worth_spatial::facade::workload_inventory::{
    InventoryDecision, ReceiptPosture, SeedInventoryReport, SurfaceAuthority, SurfaceKind,
    SurfaceScope, TopologyPosture,
};

#[test]
fn workload_seed_inventory_classifies_existing_topology_and_spatial_setup() {
    let report = SeedInventoryReport::certify_existing_surfaces()
        .expect("existing seed inventory should certify");

    assert_eq!(report.counters().registered_surfaces(), 24);
    assert_eq!(report.counters().query_backed_surfaces(), 3);
    assert_eq!(report.counters().production_receipt_surfaces(), 3);
    assert_eq!(report.counters().workload_candidates(), 3);
    assert_eq!(report.counters().unit_only_fixtures(), 10);
    assert_eq!(report.counters().legacy_migration_surfaces(), 11);
    assert_eq!(report.counters().test_local_surfaces(), 21);
    assert!(report.assert_every_surface_has_human_readable_decision());

    assert_real_topology_seed(&report, "MinimalTopologySeed");
    assert_real_topology_commit(&report);
    assert_primitive_corpus(&report);
    assert_planar_proof_fixture_rows(&report);
    assert_local_fixture(&report, "planar_overlap::runtime_handles");
    assert_metaboss_support_rows(&report);
    assert_legacy_migration(
        &report,
        "planar_overlap::metaboss::certify_storm_with_retained_replay",
    );
    assert_legacy_migration(&report, "planar_m6_closeout::fixture");
}

#[test]
fn workload_seed_inventory_covers_every_current_planar_proof_fixture_file() {
    let report = SeedInventoryReport::certify_existing_surfaces()
        .expect("existing seed inventory should certify");
    let workspace_root = workspace_root();
    let contracts_root = workspace_root
        .join("crates/worth-spatial/src/certification/public_facade_contracts/contracts");

    for proof_fixture_path in discover_named_files(&contracts_root, "proof_fixture.rs") {
        let source_path = relative_workspace_path(&workspace_root, &proof_fixture_path);
        let row = report
            .rows()
            .iter()
            .find(|row| row.source_path() == source_path)
            .unwrap_or_else(|| panic!("missing inventory row for {source_path}"));

        assert_eq!(
            row.classification().scope(),
            SurfaceScope::UnitSupportOnly,
            "{source_path} must remain unit support until elevated by workload platform work"
        );
        assert_eq!(
            row.decision(),
            InventoryDecision::WrapAsLocalUnitSupport,
            "{source_path} must name its local fixture migration fate"
        );
    }
}

#[test]
fn workload_seed_inventory_covers_every_current_mb_overlap_support_file() {
    let report = SeedInventoryReport::certify_existing_surfaces()
        .expect("existing seed inventory should certify");
    let workspace_root = workspace_root();
    let metaboss_root = workspace_root.join(
        "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss",
    );

    for support_path in discover_rust_files_except_module(&metaboss_root) {
        let source_path = relative_workspace_path(&workspace_root, &support_path);
        let row = report
            .rows()
            .iter()
            .find(|row| row.source_path() == source_path)
            .unwrap_or_else(|| panic!("missing inventory row for {source_path}"));

        assert_eq!(
            row.classification().scope(),
            SurfaceScope::LegacyMigrationOnly,
            "{source_path} must be fenced as legacy migration support"
        );
        assert_eq!(
            row.decision(),
            InventoryDecision::DeleteAfterReplacement,
            "{source_path} must name its replacement fate"
        );
    }
}

#[test]
fn workload_seed_inventory_source_paths_stay_real() {
    let report = SeedInventoryReport::certify_existing_surfaces()
        .expect("existing seed inventory should certify");
    let workspace_root = workspace_root();

    for row in report.rows() {
        let path = workspace_root.join(row.source_path());
        assert!(
            path.exists(),
            "inventory source path should exist: {}",
            row.source_path()
        );
    }
}

fn assert_real_topology_seed(report: &SeedInventoryReport, surface: &str) {
    let row = report
        .require_surface(surface)
        .expect("topology seed row should be present");

    assert_eq!(
        row.classification().surface_kind(),
        SurfaceKind::TopologySeed
    );
    assert_eq!(
        row.classification().authority(),
        SurfaceAuthority::QueryBackedTopology
    );
    assert_eq!(
        row.classification().topology_posture(),
        TopologyPosture::OwnsTopologyTruth
    );
    assert_eq!(
        row.classification().receipt_posture(),
        ReceiptPosture::ProductionOwned
    );
    assert_eq!(
        row.classification().scope(),
        SurfaceScope::WorkloadCandidate
    );
    assert_eq!(row.decision(), InventoryDecision::ElevateToWorkloadPlatform);
    assert!(!row.classification().human_reason().is_empty());
}

fn assert_real_topology_commit(report: &SeedInventoryReport) {
    let row = report
        .require_surface("SeededTopologyCommit")
        .expect("topology commit row should be present");

    assert_eq!(
        row.classification().surface_kind(),
        SurfaceKind::TopologyCommit
    );
    assert_eq!(
        row.classification().authority(),
        SurfaceAuthority::QueryBackedTopology
    );
    assert_eq!(
        row.classification().topology_posture(),
        TopologyPosture::OwnsTopologyTruth
    );
    assert_eq!(
        row.classification().receipt_posture(),
        ReceiptPosture::ProductionOwned
    );
    assert_eq!(
        row.classification().scope(),
        SurfaceScope::WorkloadCandidate
    );
    assert_eq!(row.decision(), InventoryDecision::ElevateToWorkloadPlatform);
}

fn assert_primitive_corpus(report: &SeedInventoryReport) {
    let row = report
        .require_surface("worth-topo primitive corpus")
        .expect("primitive corpus row should be present");

    assert_eq!(
        row.classification().surface_kind(),
        SurfaceKind::PrimitiveCorpus
    );
    assert_eq!(
        row.classification().topology_posture(),
        TopologyPosture::ConsumesTopologyTruth
    );
    assert_eq!(row.decision(), InventoryDecision::ElevateToWorkloadPlatform);
}

fn assert_planar_proof_fixture_rows(report: &SeedInventoryReport) {
    for surface in [
        "planar_predicate::proof_fixture",
        "planar_precision::proof_fixture",
        "planar_local_frame::proof_fixture",
        "planar_projection::proof_fixture",
        "planar_segment_segment::proof_fixture",
        "planar_winding::proof_fixture",
        "planar_signed_area::proof_fixture",
        "planar_overlap::proof_fixture",
        "planar_contract_bundle::proof_fixture",
    ] {
        assert_local_fixture(report, surface);
    }
}

fn assert_metaboss_support_rows(report: &SeedInventoryReport) {
    for surface in [
        "planar_overlap::metaboss::scenario",
        "planar_overlap::metaboss::outcome_matrix",
        "planar_overlap::metaboss::proof",
        "planar_overlap::metaboss::coplanar_overlap_storm",
        "planar_overlap::metaboss::diagnostics",
        "planar_overlap::metaboss::platform_storm_subject",
        "planar_overlap::metaboss::storm_extraction_subject",
        "planar_overlap::metaboss::high_valence_singularity",
        "planar_overlap::metaboss::high_valence_subject",
    ] {
        assert_legacy_migration(report, surface);
    }
}

fn assert_local_fixture(report: &SeedInventoryReport, surface: &str) {
    let row = report
        .require_surface(surface)
        .expect("local fixture row should be present");

    assert_eq!(
        row.classification().authority(),
        SurfaceAuthority::TestLocalConvenience
    );
    assert_eq!(
        row.classification().topology_posture(),
        TopologyPosture::BypassesTopologyTruth
    );
    assert_eq!(row.classification().scope(), SurfaceScope::UnitSupportOnly);
    assert_ne!(row.decision(), InventoryDecision::ElevateToWorkloadPlatform);
}

fn assert_legacy_migration(report: &SeedInventoryReport, surface: &str) {
    let row = report
        .require_surface(surface)
        .expect("legacy migration row should be present");

    assert_eq!(
        row.classification().authority(),
        SurfaceAuthority::TestLocalConvenience
    );
    assert_eq!(
        row.classification().scope(),
        SurfaceScope::LegacyMigrationOnly
    );
    assert_eq!(row.decision(), InventoryDecision::DeleteAfterReplacement);
}

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|crates| crates.parent())
        .expect("worth-spatial should live under workspace/crates")
        .to_path_buf()
}

fn discover_named_files(root: &std::path::Path, file_name: &str) -> Vec<std::path::PathBuf> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(root).expect("read discovery root") {
        let path = entry.expect("read discovery entry").path();
        if path.is_dir() {
            found.extend(discover_named_files(&path, file_name));
        } else if path.file_name().and_then(|name| name.to_str()) == Some(file_name) {
            found.push(path);
        }
    }
    found.sort();
    found
}

fn discover_rust_files_except_module(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(root).expect("read MB support root") {
        let path = entry.expect("read MB support entry").path();
        let file_name = path.file_name().and_then(|name| name.to_str());
        if path.extension().and_then(|extension| extension.to_str()) == Some("rs")
            && file_name != Some("mod.rs")
        {
            found.push(path);
        }
    }
    found.sort();
    found
}

fn relative_workspace_path(workspace_root: &std::path::Path, path: &std::path::Path) -> String {
    path.strip_prefix(workspace_root)
        .expect("path should live under workspace root")
        .to_string_lossy()
        .replace('\\', "/")
}
