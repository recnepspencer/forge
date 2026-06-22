//! `forge-server` owns the typed server facade and network bootstrap boundary.
//!
//! Milestone 1 establishes:
//!
//! - one facade-owned bootstrap path
//! - explicit surface-family registration
//! - validated runtime assembly before serving
//! - an internal transport boundary rather than framework-shaped public API

#![forbid(unsafe_code)]

mod config;
mod declaration_intake;
mod diagnostics;
pub mod facade;
pub mod forge_native;
pub mod middleware;
mod operation_admission;
mod operation_planning;
mod operation_readiness;
mod operation_registry;
mod operation_request;
mod operation_runtime_certification;
mod operation_scheduler;
pub mod operator_evidence;
mod product_adapter;
mod product_operation_contract;
mod product_session;
mod product_session_coordination;
pub mod query_dependency_audit;
pub mod query_handoff;
mod registration;
pub mod request_context;
pub mod response;
mod runtime;
pub mod surfaces;
mod transport;

pub use config::{
    ForgeServerBindAddress, ForgeServerConfig, ForgeServerConfigBuilder, ForgeServerConfigError,
    ForgeServerMiddlewareConfig, ForgeServerMiddlewareConfigBuilder,
    ForgeServerMiddlewareConfigError, ForgeServerOperatorEvidenceConfig,
    ForgeServerOperatorEvidenceConfigBuilder, ForgeServerOperatorEvidenceConfigError,
    ForgeServerQueryHandoffConfig, ForgeServerQueryHandoffConfigBuilder,
    ForgeServerQueryHandoffConfigError, ForgeServerRequestContextConfig,
    ForgeServerRequestContextConfigBuilder, ForgeServerRequestContextConfigError,
    ForgeServerResponseConfig, ForgeServerResponseConfigBuilder, ForgeServerResponseConfigError,
};
pub use declaration_intake::{
    ForgeServerAdmittedDirectDeclaration, ForgeServerDirectDeclaration,
    ForgeServerDirectDeclarationBuilder, ForgeServerDirectDeclarationDenial,
    ForgeServerDirectDeclarationDenialCode, ForgeServerDirectDeclarationError,
    ForgeServerDirectDeclarationSource, ForgeServerDirectDeclarationSourceKind,
    ForgeServerDirectDeclarationSourceSupportStatus, ForgeServerDirectSupportSnapshot,
    ForgeServerDirectViewShape, ForgeServerPreparedDirectDeclaration,
};
pub use diagnostics::ForgeServerCounterSnapshot;
pub use facade::{ForgeServer, ForgeServerBuildError, ForgeServerBuilder};
pub use forge_native::{
    ForgeServerDirectAsyncResultState, ForgeServerDirectContextArtifact,
    ForgeServerDirectDeclarationSnapshot, ForgeServerDirectDeliveryClass,
    ForgeServerDirectDeliveryContract, ForgeServerDirectDeliveryOutcome,
    ForgeServerDirectDeliveryRequest, ForgeServerDirectFactReceipt, ForgeServerDirectFreshnessMode,
    ForgeServerDirectInspection, ForgeServerDirectInspectionOutcome,
    ForgeServerDirectLeaseDeclaration, ForgeServerDirectLeaseDeclarationOutcome,
    ForgeServerDirectMaterializationDigest, ForgeServerDirectMaterializedRemaskArtifact,
    ForgeServerDirectMutation, ForgeServerDirectMutationOutcome, ForgeServerDirectMutationResult,
    ForgeServerDirectProductFlow, ForgeServerDirectProjection,
    ForgeServerDirectProjectionConsumption, ForgeServerDirectProjectionFactReceipt,
    ForgeServerDirectProjectionOutcome, ForgeServerDirectProjectionRequest,
    ForgeServerDirectProvenance, ForgeServerDirectRead, ForgeServerDirectReadOutcome,
    ForgeServerDirectRemaskArtifact, ForgeServerDirectRemaskDisposition,
    ForgeServerDirectRemaskPosture, ForgeServerDirectRetainedPosture, ForgeServerDirectState,
    ForgeServerDirectStateOutcome, ForgeServerDirectTemporalState, ForgeServerForgeNativeDeferred,
    ForgeServerForgeNativeDirectFacade, ForgeServerForgeNativeFacade,
    ForgeServerForgeNativeFailure, ForgeServerForgeNativePreparationOutcome,
    ForgeServerForgeNativePreparedSession, ForgeServerForgeNativeProductFacade,
    ForgeServerForgeNativeProductMutationCommand, ForgeServerForgeNativeProductSessionFacade,
    ForgeServerForgeNativeRebindRequired, ForgeServerForgeNativeSession,
    ForgeServerForgeNativeSessionDenial, ForgeServerForgeNativeSessionDenialCode,
    ForgeServerForgeNativeSessionInput, ForgeServerForgeNativeSessionInputBuilder,
    ForgeServerForgeNativeSessionInputError, ForgeServerForgeNativeSessionOutcome,
    ForgeServerForgeNativeStale, ForgeServerForgeNativeSurfaceRoot,
};
pub use middleware::{
    ForgeServerAdmission, ForgeServerAdmissionOutcome, ForgeServerDenial, ForgeServerDenialCode,
    ForgeServerDenialPriority, ForgeServerMiddlewareDeferred, ForgeServerMiddlewareFacade,
    ForgeServerMiddlewareFailure, ForgeServerMiddlewareRebindRequired, ForgeServerMiddlewareStale,
    ForgeServerPipelineInput, ForgeServerPipelineIntent, ForgeServerPipelineStep,
    ForgeServerPreparedQueryHandoffIntent, ForgeServerPreparedQueryHandoffKind,
};
pub use operation_admission::{
    ForgeServerOperationAdmissionDenial, ForgeServerOperationAdmissionDenialCode,
    ForgeServerOperationAdmissionFacade, ForgeServerOperationAdmissionPosture,
    ForgeServerOperationAuthorityDeclaration, ForgeServerOperationAuthorityFootprint,
    ForgeServerOperationAuthorityKind, ForgeServerOperationAuthorityMetadata,
    ForgeServerOperationAuthorizationProof, ForgeServerOperationConcurrencyClass,
    ForgeServerOperationConcurrencyDenial, ForgeServerOperationConcurrencyDenialCode,
    ForgeServerOperationConcurrencyFacade, ForgeServerOperationFootprintReceipt,
    ForgeServerOperationScope, ForgeServerProductSessionCoordinationTarget,
    ForgeServerProductSupportPosture, ForgeServerSharedReadBasisKind,
};
pub use operation_planning::{
    ForgeServerLoweredOperationPlan, ForgeServerOperationExecutionStrategy,
    ForgeServerOperationPlanCounters, ForgeServerOperationPlanDenial,
    ForgeServerOperationPlanDenialCode, ForgeServerOperationPlanEvidencePolicy,
    ForgeServerOperationPlanProof, ForgeServerOperationPlanReceipt, ForgeServerOperationPlanner,
    ForgeServerOperationPlannerInput,
};
pub use operation_readiness::{
    ForgeServerCompatibilityMutationPrecondition,
    ForgeServerCompatibilityMutationPreconditionContext, ForgeServerOperationPreconditionPosture,
    ForgeServerOperationQuerySupportContext, ForgeServerOperationReadinessClosure,
    ForgeServerOperationReadinessDenial, ForgeServerOperationReadinessDenialCode,
    ForgeServerOperationReadinessDenialFacts, ForgeServerOperationReadinessFacade,
    ForgeServerOperationSupportCompositionReceipt, ForgeServerOperationSupportPosture,
    ForgeServerProductBasisPrecondition,
};
pub use operation_registry::{
    ForgeServerOperationAuthorizationPolicy, ForgeServerOperationCapabilities,
    ForgeServerOperationDenial, ForgeServerOperationFamily, ForgeServerOperationInventory,
    ForgeServerOperationInventoryRow, ForgeServerOperationRegistration,
    ForgeServerOperationRegistry, ForgeServerOperationRegistryError,
};
pub use operation_request::{
    ForgeServerOperationIdentity, ForgeServerOperationInputEnvelope, ForgeServerOperationRequest,
    ForgeServerOperationRequestDenial, ForgeServerOperationRequestDenialCode,
    ForgeServerOperationRequestFacade, ForgeServerOperationRequestInput,
    ForgeServerOperationRequestInputBuilder, ForgeServerOperationRequestReceipt,
};
pub use operation_runtime_certification::{
    ForgeServerEditorLikeOperationFixture, ForgeServerNoProductSemanticsCertification,
    ForgeServerOperationRuntimeCloseoutDigest, ForgeServerProductEditorReadinessCertification,
    ForgeServerProductIdempotentReplayCertificationProof,
    ForgeServerProductMutationCertificationProof,
    ForgeServerProductOperationRuntimeArtifactRequirements,
    ForgeServerProductOperationRuntimeCertification,
    ForgeServerProductOperationRuntimeCertificationFacade,
    ForgeServerProductOperationRuntimeRequirementRow,
    ForgeServerProductOperationRuntimeRequirementStatus,
    ForgeServerProductOperationRuntimeSupportRow,
    ForgeServerProductPressureShapeCertificationProof,
    ForgeServerProductRouteParityCertificationProof, ForgeServerProductRouteParityEntry,
    ForgeServerProductSharedReadCertificationProof,
    ForgeServerProductStaleApplyDenialCertificationProof,
};
pub use operation_scheduler::{
    ForgeServerExecutedOperationBatch, ForgeServerOperationExecutionSlot,
    ForgeServerOperationScheduler, ForgeServerOperationSchedulerCounters,
    ForgeServerScheduledMutationResult, ForgeServerScheduledOperationBatch,
    ForgeServerScheduledOperationOutcome, ForgeServerScheduledOperationTraceEntry,
    ForgeServerSchedulerCancellationDirective, ForgeServerSchedulerCancellationPosture,
    ForgeServerSchedulerCertificationSabotage, ForgeServerSchedulerConflictDenial,
    ForgeServerSchedulerConflictDenialCode, ForgeServerSchedulerConflictDenialFacts,
    ForgeServerSchedulerFailurePosture, ForgeServerSchedulerRuntimeFailure,
};
pub use operator_evidence::{
    ForgeServerEvidenceInput, ForgeServerEvidenceTransform, ForgeServerObservedCounter,
    ForgeServerOperatorCounterReceipt, ForgeServerOperatorEvidenceClass,
    ForgeServerOperatorEvidenceFacade, ForgeServerOperatorEvidenceMaterializationError,
    ForgeServerOperatorEvidencePlan, ForgeServerOperatorEvidenceRecord,
};
pub use product_adapter::{
    ForgeServerCompletedProductOperation, ForgeServerExecutedProductReadBatch,
    ForgeServerLoweredProductOperationPlan, ForgeServerProductAdapterCertificationCode,
    ForgeServerProductAdapterCertificationError, ForgeServerProductAdapterExecutionError,
    ForgeServerProductAdapterRegistrationReceipt, ForgeServerProductAdapterRegistry,
    ForgeServerProductAdapterRegistryError, ForgeServerProductApplicationAdapter,
    ForgeServerProductApplicationAdapterRegistration,
    ForgeServerProductOperationAuthorityRequirement, ForgeServerProductOperationBasisKind,
    ForgeServerProductOperationDeclaration, ForgeServerProductOperationDenial,
    ForgeServerProductOperationDenialCode, ForgeServerProductOperationDenialFacts,
    ForgeServerProductOperationEnvelope, ForgeServerProductOperationEnvelopeKind,
    ForgeServerProductOperationErrorMap, ForgeServerProductOperationErrorMaps,
    ForgeServerProductOperationExecutionBoundary, ForgeServerProductOperationFailure,
    ForgeServerProductOperationInput, ForgeServerProductOperationOutcome,
    ForgeServerProductOperationPayload, ForgeServerProductOperationReplayClass,
    ForgeServerProductOperationReplayDiagnostics, ForgeServerProductOperationRuntime,
    ForgeServerProductOperationSuccess, ForgeServerProductOperationSupportSnapshot,
    ForgeServerProductOperationSurfaceDenial, ForgeServerProductOperationSurfaceDenialCode,
    ForgeServerProductOperationSurfaceDenialFacts, ForgeServerProductPayloadSchemaValidator,
    ForgeServerProductSchedulerAdmission, ForgeServerScheduledProductOperation,
};
pub use product_operation_contract::{
    ForgeServerProductIdempotencyConflict, ForgeServerProductIdempotencyKey,
    ForgeServerProductIdempotencyRecord, ForgeServerProductOperationBaseDigest,
    ForgeServerProductOperationReplayReceipt, ForgeServerProductRebaseRequired,
    ForgeServerProductSnapshotPrecondition, ForgeServerProductStaleBasisDenial,
};
pub use product_session::{
    ForgeServerProductSession, ForgeServerProductSessionClock,
    ForgeServerProductSessionCounterSnapshot, ForgeServerProductSessionCreationRequest,
    ForgeServerProductSessionDenial, ForgeServerProductSessionDenialCode,
    ForgeServerProductSessionExpiryPosture, ForgeServerProductSessionIdentity,
    ForgeServerProductSessionLifecycle, ForgeServerProductSessionRegistry,
    ForgeServerSystemProductSessionClock,
};
pub use product_session_coordination::{
    ForgeServerCompletedProductSessionCoordination,
    ForgeServerLoweredProductSessionCoordinationPlan, ForgeServerProductSessionCoordinationCommand,
    ForgeServerProductSessionCoordinationRuntime, ForgeServerProductSessionSchedulerAdmission,
};
pub use query_dependency_audit::{
    ForgeServerQueryDependencyAudit, ForgeServerQueryDependencyAuditFacade,
    ForgeServerQueryDependencyAuditPathKind, ForgeServerQueryDependencyAuditProvenance,
    ForgeServerQueryDependencyAuditReceipt, ForgeServerQueryDependencyAuditRow,
    ForgeServerQueryDependencyAuditRowId, ForgeServerQueryDependencyBoundaryAuditProvenance,
    ForgeServerQueryDependencyClosurePosture, ForgeServerQueryDependencyConsumerKitPosture,
    ForgeServerQueryDependencyCoveredPathInventory, ForgeServerQueryDependencyRuntimeReadiness,
    ForgeServerQueryDependencyScopePosture, ForgeServerQueryDependencySupportPinProvenance,
    ForgeServerQueryDependencySupportPosture,
    ForgeServerQueryDependencyTestBackendResidueProvenance,
};
pub use query_handoff::{
    ForgeServerQueryHandoff, ForgeServerQueryHandoffDeferred, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDenialCode, ForgeServerQueryHandoffDenialFacts,
    ForgeServerQueryHandoffDenialFamily, ForgeServerQueryHandoffFacade,
    ForgeServerQueryHandoffFailure, ForgeServerQueryHandoffInput, ForgeServerQueryHandoffOperation,
    ForgeServerQueryHandoffOutcome, ForgeServerQueryHandoffRebindRequired,
    ForgeServerQueryHandoffStale, ForgeServerQueryOperation, ForgeServerQueryOperationKind,
    ForgeServerQueryRequestedResume, ForgeServerQueryRequestedResumeKind,
    ForgeServerQuerySupportPosture, ForgeServerQueryWorkspaceBindingError,
    ForgeServerQueryWorkspaceBindingRequest, ForgeServerQueryWorkspaceBindingTarget,
    ForgeServerQueryWorkspaceProvider,
};
pub use registration::{
    ForgeServerSurfaceFamily, ForgeServerSurfaceInventory, ForgeServerSurfaceRegistration,
    ForgeServerSurfaceRegistryError,
};
pub use request_context::{
    ForgeServerAuthenticatedPrincipal, ForgeServerBranchTarget, ForgeServerRequestContext,
    ForgeServerRequestContextDeferred, ForgeServerRequestContextDenial,
    ForgeServerRequestContextDenialCode, ForgeServerRequestContextFacade,
    ForgeServerRequestContextFailure, ForgeServerRequestContextInput,
    ForgeServerRequestContextInputBuilder, ForgeServerRequestContextInputError,
    ForgeServerRequestContextRebindRequired, ForgeServerRequestContextStale,
    ForgeServerResolvedRequestContext, ForgeServerTransportClass, ForgeServerWorkspaceTarget,
};
pub use response::{
    ForgeServerDenialBoundary, ForgeServerDenialCause, ForgeServerDenialEnvelope,
    ForgeServerResponseEnvelope, ForgeServerResponseFacade, ForgeServerResponseInput,
    ForgeServerResponsePlan, ForgeServerResponseReceipt, ForgeServerResponseTransform,
    ForgeServerSuccessEnvelope, ForgeServerSuccessKind, ForgeServerSuccessPayload,
};
pub use surfaces::{
    BinarySurface, BinarySurfaceRoot, CompatHttpSurface, CompatHttpSurfaceRoot, ForgeNativeSurface,
    ForgeNativeSurfaceRoot, ForgeServerAbuseBudgetReceipt, ForgeServerBackgroundExportRequest,
    ForgeServerBinaryCertificationBundle, ForgeServerBinaryCounterSet, ForgeServerBinaryDownload,
    ForgeServerBinaryDownloadAuthorization, ForgeServerBinaryDownloadExecutionInput,
    ForgeServerBinaryDownloadOutcome, ForgeServerBinaryDownloadRequest,
    ForgeServerBinaryEgressPerformanceReceipt, ForgeServerBinaryEgressSession,
    ForgeServerBinaryIngressSession, ForgeServerBinaryIntegrityDigest,
    ForgeServerBinaryPolicyDecision, ForgeServerBinaryResumeRequest, ForgeServerBinaryRetryPosture,
    ForgeServerBinarySessionResume, ForgeServerCacheabilityPolicy, ForgeServerCanonicalFilename,
    ForgeServerCanonicalHeaderSet, ForgeServerCompatHttpRouteFamilies,
    ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityAdmittedProductMutationCommand,
    ForgeServerCompatibilityCachePolicy, ForgeServerCompatibilityCertificationBundle,
    ForgeServerCompatibilityDeferred, ForgeServerCompatibilityDenial,
    ForgeServerCompatibilityDenialCode, ForgeServerCompatibilityExecutionInput,
    ForgeServerCompatibilityExecutionOutcome, ForgeServerCompatibilityExport,
    ForgeServerCompatibilityFacade, ForgeServerCompatibilityFailure,
    ForgeServerCompatibilityFileEnvelope, ForgeServerCompatibilityInspection,
    ForgeServerCompatibilityMutation, ForgeServerCompatibilityMutationCommand,
    ForgeServerCompatibilityMutationEnvelope, ForgeServerCompatibilityMutationExecutionInput,
    ForgeServerCompatibilityMutationOutcome, ForgeServerCompatibilityMutationRequest,
    ForgeServerCompatibilityMutationResult, ForgeServerCompatibilityOpenedProductSession,
    ForgeServerCompatibilityPreparedRequest, ForgeServerCompatibilityProductSessionContinuation,
    ForgeServerCompatibilityProductSessionFacade, ForgeServerCompatibilityRead,
    ForgeServerCompatibilityRebindRequired, ForgeServerCompatibilityRequest,
    ForgeServerCompatibilityRequestInput, ForgeServerCompatibilityRequestInputBuilder,
    ForgeServerCompatibilityRequestInputError, ForgeServerCompatibilityRequestOutcome,
    ForgeServerCompatibilityStale, ForgeServerCompatibilityState, ForgeServerCompatibilityStream,
    ForgeServerCompatibilityUpload, ForgeServerCompatibilityUploadExecutionInput,
    ForgeServerCompatibilityUploadOutcome, ForgeServerCompatibilityVersion,
    ForgeServerConditionalRangeRequest, ForgeServerConditionalRead,
    ForgeServerExternalBasisRequest, ForgeServerExternalCounterSet,
    ForgeServerExternalEvidenceRecord, ForgeServerExternalRequestContract,
    ForgeServerFileMetadataReceipt, ForgeServerFileMetadataTruthKind,
    ForgeServerFileTransferDisposition, ForgeServerFileTransferProvenance,
    ForgeServerIdempotencyKey, ForgeServerIdempotentReplayReceipt,
    ForgeServerIngressIntegrityDigest, ForgeServerIngressPerformanceReceipt,
    ForgeServerMetadataNormalizationReceipt, ForgeServerMultipartUpload,
    ForgeServerMutationPrecondition, ForgeServerNegotiatedRepresentation,
    ForgeServerPreparedMultipartUpload, ForgeServerRangeRequest, ForgeServerReadValidator,
    ForgeServerStreamCancellationKind, ForgeServerStreamCancellationReceipt,
    ForgeServerStreamFinishError, ForgeServerStreamSelection, ForgeServerStreamingChunk,
    ForgeServerStreamingPerformanceReceipt, ForgeServerStreamingResponse,
    ForgeServerSurfaceCapabilities, ForgeServerSurfaceRoot, ForgeServerSurfacesFacade,
    ForgeServerTransferByteClass, ForgeServerTransferCleanupEvidence,
    ForgeServerTransferCleanupReason, ForgeServerUploadChunk, ForgeServerUploadCleanupReason,
    ForgeServerUploadCleanupReceipt, ForgeServerUploadContentEncoding,
    ForgeServerUploadExpectation, ForgeServerUploadManifest, ForgeServerUploadPart,
    ForgeServerUploadTransferMode, ForgeServerVerifiedBinaryIngress, IntegrationSurface,
    IntegrationSurfaceRoot, LeaseSurface, LeaseSurfaceRoot, SyncSurface, SyncSurfaceRoot,
};
pub use transport::{
    ForgeServerDeclaredRoute, ForgeServerOperationRouter, ForgeServerOperationalRoute,
    ForgeServerOperationalRouteKind, ForgeServerOperationalRouteOutcome,
    ForgeServerProjectedRouter, ForgeServerRouteAssembly, ForgeServerRouteAssemblyError,
    ForgeServerRouteBranchTarget, ForgeServerRouteExecutionBridge,
    ForgeServerRouteExecutionOutcome, ForgeServerRouteInventory, ForgeServerRouteInventoryRow,
    ForgeServerRouteTransportRequest, ForgeServerTransportDenial, ForgeServerTransportDenialCode,
};
