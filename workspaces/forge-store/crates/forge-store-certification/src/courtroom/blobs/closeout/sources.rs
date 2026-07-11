use forge_store_blob_chunks::ExecutedBlobLifecycleEvidenceBundle;

use super::{BlobCloseoutProofSummary, BlobCloseoutProofTopology, BlobCloseoutSourceDenial};
#[cfg(any(test, feature = "certification-test-support"))]
use forge_store_physical_certification::{
    blob_harness_replay_artifacts_for_certification, BlobHarnessScenarioSeed,
};
use forge_store_physical_certification::{
    OracleFamilyKind, PhysicalCertificationEvidenceBundle, PhysicalProofOracleKind,
    SimulationReplayBundle,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobCloseoutSources {
    lifecycle_evidence: ExecutedBlobLifecycleEvidenceBundle,
    evidence_bundle: PhysicalCertificationEvidenceBundle,
    proof_summary: BlobCloseoutProofSummary,
}

#[cfg(any(test, feature = "certification-test-support"))]
pub fn blob_harness_closeout_sources_for_certification(
    seed: BlobHarnessScenarioSeed,
) -> Result<BlobCloseoutSources, BlobCloseoutSourceDenial> {
    let artifacts = blob_harness_replay_artifacts_for_certification(seed);
    BlobCloseoutSources::from_replay_and_lifecycle(artifacts.replay, artifacts.lifecycle_evidence)
}

impl BlobCloseoutSources {
    pub fn from_replay_and_lifecycle(
        replay: SimulationReplayBundle,
        lifecycle_evidence: ExecutedBlobLifecycleEvidenceBundle,
    ) -> Result<Self, BlobCloseoutSourceDenial> {
        let evidence_bundle =
            PhysicalCertificationEvidenceBundle::from_replay_bundle(replay.clone())?;
        let proof_summary = proof_summary(&replay)?;
        Ok(Self {
            lifecycle_evidence,
            evidence_bundle,
            proof_summary,
        })
    }

    pub const fn lifecycle_evidence(&self) -> &ExecutedBlobLifecycleEvidenceBundle {
        &self.lifecycle_evidence
    }

    pub const fn evidence_bundle(&self) -> &PhysicalCertificationEvidenceBundle {
        &self.evidence_bundle
    }

    pub const fn proof_summary(&self) -> BlobCloseoutProofSummary {
        self.proof_summary
    }
}

fn proof_summary(
    replay: &SimulationReplayBundle,
) -> Result<BlobCloseoutProofSummary, BlobCloseoutSourceDenial> {
    let has_transcript_replay = replay
        .oracle_verdicts()
        .iter()
        .any(|verdict| verdict.oracle() == PhysicalProofOracleKind::TranscriptReplay);
    let has_blob_family = replay
        .oracle_verdicts()
        .iter()
        .any(|verdict| verdict.family() == OracleFamilyKind::BlobHarnessEvidence);
    let has_heavy_family = replay
        .oracle_verdicts()
        .iter()
        .any(|verdict| verdict.family() == OracleFamilyKind::BlobHeavyQualification);
    if !has_blob_family {
        return Err(BlobCloseoutSourceDenial::MissingRequiredOracleFamily(
            OracleFamilyKind::BlobHarnessEvidence,
        ));
    }
    if !has_heavy_family {
        return Err(BlobCloseoutSourceDenial::MissingRequiredOracleFamily(
            OracleFamilyKind::BlobHeavyQualification,
        ));
    }
    let trace = replay.trace();
    let observation = trace.blob_harness_observation().expect("blob observation");
    if !observation.heavy_evidence_verified() {
        return Err(BlobCloseoutSourceDenial::HeavyQualificationEvidenceMissing);
    }
    if !observation.heavy_cleanup_verified() {
        return Err(BlobCloseoutSourceDenial::HeavyCleanupEvidenceMissing);
    }
    if !observation.heavy_pattern_lane_verified() {
        return Err(BlobCloseoutSourceDenial::HeavyPatternLaneEvidenceMissing);
    }
    let topology = BlobCloseoutProofTopology::new(
        true,
        has_transcript_replay,
        has_blob_family,
        has_heavy_family,
    );
    Ok(BlobCloseoutProofSummary::new(
        topology.checked_execution(),
        replay.oracle_verdicts().len(),
        replay.counter_receipt().rows().len(),
        topology,
    ))
}
