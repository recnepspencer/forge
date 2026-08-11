use super::super::{
    RawSupportProgramAction, SubscriptionSupportCompatibilityDecision,
    SubscriptionSupportCompatibilityDecisionKind, SubscriptionSupportCompatibilityReport,
    SubscriptionSupportOperationalBasis, SupportActionBreadthBudget, SupportActionId,
    SupportAllocationScope, SupportCompatibilityAffectedSet, SupportCompatibilityBatchPlan,
    SupportCompatibilityReceiptWitness, SupportDecodedRowSemanticAccess,
    SupportManifestAdmissionWitness, SupportPathClass, SupportProgramDensityClass,
};
use super::SubscriptionSupportPublicationPipeline;
use crate::failure::StoreError;

impl SubscriptionSupportPublicationPipeline {
    pub fn admit_support_compatibility_batch(
        &mut self,
        action_id: SupportActionId,
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
        compatibility_receipt: SupportCompatibilityReceiptWitness,
        semantic_digest: impl Into<String>,
        decision: SubscriptionSupportCompatibilityDecision,
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        payload_header_bytes: u64,
    ) -> Result<SupportCompatibilityBatchPlan, StoreError> {
        let affected_set =
            SupportCompatibilityAffectedSet::from_compatibility_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        let path_plan = self.admit_support_program_path(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            payload_header_bytes,
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
        self.counters
            .record_support_compatibility_plan(plan.affected_set().affected_count());
        Ok(plan)
    }

    pub fn publish_support_compatibility_consequence(
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
        let completed = self
            .publish_support_consequence(raw_action.plan().verify().execute(), path_plan.budget())?
            .complete();
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
                self.counters.record_support_exact_compatible_migration();
            }
            SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility => {
                self.counters.record_support_degraded_compatibility();
            }
            SubscriptionSupportCompatibilityDecisionKind::OldReaderRejected
            | SubscriptionSupportCompatibilityDecisionKind::UnknownFamilyRejected
            | SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected => {
                self.counters.record_support_version_skew_rejection();
            }
        }
        Ok(report)
    }
}
