//! Face, loop, edge, and vertex lifecycle operators.
//!
//! DOMAIN: Creation, destruction, cloning, splitting, merging,
//! detaching, attaching, and reversing of individual topological entities.
//!
//! OPERATORS (from operators-list.md §C):
//! - C1: CreateVertex, DestroyVertex, CloneVertex, SplitVertex, MergeVertices, DetachVertex, AttachVertex
//! - C2: CreateEdge, DestroyEdge, CloneEdge, SplitEdge, MergeEdges, DetachEdge, AttachEdge, ReverseEdge
//! - C3: CreateFace, DestroyFace, CloneFace, SplitFace, MergeFaces, DetachFace, AttachFace, ReverseFace
//! - C4: CreateLoop, DestroyLoop, CloneLoop, SplitLoop, MergeLoops, DetachLoop, AttachLoop, ReverseLoop
//!
//! DEPENDENCIES: `euler` (primitives), `arena` (entity storage)

pub mod make_isolated_vertex;
pub mod make_shell_face;
pub mod kill_shell_face;
pub mod kill_vertex_face;
pub mod make_vertex_face;
pub mod make_edge_vertex;
pub mod kill_edge_vertex;
pub mod split_edge;
pub mod make_edge_face;

pub use make_isolated_vertex::*;
pub use make_shell_face::*;
pub use kill_shell_face::*;
pub use kill_vertex_face::*;
pub use make_vertex_face::*;
pub use make_edge_vertex::*;
pub use kill_edge_vertex::*;
pub use split_edge::*;
pub use make_edge_face::*;
