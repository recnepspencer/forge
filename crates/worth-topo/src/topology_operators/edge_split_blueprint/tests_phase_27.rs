use super::{
    EdgeSplitOperatorBlueprint, EdgeSplitOperatorClassification, EdgeSplitRequiredQuerySurface,
    EdgeSplitValidatorRuntimeLane,
};

#[test]
fn phase_27_summum_bonum_closeout_rows_are_query_registered() {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();

    for operator_name in [
        "CertifyPlanarBooleanEdgeSplittingMetaboss",
        "BuildEdgeSplitMetabossWorkloadRecipe",
        "EmitEdgeSplitMetabossProofBundle",
    ] {
        let operator = blueprint
            .operator(operator_name)
            .expect("summum bonum closeout operator should be registered");
        assert_eq!(
            operator.classification(),
            EdgeSplitOperatorClassification::QueryGraphCompositionProgram
        );
        assert_eq!(
            operator.required_query_surface(),
            EdgeSplitRequiredQuerySurface::QueryGraphComposition
        );
    }

    for operator_name in [
        "ValidateEdgeSplitSummumBonumCloseout",
        "RegisterMilestone7_3CloseoutRows",
    ] {
        let operator = blueprint
            .operator(operator_name)
            .expect("summum bonum invariant operator should be registered");
        assert_eq!(
            operator.required_query_surface(),
            EdgeSplitRequiredQuerySurface::QueryInvariantRegistration
        );
    }

    for validator_name in [
        "ValidateEdgeSplitMetabossCandidateIndexProof",
        "ValidateEdgeSplitMetabossLedgerAndReplayProof",
        "ValidateEdgeSplitSummumBonumCloseout",
        "RejectCrossProductCandidateDiscoveryAsCloseoutProof",
        "RejectSyntheticMetabossCloseoutProofBundle",
    ] {
        let validator = blueprint
            .validator(validator_name)
            .expect("summum bonum validator should be registered");
        assert_eq!(
            validator.runtime_lane(),
            EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack
        );
        assert!(validator.governs_topology_legality());
    }
}
