//! Public API boundary for forge-spec.

pub use crate::data::graph::{NodeRecord, RelationRecord, SpecGraph};
pub use crate::data::identity::{
    DeterministicIdAllocator, NamingAnchorId, SpecNodeId, SpecRelationId,
};
pub use crate::data::journal::{MutationJournal, MutationJournalEntry};
pub use crate::data::lineage::LineageRecord;
pub use crate::data::naming::{NamingAnchor, PersistentName};
pub use crate::data::payload::{PayloadKey, PayloadRecord, PayloadStore};
pub use crate::data::replay::SpecReplayRecord;
pub use crate::data::schema::{GraphDomain, RelationCardinality, RelationKind, SpecNodeKind};
pub use crate::data::snapshot::SpecState;
pub use crate::logic::mutation::{MutationResult, SpecLineageRecorder, SpecMutation, TouchedDomain};
pub use crate::logic::mutation::topology::{
    DestroyBodyMutation, DestroyLumpMutation, DestroyShellMutation, KillEdgeVertexMutation,
    KillFaceVertexMutation, KillShellFaceMutation, KillVertexFaceMutation, MakeEdgeFaceMutation,
    MakeEdgeFaceOutput, MakeEdgeVertexMutation, MakeEdgeVertexOutput, MakeEmptyShellMutation,
    MakeEmptyShellOutput, MakeFaceVertexMutation, MakeFaceVertexOutput, MakeLumpRegionMutation,
    MakeLumpRegionOutput, MakeShellFaceMutation, MakeShellFaceOutput, MakeSolidMutation,
    MakeSolidOutput, MakeVertexFaceMutation, MakeVertexFaceOutput, SplitEdgeMutation,
    SplitEdgeOutput,
};
pub use crate::logic::transaction::SpecDraft;
pub use crate::logic::validation::validate_spec_graph;
pub use crate::presentation::contracts::{
    ProjectionBoundaryContract, SpecGraphTruthContract, TruthGraphBoundaryContract,
};
pub use crate::data::error::SpecError;
