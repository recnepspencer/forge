use super::*;

impl RuntimeBridge {
    /// Admits a first-class comparison plan before sealed bundles are compared.
    pub fn plan_subscription_certification_comparison(
        &self,
        relationship: BridgeSubscriptionCertificationComparisonRelationship,
        expected_failure_boundary: Option<BridgeSubscriptionCertificationFailureBoundary>,
        divergence_axis: Option<BridgeSubscriptionCertificationDivergenceAxis>,
    ) -> Result<
        BridgeSubscriptionCertificationComparisonPlan,
        BridgeSubscriptionCertificationComparisonPlanRejection,
    > {
        let _ = self;
        BridgeSubscriptionCertificationComparisonPlan::admit(
            relationship,
            expected_failure_boundary,
            divergence_axis,
        )
    }

    /// Compares sealed certification bundles through an admitted relationship
    /// plan. Draft bundles cannot reach this phase.
    pub fn compare_subscription_certification_bundles(
        &self,
        plan: BridgeSubscriptionCertificationComparisonPlan,
        left: &BridgeSubscriptionCertificationBundleSealed,
        right: &BridgeSubscriptionCertificationBundleSealed,
    ) -> BridgeSubscriptionCertificationComparisonReport {
        let _ = self;
        BridgeSubscriptionCertificationComparisonReport::compare(plan, left, right)
    }
}
