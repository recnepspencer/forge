use worth_query::facade::{
    derive_preview_comparison_eligibility, PreviewComparisonEligibilityArtifact,
    PreviewSessionPlanBinding,
};

fn main() {
    let _: fn(&PreviewSessionPlanBinding) -> PreviewComparisonEligibilityArtifact =
        derive_preview_comparison_eligibility;
}
