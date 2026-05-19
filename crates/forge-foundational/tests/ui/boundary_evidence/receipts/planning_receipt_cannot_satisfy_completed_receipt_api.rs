use forge_foundational::{
    boundary_evidence, BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceReceiptBoundary,
    FoundationalCommitId, FoundationalCommitParentBasis, FoundationalCommitParentageLocator,
    FoundationalTransitionLocator,
};

fn main() {
    let provenance = boundary_evidence()
        .provenance()
        .historical(forge_foundational::FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(
            BoundaryArtifactLocator::new(BoundaryArtifactId::new(1), BoundaryArtifactField::Basis),
        ))
        .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::StaleRetained)
        .unwrap_success();

    let planning = boundary_evidence()
        .receipt()
        .planning(FoundationalBoundaryEvidenceReceiptBoundary::transition(
            FoundationalTransitionLocator::CommitParentage(FoundationalCommitParentageLocator::new(
                FoundationalCommitId::new(forge_foundational::BoundaryHandle::new(1)),
                FoundationalCommitParentBasis::new(forge_foundational::EquivalenceBasisId::new(1)),
            )),
        ))
        .with_provenance(provenance);

    let _ = planning.completed_boundary();
}

trait UnwrapSuccess<T> {
    fn unwrap_success(self) -> T;
}

impl<T, E> UnwrapSuccess<T> for forge_proof::TransitionOutcome<T, E> {
    fn unwrap_success(self) -> T {
        match self {
            forge_proof::TransitionOutcome::Success(value) => value,
            _ => panic!("expected success"),
        }
    }
}
