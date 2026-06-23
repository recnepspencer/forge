use super::super::selection_substrate::{
    QueryObligationSelectionAuthorityKind, QuerySelectedGraphObligationCloseout,
    QuerySelectorPrecisionPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneFiveQueryObligationSelectionCloseoutReport {
    open_finding_count: usize,
    topology_lane_count: usize,
    spatial_lane_count: usize,
    capped_broad_selector_residue_count: usize,
    uncapped_broad_selector_residue_count: usize,
    owned_query_gap_count: usize,
    incomplete_query_gap_count: usize,
    graph_read_access_planning_claimed_count: usize,
}

impl MilestoneFiveQueryObligationSelectionCloseoutReport {
    pub(crate) fn from_closeout_rows(rows: &[QuerySelectedGraphObligationCloseout]) -> Self {
        let topology_lane_count = authority_count(
            rows,
            QueryObligationSelectionAuthorityKind::TopologyTouchedBasis,
        );
        let spatial_lane_count = authority_count(
            rows,
            QueryObligationSelectionAuthorityKind::SpatialQueryDescriptor,
        );
        let capped_broad_selector_residue_count = rows
            .iter()
            .flat_map(|row| row.broad_selector_residue_rows().rows())
            .filter(|row| row.current_count() <= row.must_not_exceed_count())
            .count();
        let uncapped_broad_selector_residue_count = rows
            .iter()
            .flat_map(|row| row.broad_selector_residue_rows().rows())
            .filter(|row| row.current_count() > row.must_not_exceed_count())
            .count();
        let owned_query_gap_count = rows
            .iter()
            .flat_map(|row| row.query_selector_gap_rows().rows())
            .filter(|row| {
                !row.owner().is_empty()
                    && !row.blocker().is_empty()
                    && !row.follow_on_milestone().is_empty()
            })
            .count();
        let incomplete_query_gap_count = rows
            .iter()
            .flat_map(|row| row.query_selector_gap_rows().rows())
            .filter(|row| {
                row.owner().is_empty()
                    || row.blocker().is_empty()
                    || row.follow_on_milestone().is_empty()
            })
            .count();
        let graph_read_access_planning_claimed_count = rows
            .iter()
            .filter(|row| row.graph_read_access_planning_claimed())
            .count();
        let open_finding_count = closeout_open_finding_count(
            rows,
            topology_lane_count,
            spatial_lane_count,
            uncapped_broad_selector_residue_count,
            incomplete_query_gap_count,
            graph_read_access_planning_claimed_count,
        );

        Self {
            open_finding_count,
            topology_lane_count,
            spatial_lane_count,
            capped_broad_selector_residue_count,
            uncapped_broad_selector_residue_count,
            owned_query_gap_count,
            incomplete_query_gap_count,
            graph_read_access_planning_claimed_count,
        }
    }

    pub const fn open_finding_count(&self) -> usize {
        self.open_finding_count
    }

    pub const fn topology_lane_count(&self) -> usize {
        self.topology_lane_count
    }

    pub const fn spatial_lane_count(&self) -> usize {
        self.spatial_lane_count
    }

    pub const fn capped_broad_selector_residue_count(&self) -> usize {
        self.capped_broad_selector_residue_count
    }

    pub const fn uncapped_broad_selector_residue_count(&self) -> usize {
        self.uncapped_broad_selector_residue_count
    }

    pub const fn owned_query_gap_count(&self) -> usize {
        self.owned_query_gap_count
    }

    pub const fn incomplete_query_gap_count(&self) -> usize {
        self.incomplete_query_gap_count
    }

    pub const fn graph_read_access_planning_claimed_count(&self) -> usize {
        self.graph_read_access_planning_claimed_count
    }
}

fn closeout_open_finding_count(
    rows: &[QuerySelectedGraphObligationCloseout],
    topology_lane_count: usize,
    spatial_lane_count: usize,
    uncapped_broad_selector_residue_count: usize,
    incomplete_query_gap_count: usize,
    graph_read_access_planning_claimed_count: usize,
) -> usize {
    rows.iter()
        .filter(|row| {
            row.selector_precision_report().posture()
                == QuerySelectorPrecisionPosture::CounterEvidenceUnbounded
        })
        .count()
        + rows
            .iter()
            .filter(|row| !row.local_ceremony_closeout().is_clean())
            .count()
        + usize::from(topology_lane_count == 0)
        + usize::from(spatial_lane_count == 0)
        + uncapped_broad_selector_residue_count
        + incomplete_query_gap_count
        + graph_read_access_planning_claimed_count
}

fn authority_count(
    rows: &[QuerySelectedGraphObligationCloseout],
    authority: QueryObligationSelectionAuthorityKind,
) -> usize {
    rows.iter()
        .filter(|row| row.authority_kind() == authority)
        .count()
}
