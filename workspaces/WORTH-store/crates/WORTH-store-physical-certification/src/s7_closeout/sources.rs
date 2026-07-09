use worth_store_blob_chunks::S7ExecutedLifecycleEvidenceBundle;

use crate::{
    OracleFamilyKind, PhysicalCertificationEvidenceBundle, PhysicalProofOracleKind,
    S7CloseoutProofSummary, S7CloseoutProofTopology, S7CloseoutSourceDenial, SimulationReplayBundle,
};
#[cfg(any(test, feature = "certification-test-support"))]
use crate::s7_blob_harness::{execute_replay_artifacts_for_seed, BlobHarnessScenarioSeed};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S7ExecutedCloseoutSources {
    lifecycle_evidence: S7ExecutedLifecycleEvidenceBundle,
    evidence_bundle: PhysicalCertificationEvidenceBundle,
    proof_summary: S7CloseoutProofSummary,
}

#[cfg(any(test, feature = "certification-test-support"))]
pub fn s7_blob_harness_closeout_sources_for_seed(
    seed: BlobHarnessScenarioSeed,
) -> Result<S7ExecutedCloseoutSources, S7CloseoutSourceDenial> {
    let artifacts = execute_replay_artifacts_for_seed(seed);
    S7ExecutedCloseoutSources::from_replay_and_lifecycle(artifacts.replay, artifacts.lifecycle_evidence)
}

impl S7ExecutedCloseoutSources {
    pub fn from_replay_and_lifecycle(
        replay: SimulationReplayBundle,
        lifecycle_evidence: S7ExecutedLifecycleEvidenceBundle,
    ) -> Result<Self, S7CloseoutSourceDenial> {
        let evidence_bundle = PhysicalCertificationEvidenceBundle::from_replay_bundle(replay.clone())?;
        let proof_summary = proof_summary(&replay)?;
        Ok(Self {
            lifecycle_evidence,
            evidence_bundle,
            proof_summary,
        })
    }

    pub const fn lifecycle_evidence(&self) -> &S7ExecutedLifecycleEvidenceBundle {
        &self.lifecycle_evidence
    }

    pub const fn evidence_bundle(&self) -> &PhysicalCertificationEvidenceBundle {
        &self.evidence_bundle
    }

    pub const fn proof_summary(&self) -> S7CloseoutProofSummary {
        self.proof_summary
    }
}

fn proof_summary(
    replay: &SimulationReplayBundle,
) -> Result<S7CloseoutProofSummary, S7CloseoutSourceDenial> {
    let has_transcript_replay = replay
        .oracle_verdicts()
        .iter()
        .any(|verdict| verdict.oracle() == PhysicalProofOracleKind::TranscriptReplay);
    let has_blob_family = replay
        .oracle_verdicts()
        .iter()
        .any(|verdict| verdict.family() == OracleFamilyKind::S7BlobHarnessEvidence);
    let has_heavy_family = replay
        .oracle_verdicts()
        .iter()
        .any(|verdict| verdict.family() == OracleFamilyKind::S7BlobHeavyQualification);
    if !has_blob_family {
        return Err(S7CloseoutSourceDenial::MissingRequiredOracleFamily(
            OracleFamilyKind::S7BlobHarnessEvidence,
        ));
    }
    if !has_heavy_family {
        return Err(S7CloseoutSourceDenial::MissingRequiredOracleFamily(
            OracleFamilyKind::S7BlobHeavyQualification,
        ));
    }
    let trace = replay.trace();
    let observation = trace.s7_blob_harness_observation().expect("blob observation");
    if !observation.heavy_evidence_verified() {
        return Err(S7CloseoutSourceDenial::HeavyQualificationEvidenceMissing);
    }
    if !observation.heavy_cleanup_verified() {
        return Err(S7CloseoutSourceDenial::HeavyCleanupEvidenceMissing);
    }
    if !observation.heavy_pattern_lane_verified() {
        return Err(S7CloseoutSourceDenial::HeavyPatternLaneEvidenceMissing);
    }
    let topology = S7CloseoutProofTopology::new(true, has_transcript_replay, has_blob_family, has_heavy_family);
    Ok(S7CloseoutProofSummary::new(
        topology.checked_execution(),
        replay.oracle_verdicts().len(),
        replay.counter_receipt().rows().len(),
        topology,
    ))
}
