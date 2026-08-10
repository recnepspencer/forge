use super::*;

impl RuntimeBridge {
    /// Builds an offline audit index from sealed certification bundles. This is
    /// the only bundle collection shape accepted by the offline audit phase.
    pub fn build_subscription_offline_audit_bundle_index(
        &self,
        bundles: Vec<&BridgeSubscriptionCertificationBundleSealed>,
    ) -> BridgeSubscriptionOfflineAuditBundleIndex {
        let _ = self;
        BridgeSubscriptionOfflineAuditBundleIndex::build(bundles)
    }

    /// Admits an offline audit plan from sealed bundle indexes and comparison
    /// reports. Host logs and live runtime handles are explicitly rejected.
    pub fn plan_subscription_offline_audit(
        &self,
        bundle_index: &BridgeSubscriptionOfflineAuditBundleIndex,
        comparison_reports: Vec<&BridgeSubscriptionCertificationComparisonReport>,
        host_log_dependency_requested: bool,
        live_state_dependency_requested: bool,
    ) -> Result<BridgeSubscriptionOfflineAuditPlan, BridgeSubscriptionOfflineAuditPlanRejection>
    {
        let _ = self;
        BridgeSubscriptionOfflineAuditPlan::admit(
            bundle_index,
            comparison_reports,
            host_log_dependency_requested,
            live_state_dependency_requested,
        )
    }

    /// Diagnoses subscription certification offline from an admitted audit
    /// plan. This does not replay host behavior or query live runtime state.
    pub fn audit_subscription_certification_bundle_offline(
        &self,
        audit_plan: BridgeSubscriptionOfflineAuditPlan,
    ) -> BridgeSubscriptionOfflineAuditReport {
        let _ = self;
        BridgeSubscriptionOfflineAuditReport::audit(audit_plan)
    }

    /// Produces the public certification inspection view for the diagnostics
    /// entrypoint from an offline audit report.
    pub fn inspect_subscription_certification(
        &self,
        report: &BridgeSubscriptionOfflineAuditReport,
    ) -> BridgeSubscriptionCertificationInspection {
        let _ = self;
        BridgeSubscriptionCertificationInspection::from_offline_audit(report)
    }

    /// Produces the public certification inspection view for a complete
    /// Milestone 16 reference workload report without reopening sealed bundles
    /// or replaying host behavior.
    pub fn inspect_subscription_reference_workload_certification(
        &self,
        report: &BridgeSubscriptionReferenceWorkloadReport,
    ) -> BridgeSubscriptionReferenceWorkloadInspection {
        let _ = self;
        BridgeSubscriptionReferenceWorkloadInspection::from_reference_workload(report)
    }
}
