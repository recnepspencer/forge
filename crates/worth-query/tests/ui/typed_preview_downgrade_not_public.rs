use worth_query::facade::policy::{PromotionEligiblePreviewExecutionEnvelope, PromotionEligiblePreviewSessionPlanBinding, ReadOnlyPreviewExecutionEnvelope, ReadOnlyPreviewSessionPlanBinding};

fn binding_downgrade(
    read_only: &ReadOnlyPreviewSessionPlanBinding,
    promotion: &PromotionEligiblePreviewSessionPlanBinding,
) {
    let _ = read_only.as_preview_binding();
    let _ = promotion.as_preview_binding();
}

fn execution_downgrade(
    read_only: &ReadOnlyPreviewExecutionEnvelope,
    promotion: &PromotionEligiblePreviewExecutionEnvelope,
) {
    let _ = read_only.as_preview_execution();
    let _ = promotion.as_preview_execution();
}

fn main() {
    let _ = binding_downgrade;
    let _ = execution_downgrade;
}
