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
