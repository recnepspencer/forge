//! Public API boundary for topology.

pub mod compiled_product_family {
    pub use crate::compiled_product_family::{
        current_topology_compiled_product_family_catalog, select_topology_compiled_product_family,
        DeterministicDigest, SelectedTopologyCompiledProductFamily, TopologyAuthorityBasisPosture,
        TopologyCompiledProductConsumer, TopologyCompiledProductFamilyAdmittedInput,
        TopologyCompiledProductFamilyCatalog, TopologyCompiledProductFamilyCatalogCounters,
        TopologyCompiledProductFamilyDeclaration, TopologyCompiledProductFamilyError,
        TopologyCompiledProductFamilyErrorKind, TopologyCompiledProductFamilyIdentity,
        TopologyCompiledProductLoweredIdentity, TopologyEquivalencePolicyPosture,
        TopologyLocalityFootprintPosture, TopologyPriorProofPosture, TopologyStageIdentityPosture,
        TopologyValidatorEvidenceRolePosture,
    };
}

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
pub use crate::compiled_product_reuse_decision::TopologyDerivedReuseDecisionPosture;
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
pub use crate::derived_invalidation_deletion_closeout::{
    close_derived_invalidation_deletion, current_deletion_source_firewall,
    DerivedInvalidationDeletionAudit, DerivedInvalidationDeletionCloseout,
    DerivedInvalidationDeletionCounters, DerivedInvalidationDeletionDisposition,
    DerivedInvalidationDeletionError, DerivedInvalidationDeletionErrorKind,
    DerivedInvalidationDeletionLedger, DerivedInvalidationDeletionRow,
    DerivedInvalidationDeletionSourceFirewall, DerivedInvalidationDeletionSourceFirewallViolation,
    DerivedInvalidationPhaseNineSeed, DerivedInvalidationResidueAudit,
    DerivedInvalidationResidueAuditRow,
};
pub use crate::derived_invalidation_execution::{
    DerivedInvalidationDeniedProductExecutionRow, DerivedInvalidationDiagnosticProjection,
    DerivedInvalidationDiagnosticRow, DerivedInvalidationExecutedProductRow,
    DerivedInvalidationExecutionCounters, DerivedInvalidationExecutionError,
    DerivedInvalidationExecutionErrorKind, DerivedInvalidationExecutionOutcome,
    DerivedInvalidationExecutionReceipt, DerivedInvalidationResidueExecutionRow,
    DerivedInvalidationUnaffectedProductExecutionRow,
};
pub use crate::derived_invalidation_migrated_products::{
    close_loop_cycle_migration_slice, close_materialized_graph_migration_slice,
    close_traversal_views_migration_slice, LoopCycleBoundarySourceRow,
    LoopCycleDerivedProductOutput, LoopCycleExecutionInput, LoopCycleMigrationCloseout,
    LoopCycleMigrationCounters, LoopCycleMigrationError, LoopCycleOldAuthorityResidue,
    LoopCycleOldAuthorityResidueRow, LoopCyclePhaseSixSeed, LoopCycleProductRow,
    MaterializedGraphDerivedProductOutput, MaterializedGraphDiagnosticProjection,
    MaterializedGraphExecutionInput, MaterializedGraphMigrationCloseout,
    MaterializedGraphMigrationCounters, MaterializedGraphMigrationError,
    MaterializedGraphOldAuthorityResidue, MaterializedGraphOldAuthorityResidueRow,
    MaterializedGraphPhaseTenSeed, MaterializedGraphProductEntityRow,
    MaterializedGraphProductRelationRow, MaterializedGraphReadEntityRow,
    MaterializedGraphReadRelationRow, MaterializedGraphReadSource,
    MaterializedGraphReadStageExecutor, MaterializedGraphReadStageReceipt,
    TraversalViewsDerivedProductOutput, TraversalViewsDiagnosticProjection,
    TraversalViewsExecutionInput, TraversalViewsMigrationCloseout, TraversalViewsMigrationCounters,
    TraversalViewsMigrationError, TraversalViewsOldAuthorityResidue,
    TraversalViewsOldAuthorityResidueRow, TraversalViewsPhaseElevenSeed, TraversalViewsProductRow,
    TraversalViewsReadSource, TraversalViewsReadStageExecutor, TraversalViewsReadStageReceipt,
    TraversalViewsSourceRow,
};
pub use crate::derived_invalidation_milestone_ten_closeout::{
    close_derived_invalidation_milestone_ten, DerivedInvalidationMilestoneElevenLookupReadiness,
    DerivedInvalidationMilestoneElevenProductReceiptRef, DerivedInvalidationMilestoneElevenSeed,
    DerivedInvalidationMilestoneTenCloseout, DerivedInvalidationMilestoneTenCounters,
    DerivedInvalidationMilestoneTenError, DerivedInvalidationMilestoneTenErrorKind,
    DerivedInvalidationMilestoneTenPerformanceProof,
    DerivedInvalidationMilestoneTenPerformanceSlopeCase,
    DerivedInvalidationMilestoneTenProductSummaryReport,
    DerivedInvalidationMilestoneTenProductSummaryRow,
};
pub use crate::derived_invalidation_operator_cutover::{
    current_operator_cutover_source_firewall, DerivedInvalidationOperatorCutoverCloseout,
    DerivedInvalidationOperatorCutoverCounters, DerivedInvalidationOperatorCutoverError,
    DerivedInvalidationOperatorCutoverErrorKind, DerivedInvalidationOperatorCutoverReceipt,
    DerivedInvalidationOperatorCutoverSourceFirewall,
    DerivedInvalidationOperatorCutoverSourceFirewallViolation, DerivedInvalidationPhaseEightSeed,
    DerivedInvalidationProjectionReadStageReceipt, ProjectionReadStageConsumptionScope,
};
pub use crate::derived_invalidation_selected_plan::{
    DerivedInvalidationDenialKind, DerivedInvalidationDenialRow, DerivedInvalidationDensityPolicy,
    DerivedInvalidationExecutionAdmission, DerivedInvalidationLegalitySupportEvidence,
    DerivedInvalidationPhaseFourSeed, DerivedInvalidationPlannedDisposition,
    DerivedInvalidationQuerySupportEvidence, DerivedInvalidationResidueRow,
    DerivedInvalidationSelectedPlan, DerivedInvalidationSelectedRow,
    DerivedInvalidationSelectionCounters, DerivedInvalidationSelectionError,
    DerivedInvalidationSelectionErrorKind, DerivedInvalidationTouchedClosure,
    DerivedInvalidationUnaffectedRow,
};
pub use crate::derived_topology::compiled_product_consumer_cutover::{
    current_topology_consumer_residue_manifest, TopologyConsumerResidueDisposition,
    TopologyConsumerResidueOwner, TopologyConsumerResidueRow,
};
pub use crate::projection::planner_owned_routing::compiled_product_reuse_route::{
    current_topology_compiled_product_reuse_route_packet, TopologyCompiledProductReuseRoutePacket,
};
pub use crate::projection::planner_owned_routing::{
    admit_milestone_seven_five_overlap_readiness_consumer,
    TopologyMilestoneSevenFiveOverlapReadinessConsumer, TopologyMilestoneSevenFiveReadinessError,
    TopologyMilestoneSevenFiveReadinessErrorKind,
};
pub use crate::projection::query_backed_consumer_cutover::{
    admit_topology_query_backed_consumer_cutover, current_query_backed_consumer_residue_manifest,
    current_topology_query_backed_consumer_cutover, QueryBackedConsumerResidueDisposition,
    QueryBackedConsumerResidueOwner, QueryBackedConsumerResidueRow,
    TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerCutoverCurrentError,
    TopologyQueryBackedConsumerFamilyRow, TopologyReadModelReusePosture,
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
pub use crate::projection::touched_graph_parity_closeout::invalidation_family::{
    current_topology_invalidation_route_packet, TopologyInvalidationRoutePacket,
    TopologyInvalidationRoutePacketCurrentError,
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
pub use crate::query_native_runtime_boundary::{
    WorthTopologyNativeAspectField, WorthTopologyNativeAspectValue,
    WorthTopologyNativeCarrierBoundaryError, WorthTopologyNativeFieldPath,
    WorthTopologyNativeSetAspectInput, WorthTopologyQueryNativeRuntimeBoundaryInventory,
    WorthTopologyQueryNativeRuntimeBoundaryInventoryError,
    WorthTopologyQueryNativeRuntimeBoundaryInventoryRow,
    WorthTopologyQueryNativeRuntimeBoundaryResidueStatus,
    WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol,
};
pub use crate::replay_family_catalog::{
    admit_topology_replay_family_declaration, admit_topology_replay_family_identity,
    current_topology_replay_family_catalog, TopologyReplayFamilyCatalog,
    TopologyReplayFamilyDeclaration, TopologyReplayFamilyDeclarationInput,
    TopologyReplayFamilyIdentity, TopologyReplayFamilyIdentityAuthority,
    TopologyReplayFamilyLocalityPosture, TopologyReplayFamilyPriorProofPosture,
    TopologyReplayFamilyScopeProductPosture, TopologyReplayFamilyStageIndexPosture,
    TopologyReplayFamilyWorkloadDependencyPosture,
};
pub use crate::replay_undo_semantic_graph::{
    admit_prepared_topology_replay_semantic_graph_input,
    admit_topology_replay_semantic_graph_input, admit_topology_undo_semantic_graph_input,
    lower_topology_replay_equivalence_basis,
    lower_topology_replay_equivalence_basis_from_admitted_input,
    lower_topology_replay_equivalence_basis_from_scope_product,
    lower_topology_replay_equivalence_basis_from_selected_plan,
    lower_topology_replay_equivalence_basis_from_touched_closure,
    lower_topology_replay_scope_identity, lower_topology_replay_scope_identity_from_admitted_input,
    lower_topology_replay_scope_identity_from_scope_product,
    lower_topology_replay_scope_identity_from_touched_closure,
    lower_topology_replay_scope_product_from_admitted_input,
    lower_topology_replay_scope_product_from_selected_plan, lower_topology_undo_equivalence_basis,
    lower_topology_undo_equivalence_basis_from_admitted_input,
    lower_topology_undo_equivalence_basis_from_scope_product,
    lower_topology_undo_equivalence_basis_from_selected_plan,
    lower_topology_undo_equivalence_basis_from_touched_closure, lower_topology_undo_scope_identity,
    lower_topology_undo_scope_identity_from_admitted_input,
    lower_topology_undo_scope_identity_from_scope_product,
    lower_topology_undo_scope_identity_from_touched_closure,
    lower_topology_undo_scope_product_from_admitted_input,
    lower_topology_undo_scope_product_from_materialized_graph_request,
    lower_topology_undo_scope_product_from_selected_plan,
    lower_topology_undo_scope_product_from_traversal_views_request,
    prepare_topology_replay_semantic_graph_request,
    prepare_topology_replay_semantic_graph_stage_identity, select_topology_replay_plan,
    select_topology_undo_plan, MaterializedGraphRollbackRequest, TopologyReplayPlanError,
    TopologyReplayScopeProduct, TopologyReplayScopeProductCounters, TopologyReplaySelectedPlan,
    TopologyReplaySemanticGraphAdmissionError, TopologyReplaySemanticGraphAdmissionRequest,
    TopologyReplaySemanticGraphAdmittedInput, TopologyReplaySemanticGraphPreparationRequest,
    TopologyReplaySemanticGraphPreparedRequest, TopologyReplaySemanticGraphPreparedStageAuthority,
    TopologyReplaySemanticGraphSelectedPlanIdentity, TopologyReplaySemanticGraphStageIdentity,
    TopologyReplaySemanticGraphStageReceiptAuthority, TopologyUndoFamilyExecutionError,
    TopologyUndoPlanError, TopologyUndoScopeProduct, TopologyUndoScopeProductCounters,
    TopologyUndoSelectedPlan, TopologyUndoSemanticGraphAdmissionError,
    TopologyUndoSemanticGraphAdmissionRequest, TopologyUndoSemanticGraphAdmittedInput,
    TraversalViewsRollbackRequest,
};
#[cfg(any(test, feature = "test-support-lowering"))]
pub use crate::replay_undo_semantic_graph::{
    traversal_views_topology_undo_fixture, TraversalViewsTopologyUndoFixture,
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
pub use crate::topology_operators::{
    PlanarBooleanOverlapBlueprintRegistry, PlanarBooleanOverlapBlueprintRegistryIdentity,
    PlanarBooleanOverlapOperatorClassification, PlanarBooleanOverlapOperatorClassificationMatrix,
    PlanarBooleanOverlapOperatorRow, PlanarBooleanOverlapOperatorTruthAuthority,
    PlanarBooleanOverlapRequiredQuerySurface, PlanarBooleanOverlapValidatorRegistrationPlan,
    PlanarBooleanOverlapValidatorRow, PlanarBooleanOverlapValidatorRuntimeLane,
};
pub use crate::touched_graph_parity_closeout::{
    current_topology_invalidation_coverage_contributor, TOPOLOGY_INVALIDATION_CLAIM_PATH,
};
pub use crate::touched_graph_parity_closeout::{
    current_topology_read_family_coverage_contributor, TOPOLOGY_READ_FAMILY_CLAIM_PATH,
};
pub use crate::touched_graph_parity_closeout::{
    current_topology_validator_invariant_coverage_contributor,
    TOPOLOGY_VALIDATOR_INVARIANT_CLAIM_PATH,
};
#[doc(hidden)]
pub use crate::validation_authority_inventory::{
    validation_authority_inventory_compile_fail_targets,
    VALIDATION_AUTHORITY_INVENTORY_COMPILE_FAIL_TARGET_COUNT,
};
pub use crate::validator_invariant_catalog::{
    current_worth_topology_legality_catalog_closeout, WorthTopologyDiagnosticProjectionPosture,
    WorthTopologyEnforcementPhase,
    WorthTopologyGraphScopedCustomInvariantRegistrationProjectionRow,
    WorthTopologyInvariantFamilyIdentity, WorthTopologyInvariantFamilyRecord,
    WorthTopologyLegalityCatalog, WorthTopologyLegalityCatalogCloseout,
    WorthTopologyLegalityCatalogError, WorthTopologyLegalityCatalogNoExecutionProof,
    WorthTopologyLegalityCatalogPhaseThreeSeed, WorthTopologyLegalityCatalogSourceFirewallReport,
    WorthTopologyLegalityCatalogSourceFirewallViolation, WorthTopologyLegalityFamilyIdentity,
    WorthTopologyLegalityFamilyRecord, WorthTopologyLegalityFamilySourceAuthorityKind,
    WorthTopologyLegalityFamilySourceProof, WorthTopologyLegalitySelectionCloseout,
    WorthTopologyLegalitySelectionCounters, WorthTopologyLegalitySelectionDenial,
    WorthTopologyLegalitySelectionDenialKind, WorthTopologyLegalitySelectionPhaseFourSeed,
    WorthTopologyLoopWiringAdmittedLocalFacts, WorthTopologyLoopWiringDiagnosticProjection,
    WorthTopologyLoopWiringHalfEdgeWitnessRow, WorthTopologyLoopWiringLoopWitnessRow,
    WorthTopologyLoopWiringViolationKind, WorthTopologyLoopWiringWitnessInput,
    WorthTopologyLoopWiringWitnessIntakeReceipt, WorthTopologyLoopWiringWitnessRow,
    WorthTopologyMilestoneNineCloseout, WorthTopologyMilestoneNineCloseoutCounters,
    WorthTopologyMilestoneNineCloseoutDenial, WorthTopologyMilestoneNineCloseoutDenialKind,
    WorthTopologyMilestoneNineDeletionDisposition, WorthTopologyMilestoneNineDeletionLedgerReport,
    WorthTopologyMilestoneNineDeletionLedgerRow, WorthTopologyMilestoneNinePublicProof,
    WorthTopologyMilestoneNineResidueAuditReport, WorthTopologyMilestoneNineResidueAuditRow,
    WorthTopologyMilestoneNineResidueStatus, WorthTopologyMilestoneNineSourceFirewallReport,
    WorthTopologyMilestoneTenSeed, WorthTopologyOperatorCertificationCutoverCloseout,
    WorthTopologyOperatorCertificationCutoverCounters,
    WorthTopologyOperatorCertificationCutoverDenial,
    WorthTopologyOperatorCertificationCutoverDenialKind,
    WorthTopologyOperatorCertificationCutoverPhaseEightSeed,
    WorthTopologyOperatorCertificationCutoverSourceFirewallReport,
    WorthTopologyOperatorCertificationOldExpectationResidueReport,
    WorthTopologyOperatorCertificationOldExpectationResidueRow,
    WorthTopologyOperatorCertificationOldExpectationResidueStatus,
    WorthTopologyOperatorSelectedObligationCloseoutRow,
    WorthTopologyOperatorSelectedObligationSupportPostureRow,
    WorthTopologyQueryGraphObligationCatalogProjection,
    WorthTopologyQueryGraphObligationRegistrationProjectionRow,
    WorthTopologyRelationalInvariantCatalogCloseout,
    WorthTopologyRelationalInvariantCatalogCounters, WorthTopologyRelationalInvariantCatalogDenial,
    WorthTopologyRelationalInvariantCatalogDenialKind,
    WorthTopologyRelationalInvariantCatalogPhaseSixSeed,
    WorthTopologyRelationalInvariantCatalogSourceFirewallReport,
    WorthTopologyRelationalInvariantOldPackResidueReport,
    WorthTopologyRelationalInvariantOldPackResidueRow,
    WorthTopologyRelationalInvariantOldPackResidueStatus,
    WorthTopologyRelationalInvariantOrdinaryAuthorityAdmission,
    WorthTopologyRelationalInvariantQueryRegistrationArtifactProjection,
    WorthTopologyRelationalInvariantQueryRegistrationBundle,
    WorthTopologyRelationalInvariantRejectedAuthorityKind, WorthTopologyRequiredAccessPosture,
    WorthTopologySelectedGraphObligationDiagnosticWitness,
    WorthTopologySelectedGraphObligationEnforcementCloseout,
    WorthTopologySelectedGraphObligationEnforcementCounters,
    WorthTopologySelectedGraphObligationEnforcementDenial,
    WorthTopologySelectedGraphObligationEnforcementDenialKind,
    WorthTopologySelectedGraphObligationEnforcementOutcome,
    WorthTopologySelectedGraphObligationEnforcementPhaseSevenSeed,
    WorthTopologySelectedGraphObligationEnforcementReceipt,
    WorthTopologySelectedGraphObligationEnforcementSourceFirewallReport,
    WorthTopologySelectedGraphObligationExecutionInput,
    WorthTopologySelectedLegalityObligationPlan, WorthTopologySelectedLegalityObligationRow,
    WorthTopologySelectedRelationalInvariantFamilyRow,
    WorthTopologySelectedValidatorEnforcementCloseout,
    WorthTopologySelectedValidatorEnforcementCounters,
    WorthTopologySelectedValidatorEnforcementDenial,
    WorthTopologySelectedValidatorEnforcementDenialKind,
    WorthTopologySelectedValidatorEnforcementOutcome,
    WorthTopologySelectedValidatorEnforcementPhaseFiveSeed,
    WorthTopologySelectedValidatorEnforcementReceipt,
    WorthTopologySelectedValidatorEnforcementSourceFirewallReport,
    WorthTopologyTouchedApplicability, WorthTopologyValidatorFamilyIdentity,
    WorthTopologyValidatorFamilyRecord, WorthTopologyValidatorRoutingClosure,
    WorthTopologyWitnessPosture,
};
#[doc(hidden)]
pub use crate::validator_invariant_catalog::{
    worth_topology_legality_catalog_compile_fail_targets,
    WorthTopologyLegalityCatalogCompileFailTarget,
    WORTH_TOPOLOGY_LEGALITY_CATALOG_COMPILE_FAIL_TARGET_COUNT,
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
pub use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};

pub use crate::topology_operators::{
    topology_grouped_operator_neighborhood, topology_operator_continuation_target,
    topology_operator_contribution_workflow, topology_operator_signal_workflow,
    TopologyOperatorWorkflowHandleExt,
};
pub use crate::undo_family_catalog::{
    admit_topology_undo_family_declaration, admit_topology_undo_family_identity,
    current_topology_undo_family_catalog, TopologyUndoFamilyCatalog, TopologyUndoFamilyDeclaration,
    TopologyUndoFamilyDeclarationInput, TopologyUndoFamilyIdentity,
    TopologyUndoFamilyIdentityAuthority, TopologyUndoFamilyLocalityPosture,
    TopologyUndoFamilyPriorProofPosture, TopologyUndoFamilyScopeProductPosture,
    TopologyUndoFamilyStageIndexPosture, TopologyUndoFamilyWorkloadDependencyPosture,
};

#[cfg(test)]
pub use crate::certification::*;

#[cfg(test)]
pub use crate::projection::runtime_boundary::query_runtime::{
    TopologyQueryMutationLane, TopologyQueryMutationLaneExecutionShape,
    TopologyQueryMutationLaneSupportStatus,
};
