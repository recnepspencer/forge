mod boundary;
mod loop_prevention;
mod zero_residue;

use crate::facade::{
    BridgeWritebackError, BridgeWritebackErrorKind, BridgeWritebackLoopPreventionReport,
};

pub(in crate::harness::adapter::adapter_impl) use boundary::{
    AuthorityDenialBoundaryClass, AuthorityDenialBoundaryEvidence, AuthorityDenialBoundaryFailure,
    AuthorityDenialBoundaryFailureEvidence, AuthorityDenialBoundaryMatrix,
};
pub(in crate::harness::adapter::adapter_impl) use loop_prevention::AuthorityDenialLoopPreventionEvidence;
pub(in crate::harness::adapter::adapter_impl) use zero_residue::AuthorityDenialZeroResidueProof;

pub(in crate::harness::adapter::adapter_impl) struct WritebackAuthorityDenialMatrix {
    validation_failure_kind: BridgeWritebackErrorKind,
    validation_detail: String,
    unsafe_feedback_partial: AuthorityDenialLoopPreventionEvidence,
    unsafe_feedback_contradictory: AuthorityDenialLoopPreventionEvidence,
    authority_boundary: AuthorityDenialBoundaryMatrix,
}

impl WritebackAuthorityDenialMatrix {
    pub(in crate::harness::adapter::adapter_impl) fn from_authority_evidence(
        validation_error: &BridgeWritebackError,
        validation_detail: impl Into<String>,
        unsafe_feedback_partial: &BridgeWritebackLoopPreventionReport,
        unsafe_feedback_contradictory: &BridgeWritebackLoopPreventionReport,
        authority_boundary: AuthorityDenialBoundaryEvidence<'_>,
    ) -> Self {
        Self {
            validation_failure_kind: validation_error.kind(),
            validation_detail: validation_detail.into(),
            unsafe_feedback_partial: AuthorityDenialLoopPreventionEvidence::from_loop_prevention(
                unsafe_feedback_partial,
            ),
            unsafe_feedback_contradictory:
                AuthorityDenialLoopPreventionEvidence::from_loop_prevention(
                    unsafe_feedback_contradictory,
                ),
            authority_boundary: AuthorityDenialBoundaryMatrix::from_boundary_evidence(
                authority_boundary,
            ),
        }
    }

    pub(in crate::harness::adapter::adapter_impl) fn validation_failure_kind(
        &self,
    ) -> BridgeWritebackErrorKind {
        self.validation_failure_kind
    }

    pub(in crate::harness::adapter::adapter_impl) fn validation_detail(&self) -> &str {
        &self.validation_detail
    }

    pub(in crate::harness::adapter::adapter_impl) fn unsafe_feedback_partial(
        &self,
    ) -> &AuthorityDenialLoopPreventionEvidence {
        &self.unsafe_feedback_partial
    }

    pub(in crate::harness::adapter::adapter_impl) fn unsafe_feedback_contradictory(
        &self,
    ) -> &AuthorityDenialLoopPreventionEvidence {
        &self.unsafe_feedback_contradictory
    }

    pub(in crate::harness::adapter::adapter_impl) fn authority_boundary(
        &self,
    ) -> &AuthorityDenialBoundaryMatrix {
        &self.authority_boundary
    }
}
