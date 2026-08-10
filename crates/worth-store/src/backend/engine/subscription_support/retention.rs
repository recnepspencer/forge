use crate::{
    failure::{StoreError, StoreErrorKind},
    RawSupportProgramAction, SubscriptionSupportPostActionReport,
    SubscriptionSupportRetentionBatchRequest, SubscriptionSupportRetentionMaterialization,
    SupportAffectedSet, SupportReclaimConsequence, SupportRetentionBatchPlan,
    SupportRetentionSurvivalWitness,
};

use super::super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn admit_subscription_support_retention_batch(
        &mut self,
        request: SubscriptionSupportRetentionBatchRequest,
    ) -> Result<SupportRetentionBatchPlan, StoreError> {
        let SubscriptionSupportRetentionBatchRequest {
            action_id,
            affected_bases,
            decision,
            path,
        } = request;
        let affected_set = SupportAffectedSet::from_retention_bases(affected_bases)?;
        let path_plan = self.admit_subscription_support_program_path(
            path.admission_request(affected_set.affected_count()),
        )?;
        let plan = SupportRetentionBatchPlan::new(action_id, affected_set, path_plan, decision)?;
        self.state
            .subscription_support_counter_snapshot
            .record_support_retention_plan(plan.affected_set().affected_count());
        Ok(plan)
    }

    pub fn publish_subscription_support_retention_consequence(
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
        let completed =
            self.publish_support_action_with_durable_recovery(raw_action, path_plan.budget())?;
        let survival_witness =
            SupportRetentionSurvivalWitness::new(&completed, decision.verdict(), &affected_set)?;
        let translation_basis = affected_set.primary_basis().clone();
        let materialization =
            SubscriptionSupportRetentionMaterialization::from_decision(affected_set, &decision)?;
        let report = SubscriptionSupportPostActionReport::new(
            completed,
            translation_basis,
            survival_witness,
            materialization,
            decision_kind,
            &path_plan,
        )?;
        if decision.is_reclaim() {
            self.state
                .subscription_support_counter_snapshot
                .record_support_reclaim_consequence();
        }
        match decision_kind {
            crate::SubscriptionSupportRetentionDecisionKind::RetainExact
            | crate::SubscriptionSupportRetentionDecisionKind::RetainDegraded => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_retained_family();
            }
            crate::SubscriptionSupportRetentionDecisionKind::CompactExact => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_compacted_basis();
            }
            crate::SubscriptionSupportRetentionDecisionKind::ReclaimWithRebuild
            | crate::SubscriptionSupportRetentionDecisionKind::ReclaimWithoutRebuild => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_reclaimed_family();
            }
            crate::SubscriptionSupportRetentionDecisionKind::ExpireByPolicy => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_expired_family();
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_policy_expiration();
            }
        }
        Ok(report)
    }

    pub fn publish_subscription_support_reclaim_consequence(
        &mut self,
        plan: SupportRetentionBatchPlan,
    ) -> Result<SupportReclaimConsequence, StoreError> {
        if !plan.decision().is_reclaim() {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support reclaim consequences require a reclaim retention decision",
            ));
        }
        let report = self.publish_subscription_support_retention_consequence(plan)?;
        SupportReclaimConsequence::new(
            report.completed_action().clone(),
            report.survival_witness().clone(),
            report.retention_record().clone(),
            report.materialization().clone(),
        )
    }
}
