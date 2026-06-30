//! Worth authority vocabulary and lower authority support.
//!
//! This module owns Worth-specific write-side truth semantics and related
//! authority descriptors. It is not the ordinary Query lifecycle entry
//! surface; public consumers should reach it through
//! `worth_schema::facade::platform::authority`.

pub(crate) mod aspect_field_patches;
pub(crate) mod commit_flow;
pub(crate) mod derived_invalidation;
pub(crate) mod gateway;
pub(crate) mod geometry_binding;
pub(crate) mod interpretation;
pub(crate) mod precision_fallback;
pub(crate) mod replay_undo_semantic_graph;
pub(crate) mod topology_class;
pub(crate) mod touched_graph_basis;
pub(crate) mod touched_graph_conflict;

#[allow(unused_imports)]
pub(crate) use commit_flow::{
    AuthoritativeTopologySnapshot, CertifiedTopologyInterpretation, CreateKey,
    DerivedTopologyReadBasis, DerivedTruthBasisIdentity, EntityReference, MutationOrigin,
    PersistedTopologyTruth, RawTopologyIntent, TopologyCommittedMutationSet, TopologyMutation,
    TopologyReadArtifact,
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
pub use precision_fallback::{
    FallbackDisposition, FallbackProofClass, PrecisionBudgetFallbackRecord,
    PrecisionEscalationCause, PrecisionFallbackRecord, PrecisionRegime,
};
#[allow(unused_imports)]
pub use replay_undo_semantic_graph::{
    admit_replay_scope_identity, admit_replay_undo_stage_index_identity,
    admit_spatial_evidence_lookup_prior_proof_identity,
    admit_topology_derived_invalidation_prior_proof_identity, admit_undo_scope_identity,
    ReplayScopeIdentity, ReplayScopeIdentityInput, ReplayUndoSemanticGraphEquivalenceBasis,
    ReplayUndoSemanticGraphLocalityScope, ReplayUndoSemanticGraphPriorProofClass,
    ReplayUndoSemanticGraphPriorProofIdentity, ReplayUndoSemanticGraphStageIndexIdentity,
    ReplayUndoSemanticGraphTouchedSubject, ReplayUndoTransactionScopeClaim,
    ReplayUndoTransactionScopeKind, UndoScopeIdentity, UndoScopeIdentityInput,
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
    admit_conflict_participant_identity, admit_conflict_routing_contract, ConflictAspectClass,
    ConflictLocalityIdentity, ConflictOverlapCategory, ConflictOverlapIdentity,
    ConflictOverlapIdentityInput, ConflictParticipantAuthority, ConflictParticipantIdentity,
    ConflictParticipantIdentityInput, ConflictPriorProofIdentity, ConflictPriorProofInput,
    ConflictRoutingContract, ConflictRoutingPosture, ConflictRoutingVocabularyError,
    ConflictTransactionProofInput,
};
