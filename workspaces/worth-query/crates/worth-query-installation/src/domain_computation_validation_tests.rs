use worth_foundational::facade::RetentionDeliveryProfile;

use crate::domain_computation_artifact_fixture::*;
use crate::facade::*;

#[test]
fn forbidden_identity_ownership_version_and_reconstruction_shapes_are_rejected() {
    let caller_digest = base_builder()
        .identity(WorthQueryArtifactContentIdentityContract::CallerDigestDefined)
        .compatibility(active_compatibility())
        .finish()
        .unwrap_err();
    assert_eq!(
        caller_digest.kind(),
        WorthQueryArtifactContractValidationDenialKind::CallerDigestIdentity
    );

    let unversioned_schema = base_builder_with_versions(
        WorthQueryArtifactSchemaVersion::new(0),
        WorthQueryArtifactProtocolVersion::new(1),
    )
    .compatibility(active_compatibility())
    .finish()
    .unwrap_err();
    assert_eq!(
        unversioned_schema.kind(),
        WorthQueryArtifactContractValidationDenialKind::UnversionedSchema
    );

    let unversioned_protocol = base_builder_with_versions(
        WorthQueryArtifactSchemaVersion::new(2),
        WorthQueryArtifactProtocolVersion::new(0),
    )
    .compatibility(active_compatibility())
    .finish()
    .unwrap_err();
    assert_eq!(
        unversioned_protocol.kind(),
        WorthQueryArtifactContractValidationDenialKind::UnversionedProtocol
    );

    let ambiguous_owner = base_builder()
        .ownership(WorthQueryArtifactOwnershipContract::NotDeclared)
        .compatibility(active_compatibility())
        .finish()
        .unwrap_err();
    assert_eq!(
        ambiguous_owner.kind(),
        WorthQueryArtifactContractValidationDenialKind::AmbiguousOwnership
    );

    let authoritative_reconstruction = base_builder()
        .lifecycle(WorthQueryArtifactLifecycleContract::ReconstructibleAsAuthoritative)
        .compatibility(active_compatibility())
        .finish()
        .unwrap_err();
    assert_eq!(
        authoritative_reconstruction.kind(),
        WorthQueryArtifactContractValidationDenialKind::DerivedReconstructionClaimsAuthority
    );
}

#[test]
fn reusable_noncomparable_artifacts_and_contradictory_carriage_are_rejected() {
    let missing_comparator = base_builder()
        .reproducibility(WorthQueryArtifactReproducibilityContract::new(
            WorthQueryArtifactReproducibilityClass::NonReplayable,
            WorthQueryArtifactDeterminismPosture::Nondeterministic,
            WorthQueryArtifactComparisonAuthority::NotComparable,
            std::iter::empty::<String>(),
            ["external-observation"],
        ))
        .lifecycle(WorthQueryArtifactLifecycleContract::Retained)
        .compatibility(active_compatibility())
        .finish()
        .unwrap_err();
    assert_eq!(
        missing_comparator.kind(),
        WorthQueryArtifactContractValidationDenialKind::MissingReusableComparator
    );

    let contradictory_carriage = base_builder()
        .carriage(WorthQueryArtifactCarriageContract::new(
            WorthQueryArtifactMovePosture::Forbidden,
            WorthQueryArtifactBorrowPosture::Forbidden,
            WorthQueryArtifactClonePosture::Forbidden,
            WorthQueryArtifactProviderTransferPosture::MoveOwnership,
            WorthQueryArtifactSerializationPosture::Forbidden,
        ))
        .compatibility(active_compatibility())
        .finish()
        .unwrap_err();
    assert_eq!(
        contradictory_carriage.kind(),
        WorthQueryArtifactContractValidationDenialKind::InvalidCarriageContract
    );

    let mismatched_clone_boundary = base_builder()
        .carriage(WorthQueryArtifactCarriageContract::new(
            WorthQueryArtifactMovePosture::Forbidden,
            WorthQueryArtifactBorrowPosture::Forbidden,
            WorthQueryArtifactClonePosture::Declared {
                mechanism: WorthQueryArtifactCloneMechanism::ProviderDefinedCopy,
                boundary: WorthQueryArtifactCloneBoundary::ProviderTransfer,
            },
            WorthQueryArtifactProviderTransferPosture::Forbidden,
            WorthQueryArtifactSerializationPosture::Forbidden,
        ))
        .compatibility(active_compatibility())
        .finish()
        .unwrap_err();
    assert_eq!(
        mismatched_clone_boundary.kind(),
        WorthQueryArtifactContractValidationDenialKind::InvalidCarriageContract
    );
}

#[test]
fn declared_clone_or_serialization_is_an_honest_carriage_path() {
    for carriage in [
        WorthQueryArtifactCarriageContract::new(
            WorthQueryArtifactMovePosture::Forbidden,
            WorthQueryArtifactBorrowPosture::Forbidden,
            WorthQueryArtifactClonePosture::Declared {
                mechanism: WorthQueryArtifactCloneMechanism::DeepClone,
                boundary: WorthQueryArtifactCloneBoundary::Isolation,
            },
            WorthQueryArtifactProviderTransferPosture::Forbidden,
            WorthQueryArtifactSerializationPosture::Forbidden,
        ),
        WorthQueryArtifactCarriageContract::new(
            WorthQueryArtifactMovePosture::Forbidden,
            WorthQueryArtifactBorrowPosture::Forbidden,
            WorthQueryArtifactClonePosture::Forbidden,
            WorthQueryArtifactProviderTransferPosture::Forbidden,
            WorthQueryArtifactSerializationPosture::DomainPayloadWithSchema,
        ),
    ] {
        base_builder()
            .carriage(carriage)
            .compatibility(active_compatibility())
            .finish()
            .unwrap();
    }
}

#[test]
fn structural_counter_stage_role_and_governance_dimensions_are_required() {
    let duplicate_counters = base_builder()
        .counters(WorthQueryStructuralCounterContract::required_foundation(
            counter("same"),
            counter("same"),
            counter("same"),
        ))
        .decisions(WorthQueryDecisionRecordContract::not_required())
        .compatibility(active_compatibility())
        .finish()
        .unwrap_err();
    assert_eq!(
        duplicate_counters.kind(),
        WorthQueryArtifactContractValidationDenialKind::InvalidStructuralCounterContract
    );

    let invalid_role = base_builder()
        .produced_by([""])
        .compatibility(active_compatibility())
        .finish()
        .unwrap_err();
    assert_eq!(
        invalid_role.kind(),
        WorthQueryArtifactContractValidationDenialKind::InvalidStageRole
    );

    let missing_audience = base_builder()
        .governance(WorthQueryArtifactGovernanceContract::new(
            std::iter::empty::<String>(),
            WorthQueryArtifactClassification::Internal,
            WorthQueryArtifactRedactionPosture::NotRequired,
            RetentionDeliveryProfile::Ephemeral,
            WorthQueryArtifactDeletionPosture::DeleteWithRun,
            WorthQueryArtifactLegalHoldPosture::NotEligible,
        ))
        .compatibility(active_compatibility())
        .finish()
        .unwrap_err();
    assert_eq!(
        missing_audience.kind(),
        WorthQueryArtifactContractValidationDenialKind::InvalidGovernanceContract
    );

    let non_portable_evidence_family = base_builder()
        .evidence(WorthQueryArtifactEvidenceContract::new(
            "basis family",
            "provenance",
            "dependency",
            "invalidation",
            "equivalence",
        ))
        .compatibility(active_compatibility())
        .finish()
        .unwrap_err();
    assert_eq!(
        non_portable_evidence_family.kind(),
        WorthQueryArtifactContractValidationDenialKind::InvalidSemanticEvidence
    );
}

fn counter(name: &str) -> worth_foundational::facade::FoundationalPerformanceCounterName {
    worth_foundational::facade::FoundationalPerformanceCounterName::new(name).unwrap()
}
