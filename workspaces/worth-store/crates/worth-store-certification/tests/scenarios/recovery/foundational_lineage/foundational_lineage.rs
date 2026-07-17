#[path = "../../../support/recovery/foundational_evidence_support/foundational_evidence_support.rs"]
mod evidence_support;

use worth_foundational::{
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceRuntimeAssumption,
    FoundationalBoundaryEvidenceRuntimeNonAssumption,
};
use worth_store_recovery_physics::{RecoveryEvidenceLineagePosture, RecoveryEvidenceLineageReport};

#[test]
fn recovery_lineage_materializes_distinct_provenance_postures() {
    let source = evidence_support::verifier_disagreement_source();
    let lineage = RecoveryEvidenceLineageReport::from_source(&source)
        .expect("recovery evidence lineage is admissible");

    for posture in [
        RecoveryEvidenceLineagePosture::ReplayDerived,
        RecoveryEvidenceLineagePosture::RestoredReadmitted,
        RecoveryEvidenceLineagePosture::ReconstructedEquivalence,
        RecoveryEvidenceLineagePosture::DirectContinuity,
        RecoveryEvidenceLineagePosture::RuntimeAssumption,
        RecoveryEvidenceLineagePosture::RuntimeNonAssumption,
    ] {
        assert!(lineage.postures().contains(&posture));
    }
    assert_eq!(
        lineage.replay_derived().subject().handle(),
        source.authority().handle()
    );
    assert_eq!(
        lineage.replay_derived().provenance().freshness_posture(),
        FoundationalBoundaryEvidenceFreshnessPosture::ReconstructedFromReplay
    );
    assert_eq!(
        lineage.restored().provenance().freshness_posture(),
        FoundationalBoundaryEvidenceFreshnessPosture::RestoredFromCheckpoint
    );
    assert_eq!(
        lineage.reconstructed().subject(),
        lineage.replay_derived().subject()
    );
    assert_eq!(
        lineage.direct_continuity().subject(),
        lineage.replay_derived().subject()
    );
    assert_eq!(
        lineage.runtime_assumption(),
        FoundationalBoundaryEvidenceRuntimeAssumption::ReadmissionRemainsExplicitAcrossTrustBoundaries
    );
    assert_eq!(
        lineage.runtime_non_assumption(),
        FoundationalBoundaryEvidenceRuntimeNonAssumption::ReplayDerivationUpgradesToAttestedContinuity
    );
}
