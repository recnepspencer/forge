use worth_query::facade::{
    WorthQueryCommitIdentity,
    runtime::WorthQueryEffectTriggeredIntentExecutionBinding,
};

fn main() {
    let _ = WorthQueryEffectTriggeredIntentExecutionBinding {
        handoff: todo!(),
        effect_name: String::new(),
        trigger_commit_identity: WorthQueryCommitIdentity::from_relational_commit_id(1),
        pending_delivery_digest: String::new(),
        binding_digest: String::new(),
    };
}
