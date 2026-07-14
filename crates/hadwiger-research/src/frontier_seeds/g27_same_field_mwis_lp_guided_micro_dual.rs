use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use sha2::{Digest, Sha256};

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::BitWords;
use super::g27_same_field_lp_relaxation::{
    stable_set_lp_guidance_values, stable_set_lp_relaxation_rows,
};
use super::g27_same_field_mwis_branch_certificate_preflight::dominant_and_exact_side_weight;
use super::g27_same_field_mwis_branch_prefix_replay::{
    replay_g27_same_field_mwis_branch_prefix_checked, G27MwisBranchPrefixReplayStatus,
};
use super::g27_same_field_mwis_lp_guided_branch_support::{
    branch_children, initial_frontier, lp_guided_branch, node_upper, QueueEntry,
};
use super::g27_same_field_mwis_lp_guided_micro_dual_support::{
    explicit_rows, validate_rows, write_record,
};
use super::g27_same_field_mwis_odd_cycle_dual_replay_support::{
    replay_certificate_for_candidates, CertifiedRow, ExplicitRow, Rational, POSITIVE_EPSILON,
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
const EXPECTED_SECOND_BRANCH: usize = 223;
const EXPECTED_FINAL_WORST_TOTAL: i128 = 507_877;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisLpGuidedMicroDualStatus {
    ExactMicroPrunesCertified,
    PrefixReplayMismatch,
    FrozenInstanceMismatch,
    PathIdentityMismatch,
    RowReplayUnstable,
    DualCoverageFailed,
    DualObjectiveMismatch,
    BoundAboveThreshold,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisLpGuidedMicroDualReport {
    checked_nodes: usize,
    certified_prunes: usize,
    explicit_rows: usize,
    positive_dual_rows: usize,
    max_denominator: i128,
    min_slack_floor: i128,
    max_objective_excess: i128,
    final_worst_total: i128,
    row_digest: String,
    status: G27MwisLpGuidedMicroDualStatus,
}

impl G27MwisLpGuidedMicroDualReport {
    pub fn summary(&self) -> (usize, usize, usize, usize, i128) {
        (
            self.checked_nodes,
            self.certified_prunes,
            self.explicit_rows,
            self.positive_dual_rows,
            self.final_worst_total,
        )
    }

    pub fn exact_summary(&self) -> (i128, i128, i128) {
        (
            self.max_denominator,
            self.min_slack_floor,
            self.max_objective_excess,
        )
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub fn status(&self) -> G27MwisLpGuidedMicroDualStatus {
        self.status
    }
}

pub fn replay_g27_same_field_mwis_lp_guided_micro_duals_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisLpGuidedMicroDualReport, G27GeometricFractionalError> {
    let first = build_report(handle)?;
    if first.status != G27MwisLpGuidedMicroDualStatus::ExactMicroPrunesCertified {
        return Ok(first);
    }
    let second = build_report(handle)?;
    if first.summary() != second.summary()
        || first.exact_summary() != second.exact_summary()
        || first.row_digest != second.row_digest
    {
        return Ok(G27MwisLpGuidedMicroDualReport {
            status: G27MwisLpGuidedMicroDualStatus::RowReplayUnstable,
            ..first
        });
    }
    Ok(first)
}

fn build_report(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisLpGuidedMicroDualReport, G27GeometricFractionalError> {
    let prefix = replay_g27_same_field_mwis_branch_prefix_checked(handle)?;
    let (expanded, pruned, open, best_open_total, _, _) = prefix.summary();
    if prefix.status() != G27MwisBranchPrefixReplayStatus::BranchPrefixSemanticsPreflight
        || expanded != EXPECTED_PREFIX_EXPANDED
        || pruned != EXPECTED_PREFIX_PRUNED
        || open != EXPECTED_PREFIX_OPEN
        || best_open_total != EXPECTED_BEST_OPEN_TOTAL
    {
        return Ok(empty_report(
            G27MwisLpGuidedMicroDualStatus::PrefixReplayMismatch,
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
            source: "lp_guided_micro_dual_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT {
        return Ok(empty_report(
            G27MwisLpGuidedMicroDualStatus::FrozenInstanceMismatch,
        ));
    }
    certify_path(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        small_weight,
    )
}

fn certify_path(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    exact_side_weight: i128,
) -> Result<G27MwisLpGuidedMicroDualReport, G27GeometricFractionalError> {
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
            source: "lp_guided_micro_dual_frontier",
        })?;
    let first_guidance = stable_set_lp_guidance_values(adjacency, weights, &top.node.candidates)?;
    let first_branch = lp_guided_branch(adjacency, weights, &top.node.candidates, &first_guidance);
    if first_branch != EXPECTED_FIRST_BRANCH {
        return Ok(empty_report(
            G27MwisLpGuidedMicroDualStatus::PathIdentityMismatch,
        ));
    }
    let first_children = child_entries(adjacency, weights, top, first_branch)?;
    let worse_child = first_children
        .iter()
        .max_by_key(|entry| entry.upper_bound)
        .ok_or(G27GeometricFractionalError::MalformedData {
            source: "lp_guided_micro_dual_first_child",
        })?;
    let second_guidance =
        stable_set_lp_guidance_values(adjacency, weights, &worse_child.node.candidates)?;
    let second_branch = lp_guided_branch(
        adjacency,
        weights,
        &worse_child.node.candidates,
        &second_guidance,
    );
    if second_branch != EXPECTED_SECOND_BRANCH {
        return Ok(empty_report(
            G27MwisLpGuidedMicroDualStatus::PathIdentityMismatch,
        ));
    }
    let second_children = child_entries(adjacency, weights, worse_child, second_branch)?;
    let final_worst = exact_side_weight
        + second_children
            .iter()
            .map(|entry| entry.upper_bound)
            .max()
            .unwrap_or(0);
    if final_worst != EXPECTED_FINAL_WORST_TOTAL {
        return Ok(empty_report(
            G27MwisLpGuidedMicroDualStatus::PathIdentityMismatch,
        ));
    }
    certify_children(adjacency, weights, exact_side_weight, &second_children)
}

pub(super) fn certify_children(
    adjacency: &[BitWords],
    weights: &[i128],
    exact_side_weight: i128,
    children: &[QueueEntry],
) -> Result<G27MwisLpGuidedMicroDualReport, G27GeometricFractionalError> {
    let mut report = empty_report(G27MwisLpGuidedMicroDualStatus::ExactMicroPrunesCertified);
    report.checked_nodes = children.len();
    let mut payload = String::new();
    for (index, child) in children.iter().enumerate() {
        let rows = stable_set_lp_relaxation_rows(adjacency, weights, &child.node.candidates)?;
        validate_rows(adjacency, &child.node.candidates, &rows)?;
        write_record(index, child, &rows, &mut payload);
        let explicit = explicit_rows(adjacency, &child.node.candidates, &rows);
        let certificate = solve_one_sided_dual(weights, &child.node.candidates, &explicit)?;
        let replay = replay_certificate_for_candidates(
            weights,
            &child.node.candidates,
            &explicit,
            &certificate,
        );
        report.explicit_rows += explicit.len();
        report.positive_dual_rows += certificate.len();
        report.max_denominator = report.max_denominator.max(
            certificate
                .iter()
                .map(|row| row.multiplier.den_i128())
                .max()
                .unwrap_or(1),
        );
        report.min_slack_floor = report.min_slack_floor.min(replay.min_slack_floor);
        report.max_objective_excess = report
            .max_objective_excess
            .max(replay.objective_ceil - rows.odd_cycle_objective_ceiling);
        if !replay.coverage_ok {
            report.status = G27MwisLpGuidedMicroDualStatus::DualCoverageFailed;
            return Ok(report);
        }
        if replay.objective_ceil != rows.odd_cycle_objective_ceiling {
            report.status = G27MwisLpGuidedMicroDualStatus::DualObjectiveMismatch;
            return Ok(report);
        }
        let certified_bound = child.node.chosen_weight + replay.objective_ceil;
        if certified_bound > EXPECTED_DOMINANT_THRESHOLD
            || exact_side_weight + certified_bound > TARGET_WEIGHT
        {
            report.status = G27MwisLpGuidedMicroDualStatus::BoundAboveThreshold;
            return Ok(report);
        }
        report.certified_prunes += 1;
        report.final_worst_total = report
            .final_worst_total
            .max(exact_side_weight + certified_bound);
    }
    report.row_digest = format!("{:x}", Sha256::digest(payload.as_bytes()));
    Ok(report)
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

pub(super) fn solve_one_sided_dual(
    weights: &[i128],
    candidates: &[usize],
    rows: &[ExplicitRow],
) -> Result<Vec<CertifiedRow>, G27GeometricFractionalError> {
    let mut variables = variables!();
    let ys = rows
        .iter()
        .map(|_| variables.add(variable().min(0.0)))
        .collect::<Vec<_>>();
    let objective = rows
        .iter()
        .zip(ys.iter())
        .fold(Expression::from(0.0), |sum, (row, y)| {
            sum + row.rhs as f64 * *y
        });
    let mut problem = variables.minimise(objective).using(default_solver);
    for local in 0..candidates.len() {
        let expression = rows
            .iter()
            .zip(ys.iter())
            .filter(|(row, _)| row.support.contains(&local))
            .fold(Expression::from(0.0), |sum, (_, y)| sum + *y);
        problem = problem.with(constraint!(expression >= weights[candidates[local]] as f64));
    }
    let solution = problem
        .solve()
        .map_err(|error| G27GeometricFractionalError::MatrixZip(error.to_string()))?;
    Ok(ys
        .iter()
        .enumerate()
        .filter_map(|(index, y)| {
            let value = solution.value(*y);
            (value > POSITIVE_EPSILON).then(|| CertifiedRow {
                index,
                multiplier: Rational::ceiling_from_f64(value),
            })
        })
        .collect())
}

fn empty_report(status: G27MwisLpGuidedMicroDualStatus) -> G27MwisLpGuidedMicroDualReport {
    G27MwisLpGuidedMicroDualReport {
        checked_nodes: 0,
        certified_prunes: 0,
        explicit_rows: 0,
        positive_dual_rows: 0,
        max_denominator: 1,
        min_slack_floor: i128::MAX,
        max_objective_excess: 0,
        final_worst_total: 0,
        row_digest: String::new(),
        status,
    }
}
