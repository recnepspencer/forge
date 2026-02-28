//! Sheet, wire, and laminar topology operators.
//!
//! DOMAIN: Wire body and sheet body lifecycle, laminar edge
//! management, and manifold/sheet conversions.
//!
//! OPERATORS (from operators-list.md §G):
//! - G1: Wire bodies (Create/Destroy, AddWireEdge, RemoveWireEdge, SplitWireEdge, etc.)
//! - G2: Sheet bodies (Create/Destroy, AddSheetFace, RemoveSheetFace, ConvertSheetToSolid, etc.)
//! - G3: Laminar edges (MarkEdgeLaminar, CreateBoundaryLoop, SewLaminarEdges, PromoteLaminarToManifold, etc.)
//!
//! DEPENDENCIES: `euler`, `arena`, `handles`
