mod architecture_alignment;
mod authority_chain;
mod failure_guards;
mod milestone_fifteen_seed;

use super::architecture_alignment_report::build_architecture_alignment_report;
use super::public_closeout::{
    current_public_closeout_components,
    current_public_closeout_components_with_matrix_targets_loader,
    current_worth_touched_graph_conflict_milestone_fifteen_seed,
    current_worth_touched_graph_conflict_public_closeout, publish_from_parts,
};
use super::public_closeout_types::WorthTouchedGraphConflictPublicCloseoutErrorKind;
use super::residue_chain::{
    WorthTouchedGraphConflictResidueBoundaryPosture, WorthTouchedGraphConflictResidueChain,
    WorthTouchedGraphConflictResidueRow,
};
use super::{
    current_public_closeout_consumer_residue_manifest,
    PublicCloseoutConsumerResidueBoundaryPosture, PublicCloseoutConsumerResidueDisposition,
    PublicCloseoutConsumerResidueOwner, WorthTouchedGraphConflictMilestoneFifteenSeed,
};
use crate::workload_composition::compiled_product_consumer_cutover::current_coverage_targets;
use crate::workload_composition::{
    admit_worth_touched_graph_conflict_public_proof_input,
    current_conflict_batch_admission_inventory,
    current_worth_touched_graph_conflict_deletion_closeout,
    current_worth_touched_graph_conflict_source_firewall_report,
    planner_owned_routing::current_worth_touched_graph_conflict_selected_route_packet,
    worth_workload::{
        current_worth_workload_ordinary_consumer_cutover,
        ordinary_consumer_cutover_from_inventory_for_tests,
        ordinary_consumer_cutover_from_inventory_with_test_replay_undo_identity_override,
    },
    ConflictBatchAdmissionCertificationPosture, ConflictBatchAdmissionCostPosture,
    ConflictBatchAdmissionDisposition, ConflictBatchAdmissionInventory,
    ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionSurfaceIdentity,
    KernelCompiledProductConsumerDependencyError, KernelCompiledProductConsumerDependencyMatrix,
    WorthTouchedGraphConflictAdmittedPublicProofInput,
};
use topology::facade::current_topology_query_backed_consumer_cutover;
use topology::query_domain::TopologyReadRequestFamily;
use worth_spatial::facade::evidence_lookup_public_closeout::current_evidence_lookup_public_closeout;

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

fn hostile_inventory_with_open_ordinary_dependency() -> ConflictBatchAdmissionInventory {
    let inventory = current_conflict_batch_admission_inventory().expect("current inventory");
    inventory_with_replaced_row(
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
    )
}

fn hostile_public_proof_input_with_foreign_reuse_basis(
    foreign_reuse_basis_identity_digest: &str,
) -> WorthTouchedGraphConflictAdmittedPublicProofInput {
    let packet = current_worth_touched_graph_conflict_selected_route_packet()
        .expect("current selected-route packet should build");
    let current_input = admit_worth_touched_graph_conflict_public_proof_input(&packet)
        .expect("current admitted public proof input should lower");
    WorthTouchedGraphConflictAdmittedPublicProofInput::from_parts(
        current_input.selected_route_packet_digest().to_string(),
        current_input.selected_route_identity_digest().to_string(),
        current_input.selected_family_identity().to_string(),
        current_input.selected_product_identity_digest().to_string(),
        foreign_reuse_basis_identity_digest.to_string(),
        current_input
            .selected_witness_identity_digest()
            .map(str::to_string),
        current_input
            .rebuild_denial_identity_digest()
            .map(str::to_string),
        current_input.spatial_selected_family_identity().to_string(),
        current_input
            .spatial_selected_product_identity_digest()
            .to_string(),
        current_input
            .spatial_equivalence_policy_identity_digest()
            .to_string(),
        current_input.topology_freshness_requirement_posture(),
        current_input.topology_rendered_output_comparison_posture(),
        current_input.spatial_freshness_requirement_posture(),
        current_input.spatial_rendered_output_comparison_posture(),
        current_input.topology_query_execution_count(),
        current_input.topology_row_scan_fallback_count(),
        current_input.topology_whole_view_fallback_count(),
        current_input.topology_repeated_rediscovery_denied_count(),
        current_input.spatial_receipt_proof_row_count(),
        current_input.spatial_non_ordinary_residue_row_count(),
    )
}
