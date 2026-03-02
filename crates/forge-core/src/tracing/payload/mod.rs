//! Typed trace payload transport and domain-specific payloads.
//!
//! DOMAIN: `TraceAdjunctRecord` is the versioned envelope that carries
//! structured data alongside `TracedDecision` records. Each payload type
//! (policy, resolution, reidentification) has its own validation contract.

mod transport;
pub mod policy_trace;
pub mod resolution_trace;
pub mod reidentification_trace;

pub use transport::{
    TraceAdjunctRecord, TraceAdjunctSet, POLICY_DECISION_TRACE_PAYLOAD_KIND,
    POLICY_DECISION_TRACE_PAYLOAD_VERSION, REIDENTIFICATION_TRACE_PAYLOAD_KIND,
    REIDENTIFICATION_TRACE_PAYLOAD_VERSION, RESOLUTION_TRACE_PAYLOAD_KIND,
    RESOLUTION_TRACE_PAYLOAD_VERSION,
};
pub use policy_trace::{
    CandidateValueSummary, PolicyDecisionTracePayload, PolicyResolutionOutcome,
    PolicyResolutionScopeRef, PolicyResolutionSource, PolicyTraceConsistencyError,
};
pub use resolution_trace::{
    ResolutionCandidateSummary, ResolutionMatchKind, ResolutionOutcome, ResolutionQuerySummary,
    ResolutionRoute, ResolutionTraceConsistencyError, ResolutionTracePayload,
};
pub use reidentification_trace::{
    ReidentificationCompatibilitySummary, ReidentificationFailureCauseSummary,
    ReidentificationModeSummary, ReidentificationOriginKindSummary, ReidentificationOutcome,
    ReidentificationTraceConsistencyError, ReidentificationTracePayload,
};
