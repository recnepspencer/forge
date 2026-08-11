use crate::{
    failure::StoreError, RawSupportProgramAction, SubscriptionSupportCompatibilityBatchRequest,
    SubscriptionSupportCompatibilityDecisionKind, SubscriptionSupportCompatibilityReport,
    SupportCompatibilityAffectedSet, SupportCompatibilityBatchPlan,
    SupportDecodedRowSemanticAccess, SupportManifestAdmissionWitness,
};

use super::super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn admit_subscription_support_compatibility_batch(
        &mut self,
        request: SubscriptionSupportCompatibilityBatchRequest,
    ) -> Result<SupportCompatibilityBatchPlan, StoreError> {
        let SubscriptionSupportCompatibilityBatchRequest {
            action_id,
            affected_bases,
            compatibility_receipt,
            semantic_digest,
            decision,
            path,
        } = request;
        let affected_set =
            SupportCompatibilityAffectedSet::from_compatibility_bases(affected_bases)?;
        let path_plan = self.admit_subscription_support_program_path(
            path.admission_request(affected_set.affected_count()),
        )?;
        let manifest_admission = SupportManifestAdmissionWitness::from_compatibility_receipt(
            compatibility_receipt,
            affected_set.primary_basis().compatibility_digest(),
        )?;
        let semantic_access = SupportDecodedRowSemanticAccess::from_manifest_admission(
            manifest_admission.clone(),
            semantic_digest,
        )?;
        let plan = SupportCompatibilityBatchPlan::new(
            action_id,
            affected_set,
            path_plan,
            manifest_admission,
            semantic_access,
            decision,
        )?;
        self.state
            .subscription_support_counter_snapshot
            .record_support_compatibility_plan(plan.affected_set().affected_count());
        Ok(plan)
    }

    pub fn publish_subscription_support_compatibility_consequence(
        &mut self,
        plan: SupportCompatibilityBatchPlan,
    ) -> Result<SubscriptionSupportCompatibilityReport, StoreError> {
        let (action_id, affected_set, path_plan, manifest_admission, semantic_access, decision) =
            plan.into_parts();
        let raw_action = RawSupportProgramAction::new(
            action_id,
            affected_set.primary_basis().clone(),
            decision.verdict(),
        )?;
        let completed =
            self.publish_support_action_with_durable_recovery(raw_action, path_plan.budget())?;
        let report = SubscriptionSupportCompatibilityReport::new(
            completed,
            affected_set,
            &path_plan,
            manifest_admission,
            semantic_access,
            &decision,
        )?;
        match decision.kind() {
            SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_exact_compatible_migration();
            }
            SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_degraded_compatibility();
            }
            SubscriptionSupportCompatibilityDecisionKind::OldReaderRejected
            | SubscriptionSupportCompatibilityDecisionKind::UnknownFamilyRejected
            | SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_version_skew_rejection();
            }
        }
        Ok(report)
    }
}
