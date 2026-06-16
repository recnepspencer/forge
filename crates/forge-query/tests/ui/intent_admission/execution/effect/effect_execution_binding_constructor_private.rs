use forge_query::facade::{
    ForgeQueryCommitIdentity,
    runtime::ForgeQueryEffectTriggeredIntentExecutionBinding,
};

fn main() {
    let _ = ForgeQueryEffectTriggeredIntentExecutionBinding {
        handoff: todo!(),
        effect_name: String::new(),
        trigger_commit_identity: ForgeQueryCommitIdentity::from_relational_commit_id(1),
        pending_delivery_digest: String::new(),
        binding_digest: String::new(),
    };
}
