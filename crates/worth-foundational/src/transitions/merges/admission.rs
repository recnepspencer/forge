use worth_proof::TransitionOutcome;

use super::builder::FoundationalMergeCandidate;
use super::scope_evidence::{
    FoundationalAdmittedMergeScopeEvidence, FoundationalScopeAdmissionBasis,
};
use super::scope_non_success::{
    FoundationalScopedMergeDenialEvidence, FoundationalScopedMergeUnavailableOutcomeCategory,
    FoundationalScopedMergeUnavailablePosture,
};
use super::verdict::FoundationalMergeVerdict;
use super::vocabulary::{
    FoundationalBranchBasisDrift, FoundationalMergeAdmissionDeferred,
    FoundationalMergeAdmissionDenial, FoundationalMergeAdmissionFailure,
    FoundationalMergeAdmissionOutcome, FoundationalMergeAdmissionRebindRequired,
    FoundationalMergeConflictLocus, FoundationalMergeVerdictKind,
};
use crate::transitions::FoundationalBranchId;

impl<T> FoundationalMergeCandidate<T> {
    pub fn admit_as_accepted(
        self,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        let scope_evidence = self.default_scope_evidence();
        TransitionOutcome::success(FoundationalMergeVerdict::new(
            self,
            FoundationalMergeVerdictKind::Accepted,
            scope_evidence,
            Vec::new(),
            None,
        ))
    }

    pub fn admit_as_accepted_with_scope_evidence(
        self,
        scope_evidence: FoundationalAdmittedMergeScopeEvidence,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        if let Err(denial) = self.validate_scope_evidence(&scope_evidence) {
            return TransitionOutcome::denied(denial);
        }
        TransitionOutcome::success(FoundationalMergeVerdict::new(
            self,
            FoundationalMergeVerdictKind::Accepted,
            scope_evidence,
            Vec::new(),
            None,
        ))
    }

    pub fn admit_as_advisory(
        self,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        let scope_evidence = self.default_scope_evidence();
        TransitionOutcome::success(FoundationalMergeVerdict::new(
            self,
            FoundationalMergeVerdictKind::Advisory,
            scope_evidence,
            Vec::new(),
            None,
        ))
    }

    pub fn admit_as_advisory_with_scope_evidence(
        self,
        scope_evidence: FoundationalAdmittedMergeScopeEvidence,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        if let Err(denial) = self.validate_scope_evidence(&scope_evidence) {
            return TransitionOutcome::denied(denial);
        }
        TransitionOutcome::success(FoundationalMergeVerdict::new(
            self,
            FoundationalMergeVerdictKind::Advisory,
            scope_evidence,
            Vec::new(),
            None,
        ))
    }

    pub fn admit_as_conflict(
        self,
        conflict_loci: Vec<FoundationalMergeConflictLocus>,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        if conflict_loci.is_empty() {
            return TransitionOutcome::denied(FoundationalMergeAdmissionDenial::EmptyConflictLoci);
        }
        let scope_evidence = self.default_scope_evidence();
        TransitionOutcome::success(FoundationalMergeVerdict::new(
            self,
            FoundationalMergeVerdictKind::Conflict,
            scope_evidence,
            conflict_loci,
            None,
        ))
    }

    pub fn admit_as_conflict_with_scope_evidence(
        self,
        conflict_loci: Vec<FoundationalMergeConflictLocus>,
        scope_evidence: FoundationalAdmittedMergeScopeEvidence,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        if conflict_loci.is_empty() {
            return TransitionOutcome::denied(FoundationalMergeAdmissionDenial::EmptyConflictLoci);
        }
        if let Err(denial) = self.validate_scope_evidence(&scope_evidence) {
            return TransitionOutcome::denied(denial);
        }
        TransitionOutcome::success(FoundationalMergeVerdict::new(
            self,
            FoundationalMergeVerdictKind::Conflict,
            scope_evidence,
            conflict_loci,
            None,
        ))
    }

    pub fn admit_as_superseded(
        self,
        superseded_by_branch: FoundationalBranchId,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        let scope_evidence = self.default_scope_evidence();
        TransitionOutcome::success(FoundationalMergeVerdict::new(
            self,
            FoundationalMergeVerdictKind::Superseded,
            scope_evidence,
            Vec::new(),
            Some(superseded_by_branch),
        ))
    }

    pub fn deny(
        self,
        reason: &'static str,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        let _ = self;
        TransitionOutcome::denied(FoundationalMergeAdmissionDenial::PolicyDenied { reason })
    }

    pub fn deny_selected_scope(
        self,
        evidence: FoundationalScopedMergeDenialEvidence,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        if evidence.requested_scope() != self.scope() {
            return TransitionOutcome::denied(
                FoundationalMergeAdmissionDenial::ScopedEvidenceScopeMismatch,
            );
        }
        if evidence.source_branch() != self.source_branch() {
            return TransitionOutcome::denied(
                FoundationalMergeAdmissionDenial::ScopedEvidenceSourceBranchMismatch,
            );
        }
        if evidence.target_branch() != self.target_branch() {
            return TransitionOutcome::denied(
                FoundationalMergeAdmissionDenial::ScopedEvidenceTargetBranchMismatch,
            );
        }
        TransitionOutcome::denied(FoundationalMergeAdmissionDenial::ScopedSelectionDenied(
            evidence,
        ))
    }

    pub fn scope_unavailable(
        self,
        posture: FoundationalScopedMergeUnavailablePosture,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        if posture.requested_scope() != self.scope() {
            return TransitionOutcome::denied(
                FoundationalMergeAdmissionDenial::ScopedEvidenceScopeMismatch,
            );
        }
        if posture.source_branch() != self.source_branch() {
            return TransitionOutcome::denied(
                FoundationalMergeAdmissionDenial::ScopedEvidenceSourceBranchMismatch,
            );
        }
        if posture.target_branch() != self.target_branch() {
            return TransitionOutcome::denied(
                FoundationalMergeAdmissionDenial::ScopedEvidenceTargetBranchMismatch,
            );
        }
        match posture.outcome_category() {
            FoundationalScopedMergeUnavailableOutcomeCategory::Deferred => {
                TransitionOutcome::deferred(FoundationalMergeAdmissionDeferred::scope_unavailable(
                    posture,
                ))
            }
            FoundationalScopedMergeUnavailableOutcomeCategory::Stale => {
                TransitionOutcome::stale(FoundationalBranchBasisDrift::scope_unavailable(posture))
            }
            FoundationalScopedMergeUnavailableOutcomeCategory::RebindRequired => {
                TransitionOutcome::rebind_required(
                    FoundationalMergeAdmissionRebindRequired::scope_unavailable(posture),
                )
            }
            FoundationalScopedMergeUnavailableOutcomeCategory::Failed => TransitionOutcome::failed(
                FoundationalMergeAdmissionFailure::scope_unavailable(posture),
            ),
        }
    }

    pub fn stale(
        self,
        drift: FoundationalBranchBasisDrift,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        let _ = self;
        TransitionOutcome::stale(drift)
    }

    pub fn defer(
        self,
        reason: &'static str,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        let _ = self;
        TransitionOutcome::deferred(FoundationalMergeAdmissionDeferred::new(reason))
    }

    pub fn require_rebind(
        self,
        reason: &'static str,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        let _ = self;
        TransitionOutcome::rebind_required(FoundationalMergeAdmissionRebindRequired::new(reason))
    }

    pub fn fail(
        self,
        reason: &'static str,
    ) -> FoundationalMergeAdmissionOutcome<FoundationalMergeVerdict<T>> {
        let _ = self;
        TransitionOutcome::failed(FoundationalMergeAdmissionFailure::new(reason))
    }

    fn default_scope_evidence(&self) -> FoundationalAdmittedMergeScopeEvidence {
        FoundationalAdmittedMergeScopeEvidence::admit_all_requested(
            self.source_branch().clone(),
            self.target_branch().clone(),
            self.scope(),
            FoundationalScopeAdmissionBasis::DirectSourceIdentity,
            self.structural_summary().conflict_check_width(),
        )
    }

    fn validate_scope_evidence(
        &self,
        scope_evidence: &FoundationalAdmittedMergeScopeEvidence,
    ) -> Result<(), FoundationalMergeAdmissionDenial> {
        if scope_evidence.requested_scope() != self.scope() {
            return Err(FoundationalMergeAdmissionDenial::ScopedEvidenceScopeMismatch);
        }
        if scope_evidence.source_branch() != self.source_branch() {
            return Err(FoundationalMergeAdmissionDenial::ScopedEvidenceSourceBranchMismatch);
        }
        if scope_evidence.target_branch() != self.target_branch() {
            return Err(FoundationalMergeAdmissionDenial::ScopedEvidenceTargetBranchMismatch);
        }
        if scope_evidence.breadth().conflict_check_width()
            != self.structural_summary().conflict_check_width()
        {
            return Err(FoundationalMergeAdmissionDenial::ScopedEvidenceConflictWidthMismatch);
        }
        Ok(())
    }
}
