use topology::facade::current_topology_query_backed_consumer_cutover;
use topology::query_domain::TopologyReadRequestFamily;
use worth_spatial::facade::evidence_lookup_public_closeout::current_evidence_lookup_public_closeout;

use super::closeout::validate_assembled_ordinary_sweep_closeout_for_test;
use super::current_cutover::{
    current_worth_workload_ordinary_consumer_cutover, ordinary_consumer_cutover_from_inventory,
};
use crate::workload_composition::public_closeout::{
    current_worth_touched_graph_conflict_milestone_fifteen_seed,
    current_worth_touched_graph_conflict_public_closeout,
};
use crate::workload_composition::{
    current_conflict_batch_admission_inventory, ConflictBatchAdmissionCertificationPosture,
    ConflictBatchAdmissionCostPosture, ConflictBatchAdmissionDisposition,
    ConflictBatchAdmissionInventory, ConflictBatchAdmissionInventoryRow,
    ConflictBatchAdmissionSurfaceIdentity,
};

#[test]
fn broad_scan_gate_rejects_downgraded_exact_denied_surface_in_live_inventory() {
    let inventory = current_conflict_batch_admission_inventory().expect("current inventory");
    let hostile_inventory = inventory_with_replaced_row(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::EvidenceLookupReuseDecisionBroadReceiptScanCounter,
        |row| {
            rebuilt_inventory_row(
                row,
                ConflictBatchAdmissionDisposition::CertificationOnly,
                ConflictBatchAdmissionCertificationPosture::CertificationOnlyDeniedAsOrdinaryProof,
                ConflictBatchAdmissionCostPosture::SourceFirewallOnly,
            )
        },
    );
    let cutover =
        ordinary_consumer_cutover_from_inventory(&hostile_inventory).expect("hostile cutover");
    let topology_cutover =
        current_topology_query_backed_consumer_cutover().expect("current query-backed cutover");
    let lookup_public_closeout =
        current_evidence_lookup_public_closeout().expect("lookup public closeout");
    let public_closeout =
        current_worth_touched_graph_conflict_public_closeout().expect("kernel public closeout");
    let phase_fifteen_seed =
        current_worth_touched_graph_conflict_milestone_fifteen_seed().expect("seed");

    let error = validate_assembled_ordinary_sweep_closeout_for_test(
        &hostile_inventory,
        &cutover,
        &topology_cutover,
        &lookup_public_closeout,
        &public_closeout,
        &phase_fifteen_seed,
    )
    .expect_err("downgraded broad-scan surface must fail the assembled ordinary-sweep boundary");

    assert_eq!(
        error.kind(),
        super::error::WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::BroadScanFallbackStillOrdinary
    );
    assert!(error
        .detail()
        .contains("EvidenceLookupReuseDecisionBroadReceiptScanCounter"));
}

#[test]
fn phase_eleven_bypass_guard_rejects_hostile_live_cutover_inventory() {
    let inventory = current_conflict_batch_admission_inventory().expect("current inventory");
    let hostile_inventory = inventory_with_replaced_row(
        &inventory,
        ConflictBatchAdmissionSurfaceIdentity::PlanarBooleanLoopRuntimeRegistrationProof,
        |row| {
            rebuilt_inventory_row(
                row,
                ConflictBatchAdmissionDisposition::Migrate,
                ConflictBatchAdmissionCertificationPosture::OrdinaryProductionReachable,
                ConflictBatchAdmissionCostPosture::ReceiptBackedTypedLookup,
            )
        },
    );
    let hostile_cutover =
        ordinary_consumer_cutover_from_inventory(&hostile_inventory).expect("hostile cutover");
    let topology_cutover =
        current_topology_query_backed_consumer_cutover().expect("current query-backed cutover");
    let lookup_public_closeout =
        current_evidence_lookup_public_closeout().expect("lookup public closeout");
    let public_closeout =
        current_worth_touched_graph_conflict_public_closeout().expect("kernel public closeout");
    let phase_fifteen_seed =
        current_worth_touched_graph_conflict_milestone_fifteen_seed().expect("seed");

    let error = validate_assembled_ordinary_sweep_closeout_for_test(
        &hostile_inventory,
        &hostile_cutover,
        &topology_cutover,
        &lookup_public_closeout,
        &public_closeout,
        &phase_fifteen_seed,
    )
    .expect_err("covered dependency must fail the assembled ordinary-sweep boundary");

    assert_eq!(
        error.kind(),
        super::error::WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::CoveredOrdinaryConsumerBypass
    );
    assert!(error
        .detail()
        .contains("PlanarBooleanLoopRuntimeRegistrationProof"));
}

#[test]
fn topology_fallback_counts_fail_the_assembled_broad_scan_boundary() {
    let inventory = current_conflict_batch_admission_inventory().expect("current inventory");
    let cutover =
        current_worth_workload_ordinary_consumer_cutover().expect("current ordinary cutover");
    let hostile_topology_cutover = current_topology_query_backed_consumer_cutover()
        .expect("current query-backed cutover")
        .with_test_family_fallback_counts(TopologyReadRequestFamily::LoopCycleNeighborhood, 1, 0);
    let lookup_public_closeout =
        current_evidence_lookup_public_closeout().expect("lookup public closeout");
    let public_closeout =
        current_worth_touched_graph_conflict_public_closeout().expect("kernel public closeout");
    let phase_fifteen_seed =
        current_worth_touched_graph_conflict_milestone_fifteen_seed().expect("seed");

    let error = validate_assembled_ordinary_sweep_closeout_for_test(
        &inventory,
        &cutover,
        &hostile_topology_cutover,
        &lookup_public_closeout,
        &public_closeout,
        &phase_fifteen_seed,
    )
    .expect_err("query-backed fallback counts must fail the assembled broad-scan boundary");

    assert_eq!(
        error.kind(),
        super::error::WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind::BroadScanFallbackStillOrdinary
    );
    assert!(error.detail().contains("LoopCycleNeighborhood"));
}

fn inventory_with_replaced_row(
    inventory: &ConflictBatchAdmissionInventory,
    target: ConflictBatchAdmissionSurfaceIdentity,
    replace: impl FnOnce(&ConflictBatchAdmissionInventoryRow) -> ConflictBatchAdmissionInventoryRow,
) -> ConflictBatchAdmissionInventory {
    let mut replace = Some(replace);
    let rows = inventory
        .rows()
        .iter()
        .map(|row| {
            if row.surface_identity() == target {
                replace.take().expect("replacement should run once")(row)
            } else {
                row.clone()
            }
        })
        .collect();
    ConflictBatchAdmissionInventory::from_rows_for_validation(rows)
        .expect("hostile inventory should remain structurally valid")
}

fn rebuilt_inventory_row(
    row: &ConflictBatchAdmissionInventoryRow,
    disposition: ConflictBatchAdmissionDisposition,
    certification_posture: ConflictBatchAdmissionCertificationPosture,
    cost_posture: ConflictBatchAdmissionCostPosture,
) -> ConflictBatchAdmissionInventoryRow {
    ConflictBatchAdmissionInventoryRow::builder()
        .surface_identity(row.surface_identity())
        .source_path(row.source_path())
        .surface_name(row.surface_name())
        .owner(row.owner())
        .current_caller(row.current_caller())
        .authority_kind(row.authority_kind())
        .disposition(disposition)
        .replacement_phase(row.replacement_phase())
        .blocker(row.blocker())
        .removal_trigger(row.removal_trigger())
        .certification_posture(certification_posture)
        .cost_posture(cost_posture)
        .query_surface(row.query_surface())
        .row_scope(row.row_scope())
        .build()
        .expect("rebuilt hostile inventory row should remain valid")
}
