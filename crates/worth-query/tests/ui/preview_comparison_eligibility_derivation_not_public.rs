use worth_query::facade::policy::{PreviewComparisonEligibilityArtifact, PreviewSessionPlanBinding};
use worth_query::facade::derive_preview_comparison_eligibility;

fn main() {
    let _: fn(&PreviewSessionPlanBinding) -> PreviewComparisonEligibilityArtifact =
        derive_preview_comparison_eligibility;
}
