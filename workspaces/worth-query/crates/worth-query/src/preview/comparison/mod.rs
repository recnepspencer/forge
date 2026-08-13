pub(super) mod admission;
pub(super) mod contract;
pub(in crate::preview) mod shape;

#[cfg(test)]
pub(crate) use admission::derive_preview_comparison_eligibility;
pub use admission::{
    admit_authoritative_preview_comparison_candidate, admit_preview_promotion_parity_comparison,
};
pub use contract::{
    AuthoritativePreviewComparisonCandidate, PreviewComparisonCandidateArtifact,
    PreviewComparisonEligibilityArtifact, PreviewComparisonError, PreviewComparisonFailureClass,
    PreviewExecutionComparisonAdmission, PromotionParityPreviewComparisonAdmission,
};
pub(in crate::preview) use shape::PreviewComparisonShapeContract;
