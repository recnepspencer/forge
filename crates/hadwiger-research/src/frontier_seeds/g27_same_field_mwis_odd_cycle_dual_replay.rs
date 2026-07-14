use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};

use crate::query_entry::HadwigerResearchHandle;

use super::g27_geometric_fractional::G27GeometricFractionalError;
use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};
use super::g27_same_field_mwis_branch_certificate_preflight::dominant_and_exact_side_weight;
use super::g27_same_field_mwis_odd_cycle_dual_replay_support::{
    replay_certificate, CertifiedRow, ExplicitRow, Rational, POSITIVE_EPSILON,
};
use super::g27_same_field_mwis_odd_cycle_row_replay_support::{
    collect_row_records, NodeRecord, TARGET_PRUNED_RECORDS,
};
use super::g27_same_field_threshold_mwis_bnb_setup::threshold_mwis_alignment_channel_instance_sets;

const TARGET_WEIGHT: i128 = 512_933;
const G27_ANCHOR_INDEX: usize = 7;
const W_ANCHOR_INDEX: usize = 300;
const ATOM_LIMIT: usize = 5;
const ATOM_MASK: u32 = 101_719_589;
const EXPECTED_EXACT_SIDE_WEIGHT: i128 = 61_655;
const EXPECTED_DOMINANT_THRESHOLD: i128 = 451_278;
const EXPECTED_ROOT_TOTAL: i128 = 543_428;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum G27MwisOddCycleDualReplayStatus {
    ExactOddCycleNodeDualReplayPreflight,
    OneSidedOddCycleNodeDualReplayPreflight,
    FrozenInstanceMismatch,
    MissingThresholdPrunes,
    DualObjectiveMismatch,
    DualCoverageFailed,
    RationalizationTooWeak,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct G27MwisOddCycleDualReplayReport {
    checked_node_count: usize,
    certified_pruned_node_count: usize,
    total_explicit_rows: usize,
    positive_dual_rows: usize,
    max_denominator: i128,
    root_total_bound: i128,
    min_slack_floor: i128,
    max_objective_excess: i128,
    status: G27MwisOddCycleDualReplayStatus,
}

impl G27MwisOddCycleDualReplayReport {
    pub fn summary(&self) -> (usize, usize, usize, usize, i128) {
        (
            self.checked_node_count,
            self.certified_pruned_node_count,
            self.total_explicit_rows,
            self.positive_dual_rows,
            self.root_total_bound,
        )
    }

    pub fn exact_summary(&self) -> (i128, i128, i128) {
        (
            self.max_denominator,
            self.min_slack_floor,
            self.max_objective_excess,
        )
    }

    pub fn status(&self) -> G27MwisOddCycleDualReplayStatus {
        self.status
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

pub fn replay_g27_same_field_mwis_odd_cycle_duals_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisOddCycleDualReplayReport, G27GeometricFractionalError> {
    replay_duals_checked(handle, RationalizationMode::Nearest)
}

pub fn replay_g27_same_field_mwis_odd_cycle_one_sided_duals_checked(
    handle: &HadwigerResearchHandle,
) -> Result<G27MwisOddCycleDualReplayReport, G27GeometricFractionalError> {
    replay_duals_checked(handle, RationalizationMode::OneSidedCeiling)
}

fn replay_duals_checked(
    handle: &HadwigerResearchHandle,
    mode: RationalizationMode,
) -> Result<G27MwisOddCycleDualReplayReport, G27GeometricFractionalError> {
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
            source: "odd_cycle_dual_replay_channel",
        })?;
    let (dominant, small_weight) = dominant_and_exact_side_weight(&channel.instance);
    let threshold = TARGET_WEIGHT - small_weight;
    if small_weight != EXPECTED_EXACT_SIDE_WEIGHT || threshold != EXPECTED_DOMINANT_THRESHOLD {
        return Ok(empty_report(
            G27MwisOddCycleDualReplayStatus::FrozenInstanceMismatch,
        ));
    }
    let records = collect_row_records(
        &channel.instance.adjacency,
        &channel.instance.weights,
        &dominant,
        threshold,
    )?;
    if records.len() < TARGET_PRUNED_RECORDS + 1 {
        return Ok(empty_report(
            G27MwisOddCycleDualReplayStatus::MissingThresholdPrunes,
        ));
    }
    certify_records(
        &channel.instance.adjacency,
        &channel.instance.weights,
        small_weight,
        threshold,
        &records,
        mode,
    )
}

#[derive(Clone, Copy)]
enum RationalizationMode {
    Nearest,
    OneSidedCeiling,
}

fn certify_records(
    adjacency: &[BitWords],
    weights: &[i128],
    exact_side_weight: i128,
    threshold: i128,
    records: &[NodeRecord],
    mode: RationalizationMode,
) -> Result<G27MwisOddCycleDualReplayReport, G27GeometricFractionalError> {
    let status = match mode {
        RationalizationMode::Nearest => {
            G27MwisOddCycleDualReplayStatus::ExactOddCycleNodeDualReplayPreflight
        }
        RationalizationMode::OneSidedCeiling => {
            G27MwisOddCycleDualReplayStatus::OneSidedOddCycleNodeDualReplayPreflight
        }
    };
    let mut report = empty_report(status);
    report.checked_node_count = records.len();
    for (index, record) in records.iter().enumerate() {
        let rows = explicit_rows(adjacency, record);
        let certificate = solve_dual_certificate(weights, record, &rows, mode)?;
        let replay = replay_certificate(weights, record, &rows, &certificate);
        report.total_explicit_rows += rows.len();
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
            .max(replay.objective_ceil - record.rows.odd_cycle_objective_ceiling);
        if !replay.coverage_ok {
            report.status = G27MwisOddCycleDualReplayStatus::DualCoverageFailed;
            return Ok(report);
        }
        if replay.objective_ceil != record.rows.odd_cycle_objective_ceiling {
            report.status = G27MwisOddCycleDualReplayStatus::DualObjectiveMismatch;
            return Ok(report);
        }
        if index == 0 {
            report.root_total_bound =
                exact_side_weight + record.chosen_weight() + replay.objective_ceil;
            if report.root_total_bound != EXPECTED_ROOT_TOTAL {
                report.status = G27MwisOddCycleDualReplayStatus::DualObjectiveMismatch;
                return Ok(report);
            }
        } else if record.chosen_weight() + replay.objective_ceil <= threshold {
            report.certified_pruned_node_count += 1;
        }
    }
    if report.certified_pruned_node_count < TARGET_PRUNED_RECORDS {
        report.status = G27MwisOddCycleDualReplayStatus::RationalizationTooWeak;
    }
    Ok(report)
}

fn explicit_rows(adjacency: &[BitWords], record: &NodeRecord) -> Vec<ExplicitRow> {
    let candidates = record.candidates();
    let mut rows = Vec::new();
    for left in 0..candidates.len() {
        rows.push(ExplicitRow {
            support: vec![left],
            rhs: 1,
        });
        for right in (left + 1)..candidates.len() {
            if has_bit(&adjacency[candidates[left]], candidates[right]) {
                rows.push(ExplicitRow {
                    support: vec![left, right],
                    rhs: 1,
                });
            }
        }
    }
    rows.extend(
        record
            .rows
            .clique_constraints
            .iter()
            .map(|support| ExplicitRow {
                support: support.clone(),
                rhs: 1,
            }),
    );
    rows.extend(record.rows.odd_cycle_cuts.iter().map(|cut| ExplicitRow {
        support: cut.support.clone(),
        rhs: (cut.support.len() / 2) as i128,
    }));
    rows
}

fn solve_dual_certificate(
    weights: &[i128],
    record: &NodeRecord,
    rows: &[ExplicitRow],
    mode: RationalizationMode,
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
    for local in 0..record.candidates().len() {
        let expression = rows
            .iter()
            .zip(ys.iter())
            .filter(|(row, _)| row.support.contains(&local))
            .fold(Expression::from(0.0), |sum, (_, y)| sum + *y);
        problem = problem.with(constraint!(
            expression >= weights[record.candidates()[local]] as f64
        ));
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
                multiplier: match mode {
                    RationalizationMode::Nearest => Rational::nearest_from_f64(value),
                    RationalizationMode::OneSidedCeiling => Rational::ceiling_from_f64(value),
                },
            })
        })
        .collect())
}

fn empty_report(status: G27MwisOddCycleDualReplayStatus) -> G27MwisOddCycleDualReplayReport {
    G27MwisOddCycleDualReplayReport {
        checked_node_count: 0,
        certified_pruned_node_count: 0,
        total_explicit_rows: 0,
        positive_dual_rows: 0,
        max_denominator: 1,
        root_total_bound: 0,
        min_slack_floor: i128::MAX,
        max_objective_excess: 0,
        status,
    }
}
