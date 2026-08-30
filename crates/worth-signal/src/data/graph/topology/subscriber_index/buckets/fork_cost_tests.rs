use std::env;
use std::hint::black_box;
use std::process::Command;

use stats_alloc::{Region, INSTRUMENTED_SYSTEM};

use super::{IndexedSubscriptionMembership, ReverseSubscriptionIndex};
use crate::data::aspect::Aspect;
use crate::data::handle::NodeId;
use crate::data::output::{
    DetailTokenId, InternedPartitionSubscription, PartitionMatchMode, PartitionTokenId,
};

const TEST_NAME: &str = "data::graph::topology::subscriber_index::buckets::fork_cost_tests::single_membership_first_write_is_bounded_by_nested_persistent_granule";

#[test]
fn single_membership_first_write_is_bounded_by_nested_persistent_granule() {
    const CHILD_PROCESS: &str = "WORTH_SIGNAL_SUBSCRIBER_FORK_COST_CHILD";
    if env::var_os(CHILD_PROCESS).is_none() {
        let status = Command::new(env::current_exe().expect("test executable resolves"))
            .arg("--exact")
            .arg(TEST_NAME)
            .arg("--nocapture")
            .arg("--test-threads=1")
            .env(CHILD_PROCESS, "1")
            .status()
            .expect("isolated subscriber allocation probe starts");
        assert!(
            status.success(),
            "isolated subscriber allocation probe failed"
        );
        return;
    }

    let producer = NodeId::new(0, 0);
    let aspect = Aspect::new(1);
    let mut samples = Vec::new();
    let mut eager_copy_bytes = None;

    for consumer_count in [64_u32, 4_096, 65_536] {
        let mut source = ReverseSubscriptionIndex::default();
        for index in 0..consumer_count {
            let scope = if index % 2 == 0 {
                InternedPartitionSubscription {
                    partition: PartitionTokenId(index),
                    detail: None,
                    match_mode: PartitionMatchMode::WholePartition,
                }
            } else {
                InternedPartitionSubscription {
                    partition: PartitionTokenId(index),
                    detail: Some(DetailTokenId(index)),
                    match_mode: PartitionMatchMode::PartitionAndDetail,
                }
            };
            let membership =
                IndexedSubscriptionMembership::from_edge(producer, aspect, Some(scope))
                    .expect("interned scoped membership is indexable");
            source.replace_consumer(NodeId::new(index + 1, 0), vec![membership]);
        }
        let removed = NodeId::new(consumer_count, 0);
        let mut fork = source.fork_persistent();

        if consumer_count == 65_536 {
            let eager_region = Region::new(&INSTRUMENTED_SYSTEM);
            black_box(source.operational_clone());
            eager_copy_bytes = Some(eager_region.change().bytes_allocated);
        }

        let region = Region::new(&INSTRUMENTED_SYSTEM);
        fork.remove_consumer(removed);
        let allocation = region.change();
        samples.push((
            consumer_count,
            allocation.allocations,
            allocation.bytes_allocated,
        ));

        assert_eq!(
            source.query_whole_aspect(producer, aspect).candidates.len(),
            consumer_count as usize,
            "source membership must remain independent"
        );
        assert_eq!(
            fork.query_whole_aspect(producer, aspect).candidates.len(),
            consumer_count as usize - 1,
            "fork must observe only its changed membership"
        );
    }

    let minimum_calls = samples.iter().map(|(_, calls, _)| *calls).min().unwrap();
    let minimum_bytes = samples.iter().map(|(_, _, bytes)| *bytes).min().unwrap();
    for (consumer_count, calls, bytes) in &samples {
        assert!(
            *calls <= minimum_calls + 64,
            "nested first-write allocation calls slope with {consumer_count} consumers: {calls} vs {minimum_calls}"
        );
        assert!(
            *bytes <= minimum_bytes + 64 * 1_024,
            "nested first-write bytes slope with {consumer_count} consumers: {bytes} vs {minimum_bytes}"
        );
    }
    let maximum_first_write_bytes = samples.iter().map(|(_, _, bytes)| *bytes).max().unwrap();
    assert!(
        eager_copy_bytes.expect("largest sensitivity sample exists")
            > maximum_first_write_bytes.saturating_mul(8),
        "probe must distinguish a whole-index copy from one nested persistent change"
    );
}
