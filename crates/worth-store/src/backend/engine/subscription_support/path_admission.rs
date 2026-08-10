use crate::{
    failure::{StoreError, StoreErrorKind},
    SupportBatchAdmissionReceipt, SupportProgramPathAdmissionRequest, SupportProgramPathPlan,
};

use super::super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn admit_subscription_support_program_path(
        &mut self,
        request: SupportProgramPathAdmissionRequest,
    ) -> Result<SupportProgramPathPlan, StoreError> {
        if !request.policy.path_class.admits_operational_work() {
            if matches!(
                request.policy.path_class,
                crate::SupportPathClass::ForegroundResume | crate::SupportPathClass::ForegroundRead
            ) {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_hot_path_rejection();
            }
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "foreground subscription-support paths cannot admit operational work",
            ));
        }
        if request.policy.density_class == crate::SupportProgramDensityClass::StoreGlobalDebt {
            self.state
                .subscription_support_counter_snapshot
                .record_support_store_global_debt_rejection();
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "store-global subscription-support density is debt and cannot close Phase 1 admission",
            ));
        }
        if !request.policy.budget.admits(
            request.affected_entries,
            request.policy.payload_header_bytes,
        ) {
            self.state
                .subscription_support_counter_snapshot
                .record_budget_denial();
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support path plan exceeds its breadth budget before execution",
            ));
        }
        match SupportProgramPathPlan::new(
            request.policy.path_class,
            request.policy.density_class,
            request.policy.allocation_scope,
            request.policy.budget,
            request.affected_entries,
            request.policy.payload_header_bytes,
        ) {
            Ok(plan) => Ok(plan),
            Err(error) => Err(error),
        }
    }

    pub fn reuse_subscription_support_batch_receipt<'a>(
        &mut self,
        plan: &'a SupportProgramPathPlan,
    ) -> Result<&'a SupportBatchAdmissionReceipt, StoreError> {
        let receipt = plan.batch_receipt().ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support path plan has no reusable batch receipt",
            )
        })?;
        self.state
            .subscription_support_counter_snapshot
            .record_support_batch_receipt_reuse();
        Ok(receipt)
    }
}
