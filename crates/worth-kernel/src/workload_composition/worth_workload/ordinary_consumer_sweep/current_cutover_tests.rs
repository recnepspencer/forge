use super::current_cutover::{
    current_worth_workload_ordinary_consumer_cutover, WorthWorkloadOrdinaryConsumerCutoverPosture,
    WorthWorkloadOrdinaryConsumerCutoverRow,
};
use super::current_route_witness::WorthWorkloadOrdinaryConsumerRouteKind;
use crate::workload_composition::{
    ConflictBatchAdmissionAuthorityKind, ConflictBatchAdmissionCertificationPosture,
    ConflictBatchAdmissionCostPosture, ConflictBatchAdmissionDisposition,
    ConflictBatchAdmissionInventoryRow, ConflictBatchAdmissionOwner,
    ConflictBatchAdmissionQuerySurface, ConflictBatchAdmissionReplacementPhase,
    ConflictBatchAdmissionRowScope, ConflictBatchAdmissionSurfaceIdentity,
};

#[test]
fn current_cutover_marks_phase_eleven_surfaces_with_real_posture() {
    let cutover =
        current_worth_workload_ordinary_consumer_cutover().expect("current cutover should build");
    let receipt_digest = cutover.batch_execution_receipt().execution_receipt_digest();

    assert_eq!(cutover.rows().len(), 5);
    assert_eq!(
        posture_for(
            cutover.rows(),
            "WorthWorkload::admit_lookup_consumed_workload"
        ),
        WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer
    );
    assert_eq!(
        posture_for(
            cutover.rows(),
            "CompletedBooleanSplitHandoff::admit_downstream_split_consumption"
        ),
        WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer
    );
    assert_eq!(
        posture_for(cutover.rows(), "admit_boolean_split_replay_undo_boundary"),
        WorthWorkloadOrdinaryConsumerCutoverPosture::SelectedPlanDrivenOrdinaryConsumer
    );
    assert_eq!(
        posture_for(cutover.rows(), "PlanarBooleanLoopRuntimeRegistrationProof"),
        WorthWorkloadOrdinaryConsumerCutoverPosture::QueryProofAccompanimentOnly
    );
    assert_eq!(
        posture_for(cutover.rows(), "BooleanChainIntegrationHandoff"),
        WorthWorkloadOrdinaryConsumerCutoverPosture::ReplayUndoCloseoutOnly
    );
    assert_eq!(
        selected_plan_route_for(
            cutover.rows(),
            "WorthWorkload::admit_lookup_consumed_workload"
        ),
        Some(WorthWorkloadOrdinaryConsumerRouteKind::LookupConsumedBatchExecutionCluster)
    );
    assert_eq!(
        selected_plan_route_for(
            cutover.rows(),
            "CompletedBooleanSplitHandoff::admit_downstream_split_consumption"
        ),
        Some(WorthWorkloadOrdinaryConsumerRouteKind::CompletedSplitBatchExecutionCluster)
    );
    assert_eq!(
        selected_plan_route_for(cutover.rows(), "admit_boolean_split_replay_undo_boundary"),
        Some(WorthWorkloadOrdinaryConsumerRouteKind::ReplayUndoBoundaryBatchExecutionCluster)
    );
    let lookup_digest = selected_plan_route_authority_digest(
        cutover.rows(),
        "WorthWorkload::admit_lookup_consumed_workload",
    )
    .expect("lookup route authority digest");
    let split_digest = selected_plan_route_authority_digest(
        cutover.rows(),
        "CompletedBooleanSplitHandoff::admit_downstream_split_consumption",
    )
    .expect("completed split route authority digest");
    let replay_undo_digest = selected_plan_route_authority_digest(
        cutover.rows(),
        "admit_boolean_split_replay_undo_boundary",
    )
    .expect("replay/undo route authority digest");
    assert_ne!(lookup_digest, split_digest);
    assert_ne!(split_digest, replay_undo_digest);
    for surface_name in [
        "WorthWorkload::admit_lookup_consumed_workload",
        "CompletedBooleanSplitHandoff::admit_downstream_split_consumption",
        "admit_boolean_split_replay_undo_boundary",
    ] {
        let row = cutover
            .rows()
            .iter()
            .find(|row| row.surface_name() == surface_name)
            .expect("selected-plan cutover row should exist");
        assert_eq!(
            row.selected_plan_witness()
                .expect("selected-plan row should carry a witness")
                .batch_execution_receipt_digest(),
            receipt_digest
        );
        assert!(!row
            .selected_plan_witness()
            .expect("selected-plan row should carry a witness")
            .route_lineage_digest()
            .is_empty());
        assert!(!row
            .selected_plan_witness()
            .expect("selected-plan row should carry a witness")
            .route_authority_digest()
            .is_empty());
    }
    let replay_undo_row = cutover
        .rows()
        .iter()
        .find(|row| row.surface_name() == "admit_boolean_split_replay_undo_boundary")
        .expect("replay/undo cutover row should exist");
    let replay_undo_witness = replay_undo_row
        .selected_plan_witness()
        .expect("replay/undo selected-plan row should carry a witness");
    assert!(replay_undo_witness
        .replay_undo_boundary_proof_digest()
        .is_some_and(|value| !value.is_empty()));
    assert!(replay_undo_witness
        .transaction_packet_identity()
        .is_some_and(|value| !value.is_empty()));
    assert!(replay_undo_witness
        .replay_scope_identity()
        .is_some_and(|value| !value.is_empty()));
    assert!(replay_undo_witness
        .undo_scope_identity()
        .is_some_and(|value| !value.is_empty()));
}

#[test]
fn unmapped_phase_eleven_row_becomes_covered_ordinary_dependency() {
    let row = ConflictBatchAdmissionInventoryRow::builder()
        .surface_identity(ConflictBatchAdmissionSurfaceIdentity::WorthWorkloadCompose)
        .source_path("crates/worth-kernel/src/workload_composition/worth_workload.rs")
        .surface_name("WorthWorkload::compose")
        .owner(ConflictBatchAdmissionOwner::WorthKernel)
        .current_caller("phase 13 hostile test")
        .authority_kind(ConflictBatchAdmissionAuthorityKind::WorkloadCompositionAdmission)
        .disposition(ConflictBatchAdmissionDisposition::Migrate)
        .replacement_phase(ConflictBatchAdmissionReplacementPhase::PhaseElevenConsumerSweep)
        .blocker("hostile uncovered ordinary consumer")
        .removal_trigger("route through the selected batch plan chain")
        .certification_posture(
            ConflictBatchAdmissionCertificationPosture::OrdinaryProductionReachable,
        )
        .cost_posture(ConflictBatchAdmissionCostPosture::ReceiptBackedTypedLookup)
        .query_surface(ConflictBatchAdmissionQuerySurface::NotQuery)
        .row_scope(ConflictBatchAdmissionRowScope::ConcreteSource)
        .build()
        .expect("test row should build");
    assert_eq!(
        WorthWorkloadOrdinaryConsumerCutoverRow::test_posture_from_phase_eleven_inventory_row(row),
        WorthWorkloadOrdinaryConsumerCutoverPosture::CoveredOrdinaryConsumerDependency
    );
}

fn posture_for(
    rows: &[WorthWorkloadOrdinaryConsumerCutoverRow],
    surface_name: &str,
) -> WorthWorkloadOrdinaryConsumerCutoverPosture {
    rows.iter()
        .find(|row| row.surface_name() == surface_name)
        .expect("phase 11 row should exist")
        .posture()
}

fn selected_plan_route_for(
    rows: &[WorthWorkloadOrdinaryConsumerCutoverRow],
    surface_name: &str,
) -> Option<WorthWorkloadOrdinaryConsumerRouteKind> {
    rows.iter()
        .find(|row| row.surface_name() == surface_name)
        .and_then(|row| row.selected_plan_witness())
        .map(|witness| witness.route_kind())
}

fn selected_plan_route_authority_digest<'a>(
    rows: &'a [WorthWorkloadOrdinaryConsumerCutoverRow],
    surface_name: &str,
) -> Option<&'a str> {
    rows.iter()
        .find(|row| row.surface_name() == surface_name)
        .and_then(|row| row.selected_plan_witness())
        .map(|witness| witness.route_authority_digest())
}
