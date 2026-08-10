use crate::{
    failure::StoreError, support_maintenance_batch, RawSupportProgramAction,
    SubscriptionSupportMaintenanceBatchRequest, SubscriptionSupportMaintenanceDebtReport,
    SubscriptionSupportMaintenanceReport, SupportActionBreadthBudget, SupportAllocationScope,
    SupportMaintenanceAffectedSet, SupportMaintenanceBatchPlan, SupportMaintenanceDebtRecord,
    SupportPathClass, SupportProgramDensityClass, SupportProgramPathAdmissionRequest,
    SupportProgramPathPolicy,
};

use super::super::{StateBackedStoreBackend, StatePersistence};
use super::maintenance_counters::record_maintenance_decision_counters;
use super::maintenance_debt_publication::{
    install_subscription_support_maintenance_debt, persist_subscription_support_maintenance_debt,
    rollback_subscription_support_maintenance_debt, verify_subscription_support_maintenance_debt,
    MaintenanceDebtPublicationDecision,
};
use super::maintenance_descriptor_publication::{
    install_subscription_support_maintenance_descriptors,
    persist_subscription_support_maintenance_descriptors,
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn admit_subscription_support_maintenance_batch(
        &mut self,
        request: SubscriptionSupportMaintenanceBatchRequest,
    ) -> Result<SupportMaintenanceBatchPlan, StoreError> {
        let SubscriptionSupportMaintenanceBatchRequest {
            action_id,
            affected_bases,
            decision,
            path,
        } = request;
        let affected_set = SupportMaintenanceAffectedSet::from_maintenance_bases(affected_bases)?;
        let path_plan = self.admit_subscription_support_program_path(
            path.admission_request(affected_set.affected_count()),
        )?;
        let (descriptors, coalesced_duplicate_count) = affected_set.descriptors_for(&decision)?;
        let maintenance_batch = support_maintenance_batch(&action_id, &descriptors);
        let maintenance_receipt = self.admit_maintenance_batch(maintenance_batch)?;
        let plan = SupportMaintenanceBatchPlan::new(
            action_id,
            affected_set,
            path_plan,
            descriptors,
            maintenance_receipt,
            coalesced_duplicate_count,
            decision,
        )?;
        self.state
            .subscription_support_counter_snapshot
            .record_support_maintenance_plan(
                plan.descriptors().len() as u64,
                plan.coalesced_duplicate_count(),
            );
        Ok(plan)
    }

    pub fn publish_subscription_support_maintenance_consequence(
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
        let completed =
            self.publish_support_action_with_durable_recovery(raw_action, path_plan.budget())?;
        let report = SubscriptionSupportMaintenanceReport::new(
            completed,
            affected_set,
            descriptors,
            &maintenance_receipt,
            coalesced_duplicate_count,
            &decision,
            &path_plan,
        )?;
        let update = install_subscription_support_maintenance_descriptors(self, &report)?;
        record_maintenance_decision_counters(
            &mut self.state.subscription_support_counter_snapshot,
            decision.kind(),
        );
        persist_subscription_support_maintenance_descriptors(self, update)?;
        Ok(report)
    }

    pub fn report_delayed_subscription_support_maintenance(
        &mut self,
        plan: &SupportMaintenanceBatchPlan,
        delay_reason: impl Into<String>,
        budget: SupportActionBreadthBudget,
        payload_header_bytes: u64,
    ) -> Result<SubscriptionSupportMaintenanceDebtReport, StoreError> {
        let path_plan = self.admit_subscription_support_program_path(
            SupportProgramPathPolicy {
                path_class: SupportPathClass::OperatorReporting,
                density_class: SupportProgramDensityClass::MaintenanceKeyBatch,
                allocation_scope: SupportAllocationScope::OperatorReport,
                budget,
                payload_header_bytes,
            }
            .admission_request(plan.affected_set().affected_count()),
        )?;
        let report = SubscriptionSupportMaintenanceDebtReport::new(plan, delay_reason, &path_plan)?;
        let record = SupportMaintenanceDebtRecord::from_plan_and_report(plan, &report)?;
        match install_subscription_support_maintenance_debt(self, record)? {
            MaintenanceDebtPublicationDecision::AlreadyDurable => return Ok(report),
            MaintenanceDebtPublicationDecision::Install(update) => {
                if let Err(error) = verify_subscription_support_maintenance_debt(self) {
                    rollback_subscription_support_maintenance_debt(self, update);
                    return Err(error);
                }
                persist_subscription_support_maintenance_debt(self, update)?;
            }
        }
        Ok(report)
    }
}
