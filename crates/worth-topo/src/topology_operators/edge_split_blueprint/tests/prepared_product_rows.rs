use super::*;

#[test]
fn phase_15_endpoint_boundary_rows_are_registered_as_prepared_products() {
    assert_prepared_operators(&[
        "CollapseEndpointNoOpSplits",
        "RecordEndpointContactDecision",
        "ValidateEndpointNoOpSplitPolicy",
        "RejectEndpointSplitThatWouldCreateZeroLengthFragment",
    ]);
    assert_spatial_validators(&[
        "ValidateEndpointNoOpSplitPolicy",
        "RejectEndpointSplitThatWouldCreateZeroLengthFragment",
    ]);
}

#[test]
fn phase_16_interval_normalization_rows_are_registered_as_prepared_products() {
    assert_prepared_operators(&[
        "MergeCollinearEdgeIntervals",
        "RemoveMicroBridgeEdges",
        "RemoveRedundantImprintEdges",
        "NormalizeOverlapIntervalSubdivision",
        "ValidateOverlapIntervalSubdivisionConsistency",
        "RejectMicroIntervalBelowAdmittedPolicy",
    ]);
    assert_spatial_validators(&[
        "ValidateOverlapIntervalSubdivisionConsistency",
        "RejectMicroIntervalBelowAdmittedPolicy",
    ]);
}

#[test]
fn phase_17_split_vertex_identity_rows_are_registered_as_prepared_products() {
    assert_prepared_operators(&[
        "MintBooleanSplitVertexIdentity",
        "CoalesceSharedSplitVertexIdentity",
        "ValidateSplitVertexIdentityCoalescence",
        "ExtractStableSubshapeSignatures",
        "RejectCoordinateOnlySplitVertexIdentity",
    ]);
    assert_spatial_validators(&[
        "ValidateSplitVertexIdentityCoalescence",
        "RejectCoordinateOnlySplitVertexIdentity",
    ]);
}

#[test]
fn phase_19_overlap_chain_rows_are_registered_as_prepared_products() {
    assert_prepared_operators(&[
        "BuildOverlapEdgeChain",
        "ResolveEdgeEdgePartialOverlap",
        "ResolveCoincidentButOppositeSenseEdges",
        "ResolveCoincidentEdgesDifferentParameterization",
        "ClassifyOverlapChainBoundaryRole",
        "ValidateCoincidentOppositeSensePreservation",
    ]);
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();
    let conversion = blueprint
        .operator("ConvertOverlapToSharedTopology")
        .expect("overlap topology conversion should remain registered");
    assert_eq!(
        conversion.classification(),
        EdgeSplitOperatorClassification::SupportGatedFutureTopologyMutation
    );
    assert_eq!(
        conversion.truth_authority(),
        EdgeSplitOperatorTruthAuthority::FutureSupportGated
    );
    assert_spatial_validators(&["ValidateCoincidentOppositeSensePreservation"]);
}

#[test]
fn phase_20_split_chain_validation_rows_are_registered_as_prepared_products() {
    assert_prepared_operators(&[
        "ValidateSplitEdgeChainClosure",
        "ValidateSplitFragmentDomainCoverage",
        "ValidateNoDanglingSplitChainReferences",
        "ValidateOverlapChainFragmentReferences",
        "RejectSplitChainGapOrOverlap",
    ]);
    assert_spatial_validators(&[
        "ValidateSplitEdgeChainClosure",
        "ValidateSplitFragmentDomainCoverage",
        "ValidateNoDanglingSplitChainReferences",
        "ValidateOverlapChainFragmentReferences",
        "RejectSplitChainGapOrOverlap",
    ]);
}

fn assert_prepared_operators(operator_names: &[&str]) {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();
    for operator_name in operator_names {
        let row = blueprint
            .operator(operator_name)
            .expect("prepared edge-splitting operator should be registered");
        assert_eq!(
            row.classification(),
            EdgeSplitOperatorClassification::PreparedSpatialOnly
        );
        assert_eq!(
            row.truth_authority(),
            EdgeSplitOperatorTruthAuthority::WorthSpatialPrepared
        );
        assert_eq!(
            row.required_query_surface(),
            EdgeSplitRequiredQuerySurface::None
        );
        assert!(row
            .proof_obligations()
            .contains(&EdgeSplitOperatorProofObligation::PreparedSplitProductOnly));
        assert!(row
            .proof_obligations()
            .contains(&EdgeSplitOperatorProofObligation::NoTopologyTruthMutationInMilestone73));
    }
}

fn assert_spatial_validators(validator_names: &[&str]) {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();
    for validator_name in validator_names {
        let row = blueprint
            .validator(validator_name)
            .expect("prepared edge-splitting validator should be registered");
        assert!(!row.governs_topology_legality());
        assert!(row.requires_runtime_lane(
            EdgeSplitValidatorRuntimeLane::SpatialPreparedProductValidation
        ));
    }
}
