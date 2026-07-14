use sha2::{Digest, Sha256};

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
const FRONTIER_INDEX: usize = 1;
const EXPECTED_PARENT_TOTAL: i128 = 518_543;
const EXPECTED_PARENT_DEPTH: usize = 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisLpGuidedSecondStatus {
    FundTopKFramework,
    PromisingButNeedsReplay,
    RetireSecondNodePattern,
    PrefixReplayMismatch,
    FrozenInstanceMismatch,
    ParentIdentityMismatch,
    ExactReplayFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisLpGuidedSecondReport {
    parent_total: i128,
    first_branch: usize,
    first_worst_total: i128,
    second_branch: usize,
    final_worst_total: i128,
    solver_prune_candidates: usize,
    certified_prunes: usize,
    explicit_rows: usize,
    positive_dual_rows: usize,
    max_denominator: i128,
    min_slack_floor: i128,
    max_objective_excess: i128,
    parent_digest: String,
    row_digest: String,
    status: G27MwisLpGuidedSecondStatus,
}

impl G27MwisLpGuidedSecondReport {
    pub fn summary(&self) -> (i128, usize, i128, usize, i128, usize, usize) {
        (
            self.parent_total,
            self.first_branch,
            self.first_worst_total,
            self.second_branch,
            self.final_worst_total,
            self.solver_prune_candidates,
            self.certified_prunes,
        )
    }

    pub fn exact_summary(&self) -> (usize, usize, i128, i128, i128) {
        (
            self.explicit_rows,
            self.positive_dual_rows,
            self.max_denominator,
            self.min_slack_floor,
            self.max_objective_excess,
        )
    }

    pub fn parent_digest(&self) -> &str {
        &self.parent_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub fn status(&self) -> G27MwisLpGuidedSecondStatus {
        self.status
    }
}

pub fn preflight_g27_same_field_mwis_lp_guided_second_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisLpGuidedSecondReport, G27GeometricFractionalError> {
    let prefix = replay_g27_same_field_mwis_branch_prefix_checked(handle)?;
    let (expanded, pruned, open, best_open_total, _, _) = prefix.summary();
    if prefix.status() != G27MwisBranchPrefixReplayStatus::BranchPrefixSemanticsPreflight
        || expanded != EXPECTED_PREFIX_EXPANDED
        || pruned != EXPECTED_PREFIX_PRUNED
        || open != EXPECTED_PREFIX_OPEN
        || best_open_total != EXPECTED_BEST_OPEN_TOTAL
    {
        return Ok(empty_report(
            G27MwisLpGuidedSecondStatus::PrefixReplayMismatch,
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
            source: "lp_guided_second_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT {
        return Ok(empty_report(
            G27MwisLpGuidedSecondStatus::FrozenInstanceMismatch,
        ));
    }
    run_second_node(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        small_weight,
    )
}

fn run_second_node(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    exact_side_weight: i128,
) -> Result<G27MwisLpGuidedSecondReport, G27GeometricFractionalError> {
    let frontier = initial_frontier(
        adjacency,
        weights,
        candidates,
        EXPECTED_DOMINANT_THRESHOLD,
        EXPECTED_PREFIX_EXPANDED,
        EXPECTED_PREFIX_PRUNED,
        EXPECTED_PREFIX_OPEN,
    )?;
    let parent =
        frontier
            .get(FRONTIER_INDEX)
            .ok_or(G27GeometricFractionalError::MalformedData {
                source: "lp_guided_second_frontier",
            })?;
    let parent_total = exact_side_weight + parent.upper_bound;
    let parent_digest = node_digest(parent);
    if parent_total != EXPECTED_PARENT_TOTAL || parent.node.depth != EXPECTED_PARENT_DEPTH {
        return Ok(empty_report(
            G27MwisLpGuidedSecondStatus::ParentIdentityMismatch,
        ));
    }
    let first_guidance =
        stable_set_lp_guidance_values(adjacency, weights, &parent.node.candidates)?;
    let first_branch =
        lp_guided_branch(adjacency, weights, &parent.node.candidates, &first_guidance);
    let first_children = child_entries(adjacency, weights, parent, first_branch)?;
    let worse_child = first_children
        .iter()
        .max_by_key(|entry| entry.upper_bound)
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "lp_guided_second_first_child",
        })?;
    let first_worst_total = exact_side_weight + worse_child.upper_bound;
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
    let final_worst_total = exact_side_weight
        + second_children
            .iter()
            .map(|entry| entry.upper_bound)
            .max()
            .unwrap_or(0);
    let mut report = G27MwisLpGuidedSecondReport {
        parent_total,
        first_branch,
        first_worst_total,
        second_branch,
        final_worst_total,
        solver_prune_candidates: solver_prunes,
        parent_digest,
        status: status_without_replay(final_worst_total, solver_prunes),
        ..empty_report(G27MwisLpGuidedSecondStatus::RetireSecondNodePattern)
    };
    if final_worst_total <= TARGET_WEIGHT && solver_prunes == 2 {
        let dual = certify_children(adjacency, weights, exact_side_weight, &second_children)?;
        let (checked, certified, explicit_rows, positive_dual_rows, _) = dual.summary();
        let (max_denominator, min_slack_floor, max_objective_excess) = dual.exact_summary();
        report.certified_prunes = certified;
        report.explicit_rows = explicit_rows;
        report.positive_dual_rows = positive_dual_rows;
        report.max_denominator = max_denominator;
        report.min_slack_floor = min_slack_floor;
        report.max_objective_excess = max_objective_excess;
        report.row_digest = dual.row_digest().to_string();
        report.status = if checked == 2
            && certified == 2
            && dual.status() == G27MwisLpGuidedMicroDualStatus::ExactMicroPrunesCertified
        {
            G27MwisLpGuidedSecondStatus::FundTopKFramework
        } else {
            G27MwisLpGuidedSecondStatus::ExactReplayFailed
        };
    }
    Ok(report)
}

fn status_without_replay(
    final_worst_total: i128,
    solver_prunes: usize,
) -> G27MwisLpGuidedSecondStatus {
    if final_worst_total <= TARGET_WEIGHT && solver_prunes == 2 {
        G27MwisLpGuidedSecondStatus::PromisingButNeedsReplay
    } else {
        G27MwisLpGuidedSecondStatus::RetireSecondNodePattern
    }
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

fn node_digest(entry: &QueueEntry) -> String {
    let mut payload = String::new();
    write_numbers(&entry.node.branch_included, &mut payload);
    payload.push('|');
    write_numbers(&entry.node.forced_included, &mut payload);
    payload.push('|');
    write_numbers(&entry.node.excluded, &mut payload);
    payload.push('|');
    write_numbers(&entry.node.candidates, &mut payload);
    payload.push('|');
    payload.push_str(&entry.node.chosen_weight.to_string());
    payload.push('|');
    payload.push_str(&entry.node.depth.to_string());
    format!("{:x}", Sha256::digest(payload.as_bytes()))
}

fn write_numbers(numbers: &[usize], payload: &mut String) {
    for (index, number) in numbers.iter().enumerate() {
        if index > 0 {
            payload.push(',');
        }
        payload.push_str(&number.to_string());
    }
}

fn empty_report(status: G27MwisLpGuidedSecondStatus) -> G27MwisLpGuidedSecondReport {
    G27MwisLpGuidedSecondReport {
        parent_total: 0,
        first_branch: 0,
        first_worst_total: 0,
        second_branch: 0,
        final_worst_total: 0,
        solver_prune_candidates: 0,
        certified_prunes: 0,
        explicit_rows: 0,
        positive_dual_rows: 0,
        max_denominator: 1,
        min_slack_floor: i128::MAX,
        max_objective_excess: 0,
        parent_digest: String::new(),
        row_digest: String::new(),
        status,
    }
}
