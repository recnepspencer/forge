use worth_query::facade::{
    admit_preview_promotion_parity_comparison, AuthoritativePreviewComparisonCandidate,
    PreviewComparisonError, PromotionParityPreviewComparisonAdmission, ReadOnlyPreviewExecutionEnvelope,
};

fn main() {
    let _: fn(
        &ReadOnlyPreviewExecutionEnvelope,
        &AuthoritativePreviewComparisonCandidate,
    ) -> Result<PromotionParityPreviewComparisonAdmission, PreviewComparisonError> =
        admit_preview_promotion_parity_comparison;
}
