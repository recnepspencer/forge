//! Worth authority vocabulary and lower authority support.
//!
//! This module owns Worth-specific write-side truth semantics and related
//! authority descriptors. It is not the ordinary Query lifecycle entry
//! surface; public consumers should reach it through
//! `worth_schema::facade::platform::authority`.

mod commit_flow;
mod derived_invalidation;
mod gateway;
mod geometry_binding;
mod interpretation;
mod precision_fallback;
mod topology_class;

pub use commit_flow::{
    AuthoritativeTopologySnapshot, CanonicalTopologyMutationBatch, CertifiedTopologyInterpretation,
    CreateKey, DerivedTopologyReadBasis, DerivedTruthBasisIdentity, EntityReference,
    MutationOrigin, PersistedTopologyTruthBatch, RawTopologyIntent, TopologyMutation,
    TopologyMutationBatch, TopologyReadArtifact,
};
pub use derived_invalidation::{
    milestone_two_invalidation_declarations, DerivedInvalidationTarget, DerivedTruthSurfaceKind,
    TruthToDerivedInvalidationDeclaration,
};
pub use gateway::{TopologyAuthority, TopologyAuthorityError, VerifiedTopologyCommit};
pub use geometry_binding::{
    CoedgeCurveKind, CurveBindingKind, CurveProvenanceKind, SurfaceBindingKind,
    SurfaceRelationKind, VertexGeometryProvenanceKind, VertexToleranceRegime,
};
pub use interpretation::{
    ShellInterpretationClass, ShellInterpretationRecord, TopologyInterpretationRecordSet,
    WireInterpretationClass, WireInterpretationRecord,
};
pub use precision_fallback::{
    FallbackDisposition, FallbackProofClass, PrecisionBudgetFallbackRecord,
    PrecisionEscalationCause, PrecisionFallbackRecord, PrecisionRegime,
};
pub use topology_class::TopologyClass;
