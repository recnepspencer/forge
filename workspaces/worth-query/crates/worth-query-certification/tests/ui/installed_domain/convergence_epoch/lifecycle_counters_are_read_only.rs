use std::borrow::BorrowMut;

use worth_query_host::facade::convergence_epoch::{
    WorthQueryAdmittedDirectConvergenceEpoch, WorthQueryConvergenceEpochCounters,
};

fn mutate_epoch(admitted: &mut WorthQueryAdmittedDirectConvergenceEpoch) {
    let _ = admitted.counters_mut();
}

fn construct_arbitrary_counters() {
    let _ = WorthQueryConvergenceEpochCounters::default();
}

fn invoke_private_event(counter: &mut WorthQueryConvergenceEpochCounters) {
    counter.attempted_cleanup();
}

fn extract_mutable_counters(admitted: &mut WorthQueryAdmittedDirectConvergenceEpoch) {
    fn require_mutable_counter_extraction<T: BorrowMut<WorthQueryConvergenceEpochCounters>>(
        _: &mut T,
    ) {
    }
    require_mutable_counter_extraction(admitted);
}

fn main() {}
