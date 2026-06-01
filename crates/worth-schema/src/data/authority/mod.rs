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
pub(crate) mod topology_class;

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
pub use topology_class::TopologyClass;
