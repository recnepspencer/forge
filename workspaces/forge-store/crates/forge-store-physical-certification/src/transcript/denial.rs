use crate::{
    OracleFamilyKind, PhysicalProofOracleKind, PhysicalProofOracleVerdict, PhysicalSimulationPlan,
};

use super::TranscriptReplayEvidenceIdentity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranscriptReplayDenial {
    MissingSeed,
    MissingDriverProfile,
    MissingOracleVerdict,
    MissingTranscriptReplayOracleVerdict,
    OracleFamilyNotRequired,
    RequiredOracleFamilyMissing(OracleFamilyKind),
    OracleFamilyMismatch,
    PlanScheduleIdentityMismatch,
    PlanTraceIdentityMismatch,
    CounterReceiptPlanMismatch,
    OracleVerdictPlanMismatch,
    TranscriptReplayVerdictMissingReplayEvidence,
    TranscriptReplayVerdictBasisMismatch,
    LooseLogDenied,
    TerminalJsonDenied,
    CopiedTranscriptFieldsDenied,
    SameRunSelfComparisonDenied,
}

pub fn reject_loose_log_transcript_attempt() -> Result<(), TranscriptReplayDenial> {
    Err(TranscriptReplayDenial::LooseLogDenied)
}

pub fn reject_terminal_json_transcript_attempt() -> Result<(), TranscriptReplayDenial> {
    Err(TranscriptReplayDenial::TerminalJsonDenied)
}

pub fn reject_copied_transcript_fields() -> Result<(), TranscriptReplayDenial> {
    Err(TranscriptReplayDenial::CopiedTranscriptFieldsDenied)
}

pub fn reject_same_run_self_comparison_transcript_attempt() -> Result<(), TranscriptReplayDenial> {
    Err(TranscriptReplayDenial::SameRunSelfComparisonDenied)
}

pub(crate) fn require_transcript_replay_verdict(
    verdicts: &[PhysicalProofOracleVerdict],
) -> Result<(), TranscriptReplayDenial> {
    if verdicts.is_empty() {
        return Err(TranscriptReplayDenial::MissingOracleVerdict);
    }
    if verdicts
        .iter()
        .any(|verdict| verdict.oracle() == PhysicalProofOracleKind::TranscriptReplay)
    {
        Ok(())
    } else {
        Err(TranscriptReplayDenial::MissingTranscriptReplayOracleVerdict)
    }
}

pub(crate) fn require_plan_bound_oracle_verdicts(
    plan: &PhysicalSimulationPlan,
    verdicts: &[PhysicalProofOracleVerdict],
) -> Result<(), TranscriptReplayDenial> {
    require_transcript_replay_verdict(verdicts)?;
    for verdict in verdicts {
        if !plan.oracle_families().contains(verdict.family()) {
            return Err(TranscriptReplayDenial::OracleFamilyNotRequired);
        }
        if verdict.basis().scenario_identity() != plan.scenario_identity()
            || verdict.basis().plan_identity() != plan.identity()
        {
            return Err(TranscriptReplayDenial::OracleVerdictPlanMismatch);
        }
    }
    Ok(())
}

pub(crate) fn require_plan_bound_oracle_verdicts_for_replay_basis(
    plan: &PhysicalSimulationPlan,
    verdicts: &[PhysicalProofOracleVerdict],
    replay_basis: &TranscriptReplayEvidenceIdentity,
) -> Result<(), TranscriptReplayDenial> {
    require_plan_bound_oracle_verdicts(plan, verdicts)?;
    for verdict in verdicts
        .iter()
        .filter(|verdict| verdict.oracle() == PhysicalProofOracleKind::TranscriptReplay)
    {
        let Some(verdict_basis) = verdict.transcript_replay_basis_digest() else {
            return Err(TranscriptReplayDenial::TranscriptReplayVerdictMissingReplayEvidence);
        };
        if verdict_basis != replay_basis.digest_bytes() {
            return Err(TranscriptReplayDenial::TranscriptReplayVerdictBasisMismatch);
        }
    }
    require_required_oracle_family_coverage(plan, verdicts)?;
    Ok(())
}

fn require_required_oracle_family_coverage(
    plan: &PhysicalSimulationPlan,
    verdicts: &[PhysicalProofOracleVerdict],
) -> Result<(), TranscriptReplayDenial> {
    for required_family in plan.oracle_families().iter() {
        if !verdicts
            .iter()
            .any(|verdict| verdict.family() == required_family)
        {
            return Err(TranscriptReplayDenial::RequiredOracleFamilyMissing(
                required_family,
            ));
        }
    }
    Ok(())
}
