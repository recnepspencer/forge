use super::classification::{
    EdgeSplitOperatorClassification as Class, EdgeSplitOperatorTruthAuthority as Authority,
    EdgeSplitRequiredQuerySurface as QuerySurface, EdgeSplitValidatorRuntimeLane as ValidatorLane,
};
use super::closeout::{EdgeSplitBlueprintCloseout, EdgeSplitBlueprintCloseoutDenial};
use super::operator_row::EdgeSplitOperatorRow;
use super::proof_obligation::{
    EdgeSplitOperatorProofObligation as OperatorProof,
    EdgeSplitValidatorProofObligation as ValidatorProof,
};
use super::validator_row::EdgeSplitValidatorRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeSplitOperatorBlueprint {
    operators: Vec<EdgeSplitOperatorRow>,
    validators: Vec<EdgeSplitValidatorRow>,
    closeout: EdgeSplitBlueprintCloseout,
}

impl EdgeSplitOperatorBlueprint {
    pub fn phase_1() -> Self {
        Self::from_rows(phase_1_operators(), phase_1_validators())
            .expect("phase 1 edge split blueprint registry must close out")
    }

    fn from_rows(
        operators: Vec<EdgeSplitOperatorRow>,
        validators: Vec<EdgeSplitValidatorRow>,
    ) -> Result<Self, EdgeSplitBlueprintCloseoutDenial> {
        let closeout = EdgeSplitBlueprintCloseout::certify(&operators, &validators)?;
        Ok(Self {
            operators,
            validators,
            closeout,
        })
    }

    #[cfg(test)]
    pub(crate) fn try_from_rows(
        operators: Vec<EdgeSplitOperatorRow>,
        validators: Vec<EdgeSplitValidatorRow>,
    ) -> Result<Self, EdgeSplitBlueprintCloseoutDenial> {
        Self::from_rows(operators, validators)
    }

    pub fn operators(&self) -> &[EdgeSplitOperatorRow] {
        &self.operators
    }

    pub fn validators(&self) -> &[EdgeSplitValidatorRow] {
        &self.validators
    }

    pub fn operator(&self, operator_name: &str) -> Option<&EdgeSplitOperatorRow> {
        self.operators
            .iter()
            .find(|operator| operator.operator_name() == operator_name)
    }

    pub fn validator(&self, validator_name: &str) -> Option<&EdgeSplitValidatorRow> {
        self.validators
            .iter()
            .find(|validator| validator.validator_name() == validator_name)
    }

    pub fn closeout(&self) -> &EdgeSplitBlueprintCloseout {
        &self.closeout
    }
}

fn phase_1_operators() -> Vec<EdgeSplitOperatorRow> {
    vec![
        query_declaration("RegisterEdgeSplitOperatorDeclarationFamily"),
        query_grouped("RegisterEdgeSplitGroupedOperatorFamily"),
        query_contribution("RegisterEdgeSplitContributionWorkflow"),
        query_invariant("RegisterEdgeSplitGraphInvariantPack"),
        query_graph("MapSplitLedgerToTopologyOperatorDeclarations"),
        prepared("ClassifyPreparedVsAuthoritativeSplitOperator"),
        query_declaration("ValidateSplitOperatorQueryProgression"),
        query_invariant("ValidateSplitValidatorRuntimeRegistration"),
        prepared("BuildSplitEventParticipationIndex"),
        prepared("ExtractPointSplitCandidates"),
        prepared("AdmitPointSplitParameterDomain"),
        prepared("ExtractIntervalSplitCandidates"),
        prepared("AdmitIntervalSplitParameterDomain"),
        prepared("ClassifyEndpointTouchSplitPosture"),
        prepared("AssemblePerEdgeSplitSchedule"),
        prepared("OrderEdgeSplitScheduleCanonically"),
        prepared("NormalizeDuplicateSplitCuts"),
        prepared("CollapseEndpointNoOpSplits"),
        prepared("RecordEndpointContactDecision"),
        prepared("ValidateEndpointNoOpSplitPolicy"),
        prepared("RejectEndpointSplitThatWouldCreateZeroLengthFragment"),
        prepared("MergeCollinearEdgeIntervals"),
        prepared("RemoveMicroBridgeEdges"),
        prepared("RemoveRedundantImprintEdges"),
        prepared("NormalizeOverlapIntervalSubdivision"),
        prepared("ValidateOverlapIntervalSubdivisionConsistency"),
        prepared("RejectMicroIntervalBelowAdmittedPolicy"),
        prepared("MintBooleanSplitVertexIdentity"),
        prepared("CoalesceSharedSplitVertexIdentity"),
        prepared("ValidateSplitVertexIdentityCoalescence"),
        prepared("ExtractStableSubshapeSignatures"),
        prepared("RejectCoordinateOnlySplitVertexIdentity"),
        prepared("BuildSplitEdgeFragments"),
        prepared("BuildOverlapEdgeChain"),
        prepared("ResolveEdgeEdgePartialOverlap"),
        prepared("ResolveCoincidentButOppositeSenseEdges"),
        prepared("ResolveCoincidentEdgesDifferentParameterization"),
        prepared("ClassifyOverlapChainBoundaryRole"),
        prepared("ValidateCoincidentOppositeSensePreservation"),
        prepared("ValidateSplitEdgeChainClosure"),
        prepared("ValidateSplitFragmentDomainCoverage"),
        prepared("ValidateNoDanglingSplitChainReferences"),
        prepared("ValidateOverlapChainFragmentReferences"),
        prepared("RejectSplitChainGapOrOverlap"),
        prepared("PropagatePersistentNamesThroughSplit"),
        query_grouped_with_precedent(
            "SplitConnectedHalfEdgeSetToNewWire",
            "TopologySplitConnectedHalfEdgeSetToNewWireDeclaration",
        ),
        support_gated("SplitEdge"),
        support_gated("SplitIntersectedEdges"),
        support_gated("InsertVertexOnEdgeForTJunction"),
        support_gated("SplitEdgeAtOverlapInterval"),
        support_gated("SplitEdgeAndCurves"),
        support_gated("ConvertOverlapToSharedTopology"),
        support_gated("ExtractCoplanarOverlapLoops"),
    ]
}

fn phase_1_validators() -> Vec<EdgeSplitValidatorRow> {
    vec![
        validator(
            "ValidateSplitOperatorQueryProgression",
            ValidatorLane::TopologyDeclarationReview,
            true,
        ),
        validator(
            "ValidateSplitValidatorRuntimeRegistration",
            ValidatorLane::QueryGraphInvariantPack,
            true,
        ),
        validator(
            "ValidateSplitEdgeChainClosure",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "ValidateSplitFragmentDomainCoverage",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "ValidateNoDanglingSplitChainReferences",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "ValidateOverlapChainFragmentReferences",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "RejectSplitChainGapOrOverlap",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "ValidateCoincidentOppositeSensePreservation",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "ValidateNoUnexpectedZeroLengthEdges",
            ValidatorLane::QueryGraphInvariantPack,
            true,
        ),
        validator(
            "ValidateCanonicalOrderingStable",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "ValidateEndpointNoOpSplitPolicy",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "RejectEndpointSplitThatWouldCreateZeroLengthFragment",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "ValidateOverlapIntervalSubdivisionConsistency",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "RejectMicroIntervalBelowAdmittedPolicy",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "ValidateSplitVertexIdentityCoalescence",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "RejectCoordinateOnlySplitVertexIdentity",
            ValidatorLane::SpatialPreparedProductValidation,
            false,
        ),
        validator(
            "ValidateConsistentVertexMergesInGraph",
            ValidatorLane::QueryGraphInvariantPack,
            true,
        ),
        validator(
            "ValidateNameSurvivalThroughSplitMerge",
            ValidatorLane::TopologyDeclarationReview,
            true,
        ),
    ]
}

fn prepared(operator_name: &'static str) -> EdgeSplitOperatorRow {
    row(
        operator_name,
        Class::PreparedSpatialOnly,
        Authority::WorthSpatialPrepared,
        QuerySurface::None,
        None,
        &[
            OperatorProof::PreparedSplitProductOnly,
            OperatorProof::NoTopologyTruthMutationInMilestone73,
        ],
        None,
    )
}

fn query_declaration(operator_name: &'static str) -> EdgeSplitOperatorRow {
    query_declaration_with_precedent(operator_name, "TopologyOperatorWorkflowHandleExt")
}

fn query_declaration_with_precedent(
    operator_name: &'static str,
    topology_precedent: &'static str,
) -> EdgeSplitOperatorRow {
    row(
        operator_name,
        Class::TopologyDeclarationFamily,
        Authority::WorthTopoQueryDeclaration,
        QuerySurface::TopologyDeclarationEntry,
        Some(topology_precedent),
        &[
            OperatorProof::TopologyQueryDeclarationInput,
            OperatorProof::TopologyQueryDeclarationFamilyMarker,
            OperatorProof::TopologyOperatorDeclarationReview,
        ],
        None,
    )
}

fn query_grouped(operator_name: &'static str) -> EdgeSplitOperatorRow {
    query_grouped_with_precedent(operator_name, "topology_grouped_operator_neighborhood")
}

fn query_grouped_with_precedent(
    operator_name: &'static str,
    topology_precedent: &'static str,
) -> EdgeSplitOperatorRow {
    row(
        operator_name,
        Class::TopologyGroupedDeclarationFamily,
        Authority::WorthTopoQueryDeclaration,
        QuerySurface::TopologyGroupedDeclaration,
        Some(topology_precedent),
        &[
            OperatorProof::TopologyQueryDeclarationInput,
            OperatorProof::TopologyGroupedDeclarationInput,
            OperatorProof::GroupedSupportAndContributionEvidence,
        ],
        None,
    )
}

fn query_contribution(operator_name: &'static str) -> EdgeSplitOperatorRow {
    row(
        operator_name,
        Class::TopologyContributionWorkflow,
        Authority::WorthTopoQueryDeclaration,
        QuerySurface::TopologyContributionWorkflow,
        Some("topology_operator_contribution_workflow"),
        &[
            OperatorProof::TopologyContributionDeclaration,
            OperatorProof::RetainedContributionSemanticProjection,
        ],
        None,
    )
}

fn query_graph(operator_name: &'static str) -> EdgeSplitOperatorRow {
    row(
        operator_name,
        Class::QueryGraphCompositionProgram,
        Authority::ForgeQueryGraphComposition,
        QuerySurface::QueryGraphComposition,
        Some("workspace.compose_graph_with_invariant_pack"),
        &[
            OperatorProof::QueryGraphCompositionProgram,
            OperatorProof::QueryGraphCompositionResolutionMap,
            OperatorProof::QueryGraphCompositionLifecycleOutcomes,
            OperatorProof::QueryGraphCompositionDomainInvariantDenial,
        ],
        None,
    )
}

fn query_invariant(operator_name: &'static str) -> EdgeSplitOperatorRow {
    row(
        operator_name,
        Class::QueryGraphCompositionProgram,
        Authority::ForgeQueryGraphComposition,
        QuerySurface::QueryInvariantRegistration,
        Some("ForgeQueryRuntime::builder().invariant_registration_artifact"),
        &[
            OperatorProof::QueryInvariantRegistrationArtifact,
            OperatorProof::TypedGraphCompositionDomainInvariantDenial,
        ],
        None,
    )
}

fn support_gated(operator_name: &'static str) -> EdgeSplitOperatorRow {
    row(
        operator_name,
        Class::SupportGatedFutureTopologyMutation,
        Authority::FutureSupportGated,
        QuerySurface::TopologyDeclarationEntry,
        None,
        &[OperatorProof::ExplicitFutureSupportPosture],
        Some("not admitted as a topology mutation in milestone 7.3"),
    )
}

fn row(
    operator_name: &'static str,
    classification: Class,
    truth_authority: Authority,
    required_query_surface: QuerySurface,
    topology_precedent: Option<&'static str>,
    proof_obligations: &'static [OperatorProof],
    support_warning: Option<&'static str>,
) -> EdgeSplitOperatorRow {
    EdgeSplitOperatorRow::new(
        operator_name,
        classification,
        truth_authority,
        required_query_surface,
        topology_precedent,
        proof_obligations,
        support_warning,
    )
}

fn validator(
    validator_name: &'static str,
    runtime_lane: ValidatorLane,
    governs_topology_legality: bool,
) -> EdgeSplitValidatorRow {
    EdgeSplitValidatorRow::new(
        validator_name,
        runtime_lane,
        governs_topology_legality,
        &[ValidatorProof::RuntimeFacingDenialPathTypedAndInspectable],
    )
}
