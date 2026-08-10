use super::super::{
    classification_error, SupportActionBreadthBudget, SupportAllocationScope,
    SupportBatchAdmissionReceipt, SupportBatchProofKind, SupportBatchReceiptReuseReport,
    SupportPathClass, SupportProgramDensityClass, SupportProgramPathPlan,
};
use super::SubscriptionSupportPublicationPipeline;
use crate::failure::StoreError;

impl SubscriptionSupportPublicationPipeline {
    pub fn admit_support_program_path(
        &mut self,
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        affected_entries: u64,
        payload_header_bytes: u64,
    ) -> Result<SupportProgramPathPlan, StoreError> {
        if !path_class.admits_operational_work() {
            if matches!(
                path_class,
                SupportPathClass::ForegroundResume | SupportPathClass::ForegroundRead
            ) {
                self.counters.record_support_hot_path_rejection();
            }
            return Err(classification_error(
                "foreground subscription-support paths cannot admit operational work",
            ));
        }
        if density_class == SupportProgramDensityClass::StoreGlobalDebt {
            self.counters.record_support_store_global_debt_rejection();
            return Err(classification_error(
                "store-global subscription-support density is debt and cannot close Phase 1 admission",
            ));
        }
        if !budget.admits(affected_entries, payload_header_bytes) {
            self.counters.record_budget_denial();
            return Err(classification_error(
                "subscription-support path plan exceeds its breadth budget before execution",
            ));
        }
        match SupportProgramPathPlan::new(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            payload_header_bytes,
        ) {
            Ok(plan) => Ok(plan),
            Err(err) => Err(err),
        }
    }

    pub fn reuse_support_batch_receipt<'a>(
        &mut self,
        plan: &'a SupportProgramPathPlan,
    ) -> Result<&'a SupportBatchAdmissionReceipt, StoreError> {
        let receipt = plan.batch_receipt().ok_or_else(|| {
            classification_error("subscription-support path plan has no reusable batch receipt")
        })?;
        self.counters.record_support_batch_receipt_reuse();
        Ok(receipt)
    }

    pub fn verify_support_batch_receipt_reuse(
        &mut self,
        plan: &SupportProgramPathPlan,
        reused_proofs: Vec<SupportBatchProofKind>,
    ) -> Result<SupportBatchReceiptReuseReport, StoreError> {
        let receipt = self.reuse_support_batch_receipt(plan)?;
        for _ in 1..reused_proofs.len() {
            self.reuse_support_batch_receipt(plan)?;
        }
        SupportBatchReceiptReuseReport::new(receipt, reused_proofs)
    }
}
