use forge_foundational::{
    boundary_evidence, BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator,
    FoundationalBoundaryEvidenceFreshnessPosture,
    FoundationalBoundaryEvidenceSourceBasis, FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceReceiptBoundary, FoundationalCommitId,
    FoundationalCommitParentBasis, FoundationalCommitParentageLocator, BoundaryHandle,
    FoundationalTransitionLocator,
};

fn main() {
    let _ = boundary_evidence()
        .support()
        .published_evidence()
        .with_basis_disclosure(FoundationalBoundaryEvidenceSupportBasisDisclosure::StaleBasis)
        .attested_by(
            boundary_evidence()
                .receipt()
                .blocked_closeout(FoundationalBoundaryEvidenceReceiptBoundary::transition(
                    FoundationalTransitionLocator::CommitParentage(FoundationalCommitParentageLocator::new(
                        FoundationalCommitId::new(BoundaryHandle::new(50)),
                        FoundationalCommitParentBasis::new(forge_foundational::EquivalenceBasisId::new(51)),
                    )),
                ))
                .with_provenance(
                    boundary_evidence()
                        .provenance()
                        .historical(FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(
                            BoundaryArtifactLocator::new(BoundaryArtifactId::new(5), BoundaryArtifactField::Basis),
                        ))
                        .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::StaleRetained)
                        .unwrap_success(),
                ),
        );
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
