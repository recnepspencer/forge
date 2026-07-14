use worth_store_layout_indexes::{BaselineBTreeExecutionWitness, BaselineBTreeReplayAdmission};

fn bypass(witness: &BaselineBTreeExecutionWitness, admission: &BaselineBTreeReplayAdmission) {
    let _ = witness.execute_replay_recovery(admission);
}

fn main() {}
