use forge_store::{
    CompletedSupportProgramAction, SubscriptionSupportRetentionDecisionKind,
    SubscriptionSupportRetentionMaterialization, SupportRetentionParticipationRecord,
    SupportRetentionSurvivalWitness,
};

fn attempt(
    completed: &CompletedSupportProgramAction,
    witness: &SupportRetentionSurvivalWitness,
    materialization: &SubscriptionSupportRetentionMaterialization,
) {
    let _record = SupportRetentionParticipationRecord::new(
        completed,
        witness,
        materialization,
        SubscriptionSupportRetentionDecisionKind::RetainExact,
    )
    .unwrap();
}

fn main() {}
