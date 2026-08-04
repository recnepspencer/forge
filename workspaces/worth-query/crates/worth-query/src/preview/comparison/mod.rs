pub(super) mod admission;
pub(super) mod contract;
pub(super) mod shape;

#[cfg(test)]
pub(crate) use admission::{
    admit_authoritative_preview_comparison_candidate, admit_preview_promotion_parity_comparison,
    derive_preview_comparison_eligibility,
};
pub use contract::{
    AuthoritativePreviewComparisonCandidate, PreviewComparisonCandidateArtifact,
    PreviewComparisonEligibilityArtifact, PreviewComparisonError, PreviewComparisonFailureClass,
    PreviewExecutionComparisonAdmission, PromotionParityPreviewComparisonAdmission,
};
#[cfg(test)]
pub(super) use shape::PreviewComparisonShapeContract;
