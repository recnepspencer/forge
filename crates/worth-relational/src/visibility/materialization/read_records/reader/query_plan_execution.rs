use super::query_execution::{
    execute_explicit_query_fragments_from_exact_basis, query_execution_outcome,
    query_execution_strategy, record_query_packet_metrics, PacketizedQueryMetrics,
};
use super::query_fragment_work::{
    execute_query_fragment, execute_traversal_query_fragment_from_state,
};
use super::query_packetization::{
    packetized_explicit_target_work, packetized_query_work, packetized_traversal_query_work,
};
use super::*;

impl<'runtime> VisibilityReadContext<'runtime> {
    pub fn execute_query_plan(
        &self,
        plan: SnapshotPinnedQueryPlan,
    ) -> Option<QueryExecutionOutcome> {
        if plan.packet.context_id != self.query_plan_context(&plan.snapshot)? {
            return None;
        }

        if matches!(plan.packet.scope, QueryScope::ExplicitTargets { .. }) {
            return self.execute_explicit_query_plan(plan);
        }
        if matches!(
            plan.packet.scope,
            QueryScope::OutgoingNeighborhood { .. }
                | QueryScope::IncomingNeighborhood { .. }
                | QueryScope::ConnectivityTraversal { .. }
        ) {
            return self.execute_traversal_query_plan(plan);
        }

        let read_view = self.read_snapshot(&plan.snapshot)?;
        let packets = packetized_query_work(&plan.packet, &read_view)?;
        let metrics = PacketizedQueryMetrics::from_packets(&packets);
        let strategy = query_execution_strategy(self, &plan, metrics.packet_count);
        record_query_packet_metrics(self.runtime, &plan, &metrics);

        let fragments = match strategy {
            PreparationStrategySelection::Serial => {
                self.runtime
                    .performance_access()
                    .count_query_serial_strategy();
                let mut scratch = QueryFragmentScratch::default();
                packets
                    .iter()
                    .enumerate()
                    .map(|(ordinal, packet)| {
                        execute_query_fragment(
                            &read_view,
                            &plan.packet,
                            packet,
                            ordinal as u64,
                            &mut scratch,
                        )
                    })
                    .collect::<Option<Vec<_>>>()?
            }
            PreparationStrategySelection::StagedParallel => {
                self.runtime
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
                        let mut scratch = QueryFragmentScratch::default();
                        bucket
                            .into_iter()
                            .map(|(ordinal, packet)| {
                                execute_query_fragment(
                                    &read_view,
                                    &plan.packet,
                                    packet,
                                    ordinal,
                                    &mut scratch,
                                )
                            })
                            .collect::<Option<Vec<_>>>()
                    })
                    .collect::<Option<Vec<Vec<_>>>>()?;
                bucketed_fragments.into_iter().flatten().collect()
            }
        };

        Some(query_execution_outcome(
            self.runtime,
            plan,
            metrics.packet_count,
            metrics.touched_partitions,
            metrics.target_count,
            fragments,
        ))
    }

    fn execute_explicit_query_plan(
        &self,
        plan: SnapshotPinnedQueryPlan,
    ) -> Option<QueryExecutionOutcome> {
        let packets = match &plan.packet.scope {
            QueryScope::ExplicitTargets { targets } => packetized_explicit_target_work(targets),
            _ => return None,
        };
        let metrics = PacketizedQueryMetrics::from_packets(&packets);
        let strategy = query_execution_strategy(self, &plan, metrics.packet_count);
        record_query_packet_metrics(self.runtime, &plan, &metrics);

        let basis = resolve_snapshot_basis(self.runtime, &plan.snapshot)?;
        let fragments = execute_explicit_query_fragments_from_exact_basis(
            self, &plan, &packets, &basis, strategy,
        )?;

        Some(query_execution_outcome(
            self.runtime,
            plan,
            metrics.packet_count,
            metrics.touched_partitions,
            metrics.target_count,
            fragments,
        ))
    }

    fn execute_traversal_query_plan(
        &self,
        plan: SnapshotPinnedQueryPlan,
    ) -> Option<QueryExecutionOutcome> {
        let packets = packetized_traversal_query_work(&plan.packet)?;
        let metrics = PacketizedQueryMetrics::from_packets(&packets);
        let strategy = query_execution_strategy(self, &plan, metrics.packet_count);
        record_query_packet_metrics(self.runtime, &plan, &metrics);

        let basis = resolve_snapshot_basis(self.runtime, &plan.snapshot)?;
        let state_access: &(dyn PartitionAccess + Sync) = basis.root().as_ref();
        let registry = basis.root().schema_authority().registry();
        let version_id = basis.version_id();
        let fragments = match strategy {
            PreparationStrategySelection::Serial => {
                self.runtime
                    .performance_access()
                    .count_query_serial_strategy();
                let mut scratch = QueryFragmentScratch::default();
                packets
                    .iter()
                    .enumerate()
                    .map(|(ordinal, packet)| {
                        execute_traversal_query_fragment_from_state(
                            self.runtime,
                            state_access,
                            registry,
                            version_id,
                            &plan.packet,
                            packet,
                            ordinal as u64,
                            &mut scratch,
                        )
                    })
                    .collect::<Option<Vec<_>>>()?
            }
            PreparationStrategySelection::StagedParallel => {
                self.runtime
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
                        let mut scratch = QueryFragmentScratch::default();
                        bucket
                            .into_iter()
                            .map(|(ordinal, packet)| {
                                execute_traversal_query_fragment_from_state(
                                    self.runtime,
                                    state_access,
                                    registry,
                                    version_id,
                                    &plan.packet,
                                    packet,
                                    ordinal,
                                    &mut scratch,
                                )
                            })
                            .collect::<Option<Vec<_>>>()
                    })
                    .collect::<Option<Vec<Vec<_>>>>()?;
                bucketed_fragments.into_iter().flatten().collect()
            }
        };

        Some(query_execution_outcome(
            self.runtime,
            plan,
            metrics.packet_count,
            metrics.touched_partitions,
            metrics.target_count,
            fragments,
        ))
    }
}
