use worth_foundational::facade::{
    CanonicalizationRuleVersion, FoundationalPerformanceCounterName, RetentionDeliveryProfile,
};
use worth_query_installation::facade::{
    WorthQueryArtifactBorrowPosture, WorthQueryArtifactCarriageContract,
    WorthQueryArtifactClassification, WorthQueryArtifactClonePosture,
    WorthQueryArtifactComparatorFamily, WorthQueryArtifactCompatibilityContract,
    WorthQueryArtifactCompatibilityWindow, WorthQueryArtifactContentIdentityContract,
    WorthQueryArtifactDeletionPosture, WorthQueryArtifactDowngradePosture,
    WorthQueryArtifactEvidenceContract, WorthQueryArtifactGovernanceContract,
    WorthQueryArtifactLegalHoldPosture, WorthQueryArtifactLifecycleContract,
    WorthQueryArtifactMovePosture, WorthQueryArtifactOccurrenceContract,
    WorthQueryArtifactOwnershipContract, WorthQueryArtifactProtocolVersion,
    WorthQueryArtifactProviderTransferPosture, WorthQueryArtifactRedactionPosture,
    WorthQueryArtifactReproducibilityContract, WorthQueryArtifactRetirementRule,
    WorthQueryArtifactSchemaVersion, WorthQueryArtifactSerializationPosture,
    WorthQueryCandidateOptimalityPosture, WorthQueryCandidateSearchContract,
    WorthQueryCandidateSearchEvidenceFamilies, WorthQueryCandidateSearchPosture,
    WorthQueryConvergenceContract, WorthQueryConvergenceIncumbentPosture,
    WorthQueryConvergenceOscillationPosture, WorthQueryDecisionRecordContract,
    WorthQueryPortableArtifactContract, WorthQueryStructuralCounterContract,
    WorthQueryTransformationEvidenceContract,
};

use super::fixture_identity::{CandidateFamily, ComparatorFamily, OWNER};

#[derive(Clone, Copy)]
pub(crate) enum FixtureConvergenceContract {
    Bounded,
    Pareto,
    OscillationImpossible,
    OscillationSelectIncumbent,
    OscillationDomainClassified,
    NonIterative,
    MissingSearch,
}

pub(super) fn candidate_contract(
    producer: &str,
    fixture: FixtureConvergenceContract,
) -> WorthQueryPortableArtifactContract {
    let search = match fixture {
        FixtureConvergenceContract::MissingSearch => {
            WorthQueryCandidateSearchContract::not_applicable()
        }
        FixtureConvergenceContract::Bounded
        | FixtureConvergenceContract::Pareto
        | FixtureConvergenceContract::OscillationImpossible
        | FixtureConvergenceContract::OscillationSelectIncumbent
        | FixtureConvergenceContract::OscillationDomainClassified
        | FixtureConvergenceContract::NonIterative => WorthQueryCandidateSearchContract::declared(
            WorthQueryCandidateSearchEvidenceFamilies::new(
                "worth.convergence.universe",
                "worth.convergence.termination",
                "worth.convergence.feasibility",
                "worth.convergence.comparison",
                "worth.convergence.incumbent",
            ),
            WorthQueryCandidateSearchPosture::Heuristic,
            WorthQueryCandidateOptimalityPosture::Unknown,
        ),
    };
    let convergence = match fixture {
        FixtureConvergenceContract::NonIterative => WorthQueryConvergenceContract::NotIterative,
        FixtureConvergenceContract::Pareto => WorthQueryConvergenceContract::Iterative {
            progress_measure_family: "worth.convergence.progress".into(),
            comparator_family: ComparatorFamily::SEMANTIC_FAMILY.into(),
            repeated_state_family: "worth.convergence.repeated-state".into(),
            incumbent: WorthQueryConvergenceIncumbentPosture::ParetoFrontier,
            iteration_bound: 3,
            oscillation: WorthQueryConvergenceOscillationPosture::DetectAndDeny,
        },
        FixtureConvergenceContract::OscillationImpossible => {
            iterative_contract(WorthQueryConvergenceOscillationPosture::Impossible)
        }
        FixtureConvergenceContract::OscillationSelectIncumbent => {
            iterative_contract(WorthQueryConvergenceOscillationPosture::DetectAndSelectIncumbent)
        }
        FixtureConvergenceContract::OscillationDomainClassified => {
            iterative_contract(WorthQueryConvergenceOscillationPosture::DomainClassified)
        }
        FixtureConvergenceContract::Bounded | FixtureConvergenceContract::MissingSearch => {
            WorthQueryConvergenceContract::bounded(
                "worth.convergence.progress",
                ComparatorFamily::SEMANTIC_FAMILY,
                "worth.convergence.repeated-state",
                3,
            )
        }
    };
    WorthQueryPortableArtifactContract::declare::<CandidateFamily>(
        WorthQueryArtifactSchemaVersion::new(1),
        WorthQueryArtifactProtocolVersion::new(1),
    )
    .identity(
        WorthQueryArtifactContentIdentityContract::owner_canonical_projection(
            "worth.convergence.candidate-projection",
            CanonicalizationRuleVersion::new("convergence-candidate-v1").unwrap(),
        ),
    )
    .ownership(WorthQueryArtifactOwnershipContract::domain_payload(
        OWNER,
        "worth.convergence.provider",
    ))
    .occurrence(WorthQueryArtifactOccurrenceContract::independent_per_execution())
    .evidence(WorthQueryArtifactEvidenceContract::new(
        "worth.convergence.basis",
        "worth.convergence.provenance",
        "worth.convergence.dependency",
        "worth.convergence.invalidation",
        "worth.convergence.equivalence",
    ))
    .reproducibility(
        WorthQueryArtifactReproducibilityContract::domain_comparator(
            ComparatorFamily::SEMANTIC_FAMILY,
            ["fixture-model"],
        ),
    )
    .search(search)
    .convergence(convergence)
    .transformation(WorthQueryTransformationEvidenceContract::not_a_transformation())
    .access_path(worth_query_installation::facade::WorthQueryArtifactAccessPathContract::denied())
    .carriage(WorthQueryArtifactCarriageContract::new(
        WorthQueryArtifactMovePosture::Required,
        WorthQueryArtifactBorrowPosture::SharedReadOnly,
        WorthQueryArtifactClonePosture::Forbidden,
        WorthQueryArtifactProviderTransferPosture::MoveOwnership,
        WorthQueryArtifactSerializationPosture::CanonicalProjectionOnly,
    ))
    .lifecycle(WorthQueryArtifactLifecycleContract::ArenaScoped)
    .counters(WorthQueryStructuralCounterContract::required_foundation(
        counter("candidate-bytes"),
        counter("candidate-elements"),
        counter("comparison-work"),
    ))
    .decisions(WorthQueryDecisionRecordContract::not_required())
    .governance(WorthQueryArtifactGovernanceContract::new(
        ["workflow-internal"],
        WorthQueryArtifactClassification::Internal,
        WorthQueryArtifactRedactionPosture::NotRequired,
        RetentionDeliveryProfile::Ephemeral,
        WorthQueryArtifactDeletionPosture::DeleteWithRun,
        WorthQueryArtifactLegalHoldPosture::NotEligible,
    ))
    .compatibility(WorthQueryArtifactCompatibilityContract::new(
        WorthQueryArtifactCompatibilityWindow::new(
            WorthQueryArtifactSchemaVersion::new(1),
            WorthQueryArtifactSchemaVersion::new(1),
            WorthQueryArtifactProtocolVersion::new(1),
            WorthQueryArtifactProtocolVersion::new(1),
        ),
        "worth.convergence.migration",
        WorthQueryArtifactRetirementRule::Active,
        WorthQueryArtifactDowngradePosture::Denied,
    ))
    .produced_by([producer])
    .consumed_by(["convergence-runtime"])
    .finish()
    .expect("convergence fixture artifact must validate")
}

fn iterative_contract(
    oscillation: WorthQueryConvergenceOscillationPosture,
) -> WorthQueryConvergenceContract {
    WorthQueryConvergenceContract::Iterative {
        progress_measure_family: "worth.convergence.progress".into(),
        comparator_family: ComparatorFamily::SEMANTIC_FAMILY.into(),
        repeated_state_family: "worth.convergence.repeated-state".into(),
        incumbent: WorthQueryConvergenceIncumbentPosture::BestObserved,
        iteration_bound: 3,
        oscillation,
    }
}

fn counter(name: &str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).unwrap()
}
