use super::counters::PlanarBooleanSourceLoopSplitAttributionCounters;
use super::identity::{split_attribution_identity, split_attribution_row_identity};
use super::input::PlanarBooleanSourceLoopSplitAttributionInput;
use super::product::PlanarBooleanSourceLoopSplitAttribution;
use super::row::{
    PlanarBooleanSourceLoopSplitAttributionKind, PlanarBooleanSourceLoopSplitAttributionRow,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopIslandKind;
use std::collections::BTreeMap;

pub(crate) fn attribute_source_loop_splits(
    input: PlanarBooleanSourceLoopSplitAttributionInput<'_>,
) -> PlanarBooleanSourceLoopSplitAttribution {
    let request_identity = input.island_partition().request_identity().to_string();
    let mut counters = PlanarBooleanSourceLoopSplitAttributionCounters::default();
    let mut grouped = BTreeMap::<String, (Vec<String>, bool)>::new();
    for row in input.island_partition().rows() {
        counters.consumed_island_row();
        let entry = grouped
            .entry(row.source_loop_identity().to_string())
            .or_insert_with(|| (Vec::new(), false));
        entry.0.push(row.island_identity().to_string());
        if row.kind() == PlanarBooleanLoopIslandKind::BornFromOverlapNeighborhood {
            entry.1 = true;
        }
    }
    let mut rows = grouped
        .into_iter()
        .map(
            |(source_loop_identity, (mut island_identities, contributed_to_born_loop))| {
                island_identities.sort();
                island_identities.dedup();
                counters.emitted_attribution_row();
                let kind = if contributed_to_born_loop {
                    PlanarBooleanSourceLoopSplitAttributionKind::ContributedToBornLoop
                } else if island_identities.len() > 1 {
                    PlanarBooleanSourceLoopSplitAttributionKind::SplitIntoMultipleIslands
                } else {
                    PlanarBooleanSourceLoopSplitAttributionKind::Preserved
                };
                PlanarBooleanSourceLoopSplitAttributionRow::new(
                    split_attribution_row_identity(
                        &request_identity,
                        &source_loop_identity,
                        &island_identities,
                    ),
                    source_loop_identity,
                    island_identities,
                    kind,
                )
            },
        )
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.source_loop_identity()
            .cmp(right.source_loop_identity())
    });

    PlanarBooleanSourceLoopSplitAttribution::new(
        split_attribution_identity(&request_identity, &rows),
        request_identity,
        rows,
        counters,
    )
}
