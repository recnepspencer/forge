use super::super::classification::{
    PlanarBooleanLoopOperatorClassification as Class,
    PlanarBooleanLoopOperatorTruthAuthority as Authority,
    PlanarBooleanLoopRequiredQuerySurface as Surface,
};
use super::super::operator_row::PlanarBooleanLoopOperatorRow;
use super::super::proof_obligation::PlanarBooleanLoopOperatorProofObligation as OperatorProof;

pub(super) fn phase_2_operators() -> Vec<PlanarBooleanLoopOperatorRow> {
    vec![
        prepared("ConsumePlanarBooleanSplitEdgeChainLedger"),
        prepared("DeclarePlanarBooleanLoopReconstruction"),
        prepared("AdmitPlanarBooleanLoopReconstruction"),
        prepared("BindLoopReconstructionToSplitLedgerReceipt"),
        query_invariant("RejectSyntheticLoopReconstructionEntry"),
        query_declaration("RegisterLoopReconstructionOperatorDeclarationFamily"),
        query_grouped("RegisterLoopReconstructionGroupedOperatorFamily"),
        query_contribution("RegisterLoopReconstructionContributionWorkflow"),
        query_invariant("RegisterLoopReconstructionGraphInvariantPack"),
        query_declaration("ValidateLoopOperatorQueryProgression"),
        query_invariant("ValidateLoopValidatorRuntimeRegistration"),
        prepared("RecoverBooleanLoopSourceCarriers"),
        prepared("BindFragmentToSourceLoop"),
        prepared("BuildLoopFragmentContinuationIndex"),
        prepared("CanonicalizeLoopContinuationOrder"),
        prepared("AdmitLoopContinuationPolicy"),
        prepared("ClassifyLoopContinuationAmbiguity"),
        prepared("EmitLoopContinuationOutcome"),
        prepared("SelectCanonicalLoopSeeds"),
        prepared("AssembleClosedWalkCandidates"),
        prepared("ClassifyWalkOutcome"),
        prepared("PromoteClosedWalkToLoopCandidate"),
        prepared("RejectLoopCandidateBeforeIdentity"),
        prepared("BuildReconstructedLoop"),
        prepared("BuildBornLoopFromImprintNeighborhood"),
        prepared("PartitionLoopIslands"),
        prepared("SplitSourceLoopIntoReconstructedIslands"),
        prepared("PreserveLoopRoleFromSource"),
        prepared("ClassifyLoopRoleOutcome"),
        prepared("ClassifyLoopContainmentEvidencePosture"),
        prepared("RecordLoopContainmentEvidence"),
        prepared("ClassifyDegenerateLoopOutcome"),
        prepared("RejectCollapsedLoopBeforeLedger"),
        prepared("RejectTinyCardinalityLoopBeforeLedger"),
        prepared("RejectUnsupportedSelfTouchingLoopOutcome"),
        prepared("MintBooleanLoopIdentity"),
        query_contribution("PropagatePersistentNamesThroughLoopReconstruction"),
        query_contribution("RecordLoopEntityParentage"),
        query_contribution("ForkLoopEntityLineage"),
        query_declaration("SplitLoop"),
        query_declaration("CreateLoop"),
        query_declaration("DestroyLoop"),
        query_declaration("AttachLoop"),
        query_declaration("DetachLoop"),
        query_declaration("PromoteInnerLoop"),
        query_declaration("DemoteOuterLoop"),
        query_declaration("SetLoopContainment"),
        query_graph("RecordLoopReconstructionDecisionLog"),
        query_graph("LocalizeLoopReconstructionFailure"),
        query_graph("BuildStructuredLoopReconstructionFailureReport"),
        query_graph("AssemblePlanarBooleanLoopReconstructionLedger"),
        query_graph("BuildLoopReconstructionLedgerReceipt"),
        query_graph("FenceOverlapExtractionToLoopLedgerReceipt"),
        query_graph("RequireBooleanLoopReconstructionEvidence"),
        query_invariant("RegisterBooleanLoopReconstructionStageRequirement"),
        query_graph("ReplayPlanarBooleanLoopReconstruction"),
        query_graph("CompareLoopReconstructionReplayParity"),
        query_graph("CompareLoopReconstructionCheckpointParity"),
        query_invariant("RejectUnindexedLoopFragment"),
        query_invariant("RejectSyntheticLoopLedgerConstruction"),
    ]
}

fn prepared(operator_name: &'static str) -> PlanarBooleanLoopOperatorRow {
    row(
        operator_name,
        Class::PreparedSpatialOnly,
        Authority::WorthSpatialPrepared,
        Surface::None,
        None,
        &[
            OperatorProof::PreparedLoopProductOnly,
            OperatorProof::NoTopologyTruthMutationInMilestone74,
        ],
        None,
    )
}

fn query_declaration(operator_name: &'static str) -> PlanarBooleanLoopOperatorRow {
    row(
        operator_name,
        Class::TopologyDeclarationFamily,
        Authority::WorthTopoQueryDeclaration,
        Surface::TopologyDeclarationEntry,
        Some("TopologyOperatorWorkflowHandleExt"),
        &[
            OperatorProof::TopologyQueryDeclarationInput,
            OperatorProof::TopologyQueryDeclarationFamilyMarker,
            OperatorProof::TopologyOperatorDeclarationReview,
        ],
        None,
    )
}

fn query_grouped(operator_name: &'static str) -> PlanarBooleanLoopOperatorRow {
    row(
        operator_name,
        Class::TopologyGroupedDeclarationFamily,
        Authority::WorthTopoQueryDeclaration,
        Surface::TopologyGroupedDeclaration,
        Some("topology_grouped_operator_neighborhood"),
        &[
            OperatorProof::TopologyQueryDeclarationInput,
            OperatorProof::TopologyGroupedDeclarationInput,
            OperatorProof::GroupedSupportAndContributionEvidence,
        ],
        None,
    )
}

fn query_contribution(operator_name: &'static str) -> PlanarBooleanLoopOperatorRow {
    row(
        operator_name,
        Class::TopologyContributionWorkflow,
        Authority::WorthTopoQueryDeclaration,
        Surface::TopologyContributionWorkflow,
        Some("topology_operator_contribution_workflow"),
        &[
            OperatorProof::TopologyContributionDeclaration,
            OperatorProof::RetainedContributionSemanticProjection,
        ],
        None,
    )
}

fn query_graph(operator_name: &'static str) -> PlanarBooleanLoopOperatorRow {
    row(
        operator_name,
        Class::QueryGraphCompositionProgram,
        Authority::ForgeQueryGraphComposition,
        Surface::QueryGraphComposition,
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

fn query_invariant(operator_name: &'static str) -> PlanarBooleanLoopOperatorRow {
    row(
        operator_name,
        Class::QueryGraphCompositionProgram,
        Authority::ForgeQueryGraphComposition,
        Surface::QueryInvariantRegistration,
        Some("ForgeQueryRuntime::builder().invariant_registration_artifact"),
        &[
            OperatorProof::QueryInvariantRegistrationArtifact,
            OperatorProof::TypedGraphCompositionDomainInvariantDenial,
        ],
        None,
    )
}

fn row(
    operator_name: &'static str,
    classification: Class,
    truth_authority: Authority,
    required_query_surface: Surface,
    topology_precedent: Option<&'static str>,
    proof_obligations: &'static [OperatorProof],
    support_warning: Option<&'static str>,
) -> PlanarBooleanLoopOperatorRow {
    PlanarBooleanLoopOperatorRow::new(
        operator_name,
        classification,
        truth_authority,
        required_query_surface,
        topology_precedent,
        proof_obligations,
        support_warning,
    )
}
