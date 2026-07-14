use worth_store_layout_indexes::btree_replay_runtime;
use worth_store_recovery_physics::AdmittedBTreeReplaySource;

fn bypass(source: AdmittedBTreeReplaySource<()>) {
    let _ = btree_replay_runtime().execute(source);
}

fn main() {}
