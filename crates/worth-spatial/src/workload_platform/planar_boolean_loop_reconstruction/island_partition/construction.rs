use super::counters::PlanarBooleanLoopIslandPartitionCounters;
use super::identity::{island_partition_identity, island_partition_row_identity};
use super::input::PlanarBooleanLoopIslandPartitionInput;
use super::product::PlanarBooleanLoopIslandPartition;
use super::row::{PlanarBooleanLoopIslandKind, PlanarBooleanLoopIslandPartitionRow};

pub(crate) fn partition_loop_islands(
    input: PlanarBooleanLoopIslandPartitionInput<'_>,
) -> PlanarBooleanLoopIslandPartition {
    let request_identity = input.reconstructed_loops().request_identity().to_string();
    let mut counters = PlanarBooleanLoopIslandPartitionCounters::default();
    let mut rows = Vec::new();

    for row in input.reconstructed_loops().rows() {
        counters.consumed_reconstructed_loop();
        let member_loop_identities = vec![row.reconstructed_loop_identity().to_string()];
        counters.emitted_island_row();
        rows.push(PlanarBooleanLoopIslandPartitionRow::new(
            island_partition_row_identity(
                &request_identity,
                row.source_loop_identity(),
                &member_loop_identities,
            ),
            row.source_loop_identity().to_string(),
            member_loop_identities,
            PlanarBooleanLoopIslandKind::PreservedSourceLoop,
        ));
    }
    for row in input.born_loops().rows() {
        counters.consumed_born_loop();
        for source_loop_identity in row.source_loop_identities() {
            let member_loop_identities = vec![row.born_loop_identity().to_string()];
            counters.emitted_island_row();
            rows.push(PlanarBooleanLoopIslandPartitionRow::new(
                island_partition_row_identity(
                    &request_identity,
                    source_loop_identity,
                    &member_loop_identities,
                ),
                source_loop_identity.clone(),
                member_loop_identities,
                PlanarBooleanLoopIslandKind::BornFromOverlapNeighborhood,
            ));
        }
    }

    rows.sort_by(|left, right| {
        left.source_loop_identity()
            .cmp(right.source_loop_identity())
            .then_with(|| left.island_identity().cmp(right.island_identity()))
    });

    PlanarBooleanLoopIslandPartition::new(
        island_partition_identity(&request_identity, &rows),
        request_identity,
        rows,
        counters,
    )
}
