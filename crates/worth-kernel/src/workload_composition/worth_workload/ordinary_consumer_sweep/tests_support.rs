use crate::workload_composition::CompletedBooleanSplitHandoff;
use worth_spatial::facade::replay_family_catalog::current_spatial_replay_family_catalog;
use worth_spatial::facade::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input,
    boolean_event_ledger_spatial_boundary_fixture,
    lower_spatial_replay_scope_product_from_admitted_input,
    lower_spatial_undo_scope_product_from_boolean_event_ledger_request,
    prepare_spatial_replay_semantic_graph_request, BooleanEventLedgerRollbackRequest,
    SpatialReplaySemanticGraphPreparationRequest,
};

#[path = "../../../certification/public_facade_contracts/contracts/public_api_planar_boolean_loop_reconstruction_workload_evidence_support.rs"]
mod replay_support;

pub(crate) fn ordinary_completed_split_handoff(
    label: &'static str,
) -> CompletedBooleanSplitHandoff {
    let subject = replay_support::MetabossEventExtractionSubject::certify(label);
    let replay_subject = replay_support::build_edge_split_replay_parity_subject(&subject);
    replay_support::completed_split_handoff_for(&subject, &replay_subject)
}

pub(crate) fn attached_completed_split_handoff(
    label: &'static str,
    batch_execution: &crate::workload_composition::BatchAdmissionExecutionReceipt,
) -> CompletedBooleanSplitHandoff {
    ordinary_completed_split_handoff(label)
        .with_batch_admission_execution(batch_execution)
        .expect("completed split handoff should attach the selected batch execution")
}

pub(crate) fn with_replay_undo_scope_products<T>(
    label: &'static str,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
    f: impl for<'a> FnOnce(
        &worth_spatial::facade::replay_undo_semantic_graph::SpatialReplayScopeProduct<'a>,
        &worth_spatial::facade::replay_undo_semantic_graph::SpatialUndoScopeProduct<'a>,
    ) -> T,
) -> T {
    let subject = replay_support::MetabossEventExtractionSubject::certify(label);
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split handoff admits spatial touch authority");
    let event_ledger_lookup_packet = subject
        .pair()
        .left()
        .workload()
        .require_boolean_event_ledger_lookup_execution_packet(subject.ledger())
        .expect("event-ledger lookup packet");
    let request = prepare_spatial_replay_semantic_graph_request(
        SpatialReplaySemanticGraphPreparationRequest::new(
            boolean_event_ledger_spatial_boundary_fixture().replay_family_identity(),
            &authority,
            event_ledger_lookup_packet.execution_receipt(),
            completed_split_handoff.lookup_consumed_workload_handoff(),
        )
        .with_retained_replay_receipt(
            completed_split_handoff
                .completed_workload()
                .retained_replay(),
        ),
    )
    .expect("prepared replay request");
    let admitted = admit_prepared_spatial_replay_semantic_graph_input(
        &current_spatial_replay_family_catalog(),
        &request,
    )
    .expect("admitted replay request");
    let replay_scope =
        lower_spatial_replay_scope_product_from_admitted_input(&admitted).expect("replay scope");
    let undo_scope = lower_spatial_undo_scope_product_from_boolean_event_ledger_request(
        BooleanEventLedgerRollbackRequest::new(
            &authority,
            event_ledger_lookup_packet.execution_receipt(),
            completed_split_handoff
                .completed_workload()
                .evidence_ledger()
                .stage_index(),
            completed_split_handoff.lookup_consumed_workload_handoff(),
        ),
    )
    .expect("undo scope");

    f(&replay_scope, &undo_scope)
}
