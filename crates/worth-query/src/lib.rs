//! `worth-query` owns the typed query facade and canonical query artifact
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
    WorthQueryConcurrentHostileMatrixArtifact, WorthQueryConcurrentHostileMatrixPosture,
    WorthQueryConcurrentHostileMatrixSabotage, WorthQueryConcurrentHostileMatrixSabotageKind,
    WorthQueryMilestoneClosureStatus, WorthQueryMilestoneNineSevenDerivedClosure,
    WorthQueryMilestoneNineSevenPhaseClosure, WorthQueryPublicBridgeForbiddenAccessFinding,
    WorthQueryPublicBridgeForbiddenAccessPattern,
    WorthQueryPublicBridgeProjectionConsumptionEvidence,
    WorthQueryPublicBridgePublishedProjectionReader, WorthQueryPublicBridgeReaderLaneCertification,
    WorthQueryPublicBridgeReaderLaneInventory, WorthQueryPublicBridgeReaderLanePosture,
    WorthQueryPublicBridgeReaderLaneSabotage, WorthQueryPublicBridgeReaderLaneSabotageKind,
    WorthQueryPublicBridgeReaderLaneSabotageOutcome,
};
pub use consumer_kit::{
    compare_test_backend_write_receipts, downstream_authority_adoption,
    evidence_report_adoption_audit, graph_obligation_consumer_kit, graph_read_bypass_adoption,
    graph_read_bypass_audit, hard_prohibition_boundary_audit,
    hard_prohibition_boundary_audit_coverage, hard_prohibition_compile_fail_fixtures,
    hard_prohibition_documentation_rows, hard_prohibition_documented_seam_keys,
    hard_prohibition_registry, hard_prohibition_seeded_consumer_sources, in_memory_test_runtime,
    load_support_pin_contract_terminal_json_document, load_support_snapshot_terminal_json_document,
    project_support_snapshot, project_workspace_support_snapshot, query_boundary_source_inventory,
    query_consumer_residue_audit, query_test_backend_residue_audit,
    render_hard_prohibition_reference, support_pinning_contract,
    worth_query_consumer_residue_certification_evidence, worth_query_consumer_residue_registry,
    worth_query_graph_read_bypass_registry, worth_query_test_backend_residue_classes,
    EvidenceReport, EvidenceReportDeclaration, EvidenceReportError, EvidenceReportErrorKind,
    EvidenceReportField, EvidenceReportFieldKind, EvidenceReportFieldParticipation,
    EvidenceReportFieldValue, EvidenceReportScope, WorthQueryBoundaryAuditCoverage,
    WorthQueryBoundaryAuditCoverageMechanism, WorthQueryBoundaryAuditCoverageRow,
    WorthQueryBoundaryAuditError, WorthQueryBoundaryAuditErrorKind,
    WorthQueryBoundaryAuditEvaluation, WorthQueryBoundaryAuditFailure,
    WorthQueryBoundaryAuditFinding, WorthQueryBoundaryAuditFindingKind,
    WorthQueryBoundaryAuditReport, WorthQueryBoundaryAuditSeededSource,
    WorthQueryBoundaryAuditSource, WorthQueryBoundaryAuditSourceInventory,
    WorthQueryBoundaryAuditSourceInventoryBuilder, WorthQueryBoundaryAuditSourceInventoryFile,
    WorthQueryBoundaryAuditSourceSet, WorthQueryBoundaryAuditSourceSite,
    WorthQueryBoundaryAuditSyntaxClass, WorthQueryConsumerResidueAudit,
    WorthQueryConsumerResidueCertificationCaseEvidence, WorthQueryConsumerResidueClass,
    WorthQueryConsumerResidueDetection, WorthQueryConsumerResidueFinding,
    WorthQueryConsumerResidueQueryOwnedRootAuthority, WorthQueryConsumerResidueRegistryRow,
    WorthQueryConsumerResidueReport, WorthQueryConsumerResidueSourceInventory,
    WorthQueryConsumerResidueSourceSite, WorthQueryDownstreamAuthorityAdoption,
    WorthQueryDownstreamAuthorityAdoptionManifest, WorthQueryDownstreamAuthorityAdoptionProof,
    WorthQueryEvidenceReportAdoptionAudit, WorthQueryEvidenceReportAdoptionError,
    WorthQueryEvidenceReportAdoptionErrorKind, WorthQueryEvidenceReportAdoptionEvaluation,
    WorthQueryEvidenceReportAdoptionFinding, WorthQueryEvidenceReportAdoptionFindingKind,
    WorthQueryEvidenceReportAdoptionReport, WorthQueryEvidenceReportAdoptionResidueClassification,
    WorthQueryEvidenceReportAdoptionResidueRow, WorthQueryEvidenceReportAdoptionSource,
    WorthQueryEvidenceReportAdoptionSourceSet, WorthQueryEvidenceReportAdoptionSyntaxClass,
    WorthQueryExternalSupportPinContractTerminalJsonDocument,
    WorthQueryExternalSupportSnapshotTerminalJsonDocument,
    WorthQueryGraphObligationAdoptionManifest, WorthQueryGraphObligationAdoptionProof,
    WorthQueryGraphObligationConsumerKit, WorthQueryGraphObligationConsumerKitError,
    WorthQueryGraphObligationConsumerKitErrorKind,
    WorthQueryGraphObligationConsumerRegistrationDeclaration,
    WorthQueryGraphObligationExecutionBackedAdoptionProof, WorthQueryGraphObligationExecutionProof,
    WorthQueryGraphObligationExecutionProofRow, WorthQueryGraphObligationInMemoryProof,
    WorthQueryGraphObligationInMemorySelectedObligation,
    WorthQueryGraphObligationInMemoryTestWorkspace, WorthQueryGraphObligationLocalCeremonyAudit,
    WorthQueryGraphObligationLocalCeremonyFinding, WorthQueryGraphObligationResidueCertification,
    WorthQueryGraphObligationResidueManifest, WorthQueryGraphObligationResidueRow,
    WorthQueryGraphObligationSelectorCoverageDeclaration,
    WorthQueryGraphObligationSelectorCoverageRow, WorthQueryGraphObligationSupportPin,
    WorthQueryGraphObligationSupportPinFinding, WorthQueryGraphReadBypassAdoption,
    WorthQueryGraphReadBypassAdoptionError, WorthQueryGraphReadBypassAdoptionErrorKind,
    WorthQueryGraphReadBypassAdoptionManifest, WorthQueryGraphReadBypassAdoptionProof,
    WorthQueryGraphReadBypassAudit, WorthQueryGraphReadBypassAuthorityViolation,
    WorthQueryGraphReadBypassClass, WorthQueryGraphReadBypassCounters,
    WorthQueryGraphReadBypassDetection, WorthQueryGraphReadBypassFinding,
    WorthQueryGraphReadBypassRegistryRow, WorthQueryGraphReadBypassReport,
    WorthQueryGraphReadBypassReportResidueCertification,
    WorthQueryGraphReadBypassResidueCertification, WorthQueryGraphReadBypassResidueError,
    WorthQueryGraphReadBypassResidueErrorKind, WorthQueryGraphReadBypassResidueManifest,
    WorthQueryGraphReadBypassResidueRow, WorthQueryHardProhibitionBoundaryAudit,
    WorthQueryHardProhibitionDocumentationRow, WorthQueryInMemoryTestRuntimeBuilder,
    WorthQueryObservedSupportPin, WorthQueryPinnedSupportStatus, WorthQueryPinnedTeachingPosture,
    WorthQueryProhibitedSeam, WorthQueryProhibitionCompileFailFixture,
    WorthQueryProhibitionEnforcementTier, WorthQueryProhibitionRegistry,
    WorthQueryProhibitionRegistryRow, WorthQuerySupportPinContract,
    WorthQuerySupportPinContractBuilder, WorthQuerySupportPinContractSchemaVersion,
    WorthQuerySupportPinContractTerminalJsonDocument, WorthQuerySupportPinDeclaration,
    WorthQuerySupportPinFinding, WorthQuerySupportPinFindingKind, WorthQuerySupportPinReport,
    WorthQuerySupportPinRequirement, WorthQuerySupportPinRequirementDraft,
    WorthQuerySupportPinningError, WorthQuerySupportPinningErrorKind, WorthQuerySupportSnapshot,
    WorthQuerySupportSnapshotError, WorthQuerySupportSnapshotErrorKind,
    WorthQuerySupportSnapshotRow, WorthQuerySupportSnapshotSchemaVersion,
    WorthQuerySupportSnapshotTerminalJsonDocument, WorthQueryTestBackendEquivalenceReport,
    WorthQueryTestBackendEquivalenceRow, WorthQueryTestBackendError,
    WorthQueryTestBackendErrorKind, WorthQueryTestBackendResidueAudit,
    WorthQueryTestBackendResidueFinding, WorthQueryTestBackendResidueReport,
    WorthQueryTestBackendSchema,
};
pub use continuation_pipeline::{
    WorthQueryContinuationBasisPosture, WorthQueryContinuationExecution,
    WorthQueryContinuationExecutionChecked, WorthQueryContinuationExecutionOutcome,
    WorthQueryContinuationExecutionTranscript, WorthQueryContinuationRuntimeContract,
    WorthQueryContinuationTruthContext, WorthQueryContinuationWorkspaceContract,
    WorthQueryExecutePreparedContinuationRequest, WorthQueryPreparedContinuation,
    WorthQueryPreparedContinuationAuthorityWitness, WorthQueryPreparedContinuationBasisKind,
    WorthQueryPreparedContinuationBasisWitness, WorthQueryPreparedContinuationChecked,
    WorthQueryPreparedContinuationExecutionMode,
    WorthQueryPreparedContinuationExecutionReadmission, WorthQueryPreparedContinuationFamily,
    WorthQueryPreparedContinuationFreshnessPosture, WorthQueryPreparedContinuationOutcome,
    WorthQueryPreparedContinuationRequest, WorthQueryPreparedContinuationSignalPosture,
    WorthQueryPreparedContinuationTranscript,
};
pub use contribution_composed_orchestration::{
    WorthQueryContributionComposedClassification, WorthQueryContributionComposedComposition,
    WorthQueryContributionComposedContribution, WorthQueryContributionComposedDeclarationRecord,
    WorthQueryContributionComposedIntentClassification,
    WorthQueryContributionComposedIntentRequestDescriptor,
    WorthQueryContributionComposedIntentResult, WorthQueryContributionComposedIntentStageKind,
    WorthQueryContributionComposedIntentStageResult,
    WorthQueryContributionComposedMaterializationPolicy,
    WorthQueryContributionComposedOrchestration,
    WorthQueryContributionComposedOrchestrationChecked,
    WorthQueryContributionComposedOrchestrationInput,
    WorthQueryContributionComposedOrchestrationOutcome,
    WorthQueryContributionComposedOrchestrationPosture,
    WorthQueryContributionComposedOrchestrationTranscript, WorthQueryContributionComposedStop,
    WorthQueryContributionComposedSummary, WorthQueryContributionIntent,
};
pub use evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceIdentityComparisonError,
    WorthQueryEvidenceIdentityScheme, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
pub use family_helpers::{
    WorthQueryFamilyHelpers, WorthQueryGeometryActiveFaceSelectionHelperFamily,
    WorthQueryGeometryFamilyHelpers, WorthQueryGeometryMaterialAttachmentHelperFamily,
    WorthQueryGeometryMaterialAttachmentInput, WorthQueryGeometryNeighborhoodHelperFamily,
};
pub use grouped_authoring::{
    WorthQueryGroupedAspectParticipationSummary, WorthQueryGroupedAtomicity,
    WorthQueryGroupedContinuityAssumption, WorthQueryGroupedContributionAssignment,
    WorthQueryGroupedContributionComposition, WorthQueryGroupedContributionInput,
    WorthQueryGroupedContributionMemberContext, WorthQueryGroupedContributionStop,
    WorthQueryGroupedDeclarationArtifact, WorthQueryGroupedDeclarationAspectRecord,
    WorthQueryGroupedDeclarationChecked, WorthQueryGroupedDeclarationInput,
    WorthQueryGroupedDeclarationMember, WorthQueryGroupedDeclarationStop,
    WorthQueryGroupedDeclarationStopKind, WorthQueryGroupedEnvelopeChecked,
    WorthQueryGroupedEnvelopeMember, WorthQueryGroupedEnvelopeTranscript, WorthQueryGroupedIntent,
    WorthQueryGroupedMemberOrchestrationStop, WorthQueryGroupedMemberRole,
    WorthQueryGroupedOrchestration, WorthQueryGroupedOrchestrationAlignmentStop,
    WorthQueryGroupedOrchestrationChecked, WorthQueryGroupedOrchestrationProof,
    WorthQueryGroupedOrchestrationStop, WorthQueryGroupedOrchestrationTranscript,
    WorthQueryGroupedOrdering, WorthQueryGroupedReceiptChecked, WorthQueryGroupedReceiptTranscript,
    WorthQueryGroupedRouteChecked, WorthQueryGroupedRouteTranscript, WorthQueryGroupedSemantics,
    WorthQueryGroupedSharedPostureClaim, WorthQueryGroupedSupportFeature,
    WorthQueryGroupedSupportReport, WorthQueryGroupedSupportStatus,
};
pub use orchestration_inventory::{
    WorthQueryOrchestrationAspectPosture, WorthQueryOrchestrationBasisPosture,
    WorthQueryOrchestrationBindingProjection, WorthQueryOrchestrationCheckedTopologyKind,
    WorthQueryOrchestrationCollaborativeExtensionPosture,
    WorthQueryOrchestrationContributionCompatibility,
    WorthQueryOrchestrationContributionCompatibilityKind, WorthQueryOrchestrationInventoryAudit,
    WorthQueryOrchestrationLowerAuthorityAttachment, WorthQueryOrchestrationPolicyTenantPosture,
    WorthQueryOrchestrationProofContract, WorthQueryOrchestrationSemanticProfile,
    WorthQueryOrchestrationStrategyAttachment, WorthQueryOrchestrationSupportSurface,
    WorthQueryOrchestrationSurfaceCertificationReference,
    WorthQueryOrchestrationSurfaceDocReference, WorthQueryOrchestrationSurfaceFamily,
    WorthQueryOrchestrationSurfaceInventory, WorthQueryOrchestrationSurfaceRow,
    WorthQueryOrchestrationSurfaceVisibility, WorthQueryOrchestrationTranscriptFamily,
};
pub use ordinary_outcome::{
    WorthQueryOrdinaryBindingCheckedTopologyKind, WorthQueryOrdinaryCheckedTopology,
    WorthQueryOrdinaryContinuationCheckedTopologyKind,
    WorthQueryOrdinaryContributionComposedCheckedTopologyKind, WorthQueryOrdinaryNextStep,
    WorthQueryOrdinaryOutcome, WorthQueryOrdinaryPosture, WorthQueryOrdinaryPostureKind,
    WorthQueryOrdinaryRuntimeAsyncPostureKind, WorthQueryOrdinaryRuntimeBasisPostureKind,
    WorthQueryOrdinaryRuntimeCausePostureKind, WorthQueryOrdinaryRuntimePosture,
    WorthQueryOrdinaryRuntimePostureKind,
    WorthQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
};
pub use platform_entry_closeout::{
    certify_platform_entry_closeout, worth_query_platform_entry_closeout_surface,
    worth_query_platform_entry_compile_fail_boundary_digest,
    worth_query_platform_entry_compile_fail_manifest, worth_query_platform_entry_hostile_manifest,
    worth_query_platform_entry_parity_manifest, WorthQueryPlatformEntryAlignmentAudit,
    WorthQueryPlatformEntryCloseoutBundle, WorthQueryPlatformEntryCloseoutOutput,
    WorthQueryPlatformEntryCloseoutSurface, WorthQueryPlatformEntryCompileFailAudit,
    WorthQueryPlatformEntryCompileFailManifest, WorthQueryPlatformEntryHostileAudit,
    WorthQueryPlatformEntryHostileDivergenceClass, WorthQueryPlatformEntryHostileManifest,
    WorthQueryPlatformEntryHostileRow, WorthQueryPlatformEntryParityAssertionClass,
    WorthQueryPlatformEntryParityAudit, WorthQueryPlatformEntryParityLane,
    WorthQueryPlatformEntryParityManifest, WorthQueryPlatformEntryParityRow,
    WorthQueryPlatformEntryUiProofKind, WorthQueryPlatformEntryUiProofRow,
};
pub use public_doc_coverage::{
    worth_query_public_doc_coverage_golden_transcript_digest,
    worth_query_public_doc_coverage_golden_transcripts, WorthQueryPublicDocCoverageAudit,
    WorthQueryPublicDocCoverageInventory, WorthQueryPublicDocCoverageRow,
    WorthQueryPublicDocReference, WorthQueryPublicGoldenTranscript,
    WorthQueryPublicGoldenTranscriptKind, WorthQueryPublicJourneyKind,
};
pub use recovery_boundary::{
    worth_query_recovery_brief_from_continuation_execution_checked,
    worth_query_recovery_brief_from_continuation_execution_proof,
    worth_query_recovery_brief_from_contribution_composed_checked,
    worth_query_recovery_brief_from_contribution_composed_proof,
    worth_query_recovery_brief_from_declaration_entry_checked,
    worth_query_recovery_brief_from_declaration_entry_proof,
    worth_query_recovery_brief_from_declaration_receipt_checked,
    worth_query_recovery_brief_from_declaration_route_plan_checked,
    worth_query_recovery_brief_from_ordinary_outcome,
    worth_query_recovery_brief_from_prepared_continuation_checked,
    worth_query_recovery_brief_from_prepared_continuation_proof,
    worth_query_recovery_brief_from_signal_compatibility_checked,
    worth_query_recovery_brief_from_signal_compatibility_proof, WorthQueryRecoveryAction,
    WorthQueryRecoveryAspectPosture, WorthQueryRecoveryAuthoritySurface,
    WorthQueryRecoveryBasisPosture, WorthQueryRecoveryBrief, WorthQueryRecoveryConflictPosture,
    WorthQueryRecoveryEvidenceStrength, WorthQueryRecoveryExplanation,
    WorthQueryRecoveryFoundationalDiagnosticContext, WorthQueryRecoveryFoundationalSupportContext,
    WorthQueryRecoveryGroupedMemberContext, WorthQueryRecoveryMaterialization,
    WorthQueryRecoveryRequest, WorthQueryRecoveryRequestKind, WorthQueryRecoverySourceFamily,
    WorthQueryRecoveryStopFamily, WorthQueryRecoveryStopKind,
};
pub use runtime::{
    WorthQueryConcurrentHostileMatrixCounterSnapshot, WorthQueryConcurrentHostileMatrixTopology,
    WorthQueryConcurrentSubmissionIntake, WorthQueryConcurrentSubmissionLane,
    WorthQueryConcurrentSubmissionRecord,
};
pub use session_label::{
    WorthQuerySessionLabel, WorthQuerySessionLabelError, WorthQuerySessionLabelSegment,
    WorthQuerySessionNamespace,
};
pub use signal_compatibility_orchestration::{
    WorthQuerySignalCompatibilityOrchestration, WorthQuerySignalCompatibilityOrchestrationChecked,
    WorthQuerySignalCompatibilityOrchestrationClass,
    WorthQuerySignalCompatibilityOrchestrationInput,
    WorthQuerySignalCompatibilityOrchestrationOutcome,
    WorthQuerySignalCompatibilityOrchestrationTranscript,
};

#[cfg(test)]
mod harness;
