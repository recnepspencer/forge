use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::BitWords;
use super::g27_same_field_lp_relaxation::stable_set_lp_guidance_values;
use super::g27_same_field_mwis_branch_certificate_preflight::dominant_and_exact_side_weight;
use super::g27_same_field_mwis_branch_prefix_replay::{
    replay_g27_same_field_mwis_branch_prefix_checked, G27MwisBranchPrefixReplayStatus,
};
use super::g27_same_field_mwis_lp_guided_branch_support::{
    child_entries, initial_frontier, lp_guided_branch, node_digest, QueueEntry,
};
use super::g27_same_field_mwis_lp_guided_frontier_profiles::{
    G27MwisLpGuidedTopPrefixNode, G27MwisLpGuidedTopPrefixReport, G27MwisLpGuidedTopPrefixStatus,
    EXPECTED_FRONTIER_DEPTHS, EXPECTED_FRONTIER_TOTALS, H39_INDICES, H39_REMAINING_BEST_TOTAL,
    H40_INDICES, H40_REMAINING_BEST_TOTAL, H41_INDICES, H41_REMAINING_BEST_TOTAL, H42_INDICES,
    H43_INDICES, H43_REMAINING_BEST_TOTAL,
};
use super::g27_same_field_mwis_lp_guided_micro_dual::{
    certify_children, G27MwisLpGuidedMicroDualStatus,
};
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_alignment_channel_instance_sets;

const TARGET_WEIGHT: i128 = 512_933;
const G27_ANCHOR_INDEX: usize = 7;
const W_ANCHOR_INDEX: usize = 300;
const ATOM_LIMIT: usize = 5;
const ATOM_MASK: u32 = 101_719_589;
const EXPECTED_EXACT_SIDE_WEIGHT: i128 = 61_655;
const EXPECTED_DOMINANT_THRESHOLD: i128 = 451_278;
const EXPECTED_PREFIX_EXPANDED: usize = 29;
const EXPECTED_PREFIX_PRUNED: usize = 2;
const EXPECTED_PREFIX_OPEN: usize = 28;
const EXPECTED_BEST_OPEN_TOTAL: i128 = 518_612;
pub fn preflight_g27_same_field_mwis_lp_guided_top_prefix_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisLpGuidedTopPrefixReport, G27GeometricFractionalError> {
    preflight_top_prefix_profile(
        handle,
        &H39_INDICES,
        Some(H39_REMAINING_BEST_TOTAL),
        G27MwisLpGuidedTopPrefixStatus::TopBandPrefixExactProgress,
    )
}

pub fn preflight_g27_same_field_mwis_lp_guided_next_prefix_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisLpGuidedTopPrefixReport, G27GeometricFractionalError> {
    preflight_top_prefix_profile(
        handle,
        &H40_INDICES,
        Some(H40_REMAINING_BEST_TOTAL),
        G27MwisLpGuidedTopPrefixStatus::TopBandPrefixExactProgress,
    )
}

pub fn preflight_g27_same_field_mwis_lp_guided_third_prefix_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisLpGuidedTopPrefixReport, G27GeometricFractionalError> {
    preflight_top_prefix_profile(
        handle,
        &H41_INDICES,
        Some(H41_REMAINING_BEST_TOTAL),
        G27MwisLpGuidedTopPrefixStatus::TopBandPrefixExactProgress,
    )
}

pub fn preflight_g27_same_field_mwis_lp_guided_final_top_pair_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisLpGuidedTopPrefixReport, G27GeometricFractionalError> {
    preflight_top_prefix_profile(
        handle,
        &H42_INDICES,
        None,
        G27MwisLpGuidedTopPrefixStatus::TopBandPrefixExactProgress,
    )
}

pub fn preflight_g27_same_field_mwis_lp_guided_remaining_pair_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisLpGuidedTopPrefixReport, G27GeometricFractionalError> {
    preflight_top_prefix_profile(
        handle,
        &H43_INDICES,
        Some(H43_REMAINING_BEST_TOTAL),
        G27MwisLpGuidedTopPrefixStatus::RemainingFrontierPairExactProgress,
    )
}

fn preflight_top_prefix_profile(
    handle: &HadwigerResearchHandle,
    indices: &[usize],
    expected_remaining_best_total: Option<i128>,
    success_status: G27MwisLpGuidedTopPrefixStatus,
) -> Result<G27MwisLpGuidedTopPrefixReport, G27GeometricFractionalError> {
    let prefix = replay_g27_same_field_mwis_branch_prefix_checked(handle)?;
    let (expanded, pruned, open, best_open_total, _, _) = prefix.summary();
    if prefix.status() != G27MwisBranchPrefixReplayStatus::BranchPrefixSemanticsPreflight
        || expanded != EXPECTED_PREFIX_EXPANDED
        || pruned != EXPECTED_PREFIX_PRUNED
        || open != EXPECTED_PREFIX_OPEN
        || best_open_total != EXPECTED_BEST_OPEN_TOTAL
    {
        return Ok(empty_report(
            G27MwisLpGuidedTopPrefixStatus::PrefixReplayMismatch,
        ));
    }
    let mut channel_sets = threshold_mwis_alignment_channel_instance_sets(
        handle,
        &[(G27_ANCHOR_INDEX, W_ANCHOR_INDEX)],
        ATOM_LIMIT,
    )?;
    let channel = channel_sets
        .pop()
        .and_then(|channels| {
            channels
                .into_iter()
                .find(|channel| channel.atom_mask == ATOM_MASK)
        })
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "lp_guided_top_prefix_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT {
        return Ok(empty_report(
            G27MwisLpGuidedTopPrefixStatus::FrozenInstanceMismatch,
        ));
    }
    run_top_prefix(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        small_weight,
        indices,
        expected_remaining_best_total,
        success_status,
    )
}

fn run_top_prefix(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    exact_side_weight: i128,
    indices: &[usize],
    expected_remaining_best_total: Option<i128>,
    success_status: G27MwisLpGuidedTopPrefixStatus,
) -> Result<G27MwisLpGuidedTopPrefixReport, G27GeometricFractionalError> {
    let frontier = initial_frontier(
        adjacency,
        weights,
        candidates,
        EXPECTED_DOMINANT_THRESHOLD,
        EXPECTED_PREFIX_EXPANDED,
        EXPECTED_PREFIX_PRUNED,
        EXPECTED_PREFIX_OPEN,
    )?;
    if !frontier_shape_matches(&frontier, exact_side_weight) {
        return Ok(empty_report(
            G27MwisLpGuidedTopPrefixStatus::FrontierShapeMismatch,
        ));
    }
    if !indices_are_contiguous(indices) {
        return Ok(empty_report(
            G27MwisLpGuidedTopPrefixStatus::NonContiguousPrefix,
        ));
    }
    let remaining_best_open_total = remaining_best_total(&frontier, exact_side_weight, indices)?;
    if expected_remaining_best_total.is_some_and(|expected| remaining_best_open_total != expected) {
        return Ok(empty_report(
            G27MwisLpGuidedTopPrefixStatus::RemainingBestMismatch,
        ));
    }
    let mut report = G27MwisLpGuidedTopPrefixReport {
        checked_nodes: 0,
        certified_nodes: 0,
        certified_leaves: 0,
        remaining_best_open_total,
        nodes: Vec::new(),
        status: success_status,
    };
    for index in indices.iter().copied() {
        let node = certify_parent(
            adjacency,
            weights,
            exact_side_weight,
            &frontier,
            index,
            success_status,
        )?;
        report.checked_nodes += 1;
        if node.status != success_status {
            report.status = node.status;
            report.nodes.push(node);
            return Ok(report);
        }
        report.certified_nodes += 1;
        report.certified_leaves += node.certified_leaves;
        report.nodes.push(node);
    }
    Ok(report)
}

fn certify_parent(
    adjacency: &[BitWords],
    weights: &[i128],
    exact_side_weight: i128,
    frontier: &[QueueEntry],
    index: usize,
    success_status: G27MwisLpGuidedTopPrefixStatus,
) -> Result<G27MwisLpGuidedTopPrefixNode, G27GeometricFractionalError> {
    let parent = frontier
        .get(index)
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "lp_guided_top_prefix_frontier",
        })?;
    let parent_total = exact_side_weight + parent.upper_bound;
    let parent_depth = parent.node.depth;
    if parent_total != EXPECTED_FRONTIER_TOTALS[index]
        || parent_depth != EXPECTED_FRONTIER_DEPTHS[index]
    {
        return Ok(empty_node(
            index,
            parent,
            exact_side_weight,
            G27MwisLpGuidedTopPrefixStatus::ParentIdentityMismatch,
        ));
    }
    let first_guidance =
        stable_set_lp_guidance_values(adjacency, weights, &parent.node.candidates)?;
    let first_branch =
        lp_guided_branch(adjacency, weights, &parent.node.candidates, &first_guidance);
    let first_children = child_entries(adjacency, weights, parent, first_branch)?;
    let first_child_totals = [
        exact_side_weight + first_children[0].upper_bound,
        exact_side_weight + first_children[1].upper_bound,
    ];
    let worse_index = usize::from(first_children[1].upper_bound > first_children[0].upper_bound);
    let better_index = 1 - worse_index;
    let worse_child = &first_children[worse_index];
    let second_guidance =
        stable_set_lp_guidance_values(adjacency, weights, &worse_child.node.candidates)?;
    let second_branch = lp_guided_branch(
        adjacency,
        weights,
        &worse_child.node.candidates,
        &second_guidance,
    );
    let second_children = child_entries(adjacency, weights, worse_child, second_branch)?;
    let leaves = vec![
        first_children[better_index].clone(),
        second_children[0].clone(),
        second_children[1].clone(),
    ];
    let terminal_totals = leaves
        .iter()
        .map(|entry| exact_side_weight + entry.upper_bound)
        .collect::<Vec<_>>();
    let mut node = G27MwisLpGuidedTopPrefixNode {
        index,
        parent_total,
        parent_depth,
        first_branch,
        first_child_totals,
        second_branch,
        terminal_totals,
        parent_digest: node_digest(parent),
        status: G27MwisLpGuidedTopPrefixStatus::FloatingBoundAboveTarget,
        ..empty_node(
            index,
            parent,
            exact_side_weight,
            G27MwisLpGuidedTopPrefixStatus::FloatingBoundAboveTarget,
        )
    };
    if node.terminal_totals.len() != 3 {
        node.status = G27MwisLpGuidedTopPrefixStatus::IncompleteLeafPartition;
        return Ok(node);
    }
    if node
        .terminal_totals
        .iter()
        .any(|total| *total > TARGET_WEIGHT)
    {
        return Ok(node);
    }
    let dual = certify_children(adjacency, weights, exact_side_weight, &leaves)?;
    let (checked, certified, explicit_rows, positive_dual_rows, _) = dual.summary();
    let (max_denominator, min_slack_floor, max_objective_excess) = dual.exact_summary();
    node.certified_leaves = certified;
    node.explicit_rows = explicit_rows;
    node.positive_dual_rows = positive_dual_rows;
    node.max_denominator = max_denominator;
    node.min_slack_floor = min_slack_floor;
    node.max_objective_excess = max_objective_excess;
    node.row_digest = dual.row_digest().to_string();
    node.status = if checked == 3
        && certified == 3
        && dual.status() == G27MwisLpGuidedMicroDualStatus::ExactMicroPrunesCertified
    {
        success_status
    } else {
        G27MwisLpGuidedTopPrefixStatus::ExactReplayFailed
    };
    Ok(node)
}

fn frontier_shape_matches(frontier: &[QueueEntry], exact_side_weight: i128) -> bool {
    frontier.len() == EXPECTED_PREFIX_OPEN
        && EXPECTED_FRONTIER_TOTALS
            .iter()
            .zip(EXPECTED_FRONTIER_DEPTHS.iter())
            .enumerate()
            .all(|(index, (total, depth))| {
                frontier.get(index).is_some_and(|entry| {
                    exact_side_weight + entry.upper_bound == *total && entry.node.depth == *depth
                })
            })
}

fn remaining_best_total(
    frontier: &[QueueEntry],
    exact_side_weight: i128,
    indices: &[usize],
) -> Result<i128, G27GeometricFractionalError> {
    let first_unresolved = indices.iter().copied().max().map(|index| index + 1).ok_or(
        G27GeometricFractionalError::MalformedData {
            source: "lp_guided_top_prefix_indices",
        },
    )?;
    frontier
        .get(first_unresolved)
        .map(|entry| exact_side_weight + entry.upper_bound)
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "lp_guided_top_prefix_remaining",
        })
}

fn indices_are_contiguous(indices: &[usize]) -> bool {
    indices.windows(2).all(|window| window[1] == window[0] + 1)
}

fn empty_report(status: G27MwisLpGuidedTopPrefixStatus) -> G27MwisLpGuidedTopPrefixReport {
    G27MwisLpGuidedTopPrefixReport {
        checked_nodes: 0,
        certified_nodes: 0,
        certified_leaves: 0,
        remaining_best_open_total: 0,
        nodes: Vec::new(),
        status,
    }
}

fn empty_node(
    index: usize,
    parent: &QueueEntry,
    exact_side_weight: i128,
    status: G27MwisLpGuidedTopPrefixStatus,
) -> G27MwisLpGuidedTopPrefixNode {
    G27MwisLpGuidedTopPrefixNode {
        index,
        parent_total: exact_side_weight + parent.upper_bound,
        parent_depth: parent.node.depth,
        first_branch: 0,
        first_child_totals: [0, 0],
        second_branch: 0,
        terminal_totals: Vec::new(),
        certified_leaves: 0,
        explicit_rows: 0,
        positive_dual_rows: 0,
        max_denominator: 1,
        min_slack_floor: i128::MAX,
        max_objective_excess: 0,
        parent_digest: node_digest(parent),
        row_digest: String::new(),
        status,
    }
}
