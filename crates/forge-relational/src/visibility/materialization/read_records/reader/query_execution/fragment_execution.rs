use crate::authority::commit::preparation::planning::strategy::PreparationStrategySelection;
use rayon::prelude::*;

use super::super::query_fragment_work::execute_explicit_query_fragment_from_state;
use super::super::query_packetization::PacketizedQueryWork;
use super::super::{SnapshotPinnedQueryPlan, VisibilityReadContext};

pub(in crate::visibility::materialization::read_records::reader) fn execute_explicit_query_fragments_from_snapshot_state(
    reader: &VisibilityReadContext<'_>,
    plan: &SnapshotPinnedQueryPlan,
    packets: &[PacketizedQueryWork],
    snapshot_state: &crate::storage::logic::state::SnapshotState,
    strategy: PreparationStrategySelection,
) -> Option<Vec<crate::query::data::QueryWorkerFragment>> {
    let current_state = reader.runtime().storage_access().current_state();
    let version_id = snapshot_state.handle.version_id;
    match strategy {
        PreparationStrategySelection::Serial => {
            reader
                .runtime()
                .performance_access()
                .count_query_serial_strategy();
            packets
                .iter()
                .enumerate()
                .map(|(ordinal, packet)| {
                    execute_explicit_query_fragment_from_state(
                        reader,
                        snapshot_state,
                        &current_state,
                        version_id,
                        &plan.packet,
                        packet,
                        ordinal as u64,
                    )
                })
                .collect()
        }
        PreparationStrategySelection::StagedParallel => {
            reader
                .runtime()
                .performance_access()
                .count_query_staged_parallel_strategy();
            let bucket_count = packets.len().min(rayon::current_num_threads()).max(1);
            let mut buckets = vec![Vec::new(); bucket_count];
            for (ordinal, packet) in packets.iter().enumerate() {
                buckets[ordinal % bucket_count].push((ordinal as u64, packet));
            }
            let bucketed_fragments = buckets
                .into_par_iter()
                .map(|bucket| {
                    bucket
                        .into_iter()
                        .map(|(ordinal, packet)| {
                            execute_explicit_query_fragment_from_state(
                                reader,
                                snapshot_state,
                                &current_state,
                                version_id,
                                &plan.packet,
                                packet,
                                ordinal,
                            )
                        })
                        .collect::<Option<Vec<_>>>()
                })
                .collect::<Option<Vec<Vec<_>>>>()?;
            Some(bucketed_fragments.into_iter().flatten().collect())
        }
    }
}
