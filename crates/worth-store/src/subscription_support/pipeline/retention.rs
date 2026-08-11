use super::super::{
    classification_error, RawSupportProgramAction, SubscriptionSupportOperationalBasis,
    SubscriptionSupportPostActionReport, SubscriptionSupportRetentionDecision,
    SubscriptionSupportRetentionDecisionKind, SubscriptionSupportRetentionMaterialization,
    SupportActionBreadthBudget, SupportActionId, SupportAffectedSet, SupportAllocationScope,
    SupportPathClass, SupportProgramDensityClass, SupportReclaimConsequence,
    SupportRetentionBatchPlan, SupportRetentionSurvivalWitness,
};
use super::SubscriptionSupportPublicationPipeline;
use crate::failure::StoreError;

impl SubscriptionSupportPublicationPipeline {
    pub fn admit_support_retention_batch(
        &mut self,
        action_id: SupportActionId,
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
        decision: SubscriptionSupportRetentionDecision,
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        payload_header_bytes: u64,
    ) -> Result<SupportRetentionBatchPlan, StoreError> {
        let affected_set = SupportAffectedSet::from_retention_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        let path_plan = self.admit_support_program_path(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            payload_header_bytes,
        )?;
        let plan = SupportRetentionBatchPlan::new(action_id, affected_set, path_plan, decision)?;
        self.counters
            .record_support_retention_plan(plan.affected_set().affected_count());
        Ok(plan)
    }

    pub fn publish_support_retention_consequence(
        &mut self,
        plan: SupportRetentionBatchPlan,
    ) -> Result<SubscriptionSupportPostActionReport, StoreError> {
        let (action_id, affected_set, path_plan, decision) = plan.into_parts();
        let decision_kind = decision.kind();
        let raw_action = RawSupportProgramAction::new(
            action_id,
            affected_set.primary_basis().clone(),
            decision.verdict(),
        )?;
        let completed = self
            .publish_support_consequence(raw_action.plan().verify().execute(), path_plan.budget())?
            .complete();
        let survival_witness =
            SupportRetentionSurvivalWitness::new(&completed, decision.verdict(), &affected_set)?;
        let translation_basis = affected_set.primary_basis().clone();
        let materialization =
            SubscriptionSupportRetentionMaterialization::from_decision(affected_set, &decision)?;
        if decision.is_reclaim() {
            self.counters.record_support_reclaim_consequence();
        }
        let report = SubscriptionSupportPostActionReport::new(
            completed,
            translation_basis,
            survival_witness,
            materialization,
            decision_kind,
            &path_plan,
        )?;
        match decision_kind {
            SubscriptionSupportRetentionDecisionKind::RetainExact
            | SubscriptionSupportRetentionDecisionKind::RetainDegraded => {
                self.counters.record_support_retained_family();
            }
            SubscriptionSupportRetentionDecisionKind::CompactExact => {
                self.counters.record_support_compacted_basis();
            }
            SubscriptionSupportRetentionDecisionKind::ReclaimWithRebuild
            | SubscriptionSupportRetentionDecisionKind::ReclaimWithoutRebuild => {
                self.counters.record_support_reclaimed_family();
            }
            SubscriptionSupportRetentionDecisionKind::ExpireByPolicy => {
                self.counters.record_support_expired_family();
                self.counters.record_support_policy_expiration();
            }
        }
        Ok(report)
    }

    pub fn publish_support_reclaim_consequence(
        &mut self,
        plan: SupportRetentionBatchPlan,
    ) -> Result<SupportReclaimConsequence, StoreError> {
        if !plan.decision().is_reclaim() {
            return Err(classification_error(
                "subscription-support reclaim consequences require a reclaim retention decision",
            ));
        }
        let report = self.publish_support_retention_consequence(plan)?;
        SupportReclaimConsequence::new(
            report.completed_action().clone(),
            report.survival_witness().clone(),
            report.retention_record().clone(),
            report.materialization().clone(),
        )
    }
}
