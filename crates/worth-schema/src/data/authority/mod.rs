//! Worth authority vocabulary and lower authority support.
//!
//! This module owns Worth-specific write-side truth semantics and related
//! authority descriptors. It is not the ordinary Query lifecycle entry
//! surface; public consumers should reach it through
//! `worth_schema::facade::platform::authority`.

pub(crate) mod aspect_field_patches;
pub(crate) mod commit_flow;
pub(crate) mod compiled_product_semantic_graph;
pub(crate) mod derived_invalidation;
pub(crate) mod gateway;
pub(crate) mod geometry_binding;
pub(crate) mod interpretation;
pub(crate) mod planner_owned_routing_semantic_graph;
pub(crate) mod precision_fallback;
pub(crate) mod replay_undo_semantic_graph;
pub(crate) mod topology_class;
pub(crate) mod touched_graph_basis;
pub(crate) mod touched_graph_conflict;
pub(crate) mod touched_graph_parity_closeout;

#[allow(unused_imports)]
pub(crate) use commit_flow::{
    AuthoritativeTopologySnapshot, CertifiedTopologyInterpretation, CreateKey,
    DerivedTopologyReadBasis, DerivedTruthBasisIdentity, EntityReference, MutationOrigin,
    PersistedTopologyTruth, RawTopologyIntent, TopologyCommittedMutationSet, TopologyMutation,
    TopologyReadArtifact,
};
#[allow(unused_imports)]
pub use compiled_product_semantic_graph::{
    admit_compiled_product_authority_truth_identity,
    admit_compiled_product_authority_truth_identity_with_coordinates,
    admit_compiled_product_equivalence_policy_identity, admit_compiled_product_identity,
    admit_compiled_product_prior_proof_identity, admit_compiled_product_rebuild_denial_identity,
    admit_compiled_product_reuse_decision_identity, admit_compiled_product_stage_identity,
    admit_locality_footprint_identity, CompiledProductAuthorityInstanceCoordinate,
    CompiledProductAuthorityTruthIdentity, CompiledProductEquivalencePolicyIdentity,
    CompiledProductIdentity, CompiledProductLocalityFootprintIdentity,
    CompiledProductPriorProofIdentity, CompiledProductPriorProofRole,
    CompiledProductRebuildDenialIdentity, CompiledProductReuseDecisionIdentity,
    CompiledProductSemanticGraphVocabularyError, CompiledProductSemanticGraphVocabularyErrorKind,
    CompiledProductStageIdentity,
};
#[allow(unused_imports)]
pub use derived_invalidation::{
    milestone_two_invalidation_declarations, DerivedInvalidationTarget, DerivedTruthSurfaceKind,
    TruthToDerivedInvalidationDeclaration,
};
#[allow(unused_imports)]
pub use gateway::{TopologyAuthority, TopologyAuthorityError, VerifiedTopologyCommit};
#[allow(unused_imports)]
pub use geometry_binding::{
    CoedgeCurveKind, CurveBindingKind, CurveProvenanceKind, SurfaceBindingKind,
    SurfaceRelationKind, VertexGeometryProvenanceKind, VertexToleranceRegime,
};
#[allow(unused_imports)]
pub use interpretation::{
    ShellInterpretationClass, ShellInterpretationRecord, TopologyInterpretationRecordSet,
    WireInterpretationClass, WireInterpretationRecord,
};
#[allow(unused_imports)]
pub use planner_owned_routing_semantic_graph::{
    admit_planner_admitted_explanation_input, admit_planner_decision_trace_identity,
    admit_planner_derived_diagnostic_contract_identity, admit_planner_public_proof_identity,
    admit_planner_selected_family_identity, admit_planner_selected_product_identity,
    admit_planner_selected_route_identity, admit_planner_witness_identity,
    PlannerAdmittedExplanationInput, PlannerDecisionTraceIdentity,
    PlannerDerivedDiagnosticContractIdentity, PlannerExplanationArtifactKind, PlannerMismatchLocus,
    PlannerOwnedRoutingSemanticGraphVocabularyError,
    PlannerOwnedRoutingSemanticGraphVocabularyErrorKind, PlannerPublicProofIdentity,
    PlannerSelectedFamilyIdentity, PlannerSelectedProductIdentity, PlannerSelectedRouteIdentity,
    PlannerWitnessIdentity, PlannerWitnessRole,
};
#[allow(unused_imports)]
pub use precision_fallback::{
    FallbackDisposition, FallbackProofClass, PrecisionBudgetFallbackRecord,
    PrecisionEscalationCause, PrecisionFallbackRecord, PrecisionRegime,
};
#[allow(unused_imports)]
pub use replay_undo_semantic_graph::{
    admit_replay_scope_identity, admit_replay_undo_stage_index_identity,
    admit_spatial_evidence_lookup_prior_proof_identity,
    admit_topology_derived_invalidation_prior_proof_identity, admit_undo_scope_identity,
    ReplayScopeIdentity, ReplayScopeIdentityInput, ReplayUndoPlannerRouteFamily,
    ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphLocalityScope,
    ReplayUndoSemanticGraphPriorProofClass, ReplayUndoSemanticGraphPriorProofIdentity,
    ReplayUndoSemanticGraphStageIndexIdentity, ReplayUndoSemanticGraphTouchedSubject,
    ReplayUndoTransactionScopeClaim, ReplayUndoTransactionScopeKind, UndoScopeIdentity,
    UndoScopeIdentityInput,
};
#[allow(unused_imports)]
pub use topology_class::TopologyClass;
#[allow(unused_imports)]
pub use touched_graph_basis::{
    worth_topology_touched_graph_digest, WorthTopologyGraphLifecyclePosture,
    WorthTopologyTouchedAspect, WorthTopologyTouchedGraphCounters,
    WorthTopologyTouchedOperatingWorldPosture, WorthTopologyTouchedScope,
};
#[allow(unused_imports)]
pub use touched_graph_conflict::{
    admit_conflict_locality_identity, admit_conflict_overlap_identity,
    admit_conflict_participant_identity, admit_conflict_routing_contract,
    BatchAdmissionPlannerRouteFamily, BatchAdmissionPlannerRouteWitness,
    BatchAdmissionPlannerRouteWitnessKind, ConflictAspectClass,
    ConflictIndependencePlannerRouteFamily, ConflictIndependencePlannerRouteWitness,
    ConflictIndependencePlannerRouteWitnessKind, ConflictLocalityIdentity, ConflictOverlapCategory,
    ConflictOverlapIdentity, ConflictOverlapIdentityInput, ConflictParticipantAuthority,
    ConflictParticipantIdentity, ConflictParticipantIdentityInput, ConflictPriorProofIdentity,
    ConflictPriorProofInput, ConflictRoutingContract, ConflictRoutingPosture,
    ConflictRoutingVocabularyError, ConflictTransactionProofInput,
};
#[allow(unused_imports)]
pub(crate) use touched_graph_parity_closeout::{
    admit_touched_graph_parity_readiness_claim, admit_touched_graph_parity_readiness_input,
};
#[allow(unused_imports)]
pub use touched_graph_parity_closeout::{
    TouchedGraphParityArchitectureClaim, TouchedGraphParityClaimKind,
    TouchedGraphParityCoverageContributor, TouchedGraphParityCoverageRow,
    TouchedGraphParityFamilyKind, TouchedGraphParityQuerySurfaceKind,
    TouchedGraphParityReadinessError, TouchedGraphParityReadinessErrorKind,
    TouchedGraphParityReadinessInput, TouchedGraphParityResidueClassification,
};
