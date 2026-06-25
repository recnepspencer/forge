pub(crate) mod adoption;
pub(crate) mod application;
pub(crate) mod authority_identity;
mod declaration_entry;
mod declared_mutation_sequence_builder;
mod edge_split_blueprint;
mod facade;
mod local_rewrites;
mod loop_reconstruction_blueprint;
mod mutation_digest;
pub(crate) mod mutation_records;
mod mutation_sequence;
mod naming_continuity;
mod query_workflow;
#[cfg(test)]
mod query_workflow_tests;
mod rejection_locality;
mod touched_graph_basis;

pub(crate) use adoption::topology_operator_runtime_graph_obligation_registrations;
pub use adoption::{
    topology_operator_command_batch_equivalent_touch_descriptor,
    topology_operator_graph_obligation_adoption_proof, topology_operator_graph_obligation_catalog,
    topology_operator_graph_obligation_registration_declaration,
    topology_operator_graph_obligation_residue_manifest,
    topology_operator_graph_obligation_selector_coverage,
    topology_operator_graph_obligation_support_matrix,
    topology_operator_graph_obligation_support_pin, topology_operator_relation_touch_descriptor,
    TopologyOperatorGraphObligationAdoptionStatus, TopologyOperatorGraphObligationCatalog,
    TopologyOperatorGraphObligationCatalogRow, TopologyOperatorGraphObligationLoweringPath,
    TOPOLOGY_OPERATOR_GRAPH_OBLIGATION_FAMILY, TOPOLOGY_OPERATOR_RELATION_COLLECTION,
    TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_OPERATION, TOPOLOGY_REWIRE_LOOP_SUCCESSOR_ASPECT_PATH,
};
pub(crate) use application::topology_relation_dependency_path;
pub use declaration_entry::{
    TopologyAttachBoundaryMembershipDeclaration, TopologyAttachBoundaryMembershipFamily,
    TopologyAttachShellOrWireMembershipDeclaration, TopologyAttachShellOrWireMembershipFamily,
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateInnerLoopOnExistingFaceFamily,
    TopologyCreateTopologyEntityDeclaration, TopologyCreateTopologyEntityFamily,
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachBoundaryMembershipFamily,
    TopologyDetachRadialAdjacencyDeclaration, TopologyDetachRadialAdjacencyFamily,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyDetachShellOrWireMembershipFamily,
    TopologyLoopSuccessorRewireMember, TopologyRadialSpliceMember,
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
pub(crate) use declared_mutation_sequence_builder::TopologyDeclaredMutationSequenceBuilder;
pub use edge_split_blueprint::{
    EdgeSplitBlueprintCloseout, EdgeSplitBlueprintCloseoutDenial, EdgeSplitOperatorBlueprint,
    EdgeSplitOperatorClassification, EdgeSplitOperatorRow, EdgeSplitOperatorTruthAuthority,
    EdgeSplitRequiredQuerySurface, EdgeSplitValidatorRow, EdgeSplitValidatorRuntimeLane,
};
pub use facade::TopologyMutationApplicationMode;
pub use loop_reconstruction_blueprint::{
    PlanarBooleanLoopBlueprintCloseout, PlanarBooleanLoopBlueprintCloseoutDenial,
    PlanarBooleanLoopBlueprintRegistry, PlanarBooleanLoopBlueprintRegistryIdentity,
    PlanarBooleanLoopOperatorClassification, PlanarBooleanLoopOperatorClassificationMatrix,
    PlanarBooleanLoopOperatorRow, PlanarBooleanLoopOperatorTruthAuthority,
    PlanarBooleanLoopRequiredQuerySurface, PlanarBooleanLoopValidatorRegistrationPlan,
    PlanarBooleanLoopValidatorRow, PlanarBooleanLoopValidatorRuntimeLane,
};
pub use mutation_digest::{TopologyMutationDigest, TopologyMutationSequenceDigest};
pub(crate) use mutation_records::TopologyDeclaredMutationActionRef;
pub use mutation_records::{
    BoundaryMembershipKind, LoopEndpointKind, LoopSuccessorKind, ShellOrWireMembershipKind,
    TopologyDerivedRegion, TopologyMutationChangedScope, TopologyMutationDerivedFallbackPolicy,
    TopologyMutationFamily, TopologyMutationNamingOutcome, TopologyMutationNamingReport,
    TopologyMutationNamingRow, TopologyMutationNamingScope,
};
#[cfg(test)]
pub(crate) use mutation_sequence::topology_mutation_digest_for_records;
pub(crate) use mutation_sequence::TopologyDeclaredMutationMember;
pub(crate) use mutation_sequence::TopologyDeclaredMutationSequence;
pub use naming_continuity::NamingMutationContinuityMatrix;
pub use query_workflow::{
    topology_grouped_operator_neighborhood, topology_operator_continuation_target,
    topology_operator_contribution_workflow, topology_operator_signal_workflow,
    TopologyOperatorWorkflowHandleExt,
};
pub(crate) use query_workflow::{
    validated_topology_retained_contribution_semantic_projection,
    TopologyRetainedContributionSemanticProjection,
};
pub use query_workflow::{
    TopologyOperatorCanonicalDeclaration, TopologyOperatorContinuationExecution,
    TopologyOperatorContinuationExecutionChecked, TopologyOperatorContinuationExecutionOutcome,
    TopologyOperatorContinuationExecutionProof, TopologyOperatorContinuationTarget,
    TopologyOperatorContributionArtifact, TopologyOperatorContributionChecked,
    TopologyOperatorContributionCheckedOutcome, TopologyOperatorContributionDeclaration,
    TopologyOperatorContributionInput, TopologyOperatorContributionIntent,
    TopologyOperatorContributionOutcome, TopologyOperatorContributionProof,
    TopologyOperatorDeclarationAdmissionError, TopologyOperatorDeclarationLegalityDenial,
    TopologyOperatorDeclarationLegalityEvidence, TopologyOperatorDeclarationOutcome,
    TopologyOperatorDeclarationReceipt, TopologyOperatorDeclarationReceiptChecked,
    TopologyOperatorDeclarationReceiptProof, TopologyOperatorDeclarationReceiptTerminalError,
    TopologyOperatorEnvelope, TopologyOperatorEnvelopeChecked,
    TopologyOperatorEnvelopeFromProgressedChecked, TopologyOperatorEnvelopeFromProgressedProof,
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
};
pub use rejection_locality::{
    RejectedMutationScopeReport, RejectedMutationScopeRow, TopologyMutationRejectionClass,
};
#[cfg(test)]
pub(crate) use touched_graph_basis::test_basis_from_parts;
pub use touched_graph_basis::{
    topology_operator_touch_descriptor_from_touched_graph_basis, TopologyDeclaredTouchedGraphBasis,
    TopologyDeclaredTouchedGraphBasisProof, TopologyGraphLifecyclePosture, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedGraphBasis, TopologyTouchedGraphCounters,
    TopologyTouchedOperatingWorld, TopologyTouchedOperatingWorldPosture, TopologyTouchedRelation,
    TopologyTouchedScope,
};
pub(crate) use touched_graph_basis::{
    topology_touched_graph_basis_from_mutation_sequence,
    TopologyTouchedOperatingWorldIdentityDigest,
};
