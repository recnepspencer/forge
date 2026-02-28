//! Global editing and topology operations (composite).
//!
//! DOMAIN: Slicing, sectioning, cutting, body union/separation,
//! disconnection, coplanar/tangent face merging, redundant edge
//! removal, topology simplification, and loop containment normalization.
//!
//! OPERATORS (from operators-list.md §M):
//! - SliceSolidWithPlane/Surface, SectionWithPlane/Surface
//! - CutWithSheet/Wire
//! - UniteBodies, SeparateBodies
//! - DisconnectAtFaces/Edges/Vertices
//! - ExtractConnectedComponent
//! - MergeCoplanarFaces, MergeTangentFaces
//! - RemoveRedundantEdges, SimplifyTopology
//! - NormalizeLoopContainment, RebuildFaceLoopsFromUses
//!
//! DEPENDENCIES: `euler`, `algorithms`, `arena`, `handles`
