//! Boolean, imprint, and intersection surgery operators (composite).
//!
//! DOMAIN: Intersection construction, imprinting, splitting along
//! imprints, classification/selection, deletion/extraction,
//! stitch/merge/resolve, and finalization.
//!
//! OPERATORS (from operators-list.md §J):
//! - J1: Intersection construction (IntersectFaceFace, BuildIntersectionGraph, etc.)
//! - J2: Imprinting (ImprintVertexOnEdge/Face, ImprintEdgeOnFace, etc.)
//! - J3: Splitting along imprints
//! - J4: Classification/selection (KeepDiscard)
//! - J5: Deletion/extraction
//! - J6: Stitch/merge/resolve
//! - J7: Finalization
//!
//! DEPENDENCIES: `euler`, `algorithms`, `arena`, `handles`, `forge-geom`
