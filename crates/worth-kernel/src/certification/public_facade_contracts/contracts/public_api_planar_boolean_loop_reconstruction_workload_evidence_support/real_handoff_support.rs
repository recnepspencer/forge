use topology::facade::PlanarBooleanLoopBlueprintRegistry;
use worth_kernel::workload_composition::{
    PlanarBooleanLoopReconstructionCloseoutInput, WorkloadCompositionError,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    ComparePlanarBooleanLoopReplayParity, PlanarBooleanLoopReplayParityInput,
    PlanarBooleanLoopReplayParityReceipt,
};

use super::continuation_contract_support;
use super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report,
};
use super::metaboss_support::MetabossEventExtractionSubject;

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
    let replay_subject = build_edge_split_replay_parity_subject(subject);
    let replay_report = replay_parity_report(&replay_subject);
    let completed_split_handoff =
        continuation_contract_support::completed_split_handoff_for(subject, &replay_subject);

    match branch {
        ReplayBranch::Original => complete_loop_handoff(
            subject,
            &completed_split_handoff,
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
        ReplayBranch::Replayed => complete_loop_handoff(
            subject,
            &completed_split_handoff,
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
    }
}

#[allow(clippy::too_many_arguments)]
fn complete_loop_handoff(
    subject: &MetabossEventExtractionSubject,
    completed_split_handoff: &worth_kernel::workload_composition::CompletedBooleanSplitHandoff,
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
    worth_kernel::workload_composition::CompletedBooleanLoopReconstructionHandoff,
    WorkloadCompositionError,
> {
    let recovered_source_carriers =
        continuation_contract_support::recovered_source_carriers(subject, split_request);
    completed_split_handoff.complete_boolean_loop_reconstruction(
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
