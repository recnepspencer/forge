use worth_spatial::facade::workload_inventory::{
    InventoryDecision, ReceiptPosture, SeedInventoryReport, SurfaceScope,
};

#[test]
fn kernel_consumes_seed_inventory_without_promoting_local_fixtures() {
    let report = SeedInventoryReport::certify_existing_surfaces()
        .expect("kernel should consume certified seed inventory");

    let topology_seed = report
        .require_surface("MinimalTopologySeed")
        .expect("MinimalTopologySeed should be classified");
    assert_eq!(
        topology_seed.decision(),
        InventoryDecision::ElevateToWorkloadPlatform
    );
    assert_eq!(
        topology_seed.classification().receipt_posture(),
        ReceiptPosture::ProductionOwned
    );
    assert_eq!(
        topology_seed.classification().scope(),
        SurfaceScope::WorkloadCandidate
    );

    let overlap_fixture = report
        .require_surface("planar_overlap::proof_fixture")
        .expect("planar overlap fixture should be classified");
    assert_eq!(
        overlap_fixture.decision(),
        InventoryDecision::WrapAsLocalUnitSupport
    );
    assert_eq!(
        overlap_fixture.classification().scope(),
        SurfaceScope::UnitSupportOnly
    );

    for surface in [
        "planar_overlap::metaboss::outcome_matrix",
        "planar_overlap::metaboss::coplanar_overlap_storm",
        "planar_overlap::metaboss::diagnostics",
        "planar_overlap::metaboss::platform_storm_subject",
        "planar_overlap::metaboss::storm_extraction_subject",
        "planar_overlap::metaboss::high_valence_singularity",
        "planar_overlap::metaboss::high_valence_subject",
    ] {
        let metaboss_harness = report
            .require_surface(surface)
            .expect("legacy MB support should be classified");
        assert_eq!(
            metaboss_harness.decision(),
            InventoryDecision::DeleteAfterReplacement
        );
        assert_eq!(
            metaboss_harness.classification().scope(),
            SurfaceScope::LegacyMigrationOnly
        );
    }

    assert_eq!(report.counters().workload_candidates(), 3);
    assert_eq!(report.counters().unit_only_fixtures(), 10);
    assert_eq!(report.counters().legacy_migration_surfaces(), 8);
}
