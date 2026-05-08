//! Public API boundary for `-schema`.

pub mod topology_authoring {
    pub use crate::topology_authoring::{
        build_milestone_one_primitive_intent, created_ref,
        milestone_one_admitted_range_sweep_out_of_class_scenarios,
        milestone_one_admitted_range_sweep_scenarios, milestone_one_default_primitive_corpus,
        milestone_one_heavy_branch_local_sweep_scenarios, seed_milestone_one_primitive,
        seed_milestone_one_primitive_on_branch, seed_minimal_topology, verify_topology_intent,
        verify_topology_intent_on_branch, MilestoneOnePrimitiveAuthoringError,
        MilestoneOnePrimitiveCase, MilestoneOnePrimitiveExpectedOutcome, MilestoneOnePrimitiveRole,
        MilestoneOnePrimitiveScenario, MinimalTopologySeed, TopologyCreateBatchBuilder,
    };
}

pub use crate::data::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, LineageAspect, NamingAspect, TopologyAspect,
};
pub use crate::data::authority::{
    milestone_two_invalidation_declarations, AuthoritativeTopologySnapshot,
    CanonicalTopologyMutationBatch, CertifiedTopologyInterpretation, CoedgeCurveKind, CreateKey,
    CurveBindingKind, CurveProvenanceKind, DerivedInvalidationTarget, DerivedTopologyReadBasis,
    DerivedTruthBasisIdentity, DerivedTruthSurfaceKind, EntityReference, FallbackDisposition,
    FallbackProofClass, MutationOrigin, PersistedTopologyTruthBatch, PrecisionBudgetFallbackRecord,
    PrecisionEscalationCause, PrecisionFallbackRecord, PrecisionRegime, RawTopologyIntent,
    ShellInterpretationClass, ShellInterpretationRecord, SurfaceBindingKind, SurfaceRelationKind,
    TopologyAuthorityError, TopologyClass, TopologyInterpretationRecordSet, TopologyMutation,
    TopologyMutationBatch, TopologyReadArtifact, TruthToDerivedInvalidationDeclaration,
    VerifiedTopologyCommit, VertexGeometryProvenanceKind, VertexToleranceRegime,
    WireInterpretationClass, WireInterpretationRecord,
};
pub use crate::data::bootstrap::{
    bootstrap_invariant_plan, bootstrap_runtime_invariant_plan, bootstrap_schema_registry,
    bootstrap_tracing_plan, BootstrapInvariantPlan, BootstrapRuntimeInvariant,
    BootstrapRuntimeInvariantPlan, BootstrapTracingPlan, SchemaBuildError, SchemaBuilder,
    SCHEMA_ID, SCHEMA_VERSION_ID,
};
pub use crate::data::entities::{
    DiagnosticsEntityKind, EntityKind, GeometryEntityKind, NamingEntityKind, TopologyEntityKind,
};
pub use crate::data::explanation::{
    explain_authority_trace, explain_bridge_trace, explain_derived_trace, explain_signal_trace,
    narrate_boundary_envelope, narrate_boundary_failure, narrate_decision_trace,
    AuthorityNarrative, BridgeHistoricalNarrative, BridgeNarrative, BridgeRouteNarrative,
    DerivedNarrative, NarratedTrace, NarrativeLine, SignalNarrative,
};
pub use crate::data::invariants::{
    DiagnosticsInvariantGroup, GeometryInvariantGroup, InvariantGroup, LineageInvariantGroup,
    NamingInvariantGroup, TopologyInvariantGroup,
};
pub use crate::data::query::{
    admit_query_mutation_batch, query_aspect_path_strings, query_aspect_paths,
    query_aspect_paths_from_set, query_mutation_support_contract, QueryAspectFamily,
    QueryAspectPath, QueryCollection, QueryComputedDeclarationBuilder, QueryDeclarationError,
    QueryLiveDeclarationBuilder, QueryLiveField, QueryMutationAdmission,
    QueryMutationAdmissionBlocker, QueryMutationAdmissionReport, QueryMutationSupportContract,
    QuerySchemaBasis,
};
pub use crate::data::relations::{
    DiagnosticsRelationKind, GeometryRelationKind, NamingRelationKind, RelationKind,
    TopologyRelationKind,
};
pub use crate::data::tracing::{
    AuthorityTraceAnchor, AuthorityTraceEvidence, BoundaryEnvelope, BoundaryFailure,
    BridgeTraceAnchor, BridgeTraceEvidence, DecisionTrace, DerivedTraceAnchor,
    DerivedTraceEvidence, IntegrityMarkers, NamedCounter, PerformanceAccounting, SignalTraceAnchor,
    SignalTraceEvidence, TraceAvailability, TraceWarning,
};
