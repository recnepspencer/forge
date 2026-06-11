use super::classification::{
    LegacyFixtureClassification, ReceiptPosture, SurfaceAuthority, SurfaceKind, SurfaceScope,
    TopologyPosture, WorkloadSurfaceId,
};
use super::decision::InventoryDecision;
use super::report::SeedInventoryRow;

pub fn existing_seed_inventory_rows() -> Vec<SeedInventoryRow> {
    let mut rows = vec![
        row(
            "MinimalTopologySeed",
            SurfaceKind::TopologySeed,
            SurfaceAuthority::QueryBackedTopology,
            TopologyPosture::OwnsTopologyTruth,
            ReceiptPosture::ProductionOwned,
            SurfaceScope::WorkloadCandidate,
            InventoryDecision::ElevateToWorkloadPlatform,
            "crates/worth-schema/src/data/seed/types.rs",
            "Minimal topology seed carries persisted truth, read basis, read artifact, and certified interpretation.",
        ),
        row(
            "SeededTopologyCommit",
            SurfaceKind::TopologyCommit,
            SurfaceAuthority::QueryBackedTopology,
            TopologyPosture::OwnsTopologyTruth,
            ReceiptPosture::ProductionOwned,
            SurfaceScope::WorkloadCandidate,
            InventoryDecision::ElevateToWorkloadPlatform,
            "crates/worth-schema/src/data/seed/types.rs",
            "Seeded topology commit is created from a verified topology commit and exposes committed mutation/read-basis authority.",
        ),
        row(
            "worth-topo primitive corpus",
            SurfaceKind::PrimitiveCorpus,
            SurfaceAuthority::QueryBackedTopology,
            TopologyPosture::ConsumesTopologyTruth,
            ReceiptPosture::ProductionOwned,
            SurfaceScope::WorkloadCandidate,
            InventoryDecision::ElevateToWorkloadPlatform,
            "crates/worth-topo/src/test_support/primitive_corpus",
            "Primitive corpus support already builds through topology seed witnesses and should feed the workload platform.",
        ),
        row(
            "planar_m6_closeout::fixture",
            SurfaceKind::CloseoutEvidenceFixture,
            SurfaceAuthority::TestLocalConvenience,
            TopologyPosture::BypassesTopologyTruth,
            ReceiptPosture::TestLocal,
            SurfaceScope::LegacyMigrationOnly,
            InventoryDecision::DeleteAfterReplacement,
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_m6_closeout/fixture.rs",
            "M6 closeout rows record milestone evidence but must not be mistaken for reusable workload construction.",
        ),
    ];
    rows.extend(planar_proof_fixture_rows());
    rows.extend(planar_metaboss_support_rows());
    rows.extend(planar_replay_and_runtime_fixture_rows());
    rows
}

fn planar_proof_fixture_rows() -> Vec<SeedInventoryRow> {
    [
        (
            "planar_predicate::proof_fixture",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_predicate/proof_fixture.rs",
        ),
        (
            "planar_precision::proof_fixture",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_precision/proof_fixture.rs",
        ),
        (
            "planar_local_frame::proof_fixture",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_local_frame/proof_fixture.rs",
        ),
        (
            "planar_projection::proof_fixture",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_projection/proof_fixture.rs",
        ),
        (
            "planar_segment_segment::proof_fixture",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_segment_segment/proof_fixture.rs",
        ),
        (
            "planar_winding::proof_fixture",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_winding/proof_fixture.rs",
        ),
        (
            "planar_signed_area::proof_fixture",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_signed_area/proof_fixture.rs",
        ),
        (
            "planar_overlap::proof_fixture",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/proof_fixture.rs",
        ),
        (
            "planar_contract_bundle::proof_fixture",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_contract_bundle/proof_fixture.rs",
        ),
    ]
    .into_iter()
    .map(|(surface_id, source_path)| {
        row(
            surface_id,
            SurfaceKind::SpatialFixture,
            SurfaceAuthority::TestLocalConvenience,
            TopologyPosture::BypassesTopologyTruth,
            ReceiptPosture::TestLocal,
            SurfaceScope::UnitSupportOnly,
            InventoryDecision::WrapAsLocalUnitSupport,
            source_path,
            "Planar proof fixtures may stay as local unit support but cannot become workload authority.",
        )
    })
    .collect()
}

fn planar_metaboss_support_rows() -> Vec<SeedInventoryRow> {
    [
        (
            "planar_overlap::metaboss::scenario",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss/scenario.rs",
        ),
        (
            "planar_overlap::metaboss::outcome_matrix",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss/outcome_matrix.rs",
        ),
        (
            "planar_overlap::metaboss::proof",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss/proof.rs",
        ),
        (
            "planar_overlap::metaboss::coplanar_overlap_storm",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss/coplanar_overlap_storm.rs",
        ),
        (
            "planar_overlap::metaboss::diagnostics",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss/diagnostics.rs",
        ),
        (
            "planar_overlap::metaboss::platform_storm_subject",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss/platform_storm_subject.rs",
        ),
        (
            "planar_overlap::metaboss::storm_extraction_subject",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss/storm_extraction_subject.rs",
        ),
        (
            "planar_overlap::metaboss::high_valence_singularity",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss/high_valence_singularity.rs",
        ),
        (
            "planar_overlap::metaboss::high_valence_subject",
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss/high_valence_subject.rs",
        ),
    ]
    .into_iter()
    .map(|(surface_id, source_path)| {
        row(
            surface_id,
            SurfaceKind::MetabossHarness,
            SurfaceAuthority::TestLocalConvenience,
            TopologyPosture::BypassesTopologyTruth,
            ReceiptPosture::TestLocal,
            SurfaceScope::LegacyMigrationOnly,
            InventoryDecision::DeleteAfterReplacement,
            source_path,
            "Current MB overlap harness consumes real workload platform receipts but remains MB-specific support until reusable post-M6.5 workload subjects replace it.",
        )
    })
    .collect()
}

fn planar_replay_and_runtime_fixture_rows() -> Vec<SeedInventoryRow> {
    vec![
        row(
            "planar_overlap::runtime_handles",
            SurfaceKind::SpatialFixture,
            SurfaceAuthority::TestLocalConvenience,
            TopologyPosture::BypassesTopologyTruth,
            ReceiptPosture::NoReceipt,
            SurfaceScope::UnitSupportOnly,
            InventoryDecision::LeaveUnitOnly,
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/runtime_handles.rs",
            "Runtime handles are test-local route handles and are not workload authority.",
        ),
        row(
            "planar_overlap::metaboss::certify_storm_with_retained_replay",
            SurfaceKind::ReExtractionReplayHelper,
            SurfaceAuthority::TestLocalConvenience,
            TopologyPosture::BypassesTopologyTruth,
            ReceiptPosture::TestLocal,
            SurfaceScope::LegacyMigrationOnly,
            InventoryDecision::DeleteAfterReplacement,
            "crates/worth-spatial/src/certification/public_facade_contracts/contracts/planar_overlap/metaboss/proof.rs",
            "The retained-replay storm helper currently reuses local MB setup and cannot register as workload replay proof.",
        ),
    ]
}

fn row(
    surface_id: &'static str,
    surface_kind: SurfaceKind,
    authority: SurfaceAuthority,
    topology_posture: TopologyPosture,
    receipt_posture: ReceiptPosture,
    scope: SurfaceScope,
    decision: InventoryDecision,
    source_path: &'static str,
    human_reason: &'static str,
) -> SeedInventoryRow {
    SeedInventoryRow::new(
        LegacyFixtureClassification::new(
            WorkloadSurfaceId::new(surface_id),
            surface_kind,
            authority,
            topology_posture,
            receipt_posture,
            scope,
            human_reason,
        ),
        decision,
        source_path,
    )
}
