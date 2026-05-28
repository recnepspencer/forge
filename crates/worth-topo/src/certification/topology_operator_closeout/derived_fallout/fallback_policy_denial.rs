use crate::certification::TopologyCertificationError;
use crate::topology_operators::{TopologyEditDerivedFallbackPolicy, TopologyEditRejectionClass};

use super::super::report::{
    MilestoneThreeEditFalloutBreadthRow, MilestoneThreeEditFalloutClass,
    MilestoneThreeHostileSuiteReport,
};
use super::fallback_policy_denial_rows::MilestoneThreeDerivedFallbackPolicyDenialRow;

pub(in crate::certification::topology_operator_closeout) fn build_fallback_policy_denial_rows(
    fallout_rows: &[MilestoneThreeEditFalloutBreadthRow],
) -> Vec<MilestoneThreeDerivedFallbackPolicyDenialRow> {
    fallout_rows
        .iter()
        .filter(|row| fallback_can_be_denied(row))
        .map(build_fallback_policy_denial_row)
        .collect()
}

pub(in crate::certification::topology_operator_closeout) fn ensure_fallback_policy_denial_rows(
    report: &MilestoneThreeHostileSuiteReport,
) -> Result<(), TopologyCertificationError> {
    if report.derived_fallback_policy_denial_rows.is_empty() {
        return Err(closeout_requirement_error(
            "missing derived fallback policy denial rows",
        ));
    }
    for row in &report.derived_fallback_policy_denial_rows {
        if row.strict_fallback_policy != TopologyEditDerivedFallbackPolicy::RejectAnyFallback {
            return Err(closeout_requirement_error(&format!(
                "fallback denial row used a non-strict policy for {}",
                row.scenario.as_str()
            )));
        }
        if !row.policy_exceeded || row.observed_fallback_count == 0 {
            return Err(closeout_requirement_error(&format!(
                "fallback denial row did not prove policy overflow for {}",
                row.scenario.as_str()
            )));
        }
        if row.denied_rejection_class != TopologyEditRejectionClass::DerivedFallbackExceeded {
            return Err(closeout_requirement_error(&format!(
                "fallback denial row used the wrong rejection class for {}",
                row.scenario.as_str()
            )));
        }
        if !row.row_digest.starts_with(&format!(
            "scenario={};strict_policy=reject_any_fallback;",
            row.scenario.as_str()
        )) {
            return Err(closeout_requirement_error(&format!(
                "fallback denial row digest is malformed for {}",
                row.scenario.as_str()
            )));
        }
    }
    Ok(())
}

fn build_fallback_policy_denial_row(
    fallout: &MilestoneThreeEditFalloutBreadthRow,
) -> MilestoneThreeDerivedFallbackPolicyDenialRow {
    let strict_fallback_policy = TopologyEditDerivedFallbackPolicy::RejectAnyFallback;
    let policy_exceeded = strict_fallback_policy.is_exceeded_by(fallout.fallback_count);

    MilestoneThreeDerivedFallbackPolicyDenialRow {
        scenario: fallout.scenario,
        strict_fallback_policy,
        observed_fallout_class: fallout.fallout_class,
        observed_fallback_count: fallout.fallback_count,
        denied_rejection_class: TopologyEditRejectionClass::DerivedFallbackExceeded,
        policy_exceeded,
        row_digest: format!(
            "scenario={};strict_policy={};observed_fallout={:?};observed_fallback_count={};denied_rejection_class={:?};policy_exceeded={}",
            fallout.scenario.as_str(),
            strict_fallback_policy.as_str(),
            fallout.fallout_class,
            fallout.fallback_count,
            TopologyEditRejectionClass::DerivedFallbackExceeded,
            policy_exceeded
        ),
    }
}

fn fallback_can_be_denied(row: &MilestoneThreeEditFalloutBreadthRow) -> bool {
    matches!(
        row.fallout_class,
        MilestoneThreeEditFalloutClass::WholeViewFallback
            | MilestoneThreeEditFalloutClass::WholeHistoryFallback
    ) && row.fallback_count > 0
}

fn closeout_requirement_error(reason: &str) -> TopologyCertificationError {
    TopologyCertificationError::Query(format!(
        "milestone three closeout requirement failed: {reason}"
    ))
}




