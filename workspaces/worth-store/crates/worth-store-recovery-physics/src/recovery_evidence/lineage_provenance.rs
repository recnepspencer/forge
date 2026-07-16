use worth_foundational::{
    boundary_evidence, BoundaryArtifactField, BoundaryArtifactLocator,
    FoundationalBoundaryEvidenceAttestedLineageArtifact,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceLineageSubject,
    FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact,
    FoundationalBoundaryEvidenceReplayDerivedLineageArtifact,
    FoundationalBoundaryEvidenceRestoredLineageArtifact,
    FoundationalBoundaryEvidenceRuntimeAssumption,
    FoundationalBoundaryEvidenceRuntimeNonAssumption, FoundationalBoundaryEvidenceSourceBasis,
};

use super::{
    denial::RecoveryEvidenceDenial, executed_evidence_source::RecoveryPhysicsEvidenceSource,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RecoveryEvidenceLineagePosture {
    ReplayDerived,
    RestoredReadmitted,
    ReconstructedEquivalence,
    DirectContinuity,
    RuntimeAssumption,
    RuntimeNonAssumption,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryEvidenceLineageReport {
    postures: Vec<RecoveryEvidenceLineagePosture>,
    replay_derived: FoundationalBoundaryEvidenceReplayDerivedLineageArtifact,
    restored: FoundationalBoundaryEvidenceRestoredLineageArtifact,
    reconstructed: FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact,
    direct_continuity: FoundationalBoundaryEvidenceAttestedLineageArtifact,
    runtime_assumption: FoundationalBoundaryEvidenceRuntimeAssumption,
    runtime_non_assumption: FoundationalBoundaryEvidenceRuntimeNonAssumption,
}

impl RecoveryEvidenceLineageReport {
    pub fn from_source(
        source: &RecoveryPhysicsEvidenceSource,
    ) -> Result<Self, RecoveryEvidenceDenial> {
        Ok(Self {
            postures: vec![
                RecoveryEvidenceLineagePosture::ReplayDerived,
                RecoveryEvidenceLineagePosture::RestoredReadmitted,
                RecoveryEvidenceLineagePosture::ReconstructedEquivalence,
                RecoveryEvidenceLineagePosture::DirectContinuity,
                RecoveryEvidenceLineagePosture::RuntimeAssumption,
                RecoveryEvidenceLineagePosture::RuntimeNonAssumption,
            ],
            replay_derived: replay_derived_lineage(source)?,
            restored: restored_lineage(source)?,
            reconstructed: reconstructed_equivalence(source)?,
            direct_continuity: direct_continuity(source)?,
            runtime_assumption:
                FoundationalBoundaryEvidenceRuntimeAssumption::ReadmissionRemainsExplicitAcrossTrustBoundaries,
            runtime_non_assumption:
                FoundationalBoundaryEvidenceRuntimeNonAssumption::ReplayDerivationUpgradesToAttestedContinuity,
        })
    }

    pub fn postures(&self) -> &[RecoveryEvidenceLineagePosture] {
        &self.postures
    }

    pub const fn replay_derived(
        &self,
    ) -> &FoundationalBoundaryEvidenceReplayDerivedLineageArtifact {
        &self.replay_derived
    }

    pub const fn restored(&self) -> &FoundationalBoundaryEvidenceRestoredLineageArtifact {
        &self.restored
    }

    pub const fn reconstructed(
        &self,
    ) -> &FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact {
        &self.reconstructed
    }

    pub const fn direct_continuity(&self) -> &FoundationalBoundaryEvidenceAttestedLineageArtifact {
        &self.direct_continuity
    }

    pub const fn runtime_assumption(&self) -> FoundationalBoundaryEvidenceRuntimeAssumption {
        self.runtime_assumption
    }

    pub const fn runtime_non_assumption(&self) -> FoundationalBoundaryEvidenceRuntimeNonAssumption {
        self.runtime_non_assumption
    }
}

fn replay_derived_lineage(
    source: &RecoveryPhysicsEvidenceSource,
) -> Result<FoundationalBoundaryEvidenceReplayDerivedLineageArtifact, RecoveryEvidenceDenial> {
    boundary_evidence()
        .lineage()
        .replay_derived_continuity(subject(source))
        .with_provenance(replay_provenance(source)?)
        .into_result()
        .map_err(RecoveryEvidenceDenial::from)
}

fn restored_lineage(
    source: &RecoveryPhysicsEvidenceSource,
) -> Result<FoundationalBoundaryEvidenceRestoredLineageArtifact, RecoveryEvidenceDenial> {
    boundary_evidence()
        .lineage()
        .restored_continuity(subject(source))
        .attested_by(
            boundary_evidence()
                .receipt()
                .restoration(receipt_boundary(source))
                .with_provenance(restored_provenance(source)?),
        )
        .into_result()
        .map_err(RecoveryEvidenceDenial::from)
}

fn reconstructed_equivalence(
    source: &RecoveryPhysicsEvidenceSource,
) -> Result<FoundationalBoundaryEvidenceReconstructedEquivalenceArtifact, RecoveryEvidenceDenial> {
    boundary_evidence()
        .lineage()
        .reconstructed_equivalence(subject(source))
        .with_provenance(replay_provenance(source)?)
        .into_result()
        .map_err(RecoveryEvidenceDenial::from)
}

fn direct_continuity(
    source: &RecoveryPhysicsEvidenceSource,
) -> Result<FoundationalBoundaryEvidenceAttestedLineageArtifact, RecoveryEvidenceDenial> {
    Ok(boundary_evidence()
        .lineage()
        .continuity(subject(source))
        .attested_by(
            boundary_evidence()
                .receipt()
                .execution(receipt_boundary(source))
                .with_provenance(replay_provenance(source)?),
        ))
}

fn replay_provenance(
    source: &RecoveryPhysicsEvidenceSource,
) -> Result<FoundationalBoundaryEvidenceProvenanceArtifact, RecoveryEvidenceDenial> {
    boundary_evidence()
        .provenance()
        .replay_derived(source_basis(source))
        .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay)
        .into_result()
        .map_err(RecoveryEvidenceDenial::from)
}

fn restored_provenance(
    source: &RecoveryPhysicsEvidenceSource,
) -> Result<FoundationalBoundaryEvidenceProvenanceArtifact, RecoveryEvidenceDenial> {
    boundary_evidence()
        .provenance()
        .restored_readmitted(source_basis(source))
        .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::RestoredFromCheckpoint)
        .into_result()
        .map_err(RecoveryEvidenceDenial::from)
}

fn source_basis(source: &RecoveryPhysicsEvidenceSource) -> FoundationalBoundaryEvidenceSourceBasis {
    FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(source_basis_locator(source))
}

fn receipt_boundary(
    source: &RecoveryPhysicsEvidenceSource,
) -> worth_foundational::FoundationalBoundaryEvidenceReceiptBoundary {
    worth_foundational::FoundationalBoundaryEvidenceReceiptBoundary::boundary_artifact(
        source.artifact_locator(),
    )
}

fn source_basis_locator(source: &RecoveryPhysicsEvidenceSource) -> BoundaryArtifactLocator {
    BoundaryArtifactLocator::new(
        source.artifact_locator().artifact_id(),
        BoundaryArtifactField::Basis,
    )
}

fn subject(source: &RecoveryPhysicsEvidenceSource) -> FoundationalBoundaryEvidenceLineageSubject {
    FoundationalBoundaryEvidenceLineageSubject::new(source.authority().handle())
}
