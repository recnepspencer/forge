use crate::application::{ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker};
use crate::continuation_pipeline::{
    ordinary_outcome_from_continuation_checked, ordinary_outcome_from_execution_checked,
    ForgeQueryContinuationExecutionChecked, ForgeQueryContinuationExecutionOutcome,
    ForgeQueryContinuationExecutionTranscript, ForgeQueryPreparedContinuationChecked,
    ForgeQueryPreparedContinuationOutcome, ForgeQueryPreparedContinuationTranscript,
};

use crate::recovery_boundary::foundational::{
    diagnostic_context_for_stop_kind, lean_materialized_profile,
    support_context_for_basis_mismatch, support_context_for_stale_basis,
};
use crate::recovery_boundary::ordinary::forge_query_recovery_brief_from_ordinary_outcome;
use crate::recovery_boundary::{
    ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryBasisPosture, ForgeQueryRecoveryBrief,
    ForgeQueryRecoveryEvidenceStrength, ForgeQueryRecoveryStopKind,
};

pub fn forge_query_recovery_brief_from_prepared_continuation_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryPreparedContinuationChecked<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    let explanation = continuation_prepared_explanation(
        checked.outcome(),
        ForgeQueryRecoveryEvidenceStrength::CheckedRetained,
    );
    forge_query_recovery_brief_from_ordinary_outcome(&ordinary_outcome_from_continuation_checked(
        checked,
    ))
    .map(|brief| {
        let merged = explanation_with_base(&brief, explanation);
        brief.with_explanation(merged)
    })
}

pub fn forge_query_recovery_brief_from_prepared_continuation_proof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    proof: ForgeQueryPreparedContinuationTranscript<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    let explanation = continuation_prepared_explanation(
        proof.outcome(),
        ForgeQueryRecoveryEvidenceStrength::ProofRetained,
    );
    forge_query_recovery_brief_from_ordinary_outcome(&ordinary_outcome_from_continuation_checked(
        proof.into_checked(),
    ))
    .map(|brief| {
        let merged = explanation_with_base(&brief, explanation);
        brief.with_explanation(merged)
    })
}

pub fn forge_query_recovery_brief_from_continuation_execution_checked<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    checked: ForgeQueryContinuationExecutionChecked<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    let explanation = continuation_execution_explanation(
        checked.outcome(),
        ForgeQueryRecoveryEvidenceStrength::CheckedRetained,
    );
    forge_query_recovery_brief_from_ordinary_outcome(&ordinary_outcome_from_execution_checked(
        checked,
    ))
    .map(|brief| {
        let merged = explanation_with_base(&brief, explanation);
        brief.with_explanation(merged)
    })
}

pub fn forge_query_recovery_brief_from_continuation_execution_proof<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    proof: ForgeQueryContinuationExecutionTranscript<D, I>,
) -> Option<ForgeQueryRecoveryBrief> {
    let explanation = continuation_execution_explanation(
        proof.outcome(),
        ForgeQueryRecoveryEvidenceStrength::ProofRetained,
    );
    forge_query_recovery_brief_from_ordinary_outcome(&ordinary_outcome_from_execution_checked(
        proof.into_checked(),
    ))
    .map(|brief| {
        let merged = explanation_with_base(&brief, explanation);
        brief.with_explanation(merged)
    })
}

fn explanation_with_base(
    brief: &ForgeQueryRecoveryBrief,
    explanation: crate::recovery_boundary::ForgeQueryRecoveryExplanation,
) -> crate::recovery_boundary::ForgeQueryRecoveryExplanation {
    brief
        .explanation()
        .clone()
        .with_evidence_strength(explanation.evidence_strength())
        .with_basis_posture(explanation.basis_posture())
        .with_aspect_posture(explanation.aspect_posture())
        .with_diagnostic_context(
            explanation
                .diagnostic_outcome_kind()
                .map(|_| diagnostic_context_for_stop_kind(brief.stop_kind()))
                .unwrap_or_else(|| diagnostic_context_for_stop_kind(brief.stop_kind())),
        )
        .with_source_family(explanation.source_family())
        .with_conflict_posture(explanation.conflict_posture())
        .with_support_context_if_present(explanation)
}

trait RecoveryExplanationMerge {
    fn with_support_context_if_present(
        self,
        source: crate::recovery_boundary::ForgeQueryRecoveryExplanation,
    ) -> Self;
}

impl RecoveryExplanationMerge for crate::recovery_boundary::ForgeQueryRecoveryExplanation {
    fn with_support_context_if_present(
        mut self,
        source: crate::recovery_boundary::ForgeQueryRecoveryExplanation,
    ) -> Self {
        if let (Some(truth_kind), Some(basis_disclosure)) =
            (source.support_truth_kind(), source.basis_disclosure())
        {
            self = self.with_support_context(
                crate::recovery_boundary::ForgeQueryRecoveryFoundationalSupportContext::new(
                    truth_kind,
                    basis_disclosure,
                    source.degraded_recovery_posture(),
                ),
            );
        }
        self
    }
}

fn continuation_prepared_explanation<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    outcome: &ForgeQueryPreparedContinuationOutcome<D, I>,
    evidence_strength: ForgeQueryRecoveryEvidenceStrength,
) -> crate::recovery_boundary::ForgeQueryRecoveryExplanation {
    use crate::recovery_boundary::{ForgeQueryRecoveryExplanation, ForgeQueryRecoverySourceFamily};

    let stop_kind = match outcome {
        ForgeQueryPreparedContinuationOutcome::Prepared(_) => ForgeQueryRecoveryStopKind::Deferred,
        ForgeQueryPreparedContinuationOutcome::Ambiguous(_) => {
            ForgeQueryRecoveryStopKind::Ambiguous
        }
        ForgeQueryPreparedContinuationOutcome::Unavailable(_) => {
            ForgeQueryRecoveryStopKind::Unavailable
        }
        ForgeQueryPreparedContinuationOutcome::WrongWorld(_) => {
            ForgeQueryRecoveryStopKind::WrongWorld
        }
        ForgeQueryPreparedContinuationOutcome::WrongHandle(_) => {
            ForgeQueryRecoveryStopKind::WrongHandle
        }
        ForgeQueryPreparedContinuationOutcome::Stale(_) => ForgeQueryRecoveryStopKind::Stale,
        ForgeQueryPreparedContinuationOutcome::RebindRequired(_) => {
            ForgeQueryRecoveryStopKind::RebindRequired
        }
        ForgeQueryPreparedContinuationOutcome::AuthorityMismatch(_) => {
            ForgeQueryRecoveryStopKind::AuthorityMismatch
        }
        ForgeQueryPreparedContinuationOutcome::BasisMismatch(_) => {
            ForgeQueryRecoveryStopKind::BasisMismatch
        }
        ForgeQueryPreparedContinuationOutcome::Unsupported(_) => {
            ForgeQueryRecoveryStopKind::Unsupported
        }
        ForgeQueryPreparedContinuationOutcome::Deferred(_) => ForgeQueryRecoveryStopKind::Deferred,
        ForgeQueryPreparedContinuationOutcome::Denied(_) => {
            ForgeQueryRecoveryStopKind::DeclarationDenied
        }
        ForgeQueryPreparedContinuationOutcome::Failed(_) => ForgeQueryRecoveryStopKind::Failed,
    };
    let mut base = ForgeQueryRecoveryExplanation::new_with_source_family(
        crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology::continuation(
            crate::ordinary_outcome::ForgeQueryOrdinaryContinuationCheckedTopologyKind::Deferred,
            crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts::new(),
        ),
        ForgeQueryRecoverySourceFamily::Continuation,
    )
    .with_evidence_strength(evidence_strength)
    .with_aspect_posture(ForgeQueryRecoveryAspectPosture::AspectSensitiveReadmission)
    .with_diagnostic_context(diagnostic_context_for_stop_kind(stop_kind));
    if let Some(profile) = lean_materialized_profile() {
        base = base.with_profile(profile);
    }
    match outcome {
        ForgeQueryPreparedContinuationOutcome::Stale(_) => base
            .with_basis_posture(ForgeQueryRecoveryBasisPosture::StaleBasis)
            .with_support_context(support_context_for_stale_basis()),
        ForgeQueryPreparedContinuationOutcome::BasisMismatch(_) => base
            .with_basis_posture(ForgeQueryRecoveryBasisPosture::BasisMismatch)
            .with_support_context(support_context_for_basis_mismatch()),
        _ => base,
    }
}

fn continuation_execution_explanation<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
>(
    outcome: &ForgeQueryContinuationExecutionOutcome<D, I>,
    evidence_strength: ForgeQueryRecoveryEvidenceStrength,
) -> crate::recovery_boundary::ForgeQueryRecoveryExplanation {
    use crate::recovery_boundary::{ForgeQueryRecoveryExplanation, ForgeQueryRecoverySourceFamily};

    let stop_kind = match outcome {
        ForgeQueryContinuationExecutionOutcome::Executed(_) => ForgeQueryRecoveryStopKind::Deferred,
        ForgeQueryContinuationExecutionOutcome::WrongWorld(_) => {
            ForgeQueryRecoveryStopKind::WrongWorld
        }
        ForgeQueryContinuationExecutionOutcome::AsyncRequestDrift(_) => {
            ForgeQueryRecoveryStopKind::AsyncRequestDrift
        }
        ForgeQueryContinuationExecutionOutcome::ReplayDrift(_) => {
            ForgeQueryRecoveryStopKind::ReplayDrift
        }
        ForgeQueryContinuationExecutionOutcome::RemaskDrift(_) => {
            ForgeQueryRecoveryStopKind::RemaskDrift
        }
        ForgeQueryContinuationExecutionOutcome::PreviewCrossedResidue(_) => {
            ForgeQueryRecoveryStopKind::PreviewCrossedResidue
        }
        ForgeQueryContinuationExecutionOutcome::Stale(_) => ForgeQueryRecoveryStopKind::Stale,
        ForgeQueryContinuationExecutionOutcome::StaleCompletion(_) => {
            ForgeQueryRecoveryStopKind::StaleCompletion
        }
        ForgeQueryContinuationExecutionOutcome::BasisMismatch(_) => {
            ForgeQueryRecoveryStopKind::BasisMismatch
        }
        ForgeQueryContinuationExecutionOutcome::AuthorityMismatch(_) => {
            ForgeQueryRecoveryStopKind::AuthorityMismatch
        }
        ForgeQueryContinuationExecutionOutcome::WrongHandle(_) => {
            ForgeQueryRecoveryStopKind::WrongHandle
        }
        ForgeQueryContinuationExecutionOutcome::Unsupported(_) => {
            ForgeQueryRecoveryStopKind::Unsupported
        }
    };
    let mut base = ForgeQueryRecoveryExplanation::new_with_source_family(
        crate::ordinary_outcome::ForgeQueryOrdinaryCheckedTopology::continuation(
            crate::ordinary_outcome::ForgeQueryOrdinaryContinuationCheckedTopologyKind::Unsupported,
            crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts::new(),
        ),
        ForgeQueryRecoverySourceFamily::Continuation,
    )
    .with_evidence_strength(evidence_strength)
    .with_aspect_posture(ForgeQueryRecoveryAspectPosture::AspectSensitiveReadmission)
    .with_diagnostic_context(diagnostic_context_for_stop_kind(stop_kind));
    if let Some(profile) = lean_materialized_profile() {
        base = base.with_profile(profile);
    }
    match outcome {
        ForgeQueryContinuationExecutionOutcome::ReplayDrift(_)
        | ForgeQueryContinuationExecutionOutcome::StaleCompletion(_) => base
            .with_basis_posture(ForgeQueryRecoveryBasisPosture::StaleBasis)
            .with_support_context(support_context_for_stale_basis()),
        ForgeQueryContinuationExecutionOutcome::RemaskDrift(_) => {
            base.with_basis_posture(ForgeQueryRecoveryBasisPosture::Unknown)
        }
        ForgeQueryContinuationExecutionOutcome::Stale(_) => base
            .with_basis_posture(ForgeQueryRecoveryBasisPosture::StaleBasis)
            .with_support_context(support_context_for_stale_basis()),
        ForgeQueryContinuationExecutionOutcome::BasisMismatch(_) => base
            .with_basis_posture(ForgeQueryRecoveryBasisPosture::BasisMismatch)
            .with_support_context(support_context_for_basis_mismatch()),
        _ => base,
    }
}
