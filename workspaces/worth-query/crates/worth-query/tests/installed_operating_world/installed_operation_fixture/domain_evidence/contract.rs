use worth_foundational::facade::{
    CanonicalizationRuleVersion, FoundationalPerformanceCounterName, RetentionDeliveryProfile,
};
use worth_query::facade::domain;

use super::super::{canonical_bundle, semantic_closure, GeometryDomain};
use super::{EvidenceFamily, EvidenceRead};

pub(super) struct EvidenceArtifactFamily;

impl domain::WorthQueryArtifactFamily for EvidenceArtifactFamily {
    const SEMANTIC_FAMILY: &'static str = "WORTH.tests.domain-evidence";
}

pub(super) fn direct_package(
    redaction: domain::WorthQueryArtifactRedactionPosture,
) -> domain::WorthQueryDomainPackage<GeometryDomain> {
    let contract = evidence_contract(redaction);
    let mut semantics = semantic_closure(
        canonical_bundle("Vertex"),
        domain::WorthQuerySupportRequirement::Required,
        true,
    );
    semantics.evidence =
        domain::WorthQueryDomainEvidenceContract::installed_artifact(contract.reference());
    semantics.lowering = domain::WorthQueryOperationLoweringContract {
        family: "domain-evidence-read-v1".into(),
        deterministic: true,
    };
    let operation = domain::WorthQueryDomainOperationDefinition::<
        GeometryDomain,
        EvidenceRead,
        EvidenceFamily,
    >::new(
        domain::WorthQueryDomainOperationIdentity::new("evidence-read", 1),
        semantics,
    );
    domain::WorthQueryDomainPackage::declare(
        GeometryDomain,
        domain::WorthQueryDomainIdentityDeclaration::new(
            domain::WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            domain::WorthQueryDomainIdentityName::new("domain-evidence").unwrap(),
            domain::WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .operation(operation)
    .artifact_contract(contract)
}

pub(super) fn artifact_support() -> domain::WorthQueryArtifactInstallationSupport {
    domain::WorthQueryArtifactInstallationSupport::new().artifact_version::<EvidenceArtifactFamily>(
        domain::WorthQueryArtifactSchemaVersion::new(1),
        domain::WorthQueryArtifactProtocolVersion::new(1),
        domain::WorthQueryArtifactVersionSupport::Admitted,
    )
}

fn evidence_contract(
    redaction: domain::WorthQueryArtifactRedactionPosture,
) -> domain::WorthQueryPortableArtifactContract {
    domain::WorthQueryPortableArtifactContract::declare::<EvidenceArtifactFamily>(
        domain::WorthQueryArtifactSchemaVersion::new(1),
        domain::WorthQueryArtifactProtocolVersion::new(1),
    )
    .identity(
        domain::WorthQueryArtifactContentIdentityContract::owner_canonical_projection(
            "WORTH.tests.domain-evidence.projection",
            CanonicalizationRuleVersion::new("domain-evidence-v1").unwrap(),
        ),
    )
    .ownership(domain::WorthQueryArtifactOwnershipContract::domain_payload(
        "WORTH.tests.domain-evidence",
        "WORTH.tests.domain-evidence.provider",
    ))
    .occurrence(domain::WorthQueryArtifactOccurrenceContract::independent_per_execution())
    .evidence(domain::WorthQueryArtifactEvidenceContract::new(
        "domain-evidence-basis",
        "domain-evidence-provenance",
        "domain-evidence-dependency",
        "domain-evidence-invalidation",
        "domain-evidence-equivalence",
    ))
    .reproducibility(domain::WorthQueryArtifactReproducibilityContract::new(
        domain::WorthQueryArtifactReproducibilityClass::ExactDeterministic,
        domain::WorthQueryArtifactDeterminismPosture::Deterministic,
        domain::WorthQueryArtifactComparisonAuthority::ExactCanonicalValue,
        std::iter::empty::<String>(),
        std::iter::empty::<String>(),
    ))
    .search(domain::WorthQueryCandidateSearchContract::declared(
        domain::WorthQueryCandidateSearchEvidenceFamilies::new(
            "candidate-universe",
            "candidate-termination",
            "candidate-feasibility",
            "candidate-comparison",
            "candidate-incumbent",
        ),
        domain::WorthQueryCandidateSearchPosture::Sampled {
            sample_identity: "sample-v1".into(),
        },
        domain::WorthQueryCandidateOptimalityPosture::BestInDeclaredSample {
            sample_identity: "sample-v1".into(),
        },
    ))
    .convergence(domain::WorthQueryConvergenceContract::NotIterative)
    .transformation(domain::WorthQueryTransformationEvidenceContract::declared(
        domain::WorthQueryImmutableSourceOccurrenceContract::new("source-occurrence"),
        domain::WorthQueryTransformationIdentity::new("normalize-candidates", 1),
        domain::WorthQueryTransformationOutcomeContract::new(
            domain::WorthQuerySourceOutputCorrespondence::OneToMany,
            domain::WorthQueryTransformationDisposition::Normalized,
            domain::WorthQueryTransformationErrorPosture::Bounded,
            domain::WorthQueryTransformationLossPosture::DeclaredLossy,
        ),
    ))
    .access_path(domain::WorthQueryArtifactAccessPathContract::denied())
    .carriage(domain::WorthQueryArtifactCarriageContract::move_only_provider_transfer())
    .lifecycle(domain::WorthQueryArtifactLifecycleContract::Retained)
    .counters(counter_contract())
    .decisions(decision_contract())
    .governance(domain::WorthQueryArtifactGovernanceContract::new(
        ["audit", "support"],
        domain::WorthQueryArtifactClassification::Restricted,
        redaction,
        RetentionDeliveryProfile::Durable,
        domain::WorthQueryArtifactDeletionPosture::DomainControlled,
        domain::WorthQueryArtifactLegalHoldPosture::RequiredWhenDirected,
    ))
    .compatibility(domain::WorthQueryArtifactCompatibilityContract::new(
        domain::WorthQueryArtifactCompatibilityWindow::new(
            domain::WorthQueryArtifactSchemaVersion::new(1),
            domain::WorthQueryArtifactSchemaVersion::new(1),
            domain::WorthQueryArtifactProtocolVersion::new(1),
            domain::WorthQueryArtifactProtocolVersion::new(1),
        ),
        "WORTH.tests.domain-evidence.migration",
        domain::WorthQueryArtifactRetirementRule::Active,
        domain::WorthQueryArtifactDowngradePosture::Denied,
    ))
    .produced_by(["evidence-read:1", "evidence"])
    .consumed_by(["audit"])
    .finish()
    .unwrap()
}

fn counter_contract() -> domain::WorthQueryStructuralCounterContract {
    domain::WorthQueryStructuralCounterContract::declare([
        counter_schema(
            "bytes",
            domain::WorthQueryStructuralCounterRole::Bytes,
            domain::WorthQueryStructuralCounterUnit::Bytes,
            domain::WorthQueryStructuralCounterAggregation::Independent,
            domain::WorthQueryStructuralCounterScope::ArtifactOccurrence,
            domain::WorthQueryStructuralCounterResetBoundary::ArtifactOccurrence,
            domain::WorthQueryStructuralCounterReplayPosture::Exact,
        ),
        counter_schema(
            "elements",
            domain::WorthQueryStructuralCounterRole::Elements,
            domain::WorthQueryStructuralCounterUnit::Elements,
            domain::WorthQueryStructuralCounterAggregation::Independent,
            domain::WorthQueryStructuralCounterScope::ArtifactOccurrence,
            domain::WorthQueryStructuralCounterResetBoundary::ArtifactOccurrence,
            domain::WorthQueryStructuralCounterReplayPosture::Exact,
        ),
        counter_schema(
            "work",
            domain::WorthQueryStructuralCounterRole::StructuralWork,
            domain::WorthQueryStructuralCounterUnit::Operations,
            domain::WorthQueryStructuralCounterAggregation::SumOf(vec![
                counter("elements"),
                counter("candidate-comparisons"),
            ]),
            domain::WorthQueryStructuralCounterScope::ArtifactOccurrence,
            domain::WorthQueryStructuralCounterResetBoundary::ArtifactOccurrence,
            domain::WorthQueryStructuralCounterReplayPosture::Exact,
        ),
        counter_schema(
            "candidate-comparisons",
            domain::WorthQueryStructuralCounterRole::DomainWork,
            domain::WorthQueryStructuralCounterUnit::Comparisons,
            domain::WorthQueryStructuralCounterAggregation::Independent,
            domain::WorthQueryStructuralCounterScope::Operation,
            domain::WorthQueryStructuralCounterResetBoundary::Operation,
            domain::WorthQueryStructuralCounterReplayPosture::NonDecreasing,
        ),
    ])
}

fn counter_schema(
    name: &str,
    role: domain::WorthQueryStructuralCounterRole,
    unit: domain::WorthQueryStructuralCounterUnit,
    aggregation: domain::WorthQueryStructuralCounterAggregation,
    scope: domain::WorthQueryStructuralCounterScope,
    reset: domain::WorthQueryStructuralCounterResetBoundary,
    replay: domain::WorthQueryStructuralCounterReplayPosture,
) -> domain::WorthQueryStructuralCounterSchema {
    domain::WorthQueryStructuralCounterSchema::new(
        counter(name),
        role,
        unit,
        aggregation,
        domain::WorthQueryStructuralCounterMonotonicity::NonDecreasing,
        scope,
        reset,
        domain::WorthQueryStructuralCounterRequiredness::RequiredCore,
        replay,
    )
}

fn decision_contract() -> domain::WorthQueryDecisionRecordContract {
    domain::WorthQueryDecisionRecordContract::declared([domain::WorthQueryDecisionSchema::new(
        domain::WorthQueryDecisionIdentity::new(
            domain::WorthQueryDecisionKind::new("candidate-rejected").unwrap(),
            domain::WorthQueryDecisionReasonFamily::new("search-reason").unwrap(),
            domain::WorthQueryArtifactKeyFamily::new("candidate").unwrap(),
        ),
        domain::WorthQueryDecisionCausalParentShape::RequiredSingle,
        domain::WorthQueryDecisionPayloadVersion::new(1),
        domain::WorthQueryDecisionGovernance::new(
            domain::WorthQueryArtifactClassification::Restricted,
            RetentionDeliveryProfile::Durable,
        ),
    )])
}

fn counter(name: &str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).unwrap()
}
