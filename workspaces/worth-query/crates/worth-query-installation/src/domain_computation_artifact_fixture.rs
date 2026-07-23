use worth_foundational::facade::{
    CanonicalizationRuleVersion, FoundationalPerformanceCounterName, RetentionDeliveryProfile,
};

use crate::facade::*;

pub(crate) struct CandidateArtifactFamily;

impl WorthQueryArtifactFamily for CandidateArtifactFamily {
    const SEMANTIC_FAMILY: &'static str = "worth.routing.candidates";
}

pub(crate) struct CandidateComparatorFamily;

impl WorthQueryArtifactComparatorFamily for CandidateComparatorFamily {
    const SEMANTIC_FAMILY: &'static str = "worth.routing.candidate-comparator";
}

pub(crate) fn valid_contract(
    reverse_order: bool,
    lifecycle: WorthQueryArtifactLifecycleContract,
    reproducibility: WorthQueryArtifactReproducibilityContract,
) -> WorthQueryPortableArtifactContract {
    let occurrence = if reverse_order {
        WorthQueryArtifactOccurrenceContract::independent_per_execution()
            .permit(WorthQueryArtifactSubstitutionPurpose::EvidentiarySubstitution)
            .permit(WorthQueryArtifactSubstitutionPurpose::ComputationalReuse)
    } else {
        WorthQueryArtifactOccurrenceContract::independent_per_execution()
            .permit(WorthQueryArtifactSubstitutionPurpose::ComputationalReuse)
            .permit(WorthQueryArtifactSubstitutionPurpose::EvidentiarySubstitution)
    };
    let (producers, consumers, audiences) = if reverse_order {
        (
            vec!["enumerate", "collect"],
            vec!["publish", "rank"],
            vec!["diagnostic", "workflow-internal"],
        )
    } else {
        (
            vec!["collect", "enumerate"],
            vec!["rank", "publish"],
            vec!["workflow-internal", "diagnostic"],
        )
    };
    WorthQueryPortableArtifactContract::declare::<CandidateArtifactFamily>(
        WorthQueryArtifactSchemaVersion::new(2),
        WorthQueryArtifactProtocolVersion::new(1),
    )
    .identity(
        WorthQueryArtifactContentIdentityContract::owner_canonical_projection(
            "worth.routing.candidate-projection",
            CanonicalizationRuleVersion::new("routing-candidates-v2").unwrap(),
        ),
    )
    .ownership(WorthQueryArtifactOwnershipContract::domain_payload(
        "worth.routing",
        "worth.routing.candidate-provider",
    ))
    .occurrence(occurrence)
    .evidence(WorthQueryArtifactEvidenceContract::new(
        "worth.routing.candidate-basis",
        "worth.routing.candidate-provenance",
        "worth.routing.candidate-dependency",
        "worth.routing.candidate-invalidation",
        "worth.routing.candidate-equivalence",
    ))
    .reproducibility(reproducibility)
    .search(WorthQueryCandidateSearchContract::declared(
        WorthQueryCandidateSearchEvidenceFamilies::new(
            "worth.routing.feasible-universe",
            "worth.routing.termination",
            "worth.routing.feasibility",
            "worth.routing.objective",
        ),
        WorthQueryCandidateSearchPosture::ProvenTopK { count: 10 },
        WorthQueryCandidateOptimalityPosture::ProvenTopK { count: 10 },
    ))
    .convergence(WorthQueryConvergenceContract::bounded(
        "worth.routing.progress",
        "worth.routing.convergence",
        100,
    ))
    .transformation(WorthQueryTransformationEvidenceContract::not_a_transformation())
    .carriage(WorthQueryArtifactCarriageContract::move_only_provider_transfer())
    .lifecycle(lifecycle)
    .counters(WorthQueryStructuralCounterContract::new(
        counter("artifact-bytes"),
        counter("candidate-elements"),
        counter("comparison-work"),
    ))
    .governance(WorthQueryArtifactGovernanceContract::new(
        audiences,
        WorthQueryArtifactClassification::Restricted,
        WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly,
        RetentionDeliveryProfile::Retained,
        WorthQueryArtifactDeletionPosture::DeleteAfterRetention,
        WorthQueryArtifactLegalHoldPosture::DomainControlled,
    ))
    .compatibility(WorthQueryArtifactCompatibilityContract::new(
        compatibility_window(),
        "worth.routing.migration",
        WorthQueryArtifactRetirementRule::Active,
        WorthQueryArtifactDowngradePosture::Denied,
    ))
    .produced_by(producers)
    .consumed_by(consumers)
    .finish()
    .unwrap()
}

pub(crate) fn exact_reproducibility() -> WorthQueryArtifactReproducibilityContract {
    WorthQueryArtifactReproducibilityContract::new(
        WorthQueryArtifactReproducibilityClass::ExactDeterministic,
        WorthQueryArtifactDeterminismPosture::Deterministic,
        WorthQueryArtifactComparisonAuthority::ExactCanonicalValue,
        std::iter::empty::<String>(),
        std::iter::empty::<String>(),
    )
}

pub(crate) fn base_builder() -> WorthQueryPortableArtifactContractBuilder {
    base_builder_with_versions(
        WorthQueryArtifactSchemaVersion::new(2),
        WorthQueryArtifactProtocolVersion::new(1),
    )
}

pub(crate) fn base_builder_with_versions(
    schema_version: WorthQueryArtifactSchemaVersion,
    protocol_version: WorthQueryArtifactProtocolVersion,
) -> WorthQueryPortableArtifactContractBuilder {
    WorthQueryPortableArtifactContract::declare::<CandidateArtifactFamily>(
        schema_version,
        protocol_version,
    )
    .identity(
        WorthQueryArtifactContentIdentityContract::owner_canonical_projection(
            "worth.routing.candidate-projection",
            CanonicalizationRuleVersion::new("candidate-v2").unwrap(),
        ),
    )
    .ownership(WorthQueryArtifactOwnershipContract::domain_payload(
        "owner", "provider",
    ))
    .occurrence(WorthQueryArtifactOccurrenceContract::independent_per_execution())
    .evidence(WorthQueryArtifactEvidenceContract::new(
        "basis",
        "provenance",
        "dependency",
        "invalidation",
        "equivalence",
    ))
    .reproducibility(exact_reproducibility())
    .search(WorthQueryCandidateSearchContract::not_applicable())
    .convergence(WorthQueryConvergenceContract::NotIterative)
    .transformation(WorthQueryTransformationEvidenceContract::not_a_transformation())
    .carriage(WorthQueryArtifactCarriageContract::move_only_provider_transfer())
    .lifecycle(WorthQueryArtifactLifecycleContract::ArenaScoped)
    .counters(WorthQueryStructuralCounterContract::new(
        counter("bytes"),
        counter("elements"),
        counter("work"),
    ))
    .governance(WorthQueryArtifactGovernanceContract::new(
        ["internal"],
        WorthQueryArtifactClassification::Internal,
        WorthQueryArtifactRedactionPosture::NotRequired,
        RetentionDeliveryProfile::Ephemeral,
        WorthQueryArtifactDeletionPosture::DeleteWithRun,
        WorthQueryArtifactLegalHoldPosture::NotEligible,
    ))
    .produced_by(["producer"])
    .consumed_by(["consumer"])
}

pub(crate) fn active_compatibility() -> WorthQueryArtifactCompatibilityContract {
    WorthQueryArtifactCompatibilityContract::new(
        compatibility_window(),
        "migration",
        WorthQueryArtifactRetirementRule::Active,
        WorthQueryArtifactDowngradePosture::Denied,
    )
}

pub(crate) const fn compatibility_window() -> WorthQueryArtifactCompatibilityWindow {
    WorthQueryArtifactCompatibilityWindow::new(
        WorthQueryArtifactSchemaVersion::new(1),
        WorthQueryArtifactSchemaVersion::new(3),
        WorthQueryArtifactProtocolVersion::new(1),
        WorthQueryArtifactProtocolVersion::new(2),
    )
}

pub(crate) fn domain_reproducibility() -> WorthQueryArtifactReproducibilityContract {
    WorthQueryArtifactReproducibilityContract::domain_comparator(
        "worth.routing.candidate-comparator",
        ["routing-model", "solver-version"],
    )
}

pub(crate) fn package(
    contract: WorthQueryPortableArtifactContract,
) -> WorthQueryValidatedPortableDomainPackage {
    WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "worth.routing",
        1,
        0,
    ))
    .artifact_contract(contract)
    .validate()
    .unwrap()
}

pub(crate) fn admitted(
    contract: WorthQueryPortableArtifactContract,
) -> WorthQueryAdmittedPortableDomainPackage {
    WorthQueryInstallationAdmissionProfile::new("support-v1", "config-v1")
        .artifact_version::<CandidateArtifactFamily>(
            WorthQueryArtifactSchemaVersion::new(2),
            WorthQueryArtifactProtocolVersion::new(1),
            WorthQueryArtifactVersionSupport::Admitted,
        )
        .artifact_comparator::<CandidateComparatorFamily>(
            WorthQueryInstallationSupportStatus::Admitted,
        )
        .admit(package(contract))
        .unwrap()
}

fn counter(name: &str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).unwrap()
}
