use worth_spatial::facade::retained_cancellation_chain::{
    RetainedCancellationChainCounters, RetainedCancellationChainReceipt,
};

fn main() {
    let _ = RetainedCancellationChainReceipt {
        chain_digest: String::new(),
        workload_identity: String::new(),
        retained_basis_identity: String::new(),
        projection_consumed_identity: String::new(),
        checkpoints: Vec::new(),
        counters: RetainedCancellationChainCounters {
            checkpoint_count: 0,
            transform_step_count: 0,
            replayed_checkpoint_count: 0,
            trigger_local_replay_count: 0,
            retained_artifact_count: 0,
            projection_consumed_fact_count: 0,
            diagnostic_trigger_count: 0,
            user_outcome_count: 0,
        },
    };
}
