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
mod authoring;
mod authorized_projection;
mod basis;
mod basis_lifecycle;
mod binding;
mod binding_pipeline;
mod canonicalization;
mod collection;
mod composition;
mod continuation_pipeline;
mod contribution_composed_orchestration;
mod correspondence;
mod correspondence_history;
mod correspondence_history_parity;
mod declarative_live;
mod diagnostics;
mod domain_capabilities;
mod effect_lifecycle;
mod execution;
pub mod facade;
mod frontier_planning;
mod frontier_signal_adapter;
mod historical;
mod identity;
mod identity_evolution;
mod intent_admission;
mod live;
mod live_performance;
mod lower_runtime_routing;
mod memory_workspace;
mod orchestration_inventory;
mod ordinary_outcome;
mod planning;
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
mod query_basis_lifecycle;
mod query_context;
mod result_shape;
mod runtime;
mod saved_query;
#[macro_use]
mod schema_macro;
mod relationship_proof;
mod schema_view;
mod signal_compatibility_orchestration;
mod subscription;
mod target_binding;
mod tenant_basis;
mod typed;
mod validation;
mod view_shape;
mod view_shape_live;
mod workflow;

pub use continuation_pipeline::{
    ForgeQueryContinuationBasisPosture, ForgeQueryContinuationExecution,
    ForgeQueryContinuationExecutionChecked, ForgeQueryContinuationExecutionOutcome,
    ForgeQueryContinuationExecutionTranscript, ForgeQueryContinuationRuntimeContract,
    ForgeQueryContinuationTruthContext, ForgeQueryContinuationWorkspaceContract,
    ForgeQueryExecutePreparedContinuationRequest, ForgeQueryPreparedContinuation,
    ForgeQueryPreparedContinuationChecked, ForgeQueryPreparedContinuationExecutionMode,
    ForgeQueryPreparedContinuationFamily, ForgeQueryPreparedContinuationOutcome,
    ForgeQueryPreparedContinuationRequest, ForgeQueryPreparedContinuationSignalPosture,
    ForgeQueryPreparedContinuationTranscript,
};
pub use contribution_composed_orchestration::{
    ForgeQueryContributionComposedContribution,
    ForgeQueryContributionComposedMaterializationPolicy,
    ForgeQueryContributionComposedOrchestration,
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationInput,
    ForgeQueryContributionComposedOrchestrationOutcome,
    ForgeQueryContributionComposedOrchestrationPosture,
    ForgeQueryContributionComposedOrchestrationTranscript, ForgeQueryContributionComposedSummary,
    ForgeQueryContributionIntent,
};
pub use orchestration_inventory::{
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationCheckedTopologyKind,
    ForgeQueryOrchestrationInventoryAudit, ForgeQueryOrchestrationProofContract,
    ForgeQueryOrchestrationSupportSurface, ForgeQueryOrchestrationSurfaceCertificationReference,
    ForgeQueryOrchestrationSurfaceDocReference, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationSurfaceInventory, ForgeQueryOrchestrationSurfaceRow,
    ForgeQueryOrchestrationSurfaceVisibility, ForgeQueryOrchestrationTranscriptFamily,
};
pub use ordinary_outcome::{
    ForgeQueryOrdinaryBindingCheckedTopologyKind, ForgeQueryOrdinaryCheckedTopology,
    ForgeQueryOrdinaryContinuationCheckedTopologyKind,
    ForgeQueryOrdinaryContributionComposedCheckedTopologyKind, ForgeQueryOrdinaryNextStep,
    ForgeQueryOrdinaryOutcome, ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
    ForgeQueryOrdinarySignalCompatibilityOrchestrationCheckedTopologyKind,
};
pub use signal_compatibility_orchestration::{
    ForgeQuerySignalCompatibilityOrchestration, ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationClass,
    ForgeQuerySignalCompatibilityOrchestrationInput,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
    ForgeQuerySignalCompatibilityOrchestrationSubject,
    ForgeQuerySignalCompatibilityOrchestrationTranscript,
};

#[cfg(test)]
mod harness;
