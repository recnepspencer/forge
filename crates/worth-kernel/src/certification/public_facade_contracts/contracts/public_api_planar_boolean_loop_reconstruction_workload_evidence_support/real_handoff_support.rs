use topology::facade::PlanarBooleanLoopBlueprintRegistry;
use worth_kernel::workload_composition::{
    trace_scope, BooleanSplitReplayUndoBoundaryRequest, WorkloadCompositionError,
};
use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitReplayParityReport;
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    ComparePlanarBooleanLoopReplayParity, PlanarBooleanLoopReplayParityInput,
    PlanarBooleanLoopReplayParityReceipt,
};

use super::continuation_contract_support;
use super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report, EdgeSplitReplayParitySubject,
};
use super::loop_replay_boundary_support::{
    complete_replay_undo_chain_from_boundary, current_ordinary_consumer_batch_execution_receipt,
    with_matching_spatial_scope_products,
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
    pub(crate) replay_receipts: worth_spatial::facade::retained_replay_workload::ReplayReceiptSet,
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
    let replay_subject = trace_scope("loop_handoff_edge_split_replay_subject", || {
        build_edge_split_replay_parity_subject(subject)
    });
    let replay_report = trace_scope("loop_handoff_replay_parity_report", || {
        replay_parity_report(&replay_subject)
    });
    packet_backed_replay_undo_chain_for_branch_with_subject(
        subject,
        branch,
        matrix,
        validators,
        &replay_subject,
        &replay_report,
    )
}

fn packet_backed_replay_undo_chain_for_branch_with_subject(
    subject: &MetabossEventExtractionSubject,
    branch: ReplayBranch,
    matrix: &topology::facade::PlanarBooleanLoopOperatorClassificationMatrix,
    validators: &topology::facade::PlanarBooleanLoopValidatorRegistrationPlan,
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
) -> Result<
    worth_kernel::workload_composition::BooleanChainReplayUndoBoundaryHandoff,
    WorkloadCompositionError,
> {
    let batch_execution = trace_scope("loop_handoff_current_batch_execution", || {
        current_ordinary_consumer_batch_execution_receipt()
    })?;
    let completed_split_handoff = trace_scope("loop_handoff_completed_split_with_batch", || {
        continuation_contract_support::completed_split_handoff_for(subject, &replay_subject)
            .with_batch_admission_execution(&batch_execution)
    })?;
    let topology_undo_scope_support =
        trace_scope("loop_handoff_topology_undo_scope_support", || {
            ordinary_traversal_views_undo_scope_support()
        });
    let topology_undo_scope_product =
        trace_scope("loop_handoff_topology_undo_scope_product", || {
            topology_undo_scope_support
                .lower_undo_scope_product()
                .expect("topology undo scope product")
        });
    trace_scope("loop_handoff_spatial_scope_and_closeout", || {
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
    })
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
                .admit_batch_execution_cluster()
                .expect("packet-backed split handoff admits batch execution cluster")
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
    let subject = trace_scope("certified_loop_subject", || {
        MetabossEventExtractionSubject::certify(label)
    });
    certified_loop_replay_closeout_chain_for_subject(subject)
}

pub(crate) fn certified_loop_replay_closeout_chain_for_pair(
    label: &'static str,
    pair: worth_kernel::workload_composition::BuiltBooleanOperandPairRecipe,
) -> CertifiedLoopReplayCloseoutChain {
    let subject = trace_scope("certified_loop_custom_subject", || {
        MetabossEventExtractionSubject::certify_from_pair(label, pair)
    });
    certified_loop_replay_closeout_chain_for_subject(subject)
}

pub(crate) fn certified_event_carrier_loop_replay_closeout_chain(
    label: &'static str,
) -> CertifiedLoopReplayCloseoutChain {
    let subject = trace_scope("certified_event_carrier_loop_subject", || {
        MetabossEventExtractionSubject::certify_event_carrier(label)
    });
    certified_loop_replay_closeout_chain_for_subject(subject)
}

fn certified_loop_replay_closeout_chain_for_subject(
    subject: MetabossEventExtractionSubject,
) -> CertifiedLoopReplayCloseoutChain {
    trace_scope("certified_real_loop_replay_closeout_chain", || {
        let registry = trace_scope("certified_loop_blueprint_registry", || {
            PlanarBooleanLoopBlueprintRegistry::phase_2()
        });
        let matrix = trace_scope("certified_loop_operator_matrix", || {
            registry.operator_classification_matrix()
        });
        let validators = trace_scope("certified_loop_validator_plan", || {
            registry.validator_registration_plan()
        });
        let replay_subject = trace_scope("certified_loop_replay_subject", || {
            build_edge_split_replay_parity_subject(&subject)
        });
        let replay_report = trace_scope("certified_loop_replay_parity_report", || {
            replay_parity_report(&replay_subject)
        });
        let original = trace_scope("certified_loop_original_handoff", || {
            packet_backed_replay_undo_chain_for_branch_with_subject(
                &subject,
                ReplayBranch::Original,
                &matrix,
                &validators,
                &replay_subject,
                &replay_report,
            )
            .map(|handoff| handoff.into_loop_handoff())
            .expect("original loop handoff should certify through the real closeout seam")
        });
        let replayed = trace_scope("certified_loop_replayed_handoff", || {
            packet_backed_replay_undo_chain_for_branch_with_subject(
                &subject,
                ReplayBranch::Replayed,
                &matrix,
                &validators,
                &replay_subject,
                &replay_report,
            )
            .map(|handoff| handoff.into_loop_handoff())
            .expect("replayed loop handoff should certify through the real closeout seam")
        });
        let replay_parity = trace_scope("certified_loop_replay_parity", || {
            ComparePlanarBooleanLoopReplayParity::compare(
                PlanarBooleanLoopReplayParityInput::admit_from_ledger_and_evidence(
                    original.loop_ledger_receipt(),
                    replayed.loop_ledger_receipt(),
                    original.evidence_receipt(),
                    replayed.evidence_receipt(),
                    &replay_subject.replay_receipts,
                )
                .expect("real loop receipts should admit replay closeout"),
            )
            .expect("real loop replay closeout should certify parity over the production chain")
        });

        CertifiedLoopReplayCloseoutChain {
            original,
            replayed,
            replay_parity,
            replay_receipts: replay_subject.replay_receipts,
        }
    })
}
