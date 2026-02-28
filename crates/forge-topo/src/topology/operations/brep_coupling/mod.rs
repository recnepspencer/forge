//! Parametric B-Rep coupling operators (Topo↔Geom bindings).
//!
//! DOMAIN: Attaching/detaching/replacing surfaces on faces,
//! curves on edges, PCurves on coedges, coupled split/merge
//! operations, trim network editing, and seam/periodicity handling.
//!
//! OPERATORS (from operators-list.md §H):
//! - H1: Face↔Surface (Attach/Detach/Replace/Copy, Sense, Parameterization)
//! - H2: Edge↔3D Curve (Attach/Detach/Replace/Copy, Sense, Reparameterize)
//! - H3: Coedge↔PCurve (Attach/Detach/Replace/Copy, Sense, Reparameterize)
//! - H4: Coupled Split/Merge (atomic edge+curve, coedge+pcurve, face+trim, vertex+uses)
//! - H5: Trim network editing (InsertTrimLoop, SplitTrimLoop, StitchTrimEndpoints, etc.)
//! - H6: Seams/Periodicity (CreateSeamEdge, RemoveSeamEdge, CreatePoleSingularity, etc.)
//!
//! DEPENDENCIES: `euler`, `arena`, `handles`, `forge-geom` (curve/surface)
