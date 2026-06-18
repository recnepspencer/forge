//! Public API boundary for topology.

pub use crate::brep::topology_graph::{
    TopologyBody, TopologyEdge, TopologyFace, TopologyHalfEdge, TopologyLoop, TopologyLump,
    TopologyModel, TopologyRegion, TopologyShell, TopologyVertex, TopologyView, TopologyWire,
};
pub use crate::construction::{
    prepare_primitive_construction_query_admitted_handoff,
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    prepare_primitive_construction_query_envelope, prepare_primitive_construction_query_handoff,
    prepare_primitive_construction_query_receipt, TopologyConstructionQueryAdmittedHandoffError,
    TopologyConstructionQueryEnvelopeError, TopologyConstructionQueryFactKind,
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryFactRow,
    TopologyConstructionQueryHandoffError, TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryMutationSurface, TopologyConstructionQueryReadSurface,
    TopologyConstructionQueryReceiptError, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyPrimitiveConstructionQueryBirthSynopsis, TopologyPrimitiveConstructionQueryEnvelope,
    TopologyPrimitiveConstructionQueryHandoff, TopologyPrimitiveConstructionQueryReceipt,
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
pub use crate::topology_operators::{
    topology_grouped_operator_neighborhood, topology_operator_continuation_target,
    topology_operator_contribution_workflow, topology_operator_signal_workflow,
    BoundaryMembershipKind, EdgeSplitBlueprintCloseout, EdgeSplitBlueprintCloseoutDenial,
    EdgeSplitOperatorBlueprint, EdgeSplitOperatorClassification, EdgeSplitOperatorProofObligation,
    EdgeSplitOperatorRow, EdgeSplitOperatorTruthAuthority, EdgeSplitRequiredQuerySurface,
    EdgeSplitValidatorProofObligation, EdgeSplitValidatorRow, EdgeSplitValidatorRuntimeLane,
    LoopEndpointKind, LoopSuccessorKind, NamingMutationContinuityMatrix,
    PlanarBooleanLoopBlueprintCloseout, PlanarBooleanLoopBlueprintCloseoutDenial,
    PlanarBooleanLoopBlueprintRegistry, PlanarBooleanLoopBlueprintRegistryIdentity,
    PlanarBooleanLoopOperatorClassification, PlanarBooleanLoopOperatorClassificationMatrix,
    PlanarBooleanLoopOperatorProofObligation, PlanarBooleanLoopOperatorRow,
    PlanarBooleanLoopOperatorTruthAuthority, PlanarBooleanLoopRequiredQuerySurface,
    PlanarBooleanLoopValidatorProofObligation, PlanarBooleanLoopValidatorRegistrationPlan,
    PlanarBooleanLoopValidatorRow, PlanarBooleanLoopValidatorRuntimeLane,
    RejectedMutationScopeReport, RejectedMutationScopeRow, ShellOrWireMembershipKind,
    TopologyAttachBoundaryMembershipDeclaration, TopologyAttachBoundaryMembershipFamily,
    TopologyAttachShellOrWireMembershipDeclaration, TopologyAttachShellOrWireMembershipFamily,
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateInnerLoopOnExistingFaceFamily,
    TopologyCreateTopologyEntityDeclaration, TopologyCreateTopologyEntityFamily,
    TopologyDerivedRegion, TopologyDetachBoundaryMembershipDeclaration,
    TopologyDetachBoundaryMembershipFamily, TopologyDetachRadialAdjacencyDeclaration,
    TopologyDetachRadialAdjacencyFamily, TopologyDetachShellOrWireMembershipDeclaration,
    TopologyDetachShellOrWireMembershipFamily, TopologyLoopSuccessorRewireMember,
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
    TopologyOperatorEnvelopeTerminalError, TopologyOperatorGroupedContributionComposition,
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
    TopologyOperatorWorkflowHandleExt, TopologyRadialSpliceMember,
    TopologyRehomeAllOwnedFacesToNewShellDeclaration, TopologyRehomeAllOwnedFacesToNewShellFamily,
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
    TopologySplitSingleFaceFromTwoFaceShellToNewShellFamily, TopologyWireRehomeHalfEdgeMember,
    TopologyWireSplitHalfEdgeMember,
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
pub use crate::certification::*;

#[cfg(test)]
pub use crate::derived_topology::materialized_graph::{
    MaterializedTopologyView, TopologyMaterializer,
};
#[cfg(test)]
pub use crate::derived_topology::traversal_views::{
    build_topology_read_artifact, certify_topology_view, interpret_topology_view,
    InterpretedTopologyView,
};
#[cfg(test)]
pub use crate::projection::diagnostic_surfaces::derived_read_diagnostics::DerivedReadDiagnostics;
#[cfg(test)]
pub use crate::projection::runtime_boundary::query_runtime::{
    TopologyQueryMutationLane, TopologyQueryMutationLaneExecutionShape,
    TopologyQueryMutationLaneSupportStatus,
};
#[cfg(test)]
pub use crate::validation::{validate_interpreted_topology, DerivedTopologyValidationReport};
