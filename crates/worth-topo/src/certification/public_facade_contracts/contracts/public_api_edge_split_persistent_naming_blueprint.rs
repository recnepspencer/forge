#[test]
fn edge_split_persistent_naming_rows_are_publicly_registered_as_query_native_surfaces() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();

    for operator_name in [
        "BuildSplitPersistentNamingMap",
        "BuildSplitPersistentNamingSeeds",
        "AdmitSplitIdentityEvolutionQuery",
        "BindSplitPersistentNamesToQueryLineage",
    ] {
        let row = edge_split_operator_row(&blueprint, operator_name);
        assert_eq!(
            row.classification(),
            EdgeSplitOperatorClassification::QueryGraphCompositionProgram
        );
        assert_eq!(
            row.required_query_surface(),
            EdgeSplitRequiredQuerySurface::QueryGraphComposition
        );
    }

    for operator_name in [
        "PropagatePersistentNamesThroughSplit",
        "RecordSplitEntityParentage",
        "ForkSplitEntityLineage",
    ] {
        let row = edge_split_operator_row(&blueprint, operator_name);
        assert_eq!(
            row.classification(),
            EdgeSplitOperatorClassification::TopologyContributionWorkflow
        );
        assert_eq!(
            row.required_query_surface(),
            EdgeSplitRequiredQuerySurface::TopologyContributionWorkflow
        );
    }

    let signatures = edge_split_operator_row(&blueprint, "ExtractSplitStableSubshapeSignatures");
    assert_eq!(
        signatures.classification(),
        EdgeSplitOperatorClassification::PreparedSpatialOnly
    );
    assert_eq!(
        signatures.required_query_surface(),
        EdgeSplitRequiredQuerySurface::None
    );
    assert!(!signatures.may_commit_topology_in_7_3());
}

#[test]
fn edge_split_persistent_naming_validators_are_publicly_registered_as_query_invariants() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();

    for operator_name in [
        "ResolveSplitNameConflictsAfterBoolean",
        "ValidateSplitNameSurvival",
        "ValidateSplitPersistentNameUniqueness",
        "ValidateSplitSelectorResolutionDeterminism",
        "RejectDanglingSplitNameReference",
        "RejectSplitNameFromGeometryOrDisplayString",
        "RejectAmbiguousSplitIdentityEvolution",
    ] {
        let row = edge_split_operator_row(&blueprint, operator_name);
        assert_eq!(
            row.classification(),
            EdgeSplitOperatorClassification::QueryGraphCompositionProgram
        );
        assert_eq!(
            row.required_query_surface(),
            EdgeSplitRequiredQuerySurface::QueryInvariantRegistration
        );
    }

    for validator_name in [
        "ValidateSplitNameSurvival",
        "ValidateSplitPersistentNameUniqueness",
        "ValidateSplitSelectorResolutionDeterminism",
        "RejectDanglingSplitNameReference",
        "RejectSplitNameFromGeometryOrDisplayString",
        "RejectAmbiguousSplitIdentityEvolution",
    ] {
        let row = edge_split_validator_row(&blueprint, validator_name);
        assert!(row.governs_topology_legality());
        assert!(row.requires_runtime_lane(EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack));
    }
}
