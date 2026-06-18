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
        query_graph("BuildSplitPersistentNamingMap"),
        query_graph("BuildSplitPersistentNamingSeeds"),
        query_graph("AdmitSplitIdentityEvolutionQuery"),
        query_graph("BindSplitPersistentNamesToQueryLineage"),
        query_contribution("PropagatePersistentNamesThroughSplit"),
        query_contribution("RecordSplitEntityParentage"),
        query_contribution("ForkSplitEntityLineage"),
        prepared("ExtractSplitStableSubshapeSignatures"),
        query_invariant("ResolveSplitNameConflictsAfterBoolean"),
        query_invariant("ValidateSplitNameSurvival"),
        query_invariant("ValidateSplitPersistentNameUniqueness"),
        query_invariant("ValidateSplitSelectorResolutionDeterminism"),
        query_invariant("RejectDanglingSplitNameReference"),
        query_invariant("RejectSplitNameFromGeometryOrDisplayString"),
        query_invariant("RejectAmbiguousSplitIdentityEvolution"),
        query_graph("RecordBooleanDecisionLog"),
        query_graph("RecordEdgeSplitDecisionLog"),
        query_graph("LocalizePlanarBooleanFailure"),
        query_graph("BuildStructuredEdgeSplitFailureReport"),
        query_graph("AssemblePlanarBooleanSplitEdgeChainLedger"),
        query_graph("BuildSplitEdgeChain"),
        query_graph("BuildSplitLedgerReceipt"),
        query_graph("CanonicalizeSplitLedgerOrdering"),
        query_graph("ValidateSplitLedgerReceiptChain"),
        query_graph("ReplayPlanarBooleanEdgeSplit"),
        query_graph("CompareEdgeSplitReplayParity"),
        query_graph("CompareEdgeSplitCheckpointParity"),
        prepared("CanonicalizeReversedEdgeSenseSplit"),
        query_invariant("RejectSyntheticSplitLedgerConstruction"),
        query_invariant("RejectRawEventVectorSplitConsumption"),
        query_invariant("RejectHandFilledSplitEvidenceRows"),
        query_invariant("RejectCoordinateOnlySplitVertices"),
        query_graph("FenceLoopReconstructionToSplitLedgerReceipt"),
        query_graph("CertifyPlanarBooleanEdgeSplittingMetaboss"),
        query_graph("BuildEdgeSplitMetabossWorkloadRecipe"),
        query_graph("EmitEdgeSplitMetabossProofBundle"),
        query_invariant("ValidateEdgeSplitSummumBonumCloseout"),
        query_invariant("RegisterMilestone7_3CloseoutRows"),
        query_declaration("EmitPlanarBooleanOutcome"),
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
        topology_review_validator("ValidateSplitOperatorQueryProgression"),
        query_invariant_validator("ValidateSplitValidatorRuntimeRegistration"),
        spatial_validator("ValidateSplitEdgeChainClosure"),
        spatial_validator("ValidateSplitFragmentDomainCoverage"),
        spatial_validator("ValidateNoDanglingSplitChainReferences"),
        spatial_validator("ValidateOverlapChainFragmentReferences"),
        spatial_validator("RejectSplitChainGapOrOverlap"),
        spatial_validator("ValidateCoincidentOppositeSensePreservation"),
        query_invariant_validator("ValidateNoUnexpectedZeroLengthEdges"),
        spatial_validator("ValidateCanonicalOrderingStable"),
        spatial_validator("ValidateEndpointNoOpSplitPolicy"),
        spatial_validator("RejectEndpointSplitThatWouldCreateZeroLengthFragment"),
        spatial_validator("ValidateOverlapIntervalSubdivisionConsistency"),
        spatial_validator("RejectMicroIntervalBelowAdmittedPolicy"),
        spatial_validator("ValidateSplitVertexIdentityCoalescence"),
        spatial_validator("RejectCoordinateOnlySplitVertexIdentity"),
        query_invariant_validator("ValidateConsistentVertexMergesInGraph"),
        topology_review_validator("ValidateNameSurvivalThroughSplitMerge"),
        query_invariant_validator("ValidateSplitNameSurvival"),
        query_invariant_validator("ValidateSplitPersistentNameUniqueness"),
        query_invariant_validator("ValidateSplitSelectorResolutionDeterminism"),
        query_invariant_validator("RejectDanglingSplitNameReference"),
        query_invariant_validator("RejectSplitNameFromGeometryOrDisplayString"),
        query_invariant_validator("RejectAmbiguousSplitIdentityEvolution"),
        query_invariant_validator("ValidateBooleanDecisionLogCoverage"),
        query_invariant_validator("ValidateEdgeSplitDecisionLogCoverage"),
        query_invariant_validator("ValidateEdgeSplitFailureLocalizationConsistency"),
        query_invariant_validator("ValidateEdgeSplitDiagnosticsDoNotMutateOperationalDigest"),
        query_invariant_validator("ValidateSplitLedgerReceiptChain"),
        query_invariant_validator("RejectSplitLedgerMissingValidationReceipt"),
        query_invariant_validator("RejectSplitLedgerMissingPersistentNamingReceipt"),
        query_invariant_validator("RejectSplitLedgerMissingDecisionLogReceipt"),
        query_invariant_validator("RejectSplitLedgerForeignProductLineage"),
        query_invariant_validator("ValidatePlanarBooleanReplayParity"),
        query_invariant_validator("ValidatePlanarBooleanCheckpointParity"),
        query_invariant_validator("ValidateJournalReplayExactness"),
        query_invariant_validator("RejectSyntheticSplitLedgerConstruction"),
        query_invariant_validator("RejectRawEventVectorSplitConsumption"),
        query_invariant_validator("RejectHandFilledSplitEvidenceRows"),
        query_invariant_validator("RejectCoordinateOnlySplitVertices"),
        query_invariant_validator("FenceLoopReconstructionToSplitLedgerReceipt"),
        query_invariant_validator("ValidateEdgeSplitMetabossCandidateIndexProof"),
        query_invariant_validator("ValidateEdgeSplitMetabossLedgerAndReplayProof"),
        query_invariant_validator("ValidateEdgeSplitSummumBonumCloseout"),
        query_invariant_validator("RejectCrossProductCandidateDiscoveryAsCloseoutProof"),
        query_invariant_validator("RejectSyntheticMetabossCloseoutProofBundle"),
    ]
}

fn spatial_validator(validator_name: &'static str) -> EdgeSplitValidatorRow {
    validator(
        validator_name,
        ValidatorLane::SpatialPreparedProductValidation,
        false,
    )
}

fn query_invariant_validator(validator_name: &'static str) -> EdgeSplitValidatorRow {
    validator(validator_name, ValidatorLane::QueryGraphInvariantPack, true)
}

fn topology_review_validator(validator_name: &'static str) -> EdgeSplitValidatorRow {
    validator(
        validator_name,
        ValidatorLane::TopologyDeclarationReview,
        true,
    )
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
