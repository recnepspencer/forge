use worth_spatial::facade::workload_inventory::{
    InventoryDecision, InventoryValidationErrorKind, LegacyFixtureClassification, ReceiptPosture,
    SeedInventoryReport, SeedInventoryRow, SurfaceAuthority, SurfaceKind, SurfaceScope,
    TopologyPosture, WorkloadSurfaceId,
};

#[test]
fn legacy_fixture_inventory_rejects_unowned_end_to_end_claims() {
    let error = SeedInventoryReport::from_rows(vec![synthetic_mb_candidate()])
        .expect_err("spatial-only fixtures must not be accepted as MB-capable workload sources");

    assert_eq!(
        error.kind(),
        &InventoryValidationErrorKind::UnownedEndToEndClaim
    );
    assert_eq!(
        error.surface_id().as_str(),
        "synthetic::planar_overlap::storm"
    );
    assert_eq!(
        error.message(),
        "A workload candidate must be backed by Query/topology authority, not a local fixture."
    );
}

#[test]
fn legacy_fixture_inventory_rejects_workload_claim_without_production_receipt() {
    let error = SeedInventoryReport::from_rows(vec![query_backed_but_test_receipted()])
        .expect_err("Query-backed candidates still need production-owned receipts");

    assert_eq!(
        error.kind(),
        &InventoryValidationErrorKind::WorkloadCandidateWithoutProductionReceipt
    );
    assert_eq!(
        error.message(),
        "A workload candidate must carry production-owned receipts."
    );
}

#[test]
fn legacy_fixture_inventory_rejects_unit_fixture_elevation() {
    let error = SeedInventoryReport::from_rows(vec![unit_fixture_marked_for_elevation()])
        .expect_err("unit-only fixtures cannot be elevated");

    assert_eq!(
        error.kind(),
        &InventoryValidationErrorKind::UnitFixtureMarkedForElevation
    );
    assert_eq!(
        error.message(),
        "A unit-only or legacy fixture cannot be elevated as a workload source."
    );
}

#[test]
fn legacy_fixture_inventory_rejects_re_extraction_replay_as_workload_source() {
    let error = SeedInventoryReport::from_rows(vec![re_extraction_replay_workload_candidate()])
        .expect_err("re-extraction replay helpers cannot become workload sources");

    assert_eq!(
        error.kind(),
        &InventoryValidationErrorKind::UnownedEndToEndClaim
    );
    assert_eq!(
        error.message(),
        "A workload candidate must be backed by Query/topology authority, not a local fixture."
    );
}

#[test]
fn legacy_fixture_inventory_rejects_workload_candidate_without_elevation_decision() {
    let error = SeedInventoryReport::from_rows(vec![workload_candidate_without_elevation()])
        .expect_err("workload candidates must name their migration fate");

    assert_eq!(
        error.kind(),
        &InventoryValidationErrorKind::WorkloadCandidateWithoutElevationDecision
    );
    assert_eq!(
        error.message(),
        "A workload candidate must be explicitly marked for workload-platform elevation."
    );
}

#[test]
fn inventory_rows_require_source_paths() {
    let error = SeedInventoryReport::from_rows(vec![row_with_empty_source_path()])
        .expect_err("inventory decisions must name the classified source path");

    assert_eq!(
        error.kind(),
        &InventoryValidationErrorKind::MissingSourcePath
    );
    assert_eq!(
        error.message(),
        "Inventory decisions must name the source path they classify."
    );
}

#[test]
fn inventory_rows_require_human_readable_reasons() {
    let error = SeedInventoryReport::from_rows(vec![row_with_empty_reason()])
        .expect_err("inventory decisions must explain themselves");

    assert_eq!(
        error.kind(),
        &InventoryValidationErrorKind::MissingHumanReason
    );
    assert_eq!(
        error.message(),
        "Inventory decisions must include a human-readable reason."
    );
}

fn synthetic_mb_candidate() -> SeedInventoryRow {
    row(
        "synthetic::planar_overlap::storm",
        SurfaceKind::MetabossHarness,
        SurfaceAuthority::TestLocalConvenience,
        TopologyPosture::BypassesTopologyTruth,
        ReceiptPosture::TestLocal,
        SurfaceScope::WorkloadCandidate,
        InventoryDecision::LeaveUnitOnly,
        "Synthetic storm setup is spatial-only and lacks topology authority.",
    )
}

fn query_backed_but_test_receipted() -> SeedInventoryRow {
    row(
        "synthetic::query_backed_without_receipt",
        SurfaceKind::MetabossHarness,
        SurfaceAuthority::QueryBackedSpatialContract,
        TopologyPosture::ConsumesTopologyTruth,
        ReceiptPosture::TestLocal,
        SurfaceScope::WorkloadCandidate,
        InventoryDecision::LeaveUnitOnly,
        "Query-backed shape without a production receipt is not enough.",
    )
}

fn unit_fixture_marked_for_elevation() -> SeedInventoryRow {
    row(
        "synthetic::unit_fixture_elevation",
        SurfaceKind::SpatialFixture,
        SurfaceAuthority::TestLocalConvenience,
        TopologyPosture::BypassesTopologyTruth,
        ReceiptPosture::TestLocal,
        SurfaceScope::UnitSupportOnly,
        InventoryDecision::ElevateToWorkloadPlatform,
        "This tries to elevate local convenience as a workload source.",
    )
}

fn re_extraction_replay_workload_candidate() -> SeedInventoryRow {
    row(
        "synthetic::re_extraction_replay",
        SurfaceKind::ReExtractionReplayHelper,
        SurfaceAuthority::TestLocalConvenience,
        TopologyPosture::BypassesTopologyTruth,
        ReceiptPosture::TestLocal,
        SurfaceScope::WorkloadCandidate,
        InventoryDecision::ElevateToWorkloadPlatform,
        "This tries to use repeated extraction as retained replay proof.",
    )
}

fn workload_candidate_without_elevation() -> SeedInventoryRow {
    row(
        "synthetic::candidate_without_elevation",
        SurfaceKind::TopologySeed,
        SurfaceAuthority::QueryBackedTopology,
        TopologyPosture::OwnsTopologyTruth,
        ReceiptPosture::ProductionOwned,
        SurfaceScope::WorkloadCandidate,
        InventoryDecision::LeaveUnitOnly,
        "This has authority but refuses to name the required elevation decision.",
    )
}

fn row_with_empty_source_path() -> SeedInventoryRow {
    SeedInventoryRow::new(
        LegacyFixtureClassification::new(
            WorkloadSurfaceId::new("synthetic::missing_source_path"),
            SurfaceKind::SpatialFixture,
            SurfaceAuthority::TestLocalConvenience,
            TopologyPosture::BypassesTopologyTruth,
            ReceiptPosture::TestLocal,
            SurfaceScope::UnitSupportOnly,
            "This has a reason but no source path.",
        ),
        InventoryDecision::LeaveUnitOnly,
        "",
    )
}

fn row_with_empty_reason() -> SeedInventoryRow {
    row(
        "synthetic::missing_reason",
        SurfaceKind::SpatialFixture,
        SurfaceAuthority::TestLocalConvenience,
        TopologyPosture::BypassesTopologyTruth,
        ReceiptPosture::TestLocal,
        SurfaceScope::UnitSupportOnly,
        InventoryDecision::LeaveUnitOnly,
        "",
    )
}

fn row(
    surface_id: &'static str,
    surface_kind: SurfaceKind,
    authority: SurfaceAuthority,
    topology_posture: TopologyPosture,
    receipt_posture: ReceiptPosture,
    scope: SurfaceScope,
    decision: InventoryDecision,
    reason: &'static str,
) -> SeedInventoryRow {
    SeedInventoryRow::new(
        LegacyFixtureClassification::new(
            WorkloadSurfaceId::new(surface_id),
            surface_kind,
            authority,
            topology_posture,
            receipt_posture,
            scope,
            reason,
        ),
        decision,
        "synthetic test row",
    )
}
