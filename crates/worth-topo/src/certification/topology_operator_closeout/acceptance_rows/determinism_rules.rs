use crate::certification::ReplayParityStatus;
use crate::topology_operators::TopologyMutationNamingOutcome;

use super::super::report::{
    MilestoneThreeAmbiguousLocalRewireWitness, MilestoneThreeDeterminismRuleKind,
    MilestoneThreeDeterminismRuleRow, MilestoneThreeHostileScenarioReport,
};

pub(super) fn build_determinism_rule_rows(
    reports: &[MilestoneThreeHostileScenarioReport],
) -> Vec<MilestoneThreeDeterminismRuleRow> {
    let mut rows = Vec::new();
    for report in reports {
        rows.push(stable_mutation_order_row(report));
        rows.push(stable_mutation_digest_row(report));
        if report.rejection_class.is_some() {
            rows.push(stable_rejection_classification_row(report));
        }
        if let Some(witness) = report.ambiguous_local_rewire_witness.as_ref() {
            rows.push(ambiguous_tie_break_evidence_row(report, witness));
        }
    }
    rows
}

fn stable_mutation_order_row(
    report: &MilestoneThreeHostileScenarioReport,
) -> MilestoneThreeDeterminismRuleRow {
    let replay = &report.mutation_replay_parity_report;
    MilestoneThreeDeterminismRuleRow {
        scenario: report.scenario,
        rule_kind: MilestoneThreeDeterminismRuleKind::StableMutationOrder,
        evidence_count: report.topology_mutation_digest.mutation_record_count + replay.step_rows.len(),
        replay_verified: replay.replay_checked
            && replay.parity_status == ReplayParityStatus::Match
            && replay.mismatch_count == 0,
        diagnostic_classification_stable: false,
        tie_break_evidence_stable: false,
        row_digest: format!(
            "scenario={};rule={};order_policy=sequence_preserving;digest={};mutation_records={};steps={};replay_steps={}",
            report.scenario.as_str(),
            MilestoneThreeDeterminismRuleKind::StableMutationOrder.as_str(),
            report.topology_mutation_digest.digest.digest_hex,
            report.topology_mutation_digest.mutation_record_count,
            replay.step_rows.len(),
            replay.replay_step_rows.len()
        ),
    }
}

fn stable_mutation_digest_row(
    report: &MilestoneThreeHostileScenarioReport,
) -> MilestoneThreeDeterminismRuleRow {
    let replay = &report.mutation_replay_parity_report;
    MilestoneThreeDeterminismRuleRow {
        scenario: report.scenario,
        rule_kind: MilestoneThreeDeterminismRuleKind::StableMutationDigest,
        evidence_count: replay.step_rows.len() + replay.replay_step_rows.len(),
        replay_verified: replay.replay_checked
            && replay.parity_status == ReplayParityStatus::Match
            && replay.mismatch_count == 0,
        diagnostic_classification_stable: false,
        tie_break_evidence_stable: false,
        row_digest: format!(
            "scenario={};rule={};digest={};steps={};replay_steps={};mismatches={}",
            report.scenario.as_str(),
            MilestoneThreeDeterminismRuleKind::StableMutationDigest.as_str(),
            report.topology_mutation_digest.digest.digest_hex,
            replay.step_rows.len(),
            replay.replay_step_rows.len(),
            replay.mismatch_count
        ),
    }
}

fn stable_rejection_classification_row(
    report: &MilestoneThreeHostileScenarioReport,
) -> MilestoneThreeDeterminismRuleRow {
    let rejected_scope_rows = report
        .rejected_mutation_scope_report
        .as_ref()
        .map_or(0, |scope| scope.rows.len());
    MilestoneThreeDeterminismRuleRow {
        scenario: report.scenario,
        rule_kind: MilestoneThreeDeterminismRuleKind::StableRejectionClassification,
        evidence_count: usize::from(report.rejection_class.is_some()) + rejected_scope_rows,
        replay_verified: report.mutation_replay_parity_report.replay_checked
            && report.mutation_replay_parity_report.mismatch_count == 0,
        diagnostic_classification_stable: report.rejection_class.is_some()
            && report.rejected_mutation_scope_report.is_some(),
        tie_break_evidence_stable: false,
        row_digest: format!(
            "scenario={};rule={};rejection_class={:?};scope_rows={}",
            report.scenario.as_str(),
            MilestoneThreeDeterminismRuleKind::StableRejectionClassification.as_str(),
            report.rejection_class,
            rejected_scope_rows
        ),
    }
}

fn ambiguous_tie_break_evidence_row(
    report: &MilestoneThreeHostileScenarioReport,
    witness: &MilestoneThreeAmbiguousLocalRewireWitness,
) -> MilestoneThreeDeterminismRuleRow {
    let distinct_accepted_outputs = witness.chosen_materialized_topology_digest
        != witness.alternate_materialized_topology_digest;
    MilestoneThreeDeterminismRuleRow {
        scenario: report.scenario,
        rule_kind: MilestoneThreeDeterminismRuleKind::AmbiguousTieBreakEvidence,
        evidence_count: 2,
        replay_verified: report.mutation_replay_parity_report.replay_checked
            && report.mutation_replay_parity_report.mismatch_count == 0,
        diagnostic_classification_stable: report.continuity_outcome_class
            == TopologyMutationNamingOutcome::Ambiguous,
        tie_break_evidence_stable: distinct_accepted_outputs
            && !witness.chosen_successor_identity.is_empty()
            && !witness.alternate_successor_identity.is_empty(),
        row_digest: format!(
            "scenario={};rule={};chosen={};alternate={};chosen_digest={};alternate_digest={}",
            report.scenario.as_str(),
            MilestoneThreeDeterminismRuleKind::AmbiguousTieBreakEvidence.as_str(),
            witness.chosen_successor_identity,
            witness.alternate_successor_identity,
            witness.chosen_materialized_topology_digest.digest_hex,
            witness.alternate_materialized_topology_digest.digest_hex
        ),
    }
}
