use std::time::{Duration, Instant};

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::BitWords;
use super::g27_same_field_lp_relaxation::stable_set_lp_guidance_values;
use super::g27_same_field_mwis_branch_certificate_preflight::{
    branch_vertex, dominant_and_exact_side_weight,
};
use super::g27_same_field_mwis_branch_prefix_replay::{
    replay_g27_same_field_mwis_branch_prefix_checked, G27MwisBranchPrefixReplayStatus,
};
use super::g27_same_field_mwis_lp_guided_branch_support::{
    initial_frontier, lp_guided_branch, lp_score, worst_child_upper, NodeState,
};
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_alignment_channel_instance_sets;

const G27_ANCHOR_INDEX: usize = 7;
const W_ANCHOR_INDEX: usize = 300;
const ATOM_LIMIT: usize = 5;
const ATOM_MASK: u32 = 101_719_589;
const TARGET_WEIGHT: i128 = 512_933;
const EXPECTED_EXACT_SIDE_WEIGHT: i128 = 61_655;
const EXPECTED_PREFIX_EXPANDED: usize = 29;
const EXPECTED_PREFIX_PRUNED: usize = 2;
const EXPECTED_PREFIX_OPEN: usize = 28;
const EXPECTED_BEST_OPEN_TOTAL: i128 = 518_612;
const TOP_K: usize = 3;
const SECONDS_CAP: u64 = 600;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisLpGuidedBranchStatus {
    LpGuidanceUseful,
    LpGuidanceNeutral,
    LpGuidanceWorse,
    RuntimeInconclusive,
    PrefixReplayMismatch,
    FrozenInstanceMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisLpGuidedBranchReport {
    checked_nodes: usize,
    useful_nodes: usize,
    worse_nodes: usize,
    top_relative_gain: i128,
    top_absolute_drop: i128,
    max_regression: i128,
    elapsed_millis: u128,
    rows: Vec<G27MwisLpGuidedBranchRow>,
    status: G27MwisLpGuidedBranchStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisLpGuidedBranchRow {
    parent_total: i128,
    baseline_branch: usize,
    lp_branch: usize,
    lp_branch_value_ppm: i128,
    lp_branch_score: i128,
    baseline_worst_child_total: i128,
    lp_worst_child_total: i128,
    relative_gain: i128,
    absolute_drop: i128,
}

impl G27MwisLpGuidedBranchReport {
    pub fn summary(&self) -> (usize, usize, usize, i128, i128, i128) {
        (
            self.checked_nodes,
            self.useful_nodes,
            self.worse_nodes,
            self.top_relative_gain,
            self.top_absolute_drop,
            self.max_regression,
        )
    }

    pub fn elapsed_millis(&self) -> u128 {
        self.elapsed_millis
    }

    pub fn rows(&self) -> &[G27MwisLpGuidedBranchRow] {
        &self.rows
    }

    pub fn status(&self) -> G27MwisLpGuidedBranchStatus {
        self.status
    }
}

impl G27MwisLpGuidedBranchRow {
    pub fn summary(&self) -> (i128, usize, usize, i128, i128, i128, i128, i128, i128) {
        (
            self.parent_total,
            self.baseline_branch,
            self.lp_branch,
            self.lp_branch_value_ppm,
            self.lp_branch_score,
            self.baseline_worst_child_total,
            self.lp_worst_child_total,
            self.relative_gain,
            self.absolute_drop,
        )
    }
}

pub fn diagnose_g27_same_field_mwis_lp_guided_branch_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisLpGuidedBranchReport, G27GeometricFractionalError> {
    let started = Instant::now();
    let prefix = replay_g27_same_field_mwis_branch_prefix_checked(handle)?;
    let (expanded, pruned, open, best_open_total, _, _) = prefix.summary();
    if prefix.status() != G27MwisBranchPrefixReplayStatus::BranchPrefixSemanticsPreflight
        || expanded != EXPECTED_PREFIX_EXPANDED
        || pruned != EXPECTED_PREFIX_PRUNED
        || open != EXPECTED_PREFIX_OPEN
        || best_open_total != EXPECTED_BEST_OPEN_TOTAL
    {
        return Ok(empty_report(
            G27MwisLpGuidedBranchStatus::PrefixReplayMismatch,
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
            source: "lp_guided_branch_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT {
        return Ok(empty_report(
            G27MwisLpGuidedBranchStatus::FrozenInstanceMismatch,
        ));
    }
    diagnose_lp_guidance(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        small_weight,
        started,
    )
}

fn diagnose_lp_guidance(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    exact_side_weight: i128,
    started: Instant,
) -> Result<G27MwisLpGuidedBranchReport, G27GeometricFractionalError> {
    let frontier = initial_frontier(
        adjacency,
        weights,
        candidates,
        TARGET_WEIGHT - EXPECTED_EXACT_SIDE_WEIGHT,
        EXPECTED_PREFIX_EXPANDED,
        EXPECTED_PREFIX_PRUNED,
        EXPECTED_PREFIX_OPEN,
    )?;
    let mut rows = Vec::new();
    for entry in frontier.into_iter().take(TOP_K) {
        if !rows.is_empty() && started.elapsed() >= Duration::from_secs(SECONDS_CAP) {
            break;
        }
        rows.push(compare_node(
            adjacency,
            weights,
            exact_side_weight,
            &entry.node,
            entry.upper_bound,
        )?);
    }
    Ok(report_from_rows(rows, started.elapsed().as_millis()))
}

fn compare_node(
    adjacency: &[BitWords],
    weights: &[i128],
    exact_side_weight: i128,
    node: &NodeState,
    parent_upper: i128,
) -> Result<G27MwisLpGuidedBranchRow, G27GeometricFractionalError> {
    let baseline_branch = branch_vertex(adjacency, weights, &node.candidates);
    let guidance = stable_set_lp_guidance_values(adjacency, weights, &node.candidates)?;
    let lp_branch = lp_guided_branch(adjacency, weights, &node.candidates, &guidance);
    let baseline_worst = worst_child_upper(adjacency, weights, node, baseline_branch)?;
    let lp_worst = if lp_branch == baseline_branch {
        baseline_worst
    } else {
        worst_child_upper(adjacency, weights, node, lp_branch)?
    };
    let local = node
        .candidates
        .iter()
        .position(|vertex| *vertex == lp_branch)
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "lp_guided_branch_vertex",
        })?;
    Ok(G27MwisLpGuidedBranchRow {
        parent_total: exact_side_weight + parent_upper,
        baseline_branch,
        lp_branch,
        lp_branch_value_ppm: (guidance[local] * 1_000_000.0).round() as i128,
        lp_branch_score: lp_score(weights[lp_branch], guidance[local]),
        baseline_worst_child_total: exact_side_weight + baseline_worst,
        lp_worst_child_total: exact_side_weight + lp_worst,
        relative_gain: baseline_worst - lp_worst,
        absolute_drop: parent_upper - lp_worst,
    })
}

fn report_from_rows(
    rows: Vec<G27MwisLpGuidedBranchRow>,
    elapsed_millis: u128,
) -> G27MwisLpGuidedBranchReport {
    let useful_nodes = rows.iter().filter(|row| row.relative_gain >= 500).count();
    let worse_nodes = rows.iter().filter(|row| row.relative_gain < -100).count();
    let top_relative_gain = rows.first().map(|row| row.relative_gain).unwrap_or(0);
    let top_absolute_drop = rows.first().map(|row| row.absolute_drop).unwrap_or(0);
    let max_regression = rows
        .iter()
        .map(|row| (-row.relative_gain).max(0))
        .max()
        .unwrap_or(0);
    let status = if top_relative_gain >= 750 && top_absolute_drop >= 1_000
        || (useful_nodes >= 2 && max_regression <= 100)
    {
        G27MwisLpGuidedBranchStatus::LpGuidanceUseful
    } else if rows.len() < TOP_K && elapsed_millis >= u128::from(SECONDS_CAP) * 1000 {
        G27MwisLpGuidedBranchStatus::RuntimeInconclusive
    } else if worse_nodes > 0 {
        G27MwisLpGuidedBranchStatus::LpGuidanceWorse
    } else {
        G27MwisLpGuidedBranchStatus::LpGuidanceNeutral
    };
    G27MwisLpGuidedBranchReport {
        checked_nodes: rows.len(),
        useful_nodes,
        worse_nodes,
        top_relative_gain,
        top_absolute_drop,
        max_regression,
        elapsed_millis,
        rows,
        status,
    }
}

fn empty_report(status: G27MwisLpGuidedBranchStatus) -> G27MwisLpGuidedBranchReport {
    G27MwisLpGuidedBranchReport {
        checked_nodes: 0,
        useful_nodes: 0,
        worse_nodes: 0,
        top_relative_gain: 0,
        top_absolute_drop: 0,
        max_regression: 0,
        elapsed_millis: 0,
        rows: Vec::new(),
        status,
    }
}
