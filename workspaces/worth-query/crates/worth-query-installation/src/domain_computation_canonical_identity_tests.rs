use worth_foundational::facade::CanonicalizationRuleVersion;

use crate::domain_computation_artifact_fixture::*;
use crate::facade::*;

#[test]
fn equivalent_contract_order_converges_to_one_semantic_identity() {
    let canonical = valid_contract(
        false,
        WorthQueryArtifactLifecycleContract::Retained,
        domain_reproducibility(),
    );
    let reversed = valid_contract(
        true,
        WorthQueryArtifactLifecycleContract::Retained,
        WorthQueryArtifactReproducibilityContract::domain_comparator(
            "worth.routing.candidate-comparator",
            ["solver-version", "routing-model"],
        ),
    );
    let runtime = WorthQueryInstallationRuntimeIdentity::fresh();
    let canonical_index = WorthQueryInstalledPackageIndex::build(
        runtime.retained(),
        WorthQueryInstallationGeneration::initial(),
        [admitted(canonical.clone())],
    )
    .unwrap();
    let reversed_index = WorthQueryInstalledPackageIndex::build(
        runtime,
        WorthQueryInstallationGeneration::initial(),
        [admitted(reversed.clone())],
    )
    .unwrap();

    assert_eq!(canonical, reversed);
    assert_eq!(canonical.identity(), reversed.identity());
    assert_eq!(canonical_index.identity(), reversed_index.identity());
    assert_eq!(
        canonical_index
            .artifact_contract(
                "worth.routing",
                CandidateArtifactFamily::SEMANTIC_FAMILY,
                WorthQueryArtifactSchemaVersion::new(2),
                WorthQueryArtifactProtocolVersion::new(1),
            )
            .unwrap(),
        reversed_index
            .artifact_contract(
                "worth.routing",
                CandidateArtifactFamily::SEMANTIC_FAMILY,
                WorthQueryArtifactSchemaVersion::new(2),
                WorthQueryArtifactProtocolVersion::new(1),
            )
            .unwrap()
    );
}

#[test]
fn every_search_semantic_dimension_changes_contract_identity() {
    let search = |universe: &str, termination: &str, feasibility: &str, comparison: &str, count| {
        WorthQueryCandidateSearchContract::declared(
            WorthQueryCandidateSearchEvidenceFamilies::new(
                universe,
                termination,
                feasibility,
                comparison,
                "incumbent",
            ),
            WorthQueryCandidateSearchPosture::ProvenTopK { count },
            WorthQueryCandidateOptimalityPosture::ProvenTopK { count },
        )
    };
    let identity = |search| {
        base_builder()
            .search(search)
            .compatibility(active_compatibility())
            .finish()
            .unwrap()
            .identity()
            .as_str()
            .to_string()
    };
    let baseline = identity(search(
        "universe-a",
        "termination-a",
        "feasibility-a",
        "comparison-a",
        3,
    ));
    for drifted in [
        search(
            "universe-b",
            "termination-a",
            "feasibility-a",
            "comparison-a",
            3,
        ),
        search(
            "universe-a",
            "termination-b",
            "feasibility-a",
            "comparison-a",
            3,
        ),
        search(
            "universe-a",
            "termination-a",
            "feasibility-b",
            "comparison-a",
            3,
        ),
        search(
            "universe-a",
            "termination-a",
            "feasibility-a",
            "comparison-b",
            3,
        ),
        search(
            "universe-a",
            "termination-a",
            "feasibility-a",
            "comparison-a",
            4,
        ),
    ] {
        assert_ne!(baseline, identity(drifted));
    }

    let bounded_identity = |bound: &str| {
        identity(WorthQueryCandidateSearchContract::declared(
            WorthQueryCandidateSearchEvidenceFamilies::new(
                "universe",
                "termination",
                "feasibility",
                "comparison",
                "incumbent",
            ),
            WorthQueryCandidateSearchPosture::Bounded {
                bound_identity: bound.into(),
            },
            WorthQueryCandidateOptimalityPosture::BoundedGap {
                bound_identity: bound.into(),
            },
        ))
    };
    assert_ne!(bounded_identity("budget-10"), bounded_identity("budget-20"));
}

#[test]
fn canonical_projection_family_is_part_of_semantic_identity() {
    let identity = |projection: &str| {
        base_builder()
            .identity(
                WorthQueryArtifactContentIdentityContract::owner_canonical_projection(
                    projection,
                    CanonicalizationRuleVersion::new("candidate-v2").unwrap(),
                ),
            )
            .compatibility(active_compatibility())
            .finish()
            .unwrap()
            .identity()
            .as_str()
            .to_string()
    };

    assert_ne!(
        identity("worth.routing.projection-a"),
        identity("worth.routing.projection-b")
    );
}

#[test]
fn clone_semantic_boundary_is_part_of_contract_identity() {
    let identity = |boundary| {
        base_builder()
            .carriage(WorthQueryArtifactCarriageContract::new(
                WorthQueryArtifactMovePosture::Required,
                WorthQueryArtifactBorrowPosture::Forbidden,
                WorthQueryArtifactClonePosture::Declared {
                    mechanism: WorthQueryArtifactCloneMechanism::DeepClone,
                    boundary,
                },
                WorthQueryArtifactProviderTransferPosture::MoveOwnership,
                WorthQueryArtifactSerializationPosture::CanonicalProjectionOnly,
            ))
            .compatibility(active_compatibility())
            .finish()
            .unwrap()
            .identity()
            .as_str()
            .to_string()
    };

    assert_ne!(
        identity(WorthQueryArtifactCloneBoundary::Isolation),
        identity(WorthQueryArtifactCloneBoundary::Retry)
    );
}
