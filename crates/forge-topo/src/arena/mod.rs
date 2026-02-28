//! Compatibility shim — re-exports from `b_rep` component.
//!
//! All arena types now live in `crate::b_rep`. This module preserves
//! the `crate::b_rep::*` import paths for existing consumers.
//! Will be removed in Phase 3 when all imports are updated.

pub use crate::b_rep::{
    TopologyArena,
    EntityKind,
    FaceData, HalfEdgeData, VertexData, LoopData, EdgeData,
    BodyData, LumpData, RegionData, ShellData, ShellKind, ShellOrientation,
};
