use crate::application::{WorthQueryDeclarationInput, WorthQueryDomainEntryMarker};
use crate::continuation_pipeline::{
    ordinary_outcome_from_continuation_checked, ordinary_outcome_from_execution_checked,
    WorthQueryContinuationExecutionChecked, WorthQueryContinuationExecutionOutcome,
    WorthQueryContinuationExecutionTranscript, WorthQueryPreparedContinuationChecked,
    WorthQueryPreparedContinuationOutcome, WorthQueryPreparedContinuationTranscript,
};

use crate::recovery_boundary::foundational::{
    diagnostic_context_for_stop_kind, lean_materialized_profile,
    support_context_for_basis_mismatch, support_context_for_stale_basis,
};
use crate::recovery_boundary::ordinary::worth_query_recovery_brief_from_ordinary_outcome;
use crate::recovery_boundary::{
    WorthQueryRecoveryAspectPosture, WorthQueryRecoveryBasisPosture, WorthQueryRecoveryBrief,
    WorthQueryRecoveryEvidenceStrength, WorthQueryRecoveryStopKind,
};

pub fn worth_query_recovery_brief_from_prepared_continuation_checked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryPreparedContinuationChecked<D, I>,
) -> Option<WorthQueryRecoveryBrief> {
    let explanation = continuation_prepared_explanation(
        checked.outcome(),
        WorthQueryRecoveryEvidenceStrength::CheckedRetained,
    );
    worth_query_recovery_brief_from_ordinary_outcome(&ordinary_outcome_from_continuation_checked(
        checked,
    ))
    .map(|brief| {
        let merged = explanation_with_base(&brief, explanation);
        brief.with_explanation(merged)
    })
}

pub fn worth_query_recovery_brief_from_prepared_continuation_proof<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    proof: WorthQueryPreparedContinuationTranscript<D, I>,
) -> Option<WorthQueryRecoveryBrief> {
    let explanation = continuation_prepared_explanation(
        proof.outcome(),
        WorthQueryRecoveryEvidenceStrength::ProofRetained,
    );
    worth_query_recovery_brief_from_ordinary_outcome(&ordinary_outcome_from_continuation_checked(
        proof.into_checked(),
    ))
    .map(|brief| {
        let merged = explanation_with_base(&brief, explanation);
        brief.with_explanation(merged)
    })
}

pub fn worth_query_recovery_brief_from_continuation_execution_checked<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    checked: WorthQueryContinuationExecutionChecked<D, I>,
) -> Option<WorthQueryRecoveryBrief> {
    let explanation = continuation_execution_explanation(
        checked.outcome(),
        WorthQueryRecoveryEvidenceStrength::CheckedRetained,
    );
    worth_query_recovery_brief_from_ordinary_outcome(&ordinary_outcome_from_execution_checked(
        checked,
    ))
    .map(|brief| {
        let merged = explanation_with_base(&brief, explanation);
        brief.with_explanation(merged)
    })
}

pub fn worth_query_recovery_brief_from_continuation_execution_proof<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    proof: WorthQueryContinuationExecutionTranscript<D, I>,
) -> Option<WorthQueryRecoveryBrief> {
    let explanation = continuation_execution_explanation(
        proof.outcome(),
        WorthQueryRecoveryEvidenceStrength::ProofRetained,
    );
    worth_query_recovery_brief_from_ordinary_outcome(&ordinary_outcome_from_execution_checked(
        proof.into_checked(),
    ))
    .map(|brief| {
        let merged = explanation_with_base(&brief, explanation);
        brief.with_explanation(merged)
    })
}

fn explanation_with_base(
    brief: &WorthQueryRecoveryBrief,
    explanation: crate::recovery_boundary::WorthQueryRecoveryExplanation,
) -> crate::recovery_boundary::WorthQueryRecoveryExplanation {
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
        .with_retained_context_if_present(explanation)
}

trait RecoveryExplanationMerge {
    fn with_retained_context_if_present(
        self,
        source: crate::recovery_boundary::WorthQueryRecoveryExplanation,
    ) -> Self;
}

impl RecoveryExplanationMerge for crate::recovery_boundary::WorthQueryRecoveryExplanation {
    fn with_retained_context_if_present(
        mut self,
        source: crate::recovery_boundary::WorthQueryRecoveryExplanation,
    ) -> Self {
        if let (Some(truth_kind), Some(basis_disclosure)) =
            (source.support_truth_kind(), source.basis_disclosure())
        {
            self = self.with_support_context(
                crate::recovery_boundary::WorthQueryRecoveryFoundationalSupportContext::new(
                    truth_kind,
                    basis_disclosure,
                    source.degraded_recovery_posture(),
                ),
            );
        }
        if let Some(drift) = source.installed_domain_execution_drift() {
            self = self.with_installed_domain_execution_drift(drift.clone());
        }
        self
    }
}

fn continuation_prepared_explanation<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    outcome: &WorthQueryPreparedContinuationOutcome<D, I>,
    evidence_strength: WorthQueryRecoveryEvidenceStrength,
) -> crate::recovery_boundary::WorthQueryRecoveryExplanation {
    use crate::recovery_boundary::{WorthQueryRecoveryExplanation, WorthQueryRecoverySourceFamily};

    let stop_kind = match outcome {
        WorthQueryPreparedContinuationOutcome::Prepared(_) => WorthQueryRecoveryStopKind::Deferred,
        WorthQueryPreparedContinuationOutcome::Ambiguous(_) => {
            WorthQueryRecoveryStopKind::Ambiguous
        }
        WorthQueryPreparedContinuationOutcome::Unavailable(_) => {
            WorthQueryRecoveryStopKind::Unavailable
        }
        WorthQueryPreparedContinuationOutcome::WrongWorld(_) => {
            WorthQueryRecoveryStopKind::WrongWorld
        }
        WorthQueryPreparedContinuationOutcome::WrongHandle(_) => {
            WorthQueryRecoveryStopKind::WrongHandle
        }
        WorthQueryPreparedContinuationOutcome::InstalledAuthorityDrift(_) => {
            WorthQueryRecoveryStopKind::RebindRequired
        }
        WorthQueryPreparedContinuationOutcome::Stale(_) => WorthQueryRecoveryStopKind::Stale,
        WorthQueryPreparedContinuationOutcome::RebindRequired(_) => {
            WorthQueryRecoveryStopKind::RebindRequired
        }
        WorthQueryPreparedContinuationOutcome::AuthorityMismatch(_) => {
            WorthQueryRecoveryStopKind::AuthorityMismatch
        }
        WorthQueryPreparedContinuationOutcome::BasisMismatch(_) => {
            WorthQueryRecoveryStopKind::BasisMismatch
        }
        WorthQueryPreparedContinuationOutcome::Unsupported(_) => {
            WorthQueryRecoveryStopKind::Unsupported
        }
        WorthQueryPreparedContinuationOutcome::Deferred(_) => WorthQueryRecoveryStopKind::Deferred,
        WorthQueryPreparedContinuationOutcome::Denied(_) => {
            WorthQueryRecoveryStopKind::DeclarationDenied
        }
        WorthQueryPreparedContinuationOutcome::Failed(_) => WorthQueryRecoveryStopKind::Failed,
    };
    let mut base = WorthQueryRecoveryExplanation::new_with_source_family(
        crate::ordinary_outcome::WorthQueryOrdinaryCheckedTopology::continuation(
            crate::ordinary_outcome::WorthQueryOrdinaryContinuationCheckedTopologyKind::Deferred,
            crate::binding_pipeline::WorthQueryBindingLinkedArtifacts::new(),
        ),
        WorthQueryRecoverySourceFamily::Continuation,
    )
    .with_evidence_strength(evidence_strength)
    .with_aspect_posture(WorthQueryRecoveryAspectPosture::AspectSensitiveReadmission)
    .with_diagnostic_context(diagnostic_context_for_stop_kind(stop_kind));
    if let Some(profile) = lean_materialized_profile() {
        base = base.with_profile(profile);
    }
    match outcome {
        WorthQueryPreparedContinuationOutcome::InstalledAuthorityDrift(drift) => {
            base.with_installed_domain_execution_drift(drift.clone())
        }
        WorthQueryPreparedContinuationOutcome::Stale(_) => base
            .with_basis_posture(WorthQueryRecoveryBasisPosture::StaleBasis)
            .with_support_context(support_context_for_stale_basis()),
        WorthQueryPreparedContinuationOutcome::BasisMismatch(_) => base
            .with_basis_posture(WorthQueryRecoveryBasisPosture::BasisMismatch)
            .with_support_context(support_context_for_basis_mismatch()),
        _ => base,
    }
}

fn continuation_execution_explanation<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
>(
    outcome: &WorthQueryContinuationExecutionOutcome<D, I>,
    evidence_strength: WorthQueryRecoveryEvidenceStrength,
) -> crate::recovery_boundary::WorthQueryRecoveryExplanation {
    use crate::recovery_boundary::{WorthQueryRecoveryExplanation, WorthQueryRecoverySourceFamily};

    let stop_kind = match outcome {
        WorthQueryContinuationExecutionOutcome::Executed(_) => WorthQueryRecoveryStopKind::Deferred,
        WorthQueryContinuationExecutionOutcome::WrongWorld(_) => {
            WorthQueryRecoveryStopKind::WrongWorld
        }
        WorthQueryContinuationExecutionOutcome::AsyncRequestDrift(_) => {
            WorthQueryRecoveryStopKind::AsyncRequestDrift
        }
        WorthQueryContinuationExecutionOutcome::ReplayDrift(_) => {
            WorthQueryRecoveryStopKind::ReplayDrift
        }
        WorthQueryContinuationExecutionOutcome::RemaskDrift(_) => {
            WorthQueryRecoveryStopKind::RemaskDrift
        }
        WorthQueryContinuationExecutionOutcome::PreviewCrossedResidue(_) => {
            WorthQueryRecoveryStopKind::PreviewCrossedResidue
        }
        WorthQueryContinuationExecutionOutcome::InstalledAuthorityDrift(_) => {
            WorthQueryRecoveryStopKind::RebindRequired
        }
        WorthQueryContinuationExecutionOutcome::Stale(_) => WorthQueryRecoveryStopKind::Stale,
        WorthQueryContinuationExecutionOutcome::StaleCompletion(_) => {
            WorthQueryRecoveryStopKind::StaleCompletion
        }
        WorthQueryContinuationExecutionOutcome::BasisMismatch(_) => {
            WorthQueryRecoveryStopKind::BasisMismatch
        }
        WorthQueryContinuationExecutionOutcome::LowerBindingMismatch(_) => {
            WorthQueryRecoveryStopKind::AuthorityMismatch
        }
        WorthQueryContinuationExecutionOutcome::AuthorityMismatch(_) => {
            WorthQueryRecoveryStopKind::AuthorityMismatch
        }
        WorthQueryContinuationExecutionOutcome::WrongHandle(_) => {
            WorthQueryRecoveryStopKind::WrongHandle
        }
        WorthQueryContinuationExecutionOutcome::Unsupported(_) => {
            WorthQueryRecoveryStopKind::Unsupported
        }
    };
    let mut base = WorthQueryRecoveryExplanation::new_with_source_family(
        crate::ordinary_outcome::WorthQueryOrdinaryCheckedTopology::continuation(
            crate::ordinary_outcome::WorthQueryOrdinaryContinuationCheckedTopologyKind::Unsupported,
            crate::binding_pipeline::WorthQueryBindingLinkedArtifacts::new(),
        ),
        WorthQueryRecoverySourceFamily::Continuation,
    )
    .with_evidence_strength(evidence_strength)
    .with_aspect_posture(WorthQueryRecoveryAspectPosture::AspectSensitiveReadmission)
    .with_diagnostic_context(diagnostic_context_for_stop_kind(stop_kind));
    if let Some(profile) = lean_materialized_profile() {
        base = base.with_profile(profile);
    }
    match outcome {
        WorthQueryContinuationExecutionOutcome::InstalledAuthorityDrift(drift) => {
            base.with_installed_domain_execution_drift(drift.clone())
        }
        WorthQueryContinuationExecutionOutcome::ReplayDrift(_)
        | WorthQueryContinuationExecutionOutcome::StaleCompletion(_) => base
            .with_basis_posture(WorthQueryRecoveryBasisPosture::StaleBasis)
            .with_support_context(support_context_for_stale_basis()),
        WorthQueryContinuationExecutionOutcome::RemaskDrift(_) => {
            base.with_basis_posture(WorthQueryRecoveryBasisPosture::Unknown)
        }
        WorthQueryContinuationExecutionOutcome::Stale(_) => base
            .with_basis_posture(WorthQueryRecoveryBasisPosture::StaleBasis)
            .with_support_context(support_context_for_stale_basis()),
        WorthQueryContinuationExecutionOutcome::BasisMismatch(_) => base
            .with_basis_posture(WorthQueryRecoveryBasisPosture::BasisMismatch)
            .with_support_context(support_context_for_basis_mismatch()),
        _ => base,
    }
}
