use forge_store_layout_indexes::{lsm_replay_runtime, BaselineLsmReplayAdmission};
use forge_store_lsm_authority::AdmittedLsmReplaySource;

fn substitute(admission: BaselineLsmReplayAdmission, copied_source: &AdmittedLsmReplaySource) {
    let _ = lsm_replay_runtime().execute(admission, copied_source);
}

fn main() {}
