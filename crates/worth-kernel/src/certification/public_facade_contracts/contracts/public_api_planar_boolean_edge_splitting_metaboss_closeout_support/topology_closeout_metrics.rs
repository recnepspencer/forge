use topology::facade::{
    EdgeSplitOperatorBlueprint, EdgeSplitOperatorClassification, EdgeSplitRequiredQuerySurface,
    EdgeSplitValidatorRuntimeLane,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TopologyCloseoutSummary {
    pub(crate) required_operator_rows: usize,
    pub(crate) required_validator_rows: usize,
    pub(crate) phase_27_query_graph_rows: usize,
    pub(crate) phase_27_query_invariant_rows: usize,
    pub(crate) phase_27_runtime_validators: usize,
}

pub(crate) fn topology_closeout_summary() -> TopologyCloseoutSummary {
    let blueprint = EdgeSplitOperatorBlueprint::phase_1();
    let phase_27_query_graph_rows = query_graph_composition_operator_rows(&blueprint);
    let phase_27_query_invariant_rows = query_invariant_operator_rows(&blueprint);
    let phase_27_runtime_validators = query_graph_invariant_validator_rows(&blueprint);

    TopologyCloseoutSummary {
        required_operator_rows: blueprint.closeout().required_phase_1_operator_rows(),
        required_validator_rows: blueprint.closeout().required_phase_1_validator_rows(),
        phase_27_query_graph_rows,
        phase_27_query_invariant_rows,
        phase_27_runtime_validators,
    }
}

fn query_graph_composition_operator_rows(blueprint: &EdgeSplitOperatorBlueprint) -> usize {
    [
        "CertifyPlanarBooleanEdgeSplittingMetaboss",
        "BuildEdgeSplitMetabossWorkloadRecipe",
        "EmitEdgeSplitMetabossProofBundle",
    ]
    .iter()
    .filter(|operator_name| {
        blueprint.operator(operator_name).is_some_and(|operator| {
            operator.classification()
                == EdgeSplitOperatorClassification::QueryGraphCompositionProgram
                && operator.required_query_surface()
                    == EdgeSplitRequiredQuerySurface::QueryGraphComposition
        })
    })
    .count()
}

fn query_invariant_operator_rows(blueprint: &EdgeSplitOperatorBlueprint) -> usize {
    [
        "ValidateEdgeSplitSummumBonumCloseout",
        "RegisterMilestone7_3CloseoutRows",
    ]
    .iter()
    .filter(|operator_name| {
        blueprint.operator(operator_name).is_some_and(|operator| {
            operator.required_query_surface()
                == EdgeSplitRequiredQuerySurface::QueryInvariantRegistration
        })
    })
    .count()
}

fn query_graph_invariant_validator_rows(blueprint: &EdgeSplitOperatorBlueprint) -> usize {
    [
        "ValidateEdgeSplitMetabossCandidateIndexProof",
        "ValidateEdgeSplitMetabossLedgerAndReplayProof",
        "ValidateEdgeSplitSummumBonumCloseout",
        "RejectCrossProductCandidateDiscoveryAsCloseoutProof",
        "RejectSyntheticMetabossCloseoutProofBundle",
    ]
    .iter()
    .filter(|validator_name| {
        blueprint
            .validator(validator_name)
            .is_some_and(|validator| {
                validator.runtime_lane() == EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack
                    && validator.governs_topology_legality()
            })
    })
    .count()
}
