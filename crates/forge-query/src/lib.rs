//! `forge-query` owns the typed query facade and canonical query artifact
//! authority.
//!
//! Milestone 1 establishes:
//!
//! - raw authored query and result-shape forms
//! - proof-carrying canonical query and result-shape artifacts
//! - canonical bundle construction with explicit compatibility checks
//! - a single public facade for ordinary consumers

#![forbid(unsafe_code)]

mod application;
mod aspect_field_authoring;
mod authoring;
mod authorized_projection;
mod basis;
mod basis_lifecycle;
mod binding;
mod binding_pipeline;
mod canonicalization;
mod collection;
mod composition;
mod consumer_kit;
mod continuation_pipeline;
mod contribution_composed_orchestration;
mod correspondence;
mod correspondence_history;
mod correspondence_history_parity;
mod declarative_live;
mod diagnostics;
mod domain_capabilities;
mod effect_lifecycle;
mod evidence_identity;
mod execution;
pub mod facade;
mod family_helpers;
mod frontier_planning;
mod frontier_signal_adapter;
mod grouped_authoring;
mod historical;
mod identity;
mod identity_authority;
mod identity_evolution;
mod integration_harness;
mod intent_admission;
mod live;
mod live_performance;
mod lower_runtime_routing;
mod memory_workspace;
mod orchestration_inventory;
mod ordinary_outcome;
mod planning;
mod platform_entry_closeout;
mod policy_basis;
mod policy_certification;
mod policy_delivery;
mod policy_execution_seam;
mod policy_live;
mod policy_narrowing;
mod policy_plan;
mod preview;
mod program;
mod projection_consumption;
mod public_doc_coverage;
mod query_basis_lifecycle;
mod query_context;
mod recovery_boundary;
mod result_shape;
mod runtime;
mod saved_query;
#[macro_use]
mod schema_macro;
mod relationship_proof;
mod schema_view;
mod session_label;
mod signal_compatibility_orchestration;
mod subscription;
mod target_binding;
mod tenant_basis;
mod typed;
mod validation;
mod view_shape;
mod view_shape_live;
mod workflow;

#[cfg(test)]
mod future_signal_test_support;

pub use application::{
    ForgeQueryConcurrentHostileMatrixArtifact, ForgeQueryConcurrentHostileMatrixPosture,
    ForgeQueryConcurrentHostileMatrixSabotage, ForgeQueryConcurrentHostileMatrixSabotageKind,
    ForgeQueryMilestoneClosureStatus, ForgeQueryMilestoneNineSevenDerivedClosure,
    ForgeQueryMilestoneNineSevenPhaseClosure, ForgeQueryPublicBridgeForbiddenAccessFinding,
    ForgeQueryPublicBridgeForbiddenAccessPattern,
    ForgeQueryPublicBridgeProjectionConsumptionEvidence,
    ForgeQueryPublicBridgePublishedProjectionReader, ForgeQueryPublicBridgeReaderLaneCertification,
    ForgeQueryPublicBridgeReaderLaneInventory, ForgeQueryPublicBridgeReaderLanePosture,
    ForgeQueryPublicBridgeReaderLaneSabotage, ForgeQueryPublicBridgeReaderLaneSabotageKind,
    ForgeQueryPublicBridgeReaderLaneSabotageOutcome,
};
pub use consumer_kit::{
    compare_test_backend_write_receipts, evidence_report_adoption_audit,
    graph_obligation_consumer_kit, hard_prohibition_boundary_audit,
    hard_prohibition_boundary_audit_coverage, hard_prohibition_compile_fail_fixtures,
    hard_prohibition_documentation_rows, hard_prohibition_documented_seam_keys,
    hard_prohibition_registry, hard_prohibition_seeded_consumer_sources, in_memory_test_runtime,
    load_support_pin_contract_document, load_support_snapshot_document, project_support_snapshot,
    project_workspace_support_snapshot, query_boundary_source_inventory,
    query_test_backend_residue_audit, render_hard_prohibition_reference, support_pinning_contract,
    EvidenceReport, EvidenceReportDeclaration, EvidenceReportError, EvidenceReportErrorKind,
    EvidenceReportField, EvidenceReportFieldKind, EvidenceReportFieldParticipation,
    EvidenceReportFieldValue, EvidenceReportScope, ForgeQueryBoundaryAuditCoverage,
    ForgeQueryBoundaryAuditCoverageMechanism, ForgeQueryBoundaryAuditCoverageRow,
    ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditErrorKind,
    ForgeQueryBoundaryAuditEvaluation, ForgeQueryBoundaryAuditFailure,
    ForgeQueryBoundaryAuditFinding, ForgeQueryBoundaryAuditFindingKind,
    ForgeQueryBoundaryAuditReport, ForgeQueryBoundaryAuditSeededSource,
    ForgeQueryBoundaryAuditSource, ForgeQueryBoundaryAuditSourceInventory,
    ForgeQueryBoundaryAuditSourceInventoryBuilder, ForgeQueryBoundaryAuditSourceInventoryFile,
    ForgeQueryBoundaryAuditSourceSet, ForgeQueryBoundaryAuditSourceSite,
    ForgeQueryBoundaryAuditSyntaxClass, ForgeQueryEvidenceReportAdoptionAudit,
    ForgeQueryEvidenceReportAdoptionError, ForgeQueryEvidenceReportAdoptionErrorKind,
    ForgeQueryEvidenceReportAdoptionEvaluation, ForgeQueryEvidenceReportAdoptionFinding,
    ForgeQueryEvidenceReportAdoptionFindingKind, ForgeQueryEvidenceReportAdoptionReport,
    ForgeQueryEvidenceReportAdoptionResidueClassification,
    ForgeQueryEvidenceReportAdoptionResidueRow, ForgeQueryEvidenceReportAdoptionSource,
    ForgeQueryEvidenceReportAdoptionSourceSet, ForgeQueryEvidenceReportAdoptionSyntaxClass,
    ForgeQueryGraphObligationAdoptionManifest, ForgeQueryGraphObligationAdoptionProof,
    ForgeQueryGraphObligationConsumerKit, ForgeQueryGraphObligationConsumerKitError,
    ForgeQueryGraphObligationConsumerKitErrorKind,
    ForgeQueryGraphObligationConsumerRegistrationDeclaration,
    ForgeQueryGraphObligationExecutionBackedAdoptionProof, ForgeQueryGraphObligationExecutionProof,
    ForgeQueryGraphObligationExecutionProofRow, ForgeQueryGraphObligationInMemoryProof,
    ForgeQueryGraphObligationInMemorySelectedObligation,
    ForgeQueryGraphObligationInMemoryTestWorkspace, ForgeQueryGraphObligationLocalCeremonyAudit,
    ForgeQueryGraphObligationLocalCeremonyFinding, ForgeQueryGraphObligationResidueCertification,
    ForgeQueryGraphObligationResidueManifest, ForgeQueryGraphObligationResidueRow,
    ForgeQueryGraphObligationSelectorCoverageDeclaration,
    ForgeQueryGraphObligationSelectorCoverageRow, ForgeQueryGraphObligationSupportPin,
    ForgeQueryGraphObligationSupportPinFinding, ForgeQueryHardProhibitionBoundaryAudit,
    ForgeQueryHardProhibitionDocumentationRow, ForgeQueryInMemoryTestRuntimeBuilder,
    ForgeQueryObservedSupportPin, ForgeQueryPinnedSupportStatus, ForgeQueryPinnedTeachingPosture,
    ForgeQueryProhibitedSeam, ForgeQueryProhibitionCompileFailFixture,
    ForgeQueryProhibitionEnforcementTier, ForgeQueryProhibitionRegistry,
    ForgeQueryProhibitionRegistryRow, ForgeQuerySupportPinContract,
    ForgeQuerySupportPinContractBuilder, ForgeQuerySupportPinContractDocument,
    ForgeQuerySupportPinContractSchemaVersion, ForgeQuerySupportPinDeclaration,
    ForgeQuerySupportPinFinding, ForgeQuerySupportPinFindingKind, ForgeQuerySupportPinReport,
    ForgeQuerySupportPinRequirement, ForgeQuerySupportPinRequirementDraft,
    ForgeQuerySupportPinningError, ForgeQuerySupportPinningErrorKind, ForgeQuerySupportSnapshot,
    ForgeQuerySupportSnapshotDocument, ForgeQuerySupportSnapshotError,
    ForgeQuerySupportSnapshotErrorKind, ForgeQuerySupportSnapshotRow,
    ForgeQuerySupportSnapshotSchemaVersion, ForgeQueryTestBackendEquivalenceReport,
    ForgeQueryTestBackendEquivalenceRow, ForgeQueryTestBackendError,
    ForgeQueryTestBackendErrorKind, ForgeQueryTestBackendResidueAudit,
    ForgeQueryTestBackendResidueFinding, ForgeQueryTestBackendResidueReport,
    ForgeQueryTestBackendSchema,
};
pub use continuation_pipeline::{
    ForgeQueryContinuationBasisPosture, ForgeQueryContinuationExecution,
    ForgeQueryContinuationExecutionChecked, ForgeQueryContinuationExecutionOutcome,
    ForgeQueryContinuationExecutionTranscript, ForgeQueryContinuationRuntimeContract,
    ForgeQueryContinuationTruthContext, ForgeQueryContinuationWorkspaceContract,
    ForgeQueryExecutePreparedContinuationRequest, ForgeQueryPreparedContinuation,
    ForgeQueryPreparedContinuationAuthorityWitness, ForgeQueryPreparedContinuationBasisKind,
    ForgeQueryPreparedContinuationBasisWitness, ForgeQueryPreparedContinuationChecked,
    ForgeQueryPreparedContinuationExecutionMode,
    ForgeQueryPreparedContinuationExecutionReadmission, ForgeQueryPreparedContinuationFamily,
    ForgeQueryPreparedContinuationFreshnessPosture, ForgeQueryPreparedContinuationOutcome,
    ForgeQueryPreparedContinuationRequest, ForgeQueryPreparedContinuationSignalPosture,
    ForgeQueryPreparedContinuationTranscript,
};
pub use contribution_composed_orchestration::{
    ForgeQueryContributionComposedClassification, ForgeQueryContributionComposedComposition,
    ForgeQueryContributionComposedContribution, ForgeQueryContributionComposedDeclarationRecord,
    ForgeQueryContributionComposedIntentClassification,
    ForgeQueryContributionComposedIntentRequestDescriptor,
    ForgeQueryContributionComposedIntentResult, ForgeQueryContributionComposedIntentStageKind,
    ForgeQueryContributionComposedIntentStageResult,
    ForgeQueryContributionComposedMaterializationPolicy,
    ForgeQueryContributionComposedOrchestration,
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationInput,
    ForgeQueryContributionComposedOrchestrationOutcome,
    ForgeQueryContributionComposedOrchestrationPosture,
    ForgeQueryContributionComposedOrchestrationTranscript, ForgeQueryContributionComposedStop,
    ForgeQueryContributionComposedSummary, ForgeQueryContributionIntent,
};
pub use evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceIdentityComparisonError,
    ForgeQueryEvidenceIdentityScheme, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
pub use family_helpers::{
    ForgeQueryFamilyHelpers, ForgeQueryGeometryActiveFaceSelectionHelperFamily,
    ForgeQueryGeometryFamilyHelpers, ForgeQueryGeometryMaterialAttachmentHelperFamily,
    ForgeQueryGeometryMaterialAttachmentInput, ForgeQueryGeometryNeighborhoodHelperFamily,
};
pub use grouped_authoring::{
    ForgeQueryGroupedAspectParticipationSummary, ForgeQueryGroupedAtomicity,
    ForgeQueryGroupedContinuityAssumption, ForgeQueryGroupedContributionAssignment,
    ForgeQueryGroupedContributionComposition, ForgeQueryGroupedContributionInput,
    ForgeQueryGroupedContributionMemberContext, ForgeQueryGroupedContributionStop,
    ForgeQueryGroupedDeclarationArtifact, ForgeQueryGroupedDeclarationAspectRecord,
    ForgeQueryGroupedDeclarationChecked, ForgeQueryGroupedDeclarationInput,
    ForgeQueryGroupedDeclarationMember, ForgeQueryGroupedDeclarationStop,
    ForgeQueryGroupedDeclarationStopKind, ForgeQueryGroupedEnvelopeChecked,
    ForgeQueryGroupedEnvelopeMember, ForgeQueryGroupedEnvelopeTranscript, ForgeQueryGroupedIntent,
    ForgeQueryGroupedMemberOrchestrationStop, ForgeQueryGroupedMemberRole,
    ForgeQueryGroupedOrchestration, ForgeQueryGroupedOrchestrationAlignmentStop,
    ForgeQueryGroupedOrchestrationChecked, ForgeQueryGroupedOrchestrationProof,
    ForgeQueryGroupedOrchestrationStop, ForgeQueryGroupedOrchestrationTranscript,
    ForgeQueryGroupedOrdering, ForgeQueryGroupedReceiptChecked, ForgeQueryGroupedReceiptTranscript,
    ForgeQueryGroupedRouteChecked, ForgeQueryGroupedRouteTranscript, ForgeQueryGroupedSemantics,
    ForgeQueryGroupedSharedPostureClaim, ForgeQueryGroupedSupportFeature,
    ForgeQueryGroupedSupportReport, ForgeQueryGroupedSupportStatus,
};
pub use orchestration_inventory::{
    ForgeQueryOrchestrationAspectPosture, ForgeQueryOrchestrationBasisPosture,
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationCheckedTopologyKind,
    ForgeQueryOrchestrationCollaborativeExtensionPosture,
    ForgeQueryOrchestrationContributionCompatibility,
    ForgeQueryOrchestrationContributionCompatibilityKind, ForgeQueryOrchestrationInventoryAudit,
    ForgeQueryOrchestrationLowerAuthorityAttachment, ForgeQueryOrchestrationPolicyTenantPosture,
    ForgeQueryOrchestrationProofContract, ForgeQueryOrchestrationSemanticProfile,
    ForgeQueryOrchestrationStrategyAttachment, ForgeQueryOrchestrationSupportSurface,
    ForgeQueryOrchestrationSurfaceCertificationReference,
    ForgeQueryOrchestrationSurfaceDocReference, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationSurfaceInventory, ForgeQueryOrchestrationSurfaceRow,
    ForgeQueryOrchestrationSurfaceVisibility, ForgeQueryOrchestrationTranscriptFamily,
};
pub use ordinary_outcome::{
    ForgeQueryOrdinaryBindingCheckedTopologyKind, ForgeQueryOrdinaryCheckedTopology,
    ForgeQueryOrdinaryContinuationCheckedTopologyKind,
    ForgeQueryOrdinaryContributionComposedCheckedTopologyKind, ForgeQueryOrdinaryNextStep,
    ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
    ForgeQueryOrdinaryRuntimeAsyncPostureKind, ForgeQueryOrdinaryRuntimeBasisPostureKind,
    ForgeQueryOrdinaryRuntimeCausePostureKind, ForgeQueryOrdinaryRuntimePosture,
    ForgeQueryOrdinaryRuntimePostureKind,
    ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
};
pub use platform_entry_closeout::{
    certify_platform_entry_closeout, forge_query_platform_entry_closeout_surface,
    forge_query_platform_entry_compile_fail_boundary_digest,
    forge_query_platform_entry_compile_fail_manifest, forge_query_platform_entry_hostile_manifest,
    forge_query_platform_entry_parity_manifest, ForgeQueryPlatformEntryAlignmentAudit,
    ForgeQueryPlatformEntryCloseoutBundle, ForgeQueryPlatformEntryCloseoutOutput,
    ForgeQueryPlatformEntryCloseoutSurface, ForgeQueryPlatformEntryCompileFailAudit,
    ForgeQueryPlatformEntryCompileFailManifest, ForgeQueryPlatformEntryHostileAudit,
    ForgeQueryPlatformEntryHostileDivergenceClass, ForgeQueryPlatformEntryHostileManifest,
    ForgeQueryPlatformEntryHostileRow, ForgeQueryPlatformEntryParityAssertionClass,
    ForgeQueryPlatformEntryParityAudit, ForgeQueryPlatformEntryParityLane,
    ForgeQueryPlatformEntryParityManifest, ForgeQueryPlatformEntryParityRow,
    ForgeQueryPlatformEntryUiProofKind, ForgeQueryPlatformEntryUiProofRow,
};
pub use public_doc_coverage::{
    forge_query_public_doc_coverage_golden_transcript_digest,
    forge_query_public_doc_coverage_golden_transcripts, ForgeQueryPublicDocCoverageAudit,
    ForgeQueryPublicDocCoverageInventory, ForgeQueryPublicDocCoverageRow,
    ForgeQueryPublicDocReference, ForgeQueryPublicGoldenTranscript,
    ForgeQueryPublicGoldenTranscriptKind, ForgeQueryPublicJourneyKind,
};
pub use recovery_boundary::{
    forge_query_recovery_brief_from_continuation_execution_checked,
    forge_query_recovery_brief_from_continuation_execution_proof,
    forge_query_recovery_brief_from_contribution_composed_checked,
    forge_query_recovery_brief_from_contribution_composed_proof,
    forge_query_recovery_brief_from_declaration_entry_checked,
    forge_query_recovery_brief_from_declaration_entry_proof,
    forge_query_recovery_brief_from_declaration_receipt_checked,
    forge_query_recovery_brief_from_declaration_route_plan_checked,
    forge_query_recovery_brief_from_ordinary_outcome,
    forge_query_recovery_brief_from_prepared_continuation_checked,
    forge_query_recovery_brief_from_prepared_continuation_proof,
    forge_query_recovery_brief_from_signal_compatibility_checked,
    forge_query_recovery_brief_from_signal_compatibility_proof, ForgeQueryRecoveryAction,
    ForgeQueryRecoveryAspectPosture, ForgeQueryRecoveryAuthoritySurface,
    ForgeQueryRecoveryBasisPosture, ForgeQueryRecoveryBrief, ForgeQueryRecoveryConflictPosture,
    ForgeQueryRecoveryEvidenceStrength, ForgeQueryRecoveryExplanation,
    ForgeQueryRecoveryFoundationalDiagnosticContext, ForgeQueryRecoveryFoundationalSupportContext,
    ForgeQueryRecoveryGroupedMemberContext, ForgeQueryRecoveryMaterialization,
    ForgeQueryRecoveryRequest, ForgeQueryRecoveryRequestKind, ForgeQueryRecoverySourceFamily,
    ForgeQueryRecoveryStopFamily, ForgeQueryRecoveryStopKind,
};
pub use runtime::{
    ForgeQueryConcurrentHostileMatrixCounterSnapshot, ForgeQueryConcurrentHostileMatrixTopology,
    ForgeQueryConcurrentSubmissionIntake, ForgeQueryConcurrentSubmissionLane,
    ForgeQueryConcurrentSubmissionRecord,
};
pub use session_label::{
    ForgeQuerySessionLabel, ForgeQuerySessionLabelError, ForgeQuerySessionLabelSegment,
    ForgeQuerySessionNamespace,
};
pub use signal_compatibility_orchestration::{
    ForgeQuerySignalCompatibilityOrchestration, ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationClass,
    ForgeQuerySignalCompatibilityOrchestrationInput,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
    ForgeQuerySignalCompatibilityOrchestrationTranscript,
};

#[cfg(test)]
mod harness;
