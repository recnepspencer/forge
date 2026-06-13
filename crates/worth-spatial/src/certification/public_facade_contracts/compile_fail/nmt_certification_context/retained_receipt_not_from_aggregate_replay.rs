use worth_spatial::facade::nmt_certification_context::NmtScopeRetainedReplayReceipt;

fn main() {
    let _replay = NmtScopeRetainedReplayReceipt {
        parent_replay_identity: "aggregate-replay".to_string(),
        scope_identity: "scope".to_string(),
        scope_projection_identity: "aggregate-projection".to_string(),
        scope_replay_identity: "aggregate-replay-plus-label".to_string(),
        checkpoint_identity: "checkpoint".to_string(),
        counters: todo!(),
    };
}
