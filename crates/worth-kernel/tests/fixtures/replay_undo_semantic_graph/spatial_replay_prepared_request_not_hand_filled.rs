use worth_spatial::facade::replay_undo_semantic_graph::SpatialReplaySemanticGraphPreparedRequest;

fn main() {
    let _ = SpatialReplaySemanticGraphPreparedRequest {
        family_identity: fake(),
        spatial_touch_authority: fake(),
        prior_proof_identity: fake(),
        stage_index_identity: fake(),
        lookup_consumed_workload_handoff: fake(),
        retained_replay_receipt: fake(),
    };
}

fn fake<T>() -> T {
    unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
}
