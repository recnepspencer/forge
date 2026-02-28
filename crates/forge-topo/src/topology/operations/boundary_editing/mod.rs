//! Boundary editing primitives (loop wiring).
//!
//! DOMAIN: Low-level loop manipulation — inserting/removing edges
//! and vertices from loops, splicing, rerouting, promoting/demoting
//! inner loops, and recomputing loop containment.
//!
//! OPERATORS (from operators-list.md §D):
//! - InsertEdgeIntoLoop, RemoveEdgeFromLoop
//! - InsertVertexIntoEdge, RemoveVertexFromEdge
//! - SpliceLoopAtVertex/Edge, UnspliceLoopAtVertex/Edge
//! - ReplaceLoopEdgeChain, ReplaceLoopVertex
//! - RerouteLoopAcrossFace, SwapLoopOrderOnFace
//! - PromoteInnerLoop, DemoteOuterLoop
//! - SetLoopContainment, RecomputeLoopContainment
//!
//! DEPENDENCIES: `euler`, `arena`, `handles`
