use crate::foundational_evidence_support as evidence_support;

use worth_foundational::{BoundaryArtifactId, CanonicalDigestId};
use worth_store_recovery_physics::{
    CurrentBasisRecoveryEvidencePosture, FoundationalRecoveryEvidenceBundle,
    RecoveryCurrentBasisEvidence, RecoveryEvidenceDenial,
};

#[test]
fn current_basis_requires_store_authority_and_foundational_admission_or_readmission() {
    let source = evidence_support::verified_source();
    let bundle = FoundationalRecoveryEvidenceBundle::from_source(&source).unwrap();
    let current = RecoveryCurrentBasisEvidence::from_foundational_bundle(&bundle).unwrap();

    assert_eq!(current.handle().get(), BoundaryArtifactId::new(91).get());
    assert_eq!(current.epoch(), source.authority().epoch());
    assert!(current.admitted_foundational_bundle());
    assert_eq!(
        RecoveryCurrentBasisEvidence::admit(CurrentBasisRecoveryEvidencePosture::RawDigest(
            CanonicalDigestId::new([9; 32])
        ))
        .unwrap_err(),
        RecoveryEvidenceDenial::RawDigestCannotSatisfyCurrentBasis
    );
    assert_eq!(
        RecoveryCurrentBasisEvidence::admit(
            CurrentBasisRecoveryEvidencePosture::BoundaryBridgedStale
        )
        .unwrap_err(),
        RecoveryEvidenceDenial::BoundaryBridgedStaleFormRequiresReadmission
    );
}
