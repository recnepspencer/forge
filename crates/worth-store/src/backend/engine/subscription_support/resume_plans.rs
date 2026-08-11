use crate::{
    failure::StoreError, SubscriptionSupportAllocationScope, SubscriptionSupportClassificationPlan,
    SubscriptionSupportDensityClass, SubscriptionSupportPayloadBudget,
    SubscriptionSupportPlanFamily, SubscriptionSupportRole, SubscriptionSupportStoredRecordSet,
};

pub(super) fn handoff_plan_for_record(
    record_set: &SubscriptionSupportStoredRecordSet,
) -> Result<SubscriptionSupportClassificationPlan, StoreError> {
    match record_set.role() {
        SubscriptionSupportRole::ExactContinuation => {
            SubscriptionSupportClassificationPlan::exact_sparse_identity()
        }
        SubscriptionSupportRole::DegradedContinuation => {
            SubscriptionSupportClassificationPlan::new(
                SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan,
                SubscriptionSupportPayloadBudget::new(16 * 1024, 64)?,
                SubscriptionSupportAllocationScope::RestartShardBatch,
                SubscriptionSupportDensityClass::RestartShardBatchClassification,
                Some(format!(
                    "subscription-support-handoff:{}",
                    record_set.key().family_id()
                )),
            )
        }
        SubscriptionSupportRole::NarrowingMaterialization => {
            SubscriptionSupportClassificationPlan::new(
                SubscriptionSupportPlanFamily::DeniedResumeClassificationPlan,
                SubscriptionSupportPayloadBudget::new(16 * 1024, 64)?,
                SubscriptionSupportAllocationScope::FamilyLocalScratch,
                SubscriptionSupportDensityClass::FamilyBatchClassificationDebt,
                None,
            )
        }
    }
}

pub(super) fn restart_plan_for_record(
    record_set: &SubscriptionSupportStoredRecordSet,
    restart_shard: String,
) -> Result<SubscriptionSupportClassificationPlan, StoreError> {
    match record_set.role() {
        SubscriptionSupportRole::ExactContinuation => SubscriptionSupportClassificationPlan::new(
            SubscriptionSupportPlanFamily::ExactResumeClassificationPlan,
            SubscriptionSupportPayloadBudget::new(16 * 1024, 64)?,
            SubscriptionSupportAllocationScope::RestartShardBatch,
            SubscriptionSupportDensityClass::SparseIdentityClassification,
            Some(restart_shard),
        ),
        SubscriptionSupportRole::DegradedContinuation => {
            SubscriptionSupportClassificationPlan::new(
                SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan,
                SubscriptionSupportPayloadBudget::new(16 * 1024, 64)?,
                SubscriptionSupportAllocationScope::RestartShardBatch,
                SubscriptionSupportDensityClass::RestartShardBatchClassification,
                Some(restart_shard),
            )
        }
        SubscriptionSupportRole::NarrowingMaterialization => {
            SubscriptionSupportClassificationPlan::new(
                SubscriptionSupportPlanFamily::DeniedResumeClassificationPlan,
                SubscriptionSupportPayloadBudget::new(16 * 1024, 64)?,
                SubscriptionSupportAllocationScope::RestartShardBatch,
                SubscriptionSupportDensityClass::FamilyBatchClassificationDebt,
                Some(restart_shard),
            )
        }
    }
}
