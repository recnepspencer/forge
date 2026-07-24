use worth_foundational::facade::{
    CanonicalizationRuleVersion, FoundationalPerformanceCounterName, RetentionDeliveryProfile,
};
use worth_query::facade::domain;

pub struct CandidateArtifactFamily;

impl domain::WorthQueryArtifactFamily for CandidateArtifactFamily {
    const SEMANTIC_FAMILY: &'static str = "WORTH.tests.artifact-workflow.candidates";
}

pub fn candidate_contract() -> domain::WorthQueryPortableArtifactContract {
    domain::WorthQueryPortableArtifactContract::declare::<CandidateArtifactFamily>(
        domain::WorthQueryArtifactSchemaVersion::new(1),
        domain::WorthQueryArtifactProtocolVersion::new(1),
    )
    .identity(
        domain::WorthQueryArtifactContentIdentityContract::owner_canonical_projection(
            "WORTH.tests.artifact-workflow.projection",
            CanonicalizationRuleVersion::new("artifact-workflow-v1").unwrap(),
        ),
    )
    .ownership(domain::WorthQueryArtifactOwnershipContract::domain_payload(
        "WORTH.tests.geometry",
        "WORTH.tests.artifact-workflow.provider",
    ))
    .occurrence(domain::WorthQueryArtifactOccurrenceContract::independent_per_execution())
    .evidence(domain::WorthQueryArtifactEvidenceContract::new(
        "artifact-workflow-basis",
        "artifact-workflow-provenance",
        "artifact-workflow-dependency",
        "artifact-workflow-invalidation",
        "artifact-workflow-equivalence",
    ))
    .reproducibility(domain::WorthQueryArtifactReproducibilityContract::new(
        domain::WorthQueryArtifactReproducibilityClass::ExactDeterministic,
        domain::WorthQueryArtifactDeterminismPosture::Deterministic,
        domain::WorthQueryArtifactComparisonAuthority::ExactCanonicalValue,
        std::iter::empty::<String>(),
        std::iter::empty::<String>(),
    ))
    .search(domain::WorthQueryCandidateSearchContract::not_applicable())
    .convergence(domain::WorthQueryConvergenceContract::NotIterative)
    .transformation(domain::WorthQueryTransformationEvidenceContract::not_a_transformation())
    .carriage(domain::WorthQueryArtifactCarriageContract::new(
        domain::WorthQueryArtifactMovePosture::Required,
        domain::WorthQueryArtifactBorrowPosture::SharedReadOnly,
        domain::WorthQueryArtifactClonePosture::Forbidden,
        domain::WorthQueryArtifactProviderTransferPosture::MoveOwnership,
        domain::WorthQueryArtifactSerializationPosture::CanonicalProjectionOnly,
    ))
    .lifecycle(domain::WorthQueryArtifactLifecycleContract::Retained)
    .counters(domain::WorthQueryStructuralCounterContract::new(
        counter("artifact-bytes"),
        counter("artifact-elements"),
        counter("artifact-work"),
    ))
    .governance(domain::WorthQueryArtifactGovernanceContract::new(
        ["workflow-internal"],
        domain::WorthQueryArtifactClassification::Internal,
        domain::WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly,
        RetentionDeliveryProfile::Ephemeral,
        domain::WorthQueryArtifactDeletionPosture::DeleteWithRun,
        domain::WorthQueryArtifactLegalHoldPosture::NotEligible,
    ))
    .compatibility(domain::WorthQueryArtifactCompatibilityContract::new(
        domain::WorthQueryArtifactCompatibilityWindow::new(
            domain::WorthQueryArtifactSchemaVersion::new(1),
            domain::WorthQueryArtifactSchemaVersion::new(1),
            domain::WorthQueryArtifactProtocolVersion::new(1),
            domain::WorthQueryArtifactProtocolVersion::new(1),
        ),
        "WORTH.tests.artifact-workflow.migration",
        domain::WorthQueryArtifactRetirementRule::Active,
        domain::WorthQueryArtifactDowngradePosture::Denied,
    ))
    .produced_by(["produce"])
    .consumed_by(["consume", "observe-a", "observe-b"])
    .finish()
    .unwrap()
}

pub fn artifact_support() -> domain::WorthQueryArtifactInstallationSupport {
    domain::WorthQueryArtifactInstallationSupport::new()
        .artifact_version::<CandidateArtifactFamily>(
            domain::WorthQueryArtifactSchemaVersion::new(1),
            domain::WorthQueryArtifactProtocolVersion::new(1),
            domain::WorthQueryArtifactVersionSupport::Admitted,
        )
}

fn counter(name: &str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).unwrap()
}
