//! Lineage change summary for an operation.

use serde::{Deserialize, Serialize};

/// Summary of lineage changes from an operation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LineageDelta {
    /// Number of faces created.
    pub faces_created: u32,
    /// Number of faces deleted.
    pub faces_deleted: u32,
    /// Number of half-edges created.
    pub half_edges_created: u32,
    /// Number of half-edges deleted.
    pub half_edges_deleted: u32,
    /// Number of vertices created.
    pub vertices_created: u32,
    /// Number of vertices deleted.
    pub vertices_deleted: u32,
    /// Number of loops created.
    pub loops_created: u32,
    /// Number of loops deleted.
    pub loops_deleted: u32,
    /// Number of edges created.
    pub edges_created: u32,
    /// Number of edges deleted.
    pub edges_deleted: u32,
    /// Number of shells created.
    pub shells_created: u32,
    /// Number of shells deleted.
    pub shells_deleted: u32,
    /// Number of solids created.
    pub solids_created: u32,
    /// Number of solids deleted.
    pub solids_deleted: u32,
}

impl LineageDelta {
    /// Accumulate another lineage delta into this one.
    ///
    /// Adds all counters field-by-field. Used by `absorb_sub_result`
    /// and `OperationFinalizer` to roll up sub-operation lineage deltas
    /// without manual per-field addition.
    pub fn accumulate(&mut self, other: &Self) {
        self.faces_created += other.faces_created;
        self.faces_deleted += other.faces_deleted;
        self.half_edges_created += other.half_edges_created;
        self.half_edges_deleted += other.half_edges_deleted;
        self.vertices_created += other.vertices_created;
        self.vertices_deleted += other.vertices_deleted;
        self.loops_created += other.loops_created;
        self.loops_deleted += other.loops_deleted;
        self.edges_created += other.edges_created;
        self.edges_deleted += other.edges_deleted;
        self.shells_created += other.shells_created;
        self.shells_deleted += other.shells_deleted;
        self.solids_created += other.solids_created;
        self.solids_deleted += other.solids_deleted;
    }
}
