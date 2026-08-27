use std::collections::BTreeSet;

use crate::lineage::data::{HistoricalLineageResolution, HistoricalResolutionBoundednessBasis};

use super::resolution::{
    BranchScopedHistoricalResolutionRequest, HistoricalTraversalScope,
    SelectedRootReachableCommitScope,
};
use super::LineageAccess;

impl<'runtime> LineageAccess<'runtime> {
    pub(crate) fn resolve_record_history_for_observation(
        &self,
        entity_id: crate::identity::data::EntityId,
        observation: &crate::mvcc::RelationalBranchObservation,
    ) -> Option<HistoricalLineageResolution> {
        let root = observation.selected_root();
        let Some(selected_commit) = observation.commit_id() else {
            self.runtime
                .performance_access()
                .count_lineage_historical_resolution(0, 0, 0, 0, 0, 0);
            return None;
        };
        let exact_lineage = self
            .runtime
            .read_truth()
            .authoritative_entity_record_for_id_from_exact_state(
                root.as_ref(),
                root.schema_authority().registry(),
                entity_id,
            )
            .and_then(|record| record.lineage_id);
        let mut reachable_scope = SelectedRootReachableCommitScope {
            selected_commit_id: selected_commit,
            selected_branch_id: observation.identity().branch_id().clone(),
            ancestry: None,
        };
        let (lineage_id, lineage_seed_index_probe_count, lineage_seed_event_visit_count) =
            match exact_lineage {
                Some(lineage_id) => (lineage_id, 0, 0),
                None => {
                    let (lineage_id, entity_index_probe_count) =
                        self.runtime.lineage.indexed_lineage_for_entity(entity_id);
                    let Some(lineage_id) = lineage_id else {
                        self.runtime
                            .performance_access()
                            .count_lineage_historical_resolution(
                                entity_index_probe_count,
                                0,
                                0,
                                0,
                                0,
                                0,
                            );
                        return None;
                    };
                    let lineage_ids = BTreeSet::from([lineage_id]);
                    let (relevant_branches, branch_axis_probe_count) =
                        reachable_scope.indexed_branches_for_lineages(self, &lineage_ids, false);
                    let (event_positions, branch_lineage_probe_count) =
                        self.runtime.lineage.branch_event_positions_for_lineages(
                            &relevant_branches,
                            &lineage_ids,
                            false,
                        );
                    let mut participation_event_visits = 0;
                    let participates = event_positions.into_iter().any(|position| {
                        participation_event_visits += 1;
                        self.event_position_is_reachable(position, &mut reachable_scope)
                    });
                    if !participates {
                        let (
                            reachable_commit_node_visits,
                            reachable_commit_catalog_probes,
                            reachable_commit_parent_edge_visits,
                        ) = reachable_scope.ancestry_work();
                        self.runtime
                            .performance_access()
                            .count_lineage_historical_resolution(
                                entity_index_probe_count
                                    + branch_axis_probe_count
                                    + branch_lineage_probe_count,
                                participation_event_visits,
                                0,
                                reachable_commit_node_visits,
                                reachable_commit_parent_edge_visits,
                                reachable_commit_catalog_probes,
                            );
                        return None;
                    }
                    (
                        lineage_id,
                        entity_index_probe_count
                            + branch_axis_probe_count
                            + branch_lineage_probe_count,
                        participation_event_visits,
                    )
                }
            };
        Some(self.resolve_historical_lineage_in_scope(
            BranchScopedHistoricalResolutionRequest {
                branch_id: observation.identity().branch_id().clone(),
                lineage_id,
                boundedness_basis: HistoricalResolutionBoundednessBasis::BranchScopedLineageSeed,
                lineage_seed_index_probe_count,
                lineage_seed_event_visit_count,
            },
            HistoricalTraversalScope::SelectedRootReachable(reachable_scope),
        ))
    }
}
