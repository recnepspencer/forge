use super::classification::{
    EdgeSplitOperatorClassification, EdgeSplitRequiredQuerySurface, EdgeSplitValidatorRuntimeLane,
};
use super::closeout::EdgeSplitBlueprintCloseoutDenial;
use super::operator_row::EdgeSplitOperatorRow;
use super::validator_row::EdgeSplitValidatorRow;

pub(super) fn require_phase_1_operator_rows(
    operators: &[EdgeSplitOperatorRow],
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    for (required_operator, _, _) in REQUIRED_PHASE_1_OPERATOR_LANES {
        if !operators
            .iter()
            .any(|operator| operator.operator_name() == *required_operator)
        {
            return Err(EdgeSplitBlueprintCloseoutDenial::MissingRequiredOperator);
        }
    }
    Ok(())
}

pub(super) fn require_phase_1_validator_rows(
    validators: &[EdgeSplitValidatorRow],
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    for (required_validator, _, _) in REQUIRED_PHASE_1_VALIDATOR_LANES {
        if !validators
            .iter()
            .any(|validator| validator.validator_name() == *required_validator)
        {
            return Err(EdgeSplitBlueprintCloseoutDenial::MissingRequiredValidator);
        }
    }
    Ok(())
}

pub(super) fn require_phase_1_operator_lanes(
    operators: &[EdgeSplitOperatorRow],
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    for (required_operator, classification, query_surface) in REQUIRED_PHASE_1_OPERATOR_LANES {
        let Some(operator) = operators
            .iter()
            .find(|operator| operator.operator_name() == *required_operator)
        else {
            return Err(EdgeSplitBlueprintCloseoutDenial::MissingRequiredOperator);
        };
        if operator.classification() != *classification
            || operator.required_query_surface() != *query_surface
        {
            return Err(EdgeSplitBlueprintCloseoutDenial::RequiredOperatorLaneMismatch);
        }
    }
    Ok(())
}

pub(super) fn require_phase_1_validator_lanes(
    validators: &[EdgeSplitValidatorRow],
) -> Result<(), EdgeSplitBlueprintCloseoutDenial> {
    for (required_validator, runtime_lane, governs_topology_legality) in
        REQUIRED_PHASE_1_VALIDATOR_LANES
    {
        let Some(validator) = validators
            .iter()
            .find(|validator| validator.validator_name() == *required_validator)
        else {
            return Err(EdgeSplitBlueprintCloseoutDenial::MissingRequiredValidator);
        };
        if validator.runtime_lane() != *runtime_lane
            || validator.governs_topology_legality() != *governs_topology_legality
        {
            return Err(EdgeSplitBlueprintCloseoutDenial::RequiredValidatorLaneMismatch);
        }
    }
    Ok(())
}

pub(super) fn required_phase_1_operator_row_count() -> usize {
    REQUIRED_PHASE_1_OPERATOR_LANES.len()
}

pub(super) fn required_phase_1_validator_row_count() -> usize {
    REQUIRED_PHASE_1_VALIDATOR_LANES.len()
}

const REQUIRED_PHASE_1_OPERATOR_LANES: &[(
    &str,
    EdgeSplitOperatorClassification,
    EdgeSplitRequiredQuerySurface,
)] = &[
    (
        "RegisterEdgeSplitOperatorDeclarationFamily",
        EdgeSplitOperatorClassification::TopologyDeclarationFamily,
        EdgeSplitRequiredQuerySurface::TopologyDeclarationEntry,
    ),
    (
        "RegisterEdgeSplitGroupedOperatorFamily",
        EdgeSplitOperatorClassification::TopologyGroupedDeclarationFamily,
        EdgeSplitRequiredQuerySurface::TopologyGroupedDeclaration,
    ),
    (
        "RegisterEdgeSplitContributionWorkflow",
        EdgeSplitOperatorClassification::TopologyContributionWorkflow,
        EdgeSplitRequiredQuerySurface::TopologyContributionWorkflow,
    ),
    (
        "RegisterEdgeSplitGraphInvariantPack",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
    ),
    (
        "MapSplitLedgerToTopologyOperatorDeclarations",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "ClassifyPreparedVsAuthoritativeSplitOperator",
        EdgeSplitOperatorClassification::PreparedSpatialOnly,
        EdgeSplitRequiredQuerySurface::None,
    ),
    (
        "ValidateSplitOperatorQueryProgression",
        EdgeSplitOperatorClassification::TopologyDeclarationFamily,
        EdgeSplitRequiredQuerySurface::TopologyDeclarationEntry,
    ),
    (
        "ValidateSplitValidatorRuntimeRegistration",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
    ),
    (
        "BuildSplitPersistentNamingMap",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "BuildSplitPersistentNamingSeeds",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "AdmitSplitIdentityEvolutionQuery",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "BindSplitPersistentNamesToQueryLineage",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "PropagatePersistentNamesThroughSplit",
        EdgeSplitOperatorClassification::TopologyContributionWorkflow,
        EdgeSplitRequiredQuerySurface::TopologyContributionWorkflow,
    ),
    (
        "RecordSplitEntityParentage",
        EdgeSplitOperatorClassification::TopologyContributionWorkflow,
        EdgeSplitRequiredQuerySurface::TopologyContributionWorkflow,
    ),
    (
        "ForkSplitEntityLineage",
        EdgeSplitOperatorClassification::TopologyContributionWorkflow,
        EdgeSplitRequiredQuerySurface::TopologyContributionWorkflow,
    ),
    (
        "ExtractSplitStableSubshapeSignatures",
        EdgeSplitOperatorClassification::PreparedSpatialOnly,
        EdgeSplitRequiredQuerySurface::None,
    ),
    (
        "ResolveSplitNameConflictsAfterBoolean",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
    ),
    (
        "ValidateSplitNameSurvival",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
    ),
    (
        "ValidateSplitPersistentNameUniqueness",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
    ),
    (
        "ValidateSplitSelectorResolutionDeterminism",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
    ),
    (
        "RejectDanglingSplitNameReference",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
    ),
    (
        "RejectSplitNameFromGeometryOrDisplayString",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
    ),
    (
        "RejectAmbiguousSplitIdentityEvolution",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryInvariantRegistration,
    ),
    (
        "RecordEdgeSplitDecisionLog",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "LocalizePlanarBooleanFailure",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "BuildStructuredEdgeSplitFailureReport",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "AssemblePlanarBooleanSplitEdgeChainLedger",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "BuildSplitEdgeChain",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "BuildSplitLedgerReceipt",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "CanonicalizeSplitLedgerOrdering",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "ValidateSplitLedgerReceiptChain",
        EdgeSplitOperatorClassification::QueryGraphCompositionProgram,
        EdgeSplitRequiredQuerySurface::QueryGraphComposition,
    ),
    (
        "EmitPlanarBooleanOutcome",
        EdgeSplitOperatorClassification::TopologyDeclarationFamily,
        EdgeSplitRequiredQuerySurface::TopologyDeclarationEntry,
    ),
];

const REQUIRED_PHASE_1_VALIDATOR_LANES: &[(&str, EdgeSplitValidatorRuntimeLane, bool)] = &[
    (
        "ValidateSplitOperatorQueryProgression",
        EdgeSplitValidatorRuntimeLane::TopologyDeclarationReview,
        true,
    ),
    (
        "ValidateSplitValidatorRuntimeRegistration",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "ValidateSplitNameSurvival",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "ValidateSplitPersistentNameUniqueness",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "ValidateSplitSelectorResolutionDeterminism",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "RejectDanglingSplitNameReference",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "RejectSplitNameFromGeometryOrDisplayString",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "RejectAmbiguousSplitIdentityEvolution",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "ValidateEdgeSplitDecisionLogCoverage",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "ValidateEdgeSplitFailureLocalizationConsistency",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "ValidateEdgeSplitDiagnosticsDoNotMutateOperationalDigest",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "ValidateSplitLedgerReceiptChain",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "RejectSplitLedgerMissingValidationReceipt",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "RejectSplitLedgerMissingPersistentNamingReceipt",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "RejectSplitLedgerMissingDecisionLogReceipt",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
    (
        "RejectSplitLedgerForeignProductLineage",
        EdgeSplitValidatorRuntimeLane::QueryGraphInvariantPack,
        true,
    ),
];
