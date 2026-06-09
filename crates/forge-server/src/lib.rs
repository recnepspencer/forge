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
pub mod operator_evidence;
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
pub use operator_evidence::{
    ForgeServerEvidenceInput, ForgeServerEvidenceTransform, ForgeServerObservedCounter,
    ForgeServerOperatorCounterReceipt, ForgeServerOperatorEvidenceClass,
    ForgeServerOperatorEvidenceFacade, ForgeServerOperatorEvidenceMaterializationError,
    ForgeServerOperatorEvidencePlan, ForgeServerOperatorEvidenceRecord,
};
pub use query_handoff::{
    ForgeServerQueryHandoff, ForgeServerQueryHandoffDeferred, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDenialCode, ForgeServerQueryHandoffFacade,
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
    ForgeNativeSurfaceRoot, ForgeServerBackgroundExportRequest, ForgeServerBinaryIngressSession,
    ForgeServerCanonicalHeaderSet, ForgeServerCompatHttpRouteFamilies,
    ForgeServerCompatHttpRouteFamily, ForgeServerCompatibilityCachePolicy,
    ForgeServerCompatibilityDeferred, ForgeServerCompatibilityDenial,
    ForgeServerCompatibilityDenialCode, ForgeServerCompatibilityExecutionInput,
    ForgeServerCompatibilityExecutionOutcome, ForgeServerCompatibilityExport,
    ForgeServerCompatibilityFacade, ForgeServerCompatibilityFailure,
    ForgeServerCompatibilityInspection, ForgeServerCompatibilityMutation,
    ForgeServerCompatibilityMutationCommand, ForgeServerCompatibilityMutationEnvelope,
    ForgeServerCompatibilityMutationExecutionInput, ForgeServerCompatibilityMutationOutcome,
    ForgeServerCompatibilityMutationRequest, ForgeServerCompatibilityMutationResult,
    ForgeServerCompatibilityPreparedRequest, ForgeServerCompatibilityRead,
    ForgeServerCompatibilityRebindRequired, ForgeServerCompatibilityRequest,
    ForgeServerCompatibilityRequestInput, ForgeServerCompatibilityRequestInputBuilder,
    ForgeServerCompatibilityRequestInputError, ForgeServerCompatibilityRequestOutcome,
    ForgeServerCompatibilityStale, ForgeServerCompatibilityState, ForgeServerCompatibilityStream,
    ForgeServerCompatibilityUpload, ForgeServerCompatibilityUploadExecutionInput,
    ForgeServerCompatibilityUploadOutcome, ForgeServerCompatibilityVersion,
    ForgeServerConditionalRead, ForgeServerExternalBasisRequest,
    ForgeServerExternalRequestContract, ForgeServerIdempotencyKey,
    ForgeServerIdempotentReplayReceipt, ForgeServerIngressIntegrityDigest,
    ForgeServerIngressPerformanceReceipt, ForgeServerMultipartUpload,
    ForgeServerMutationPrecondition, ForgeServerNegotiatedRepresentation,
    ForgeServerPreparedMultipartUpload, ForgeServerReadValidator,
    ForgeServerStreamCancellationKind, ForgeServerStreamCancellationReceipt,
    ForgeServerStreamFinishError, ForgeServerStreamSelection, ForgeServerStreamingChunk,
    ForgeServerStreamingPerformanceReceipt, ForgeServerStreamingResponse,
    ForgeServerSurfaceCapabilities, ForgeServerSurfaceRoot, ForgeServerSurfacesFacade,
    ForgeServerUploadChunk, ForgeServerUploadCleanupReason, ForgeServerUploadCleanupReceipt,
    ForgeServerUploadContentEncoding, ForgeServerUploadExpectation, ForgeServerUploadManifest,
    ForgeServerUploadPart, ForgeServerUploadTransferMode, ForgeServerVerifiedBinaryIngress,
    IntegrationSurface, IntegrationSurfaceRoot, LeaseSurface, LeaseSurfaceRoot, SyncSurface,
    SyncSurfaceRoot,
};
