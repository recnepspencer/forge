use crate::tests::harness::fixtures::stores::StoreLane;

use super::core::LaneResult;

pub fn run_store_lanes<T>(
    lanes: &[StoreLane],
    mut run: impl FnMut(StoreLane) -> T,
) -> Vec<LaneResult<T>> {
    lanes
        .iter()
        .copied()
        .map(|lane| LaneResult::new(lane.label(), run(lane)))
        .collect()
}
