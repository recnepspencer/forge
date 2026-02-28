//! Sewing, healing, and repair operators (composite).
//!
//! DOMAIN: Edge/coedge/sheet sewing, gap/crack/hole healing,
//! remove-and-heal operations, artifact cleanup (slivers, short
//! edges, tiny loops), and surface/curve refit/rebuild.
//!
//! OPERATORS (from operators-list.md §K):
//! - K1: Sewing (SewEdges, SewCoedges, SewSheets, UnsewEdges, SewWithTolerance, etc.)
//! - K2: Gap/Crack/Hole healing (HealVertexGaps, HealEdgeGaps, PatchHoleWithFace, etc.)
//! - K3: Remove-and-heal (RemoveFaceAndHeal, ExtendNeighborSurfaces, etc.)
//! - K4: Artifact cleanup (DetectSlivers, CollapseShortEdges, SimplifyEdgeChains, etc.)
//! - K5: Refit/rebuild (RefitSurfaceToBoundary, RebuildSeamsOnPeriodicFaces, etc.)
//!
//! DEPENDENCIES: `euler`, `algorithms`, `arena`, `handles`, `forge-geom`

pub mod orientation;

pub use orientation::{heal_shell_orientation, HealingResult};
