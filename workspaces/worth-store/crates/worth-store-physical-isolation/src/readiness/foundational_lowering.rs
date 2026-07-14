use worth_foundational::{
    boundary_evidence, BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceProvenanceArtifact,
    FoundationalBoundaryEvidenceReceiptBoundary, FoundationalBoundaryEvidenceSourceBasis,
};
use worth_proof::TransitionOutcome;

use super::PhysicalIsolationEntryIdentity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIsolationEntryFoundationalEvidence {
    executed_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    provenance: FoundationalBoundaryEvidenceProvenanceArtifact,
    source_basis: FoundationalBoundaryEvidenceSourceBasis,
    freshness_posture: FoundationalBoundaryEvidenceFreshnessPosture,
}

impl PhysicalIsolationEntryFoundationalEvidence {
    pub(crate) fn lower(identity: &PhysicalIsolationEntryIdentity) -> Self {
        let source_basis = source_basis(identity);
        let freshness_posture =
            FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay;
        let provenance = boundary_evidence()
            .provenance()
            .replay_derived(source_basis.clone())
            .with_freshness(freshness_posture)
            .success_or_panic("S.5 entry foundational provenance");
        let executed_receipt = boundary_evidence()
            .receipt()
            .execution(receipt_boundary(identity))
            .with_provenance(provenance.clone());
        Self {
            executed_receipt,
            provenance,
            source_basis,
            freshness_posture,
        }
    }

    pub const fn executed_receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.executed_receipt
    }

    pub const fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        &self.provenance
    }

    pub const fn source_basis(&self) -> &FoundationalBoundaryEvidenceSourceBasis {
        &self.source_basis
    }

    pub const fn freshness_posture(&self) -> FoundationalBoundaryEvidenceFreshnessPosture {
        self.freshness_posture
    }

    pub const fn is_store_physical_stability_authority(&self) -> bool {
        false
    }
}

fn source_basis(
    identity: &PhysicalIsolationEntryIdentity,
) -> FoundationalBoundaryEvidenceSourceBasis {
    FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(BoundaryArtifactLocator::new(
        BoundaryArtifactId::new(identity.boundary_artifact_id()),
        BoundaryArtifactField::Basis,
    ))
}

fn receipt_boundary(
    identity: &PhysicalIsolationEntryIdentity,
) -> FoundationalBoundaryEvidenceReceiptBoundary {
    FoundationalBoundaryEvidenceReceiptBoundary::boundary_artifact(BoundaryArtifactLocator::new(
        BoundaryArtifactId::new(identity.boundary_artifact_id()),
        BoundaryArtifactField::Payload,
    ))
}

trait PhysicalIsolationEntryOutcomeExt<T> {
    fn success_or_panic(self, context: &str) -> T;
}

impl<T, D, De, St, R, F> PhysicalIsolationEntryOutcomeExt<T>
    for TransitionOutcome<T, D, De, St, R, F>
where
    D: core::fmt::Debug,
    De: core::fmt::Debug,
    St: core::fmt::Debug,
    R: core::fmt::Debug,
    F: core::fmt::Debug,
{
    fn success_or_panic(self, context: &str) -> T {
        match self {
            TransitionOutcome::Success(value) => value,
            _ => panic!("{context}: expected success"),
        }
    }
}
