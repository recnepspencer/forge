use std::env;
use std::hint::black_box;
use std::process::Command;

use stats_alloc::{Region, INSTRUMENTED_SYSTEM};

use super::TemporalRuntimeState;
use crate::data::temporal::{ClockTick, TemporalCondition, TemporalWakeOwner};

const TEST_NAME: &str = "logic::transaction::runtime::state::temporal::fork_cost_tests::same_tick_same_owner_first_write_is_bounded_by_inner_frontier_roots";

#[test]
fn same_tick_same_owner_first_write_is_bounded_by_inner_frontier_roots() {
    const CHILD_PROCESS: &str = "WORTH_SIGNAL_TEMPORAL_FORK_COST_CHILD";
    if env::var_os(CHILD_PROCESS).is_none() {
        let status = Command::new(env::current_exe().expect("test executable resolves"))
            .arg("--exact")
            .arg(TEST_NAME)
            .arg("--nocapture")
            .arg("--test-threads=1")
            .env(CHILD_PROCESS, "1")
            .status()
            .expect("isolated temporal allocation probe starts");
        assert!(
            status.success(),
            "isolated temporal allocation probe failed"
        );
        return;
    }

    let due_tick = ClockTick::new(7);
    let condition = TemporalCondition::after(7).expect("positive temporal delay");
    let mut samples = Vec::new();
    let mut eager_copy_bytes = None;

    for wake_count in [64_usize, 4_096, 65_536] {
        let mut source = TemporalRuntimeState::default();
        for _ in 0..wake_count {
            source
                .schedule_wake(condition.clone(), due_tick, None)
                .expect("same-tick wake schedules");
        }
        let mut fork = source.fork_persistent();

        if wake_count == 65_536 {
            let eager_region = Region::new(&INSTRUMENTED_SYSTEM);
            black_box(source.clone());
            eager_copy_bytes = Some(eager_region.change().bytes_allocated);
        }

        let region = Region::new(&INSTRUMENTED_SYSTEM);
        fork.schedule_wake(condition.clone(), due_tick, None)
            .expect("fork same-tick wake schedules");
        let allocation = region.change();
        samples.push((
            wake_count,
            allocation.allocations,
            allocation.bytes_allocated,
        ));

        assert_eq!(source.scheduled_wakes.len(), wake_count);
        assert_eq!(fork.scheduled_wakes.len(), wake_count + 1);
        assert_eq!(
            source.scheduled_frontier.get(&due_tick).unwrap().len(),
            wake_count
        );
        assert_eq!(
            fork.scheduled_frontier.get(&due_tick).unwrap().len(),
            wake_count + 1
        );
        assert_eq!(
            source
                .owner_frontier
                .get(&TemporalWakeOwner::Manual)
                .unwrap()
                .len(),
            wake_count
        );
        assert_eq!(
            fork.owner_frontier
                .get(&TemporalWakeOwner::Manual)
                .unwrap()
                .len(),
            wake_count + 1
        );
    }

    let minimum_calls = samples.iter().map(|(_, calls, _)| *calls).min().unwrap();
    let minimum_bytes = samples.iter().map(|(_, _, bytes)| *bytes).min().unwrap();
    for (wake_count, calls, bytes) in &samples {
        assert!(
            *calls <= minimum_calls + 96,
            "temporal first-write calls slope with {wake_count} same-frontier wakes: {calls} vs {minimum_calls}"
        );
        assert!(
            *bytes <= minimum_bytes + 96 * 1_024,
            "temporal first-write bytes slope with {wake_count} same-frontier wakes: {bytes} vs {minimum_bytes}"
        );
    }
    let maximum_first_write_bytes = samples.iter().map(|(_, _, bytes)| *bytes).max().unwrap();
    assert!(
        eager_copy_bytes.expect("largest sensitivity sample exists")
            > maximum_first_write_bytes.saturating_mul(8),
        "probe must distinguish a whole temporal-state copy from bounded frontier mutation"
    );
}
