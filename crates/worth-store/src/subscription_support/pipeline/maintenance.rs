use super::super::{
    classification_error, support_maintenance_batch, synthetic_support_maintenance_receipt,
    RawSupportProgramAction, SubscriptionSupportMaintenanceDebtReport,
    SubscriptionSupportMaintenanceDecision, SubscriptionSupportMaintenanceDecisionKind,
    SubscriptionSupportMaintenanceReport, SubscriptionSupportOperationalBasis,
    SupportActionBreadthBudget, SupportActionId, SupportAllocationScope,
    SupportMaintenanceAffectedSet, SupportMaintenanceBatchPlan, SupportPathClass,
    SupportProgramDensityClass,
};
use super::SubscriptionSupportPublicationPipeline;
use crate::failure::StoreError;

impl SubscriptionSupportPublicationPipeline {
    pub fn admit_support_maintenance_batch(
        &mut self,
        action_id: SupportActionId,
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
        decision: SubscriptionSupportMaintenanceDecision,
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        payload_header_bytes: u64,
    ) -> Result<SupportMaintenanceBatchPlan, StoreError> {
        let affected_set = SupportMaintenanceAffectedSet::from_maintenance_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        let path_plan = self.admit_support_program_path(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            payload_header_bytes,
        )?;
        let (descriptors, coalesced_duplicate_count) = affected_set.descriptors_for(&decision)?;
        let maintenance_batch = support_maintenance_batch(&action_id, &descriptors);
        let maintenance_receipt =
            synthetic_support_maintenance_receipt(&maintenance_batch, &descriptors);
        let plan = SupportMaintenanceBatchPlan::new(
            action_id,
            affected_set,
            path_plan,
            descriptors,
            maintenance_receipt,
            coalesced_duplicate_count,
            decision,
        )?;
        self.counters.record_support_maintenance_plan(
            plan.descriptors().len() as u64,
            plan.coalesced_duplicate_count(),
        );
        Ok(plan)
    }

    pub fn publish_support_maintenance_consequence(
        &mut self,
        plan: SupportMaintenanceBatchPlan,
    ) -> Result<SubscriptionSupportMaintenanceReport, StoreError> {
        let (
            action_id,
            affected_set,
            path_plan,
            descriptors,
            maintenance_receipt,
            coalesced_duplicate_count,
            decision,
        ) = plan.into_parts();
        let raw_action = RawSupportProgramAction::new(
            action_id,
            affected_set.primary_basis().clone(),
            decision.verdict(),
        )?;
        let completed = self
            .publish_support_consequence(raw_action.plan().verify().execute(), path_plan.budget())?
            .complete();
        let report = SupportMaintenanceReport::new(
            completed,
            affected_set,
            descriptors,
            &maintenance_receipt,
            coalesced_duplicate_count,
            &decision,
            &path_plan,
        )?;
        match decision.kind() {
            SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted => {
                self.counters.record_support_maintenance_rebuild_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::RefreshDescriptorAdmitted => {
                self.counters.record_support_maintenance_refresh_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::CompatibilityMigrationDescriptorAdmitted => {
                self.counters
                    .record_support_maintenance_compatibility_migration_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::DegradationRecoveryDescriptorAdmitted => {
                self.counters
                    .record_support_maintenance_degradation_recovery_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered => {
                self.counters
                    .record_support_maintenance_interrupted_restart_recovery();
            }
        }
        Ok(report)
    }

    pub fn report_delayed_support_maintenance(
        &mut self,
        plan: &SupportMaintenanceBatchPlan,
        delay_reason: impl Into<String>,
        budget: SupportActionBreadthBudget,
        payload_header_bytes: u64,
    ) -> Result<SubscriptionSupportMaintenanceDebtReport, StoreError> {
        let path_plan = self.admit_support_program_path(
            SupportPathClass::OperatorReporting,
            SupportProgramDensityClass::MaintenanceKeyBatch,
            SupportAllocationScope::OperatorReport,
            budget,
            plan.affected_set().affected_count(),
            payload_header_bytes,
        )?;
        let report = SubscriptionSupportMaintenanceDebtReport::new(plan, delay_reason, &path_plan)?;
        self.counters.record_support_maintenance_delay_report();
        Ok(report)
    }
}
