use super::{
    EvidenceBundleAuthority, FoundationalPhysicalCertificationEvidenceBundle,
    PhysicalEvidenceBundleDenial, SimulationFailureDigest,
};
use crate::transcript::require_plan_bound_oracle_verdicts_for_replay_basis;
use crate::SimulationReplayBundle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalEvidenceBundlePrimary {
    scenario_digest: [u8; 32],
    plan_digest: [u8; 32],
    transcript_digest: [u8; 32],
    oracle_verdict_count: usize,
    counter_row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalCertificationEvidenceBundle {
    replay: SimulationReplayBundle,
    authority: EvidenceBundleAuthority,
    failure_digest: Option<SimulationFailureDigest>,
}

impl PhysicalCertificationEvidenceBundle {
    pub fn from_replay_bundle(
        replay: SimulationReplayBundle,
    ) -> Result<Self, PhysicalEvidenceBundleDenial> {
        require_evidence_verdicts(&replay)?;
        let failure_digest = SimulationFailureDigest::from_replay_bundle(&replay);
        Ok(Self {
            replay,
            authority: EvidenceBundleAuthority::current_store_authority(),
            failure_digest,
        })
    }

    pub const fn replay(&self) -> &SimulationReplayBundle {
        &self.replay
    }

    pub const fn authority(&self) -> EvidenceBundleAuthority {
        self.authority
    }

    pub const fn failure_digest(&self) -> Option<&SimulationFailureDigest> {
        self.failure_digest.as_ref()
    }

    pub fn primary(&self) -> PhysicalEvidenceBundlePrimary {
        PhysicalEvidenceBundlePrimary {
            scenario_digest: *self.replay.plan().scenario_identity().digest_bytes(),
            plan_digest: *self.replay.plan().identity().digest_bytes(),
            transcript_digest: *self.replay.transcript_identity().digest_bytes(),
            oracle_verdict_count: self.replay.oracle_verdicts().len(),
            counter_row_count: self.replay.counter_receipt().rows().len(),
        }
    }

    pub fn materialize_foundational_evidence(
        &self,
    ) -> FoundationalPhysicalCertificationEvidenceBundle {
        FoundationalPhysicalCertificationEvidenceBundle::from_store_evidence(self)
    }
}

impl PhysicalEvidenceBundlePrimary {
    pub const fn scenario_digest(&self) -> &[u8; 32] {
        &self.scenario_digest
    }

    pub const fn plan_digest(&self) -> &[u8; 32] {
        &self.plan_digest
    }

    pub const fn transcript_digest(&self) -> &[u8; 32] {
        &self.transcript_digest
    }

    pub const fn oracle_verdict_count(&self) -> usize {
        self.oracle_verdict_count
    }

    pub const fn counter_row_count(&self) -> usize {
        self.counter_row_count
    }
}

fn require_evidence_verdicts(
    replay: &SimulationReplayBundle,
) -> Result<(), PhysicalEvidenceBundleDenial> {
    require_plan_bound_oracle_verdicts_for_replay_basis(
        replay.plan(),
        replay.oracle_verdicts(),
        replay.replay_basis_identity(),
    )
    .map_err(|denial| match denial {
        crate::TranscriptReplayDenial::MissingOracleVerdict => {
            PhysicalEvidenceBundleDenial::MissingOracleVerdict
        }
        crate::TranscriptReplayDenial::MissingTranscriptReplayOracleVerdict => {
            PhysicalEvidenceBundleDenial::MissingTranscriptReplayOracleVerdict
        }
        crate::TranscriptReplayDenial::OracleFamilyNotRequired => {
            PhysicalEvidenceBundleDenial::OracleFamilyNotRequired
        }
        crate::TranscriptReplayDenial::RequiredOracleFamilyMissing(family) => {
            PhysicalEvidenceBundleDenial::RequiredOracleFamilyMissing(family)
        }
        crate::TranscriptReplayDenial::OracleFamilyMismatch => {
            PhysicalEvidenceBundleDenial::OracleFamilyMismatch
        }
        crate::TranscriptReplayDenial::OracleVerdictPlanMismatch => {
            PhysicalEvidenceBundleDenial::OracleVerdictPlanMismatch
        }
        crate::TranscriptReplayDenial::TranscriptReplayVerdictMissingReplayEvidence => {
            PhysicalEvidenceBundleDenial::TranscriptReplayVerdictMissingReplayEvidence
        }
        crate::TranscriptReplayDenial::TranscriptReplayVerdictBasisMismatch => {
            PhysicalEvidenceBundleDenial::TranscriptReplayVerdictBasisMismatch
        }
        _ => PhysicalEvidenceBundleDenial::MissingOracleVerdict,
    })?;
    Ok(())
}
