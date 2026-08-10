use super::super::{
    PostActionResumeClassificationInput, ResumeClassificationTranslationPlan,
    SubscriptionSupportOperationalBasis, SubscriptionSupportOperationalVerdict,
};
use super::SubscriptionSupportPublicationPipeline;
use crate::failure::StoreError;

impl SubscriptionSupportPublicationPipeline {
    pub fn translate_operational_verdict(
        &mut self,
        verdict: SubscriptionSupportOperationalVerdict,
        basis: SubscriptionSupportOperationalBasis,
        maintenance_admission_key: Option<String>,
        policy_reason: Option<String>,
    ) -> Result<PostActionResumeClassificationInput, StoreError> {
        match ResumeClassificationTranslationPlan::from_operational_verdict(
            verdict,
            basis,
            maintenance_admission_key,
            policy_reason,
        ) {
            Ok(plan) => {
                self.counters.record_operational_verdict_translation();
                Ok(plan.lower())
            }
            Err(err) => {
                self.counters
                    .record_operational_verdict_translation_rejection();
                Err(err)
            }
        }
    }
}
