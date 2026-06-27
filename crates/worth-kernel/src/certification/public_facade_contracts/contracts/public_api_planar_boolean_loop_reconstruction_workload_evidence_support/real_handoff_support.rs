use topology::facade::PlanarBooleanLoopBlueprintRegistry;
use worth_kernel::workload_composition::{
    BooleanSplitReplayUndoBoundaryRequest, PlanarBooleanLoopReconstructionCloseoutInput,
    WorkloadCompositionError,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    ComparePlanarBooleanLoopReplayParity, PlanarBooleanLoopReplayParityInput,
    PlanarBooleanLoopReplayParityReceipt,
};
use worth_spatial::facade::replay_family_catalog::{
    admit_spatial_replay_family_identity, current_spatial_replay_family_catalog,
    SpatialReplayFamilyIdentityAuthority,
};
use worth_spatial::facade::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input,
    lower_spatial_replay_scope_product_from_admitted_input,
    lower_spatial_undo_scope_product_from_boolean_event_ledger_request,
    prepare_spatial_replay_semantic_graph_request, BooleanEventLedgerRollbackRequest,
    SpatialReplaySemanticGraphPreparationRequest,
};

use super::continuation_contract_support;
use super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report,
};
use super::metaboss_support::MetabossEventExtractionSubject;
use super::ordinary_topology_undo_support::ordinary_traversal_views_undo_scope_support;

pub(crate) enum ReplayBranch {
    Original,
    Replayed,
}

pub(crate) struct CertifiedLoopReplayCloseoutChain {
    pub(crate) original:
        worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff,
    pub(crate) replayed:
        worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff,
    pub(crate) replay_parity: PlanarBooleanLoopReplayParityReceipt,
}

pub(crate) fn real_loop_handoff_for_branch(
    subject: &MetabossEventExtractionSubject,
    branch: ReplayBranch,
    matrix: &topology::facade::PlanarBooleanLoopOperatorClassificationMatrix,
    validators: &topology::facade::PlanarBooleanLoopValidatorRegistrationPlan,
) -> Result<
    worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff,
    WorkloadCompositionError,
> {
    packet_backed_loop_handoff_for_branch(subject, branch, matrix, validators)
}

pub(crate) fn certified_real_loop_handoff(
    label: &'static str,
    branch: ReplayBranch,
) -> Result<
    worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff,
    WorkloadCompositionError,
> {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let subject = MetabossEventExtractionSubject::certify(label);
    real_loop_handoff_for_branch(&subject, branch, &matrix, &validators)
}

pub(crate) fn packet_backed_loop_handoff_for_branch(
    subject: &MetabossEventExtractionSubject,
    branch: ReplayBranch,
    matrix: &topology::facade::PlanarBooleanLoopOperatorClassificationMatrix,
    validators: &topology::facade::PlanarBooleanLoopValidatorRegistrationPlan,
) -> Result<
    worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff,
    WorkloadCompositionError,
> {
    Ok(
        packet_backed_replay_undo_chain_for_branch(subject, branch, matrix, validators)?
            .into_loop_handoff(),
    )
}

pub(crate) fn packet_backed_replay_undo_chain_for_branch(
    subject: &MetabossEventExtractionSubject,
    branch: ReplayBranch,
    matrix: &topology::facade::PlanarBooleanLoopOperatorClassificationMatrix,
    validators: &topology::facade::PlanarBooleanLoopValidatorRegistrationPlan,
) -> Result<
    worth_kernel::workload_composition::BooleanChainReplayUndoBoundaryHandoff,
    WorkloadCompositionError,
> {
    let replay_subject = build_edge_split_replay_parity_subject(subject);
    let replay_report = replay_parity_report(&replay_subject);
    let completed_split_handoff =
        continuation_contract_support::completed_split_handoff_for(subject, &replay_subject);
    let topology_undo_scope_support = ordinary_traversal_views_undo_scope_support();
    let topology_undo_scope_product = topology_undo_scope_support
        .lower_undo_scope_product()
        .expect("topology undo scope product");
    with_matching_spatial_scope_products(
        subject,
        &completed_split_handoff,
        |replay_scope, undo_scope| match branch {
            ReplayBranch::Original => complete_replay_undo_chain_from_boundary(
                subject,
                &completed_split_handoff,
                BooleanSplitReplayUndoBoundaryRequest::new(
                    &topology_undo_scope_product,
                    replay_scope,
                    undo_scope,
                ),
                replay_report.receipt(),
                &replay_subject.replay_receipts,
                replay_subject.original_decision_log.receipt(),
                &replay_subject.original_products.validation,
                &replay_subject.original_products.naming,
                replay_subject.original_ledger.ledger(),
                &replay_subject.original_products.vertices,
                &replay_subject.original_products.fragments,
                &replay_subject.original_products.chains,
                &replay_subject.original_products.request,
                matrix,
                validators,
            ),
            ReplayBranch::Replayed => complete_replay_undo_chain_from_boundary(
                subject,
                &completed_split_handoff,
                BooleanSplitReplayUndoBoundaryRequest::new(
                    &topology_undo_scope_product,
                    replay_scope,
                    undo_scope,
                ),
                replay_report.receipt(),
                &replay_subject.replay_receipts,
                replay_subject.replayed_decision_log.receipt(),
                &replay_subject.replayed_products.validation,
                &replay_subject.replayed_products.naming,
                replay_subject.replayed_ledger.ledger(),
                &replay_subject.replayed_products.vertices,
                &replay_subject.replayed_products.fragments,
                &replay_subject.replayed_products.chains,
                &replay_subject.replayed_products.request,
                matrix,
                validators,
            ),
        },
    )
}

pub(crate) fn with_packet_backed_loop_boundary_basis<T>(
    subject: &MetabossEventExtractionSubject,
    f: impl for<'a> FnOnce(
        &'a topology::facade::TopologyUndoScopeProduct<'a>,
        &'a worth_spatial::facade::replay_undo_semantic_graph::SpatialReplayScopeProduct<'a>,
        &'a worth_spatial::facade::replay_undo_semantic_graph::SpatialUndoScopeProduct<'a>,
    ) -> T,
) -> T {
    let replay_subject = build_edge_split_replay_parity_subject(subject);
    let completed_split_handoff =
        continuation_contract_support::completed_split_handoff_for(subject, &replay_subject);
    let topology_undo_scope_support = ordinary_traversal_views_undo_scope_support();
    let topology_undo_scope_product = topology_undo_scope_support
        .lower_undo_scope_product()
        .expect("topology undo scope product");

    with_matching_spatial_scope_products(
        subject,
        &completed_split_handoff,
        |replay_scope, undo_scope| f(&topology_undo_scope_product, replay_scope, undo_scope),
    )
}

pub(crate) fn foreign_packet_backed_boundary_error(
    subject: &MetabossEventExtractionSubject,
    foreign_subject: &MetabossEventExtractionSubject,
) -> WorkloadCompositionError {
    let replay_subject = build_edge_split_replay_parity_subject(subject);
    let completed_split_handoff =
        continuation_contract_support::completed_split_handoff_for(subject, &replay_subject);
    let foreign_replay_subject = build_edge_split_replay_parity_subject(foreign_subject);
    let foreign_completed_split_handoff =
        continuation_contract_support::completed_split_handoff_for(
            foreign_subject,
            &foreign_replay_subject,
        );
    let topology_undo_scope_support = ordinary_traversal_views_undo_scope_support();
    let topology_undo_scope_product = topology_undo_scope_support
        .lower_undo_scope_product()
        .expect("topology undo scope product");
    with_matching_spatial_scope_products(
        foreign_subject,
        &foreign_completed_split_handoff,
        |replay_scope, undo_scope| {
            completed_split_handoff
                .admit_boolean_split_replay_undo_boundary(
                    BooleanSplitReplayUndoBoundaryRequest::new(
                        &topology_undo_scope_product,
                        replay_scope,
                        undo_scope,
                    ),
                )
                .expect_err(
                    "foreign lookup execution scope products must fail the packet-backed boundary",
                )
        },
    )
}

pub(crate) fn certified_real_loop_replay_closeout_chain(
    label: &'static str,
) -> CertifiedLoopReplayCloseoutChain {
    let registry = PlanarBooleanLoopBlueprintRegistry::phase_2();
    let matrix = registry.operator_classification_matrix();
    let validators = registry.validator_registration_plan();
    let subject = MetabossEventExtractionSubject::certify(label);
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let original =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Original, &matrix, &validators)
            .expect("original loop handoff should certify through the real closeout seam");
    let replayed =
        real_loop_handoff_for_branch(&subject, ReplayBranch::Replayed, &matrix, &validators)
            .expect("replayed loop handoff should certify through the real closeout seam");
    let replay_parity = ComparePlanarBooleanLoopReplayParity::compare(
        PlanarBooleanLoopReplayParityInput::admit_from_ledger_and_evidence(
            original.loop_ledger_receipt(),
            replayed.loop_ledger_receipt(),
            original.evidence_receipt(),
            replayed.evidence_receipt(),
            &replay_subject.replay_receipts,
        )
        .expect("real loop receipts should admit replay closeout"),
    )
    .expect("real loop replay closeout should certify parity over the production chain");

    CertifiedLoopReplayCloseoutChain {
        original,
        replayed,
        replay_parity,
    }
}

#[allow(clippy::too_many_arguments)]
fn complete_replay_undo_chain_from_boundary(
    subject: &MetabossEventExtractionSubject,
    completed_split_handoff: &worth_kernel::workload_composition::CompletedBooleanSplitHandoff,
    boundary_request: BooleanSplitReplayUndoBoundaryRequest<'_>,
    replay_parity_receipt: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitReplayParityReceipt,
    replay_receipts: &worth_spatial::facade::retained_replay_workload::ReplayReceiptSet,
    decision_log_receipt: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitDecisionLogReceipt,
    validation: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitChainValidationReceipt,
    naming: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitPersistentNamingReceipt,
    ledger: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChainLedger,
    vertices: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitVertexIdentitySet,
    fragments: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentSet,
    chains: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanOverlapEdgeChainSet,
    split_request: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitRequest,
    matrix: &topology::facade::PlanarBooleanLoopOperatorClassificationMatrix,
    validators: &topology::facade::PlanarBooleanLoopValidatorRegistrationPlan,
) -> Result<
    worth_kernel::workload_composition::BooleanChainReplayUndoBoundaryHandoff,
    WorkloadCompositionError,
> {
    let recovered_source_carriers =
        continuation_contract_support::recovered_source_carriers(subject, split_request);
    completed_split_handoff.complete_boolean_chain_integration_from_replay_undo_boundary(
        boundary_request,
        PlanarBooleanLoopReconstructionCloseoutInput::new(
            decision_log_receipt,
            validation,
            naming,
            replay_parity_receipt,
            ledger,
            &recovered_source_carriers,
            vertices,
            fragments,
            chains,
            replay_receipts,
            matrix,
            validators,
        ),
    )
}

fn with_matching_spatial_scope_products<T>(
    subject: &MetabossEventExtractionSubject,
    completed_split_handoff: &worth_kernel::workload_composition::CompletedBooleanSplitHandoff,
    f: impl for<'a> FnOnce(
        &worth_spatial::facade::replay_undo_semantic_graph::SpatialReplayScopeProduct<'a>,
        &worth_spatial::facade::replay_undo_semantic_graph::SpatialUndoScopeProduct<'a>,
    ) -> T,
) -> T {
    let authority = completed_split_handoff
        .admit_split_spatial_touch_authority()
        .expect("split authority");
    let event_ledger_lookup_packet = subject
        .pair()
        .left()
        .workload()
        .require_boolean_event_ledger_lookup_execution_packet(subject.ledger())
        .expect("event-ledger lookup packet");
    let request = prepare_spatial_replay_semantic_graph_request(
        SpatialReplaySemanticGraphPreparationRequest::new(
            admit_spatial_replay_family_identity(
                SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
            ),
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
    .expect("admitted replay input");
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
