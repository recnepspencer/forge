//! Non-manifold topology operators (radial-edge / uses).
//!
//! DOMAIN: Use-entity lifecycle, radial cycle manipulation,
//! non-manifold vertex disks, and sewing/gluing operations
//! for non-manifold intermediate states.
//!
//! OPERATORS (from operators-list.md §E):
//! - E1: Use entity CRUD (EdgeUse, Coedge, VertexUse, LoopUse, FaceUse, Rehome)
//! - E2: Radial cycles (Insert/Remove/Splice/Unsplice/Rotate/Canonicalize/Split/Merge)
//! - E3: Non-manifold vertex disks (Create/Destroy/Split/Merge/Detach/Attach)
//! - E4: Sewing/gluing (GlueFaces/Edges/Vertices, Unglue, Pinch/Unpinch)
//!
//! DEPENDENCIES: `euler`, `arena`, `handles`, `queries::radial`
