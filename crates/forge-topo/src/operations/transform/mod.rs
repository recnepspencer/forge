//! Transform, copy, and pattern operators (structural + composite).
//!
//! DOMAIN: Body/shell/face/edge/vertex copying, geometric transforms,
//! mirroring, linear/circular/point patterns, and instancing.
//!
//! OPERATORS (from operators-list.md §N):
//! - CopyBody/Lump/Shell/Face/Edge/Vertex
//! - TransformBody/Shell/FaceGeometryOnly/TopologyOnly
//! - MirrorBody
//! - PatternLinear/Circular/ByPoints
//! - InstanceBody, DeinstanceBody
//!
//! DEPENDENCIES: `euler`, `arena`, `handles`
