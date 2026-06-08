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
mod diagnostics;
pub mod facade;
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
pub use diagnostics::ForgeServerCounterSnapshot;
pub use facade::{ForgeServer, ForgeServerBuildError, ForgeServerBuilder};
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
    ForgeServerQueryHandoffStale, ForgeServerQueryOperationKind, ForgeServerQueryRequestedResume,
    ForgeServerQueryRequestedResumeKind, ForgeServerQuerySupportPosture,
    ForgeServerQueryWorkspaceBindingError, ForgeServerQueryWorkspaceBindingRequest,
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
    ForgeNativeSurfaceRoot, ForgeServerSurfaceCapabilities, ForgeServerSurfaceRoot,
    ForgeServerSurfacesFacade, IntegrationSurface, IntegrationSurfaceRoot, LeaseSurface,
    LeaseSurfaceRoot, SyncSurface, SyncSurfaceRoot,
};
