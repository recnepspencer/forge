use std::time::{Duration, Instant};

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::BitWords;
use super::g27_same_field_lp_relaxation::stable_set_lp_guidance_values;
use super::g27_same_field_mwis_branch_certificate_preflight::dominant_and_exact_side_weight;
use super::g27_same_field_mwis_branch_prefix_replay::{
    replay_g27_same_field_mwis_branch_prefix_checked, G27MwisBranchPrefixReplayStatus,
};
use super::g27_same_field_mwis_lp_guided_branch_support::{
    branch_children, initial_frontier, lp_guided_branch, node_upper, QueueEntry,
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
const EXPECTED_FIRST_BRANCH: usize = 383;
const EXPECTED_H35_WORST_TOTAL: i128 = 513_179;
const SECONDS_CAP: u64 = 600;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisLpGuidedMicroStatus {
    LocalTargetCrossed,
    SolverPruneCandidateNeedsExactReplay,
    SmallContinuationRetired,
    RuntimeInconclusive,
    PrefixReplayMismatch,
    FrozenInstanceMismatch,
    H35BranchMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisLpGuidedMicroReport {
    parent_total: i128,
    first_branch: usize,
    first_worst_total: i128,
    second_branch: usize,
    final_worst_total: i128,
    additional_drop: i128,
    solver_prune_candidates: usize,
    elapsed_millis: u128,
    status: G27MwisLpGuidedMicroStatus,
}

impl G27MwisLpGuidedMicroReport {
    pub fn summary(&self) -> (i128, i128, usize, i128, i128, i128, usize) {
        (
            self.parent_total,
            self.first_worst_total,
            self.second_branch,
            self.final_worst_total,
            self.additional_drop,
            self.elapsed_millis as i128,
            self.solver_prune_candidates,
        )
    }

    pub fn status(&self) -> G27MwisLpGuidedMicroStatus {
        self.status
    }
}

pub fn preflight_g27_same_field_mwis_lp_guided_micro_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisLpGuidedMicroReport, G27GeometricFractionalError> {
    let prefix = replay_g27_same_field_mwis_branch_prefix_checked(handle)?;
    let (expanded, pruned, open, best_open_total, _, _) = prefix.summary();
    if prefix.status() != G27MwisBranchPrefixReplayStatus::BranchPrefixSemanticsPreflight
        || expanded != EXPECTED_PREFIX_EXPANDED
        || pruned != EXPECTED_PREFIX_PRUNED
        || open != EXPECTED_PREFIX_OPEN
        || best_open_total != EXPECTED_BEST_OPEN_TOTAL
    {
        return Ok(empty_report(
            G27MwisLpGuidedMicroStatus::PrefixReplayMismatch,
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
            source: "lp_guided_micro_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT {
        return Ok(empty_report(
            G27MwisLpGuidedMicroStatus::FrozenInstanceMismatch,
        ));
    }
    micro_branch(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        small_weight,
        Instant::now(),
    )
}

fn micro_branch(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    exact_side_weight: i128,
    started: Instant,
) -> Result<G27MwisLpGuidedMicroReport, G27GeometricFractionalError> {
    let frontier = initial_frontier(
        adjacency,
        weights,
        candidates,
        EXPECTED_DOMINANT_THRESHOLD,
        EXPECTED_PREFIX_EXPANDED,
        EXPECTED_PREFIX_PRUNED,
        EXPECTED_PREFIX_OPEN,
    )?;
    let top = frontier
        .first()
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "lp_guided_micro_frontier",
        })?;
    let guidance = stable_set_lp_guidance_values(adjacency, weights, &top.node.candidates)?;
    let first_branch = lp_guided_branch(adjacency, weights, &top.node.candidates, &guidance);
    if first_branch != EXPECTED_FIRST_BRANCH {
        return Ok(empty_report(G27MwisLpGuidedMicroStatus::H35BranchMismatch));
    }
    let first_children = child_entries(adjacency, weights, top, first_branch)?;
    let worse_child = first_children
        .iter()
        .max_by_key(|entry| entry.upper_bound)
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "lp_guided_micro_first_child",
        })?;
    let first_worst_total = exact_side_weight + worse_child.upper_bound;
    if first_worst_total != EXPECTED_H35_WORST_TOTAL {
        return Ok(empty_report(G27MwisLpGuidedMicroStatus::H35BranchMismatch));
    }
    if started.elapsed() >= Duration::from_secs(SECONDS_CAP) {
        return Ok(runtime_report(
            top,
            first_branch,
            first_worst_total,
            started,
        ));
    }
    let second_guidance =
        stable_set_lp_guidance_values(adjacency, weights, &worse_child.node.candidates)?;
    let second_branch = lp_guided_branch(
        adjacency,
        weights,
        &worse_child.node.candidates,
        &second_guidance,
    );
    let second_children = child_entries(adjacency, weights, worse_child, second_branch)?;
    let solver_prunes = second_children
        .iter()
        .filter(|entry| entry.upper_bound <= EXPECTED_DOMINANT_THRESHOLD)
        .count();
    let final_worst = second_children
        .iter()
        .map(|entry| entry.upper_bound)
        .max()
        .unwrap_or(0);
    let final_worst_total = exact_side_weight + final_worst;
    let additional_drop = first_worst_total - final_worst_total;
    let status = if solver_prunes > 0 {
        G27MwisLpGuidedMicroStatus::SolverPruneCandidateNeedsExactReplay
    } else if final_worst_total <= TARGET_WEIGHT {
        G27MwisLpGuidedMicroStatus::LocalTargetCrossed
    } else if additional_drop < 500 {
        G27MwisLpGuidedMicroStatus::SmallContinuationRetired
    } else {
        G27MwisLpGuidedMicroStatus::SmallContinuationRetired
    };
    Ok(G27MwisLpGuidedMicroReport {
        parent_total: exact_side_weight + top.upper_bound,
        first_branch,
        first_worst_total,
        second_branch,
        final_worst_total,
        additional_drop,
        solver_prune_candidates: solver_prunes,
        elapsed_millis: started.elapsed().as_millis(),
        status,
    })
}

fn child_entries(
    adjacency: &[BitWords],
    weights: &[i128],
    parent: &QueueEntry,
    branch: usize,
) -> Result<[QueueEntry; 2], G27GeometricFractionalError> {
    let children = branch_children(adjacency, weights, &parent.node, branch);
    Ok([
        QueueEntry {
            upper_bound: node_upper(adjacency, weights, &children[0])?,
            sequence: 0,
            node: children[0].clone(),
        },
        QueueEntry {
            upper_bound: node_upper(adjacency, weights, &children[1])?,
            sequence: 1,
            node: children[1].clone(),
        },
    ])
}

fn runtime_report(
    top: &QueueEntry,
    first_branch: usize,
    first_worst_total: i128,
    started: Instant,
) -> G27MwisLpGuidedMicroReport {
    G27MwisLpGuidedMicroReport {
        parent_total: EXPECTED_EXACT_SIDE_WEIGHT + top.upper_bound,
        first_branch,
        first_worst_total,
        second_branch: 0,
        final_worst_total: first_worst_total,
        additional_drop: 0,
        solver_prune_candidates: 0,
        elapsed_millis: started.elapsed().as_millis(),
        status: G27MwisLpGuidedMicroStatus::RuntimeInconclusive,
    }
}

fn empty_report(status: G27MwisLpGuidedMicroStatus) -> G27MwisLpGuidedMicroReport {
    G27MwisLpGuidedMicroReport {
        parent_total: 0,
        first_branch: 0,
        first_worst_total: 0,
        second_branch: 0,
        final_worst_total: 0,
        additional_drop: 0,
        solver_prune_candidates: 0,
        elapsed_millis: 0,
        status,
    }
}
