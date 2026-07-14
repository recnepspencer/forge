use super::g27_same_field_mwis_frontier_closure_campaign_support::{
    G27MwisFrontierCampaignRow, G27MwisFrontierCampaignRowClass, G27MwisFrontierCampaignStatus,
    G27MwisFrontierClosureCampaignScoutReport,
};
use super::g27_same_field_mwis_frontier_closure_exact_payload::ParentWork;
use super::g27_same_field_mwis_lp_guided_branch_support::{node_digest, QueueEntry};
use super::g27_same_field_mwis_lp_guided_frontier_profiles::{
    EXPECTED_FRONTIER_DEPTHS, EXPECTED_FRONTIER_TOTALS,
};

const CLOSED_PREFIX_COUNT: usize = 10;
const EXPECTED_PREFIX_OPEN: usize = 28;
const EXPECTED_SCOUT_ROWS: usize = 18;

pub(super) fn valid_scope(selected_start: usize, selected_end: usize) -> bool {
    selected_start >= CLOSED_PREFIX_COUNT
        && selected_start < selected_end
        && selected_end <= EXPECTED_PREFIX_OPEN
}

pub(super) fn scout_source_is_ready(scout: &G27MwisFrontierClosureCampaignScoutReport) -> bool {
    let (frontier_nodes, scout_rows, ready, failing, _, continuation_max) = scout.summary();
    scout.status() == G27MwisFrontierCampaignStatus::CampaignScoutReady
        && frontier_nodes == EXPECTED_PREFIX_OPEN
        && scout_rows == EXPECTED_SCOUT_ROWS
        && ready == EXPECTED_SCOUT_ROWS
        && failing == 0
        && continuation_max == 0
}

pub(super) fn frontier_matches_scout(
    frontier: &[QueueEntry],
    exact_side_weight: i128,
    scout: &G27MwisFrontierClosureCampaignScoutReport,
) -> bool {
    frontier.len() == EXPECTED_PREFIX_OPEN
        && scout.frontier_nodes().len() == EXPECTED_PREFIX_OPEN
        && scout.frontier_nodes().iter().all(|node| {
            let (index, total, depth, previously_closed) = node.summary();
            frontier.get(index).is_some_and(|entry| {
                total == exact_side_weight + entry.upper_bound
                    && depth == entry.node.depth
                    && node.digest() == node_digest(entry)
                    && previously_closed == (index < CLOSED_PREFIX_COUNT)
            })
        })
}

pub(super) fn parent_identity_matches(work: &ParentWork) -> bool {
    work.parent_total == EXPECTED_FRONTIER_TOTALS[work.index]
        && work.parent_depth == EXPECTED_FRONTIER_DEPTHS[work.index]
}

pub(super) fn scout_row_matches(rows: &[G27MwisFrontierCampaignRow], work: &ParentWork) -> bool {
    rows.iter()
        .find(|row| row.summary().0 == work.index)
        .is_some_and(|row| row_matches(row, work))
}

fn row_matches(row: &G27MwisFrontierCampaignRow, work: &ParentWork) -> bool {
    let (index, parent_total, parent_depth, first_branch, first_child_totals, _, second_branch) =
        row.summary();
    index == work.index
        && parent_total == work.parent_total
        && parent_depth == work.parent_depth
        && row.parent_digest() == work.parent_digest
        && first_branch == work.first_branch
        && first_child_totals == work.first_child_totals
        && second_branch == work.second_branch
        && row.terminal_totals() == work.terminal_totals.as_slice()
        && row.row_class() == G27MwisFrontierCampaignRowClass::ReadyForExactThreeLeafReplay
}
