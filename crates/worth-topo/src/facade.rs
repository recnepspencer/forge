//! Public API boundary for topology.

pub fn topology_half_edge_next_relation_name(
) -> Result<forge_query::facade::RelationName, forge_query::facade::AuthoringError> {
    forge_query::facade::RelationName::new(
        schema::facade::platform::relations::TopologyRelationKind::HalfEdgeNext.kind_name(),
    )
}

pub use crate::brep::topology_graph::{
    TopologyBody, TopologyEdge, TopologyFace, TopologyHalfEdge, TopologyLoop, TopologyLump,
    TopologyModel, TopologyRegion, TopologyShell, TopologyVertex, TopologyView, TopologyWire,
};
pub use crate::construction::{
    prepare_primitive_construction_query_admitted_handoff,
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    prepare_primitive_construction_query_envelope, prepare_primitive_construction_query_handoff,
    prepare_primitive_construction_query_receipt,
    run_primitive_construction_birth_declared_touched_basis_compose,
    topology_primitive_construction_birth_graph_authority_proof,
    topology_primitive_construction_birth_graph_obligation_registration,
    TopologyConstructionQueryAdmittedHandoffError, TopologyConstructionQueryEnvelopeError,
    TopologyConstructionQueryFactKind, TopologyConstructionQueryFactProvenance,
    TopologyConstructionQueryFactRow, TopologyConstructionQueryHandoffError,
    TopologyConstructionQueryInspectionSurface, TopologyConstructionQueryMutationSurface,
    TopologyConstructionQueryReadSurface, TopologyConstructionQueryReceiptError,
    TopologyPrimitiveConstructionBirthComposeEvidence,
    TopologyPrimitiveConstructionBirthComposeExecution,
    TopologyPrimitiveConstructionBirthComposeExecutionError,
    TopologyPrimitiveConstructionBirthComposeProgram,
    TopologyPrimitiveConstructionBirthDeclaredTouchedBasis,
    TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionBirthGraphAuthorityProof,
    TopologyPrimitiveConstructionBirthMaterializationCoverage,
    TopologyPrimitiveConstructionBirthSelectedObligationRow,
    TopologyPrimitiveConstructionBirthTopologyKind,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyPrimitiveConstructionQueryBirthSynopsis, TopologyPrimitiveConstructionQueryEnvelope,
    TopologyPrimitiveConstructionQueryHandoff, TopologyPrimitiveConstructionQueryReceipt,
    TOPOLOGY_PRIMITIVE_CONSTRUCTION_BIRTH_COMPOSE_COLLECTION,
};
pub use crate::projection::runtime_boundary::declared_query_surfaces::truth_surfaces::{
    NamingAttachmentReport, NamingAttachmentRow,
};
#[cfg(test)]
pub(crate) use crate::projection::runtime_boundary::declared_query_surfaces::TopologyDeclaredQuerySurfaces;
#[doc(hidden)]
pub use crate::projection::runtime_boundary::query_runtime::{
    topology_query_runtime_phase_eight_compile_fail_targets,
    topology_query_runtime_phase_eight_golden_paths,
    topology_query_runtime_phase_nine_compile_fail_targets,
    topology_query_runtime_phase_nine_golden_paths, PHASE_EIGHT_EXCLUDED_FOLKLORE_PATHS,
    PHASE_EIGHT_FORBIDDEN_SUBSTITUTION_PATTERNS, PHASE_EIGHT_QUERY_RUNTIME_SCAN_PATHS,
    PHASE_NINE_FORBIDDEN_SUBSTITUTION_PATTERNS, PHASE_NINE_QUERY_RUNTIME_SCAN_PATHS,
    TOPOLOGY_QUERY_RUNTIME_PHASE_EIGHT_COMPILE_FAIL_TARGET_COUNT,
    TOPOLOGY_QUERY_RUNTIME_PHASE_EIGHT_GOLDEN_PATH_COUNT,
    TOPOLOGY_QUERY_RUNTIME_PHASE_NINE_COMPILE_FAIL_TARGET_COUNT,
    TOPOLOGY_QUERY_RUNTIME_PHASE_NINE_GOLDEN_PATH_COUNT,
};
pub use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters, TopologyRuntimeFailure,
};
pub use crate::query_adoption::{
    current_topology_phase_eight_performance_counters,
    current_topology_query_consumer_kit_adoption_status, topology_query_adoption_inventory,
    WorthTopoPhaseEightPerformanceCounters, WorthTopoQueryAdoptionClassification,
    WorthTopoQueryAdoptionForbiddenPattern, WorthTopoQueryAdoptionInventoryRow,
    WorthTopoQueryAuthorityCategory, WorthTopoQueryAuthorityDomain,
    WorthTopoQueryConsumerKitAdoptionError, WorthTopoQueryConsumerKitAdoptionStatus,
};
pub use crate::query_domain::{
    topology_current_head_query_basis_evidence, TopologyCurrentHeadQueryBasisEvidence,
};
pub use crate::topology_operators::{
    topology_operator_command_batch_equivalent_touch_descriptor,
    topology_operator_graph_obligation_adoption_proof, topology_operator_graph_obligation_catalog,
    topology_operator_graph_obligation_registration_declaration,
    topology_operator_graph_obligation_residue_manifest,
    topology_operator_graph_obligation_selector_coverage,
    topology_operator_graph_obligation_support_matrix,
    topology_operator_graph_obligation_support_pin, topology_operator_relation_touch_descriptor,
    topology_operator_touch_descriptor_from_touched_graph_basis, BoundaryMembershipKind,
    EdgeSplitBlueprintCloseout, EdgeSplitBlueprintCloseoutDenial, EdgeSplitOperatorBlueprint,
    EdgeSplitOperatorClassification, EdgeSplitOperatorRow, EdgeSplitOperatorTruthAuthority,
    EdgeSplitRequiredQuerySurface, EdgeSplitValidatorRow, EdgeSplitValidatorRuntimeLane,
    LoopEndpointKind, LoopSuccessorKind, NamingMutationContinuityMatrix,
    PlanarBooleanLoopBlueprintCloseout, PlanarBooleanLoopBlueprintCloseoutDenial,
    PlanarBooleanLoopBlueprintRegistry, PlanarBooleanLoopBlueprintRegistryIdentity,
    PlanarBooleanLoopOperatorClassification, PlanarBooleanLoopOperatorClassificationMatrix,
    PlanarBooleanLoopOperatorRow, PlanarBooleanLoopOperatorTruthAuthority,
    PlanarBooleanLoopRequiredQuerySurface, PlanarBooleanLoopValidatorRegistrationPlan,
    PlanarBooleanLoopValidatorRow, PlanarBooleanLoopValidatorRuntimeLane,
    RejectedMutationScopeReport, RejectedMutationScopeRow, ShellOrWireMembershipKind,
    TopologyAttachBoundaryMembershipDeclaration, TopologyAttachBoundaryMembershipFamily,
    TopologyAttachShellOrWireMembershipDeclaration, TopologyAttachShellOrWireMembershipFamily,
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateInnerLoopOnExistingFaceFamily,
    TopologyCreateTopologyEntityDeclaration, TopologyCreateTopologyEntityFamily,
    TopologyDeclaredTouchedGraphBasisProof, TopologyDerivedRegion,
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachBoundaryMembershipFamily,
    TopologyDetachRadialAdjacencyDeclaration, TopologyDetachRadialAdjacencyFamily,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyDetachShellOrWireMembershipFamily,
    TopologyGraphLifecyclePosture, TopologyLoopSuccessorRewireMember,
    TopologyMutationApplicationMode, TopologyMutationChangedScope,
    TopologyMutationDerivedFallbackPolicy, TopologyMutationDigest, TopologyMutationFamily,
    TopologyMutationNamingOutcome, TopologyMutationNamingReport, TopologyMutationNamingRow,
    TopologyMutationNamingScope, TopologyMutationRejectionClass,
    TopologyOperatorCanonicalDeclaration, TopologyOperatorContinuationExecution,
    TopologyOperatorContinuationExecutionChecked, TopologyOperatorContinuationExecutionOutcome,
    TopologyOperatorContinuationExecutionProof, TopologyOperatorContinuationTarget,
    TopologyOperatorContributionArtifact, TopologyOperatorContributionChecked,
    TopologyOperatorContributionCheckedOutcome, TopologyOperatorContributionInput,
    TopologyOperatorContributionIntent, TopologyOperatorContributionOutcome,
    TopologyOperatorContributionProof, TopologyOperatorDeclarationAdmissionError,
    TopologyOperatorDeclarationLegalityDenial, TopologyOperatorDeclarationLegalityEvidence,
    TopologyOperatorDeclarationOutcome, TopologyOperatorDeclarationReceipt,
    TopologyOperatorDeclarationReceiptChecked, TopologyOperatorDeclarationReceiptProof,
    TopologyOperatorDeclarationReceiptTerminalError, TopologyOperatorEnvelope,
    TopologyOperatorEnvelopeChecked, TopologyOperatorEnvelopeFromProgressedChecked,
    TopologyOperatorEnvelopeFromProgressedProof,
    TopologyOperatorEnvelopeFromProgressedTerminalError, TopologyOperatorEnvelopeProof,
    TopologyOperatorEnvelopeTerminalError, TopologyOperatorGraphObligationAdoptionStatus,
    TopologyOperatorGraphObligationCatalog, TopologyOperatorGraphObligationCatalogRow,
    TopologyOperatorGraphObligationLoweringPath, TopologyOperatorGroupedContributionComposition,
    TopologyOperatorGroupedContributionInput, TopologyOperatorGroupedContributionMemberContext,
    TopologyOperatorGroupedContributionStop, TopologyOperatorGroupedDeclaration,
    TopologyOperatorGroupedDeclarationStop, TopologyOperatorGroupedInput,
    TopologyOperatorGroupedOutcome, TopologyOperatorPreparedContinuation,
    TopologyOperatorPreparedContinuationChecked, TopologyOperatorPreparedContinuationOutcome,
    TopologyOperatorPreparedContinuationProof, TopologyOperatorProgressedDeclaration,
    TopologyOperatorProgressionError, TopologyOperatorRoutePlan, TopologyOperatorRoutePlanChecked,
    TopologyOperatorRoutePlanProof, TopologyOperatorRoutePlanTerminalError,
    TopologyOperatorSignalCompatibilityArtifact, TopologyOperatorSignalCompatibilityChecked,
    TopologyOperatorSignalCompatibilityInput, TopologyOperatorSignalCompatibilityOutcome,
    TopologyOperatorSignalCompatibilityProof, TopologyOperatorSignalCompatibilitySubject,
    TopologyRadialSpliceMember, TopologyRehomeAllOwnedFacesToNewShellDeclaration,
    TopologyRehomeAllOwnedFacesToNewShellFamily,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    TopologyRehomeAllOwnedHalfEdgesToNewWireFamily, TopologyRetireTopologyEntityDeclaration,
    TopologyRetireTopologyEntityFamily, TopologyRewireLoopEndpointDeclaration,
    TopologyRewireLoopEndpointFamily, TopologyRewireLoopSuccessorProgramDeclaration,
    TopologyRewireLoopSuccessorProgramFamily, TopologyShellRehomeFaceMember,
    TopologySpliceRadialAdjacencyDeclaration, TopologySpliceRadialAdjacencyFamily,
    TopologySpliceRadialAdjacencyProgramDeclaration, TopologySpliceRadialAdjacencyProgramFamily,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireFamily,
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    TopologySplitSingleFaceFromTwoFaceShellToNewShellFamily, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedGraphBasis, TopologyTouchedGraphCounters,
    TopologyTouchedOperatingWorld, TopologyTouchedOperatingWorldPosture, TopologyTouchedRelation,
    TopologyTouchedScope, TopologyWireRehomeHalfEdgeMember, TopologyWireSplitHalfEdgeMember,
    TOPOLOGY_OPERATOR_GRAPH_OBLIGATION_FAMILY, TOPOLOGY_OPERATOR_RELATION_COLLECTION,
    TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_OPERATION, TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_PATH,
};
pub use crate::workload_platform::{
    NmtTopologyConstruction, NmtTopologyConstructionCounters, NmtTopologyConstructionDenial,
    NmtTopologyConstructionDenialClass, NmtTopologyConstructionReceipt, NmtTopologyPattern,
    NmtTopologyPosture, NmtTopologyScopeCounters, NmtTopologyScopeDenial, NmtTopologyScopeKind,
    NmtTopologyScopeReceipt, NmtTopologyScopeSet, OpenBoundaryReceipt, OpenLayerPattern,
    OpenLayerStackSpec, OpenPatternIdentityReceipt, OpenRadialFanSpec, OpenSheetPatchSpec,
    OpenWireChainSpec, RadialAdjacencyReceipt, TopologyPostureReceipt, TopologySeed,
    TopologySeedCleanFailClass, TopologySeedCleanFailReasonCode, TopologySeedCleanFailReceipt,
    TopologySeedCleanFailStage, TopologySeedCounters, TopologySeedEntityIdentities,
    TopologySeedKind, TopologySeedNeighborhoodReceipt, TopologySeedQueryReceipts,
    TopologySeedReceipt, TopologySeedRecipe, TopologySeedTopologyPosture,
    TopologySeedValidationReceipt, TopologyWorkload, TopologyWorkloadCounters,
    TopologyWorkloadDeclaration, TopologyWorkloadDeclarationIdentity, TopologyWorkloadDenial,
    TopologyWorkloadEnvelope, TopologyWorkloadFamily, TopologyWorkloadReceipt,
    TopologyWorkloadSupport, TopologyWorkloadSupportPosture,
};
pub use forge_relational::facade::identity::{EntityId, PartitionId};

#[cfg(test)]
pub(crate) use crate::topology_operators::{
    topology_grouped_operator_neighborhood, topology_operator_continuation_target,
    topology_operator_contribution_workflow, topology_operator_signal_workflow,
    TopologyOperatorWorkflowHandleExt,
};

#[cfg(test)]
pub use crate::certification::*;

#[cfg(test)]
pub use crate::projection::runtime_boundary::query_runtime::{
    TopologyQueryMutationLane, TopologyQueryMutationLaneExecutionShape,
    TopologyQueryMutationLaneSupportStatus,
};
