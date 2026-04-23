use forge_store::{
    CompletedSupportProgramAction, SubscriptionSupportOperationalVerdict,
    SupportAffectedSet, SupportRetentionSurvivalWitness,
};

fn attempt(completed: &CompletedSupportProgramAction, affected_set: &SupportAffectedSet) {
    let _ = SupportRetentionSurvivalWitness::new(
        completed,
        SubscriptionSupportOperationalVerdict::ExactResumePreserved,
        affected_set,
    );
}

fn main() {}
