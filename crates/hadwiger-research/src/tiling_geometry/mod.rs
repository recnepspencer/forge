mod boundary_ownership;
mod canonical_geometry_digest;
mod cell_artifacts;
mod contact_facts;
mod geometry_certification;
mod rectangular_regions;
mod screening_lowering;
mod tiling_geometry_errors;

pub use boundary_ownership::{BoundaryOwnershipKind, BoundaryOwnershipPolicy};
pub use cell_artifacts::{TilingCell, TilingCellBuilder};
pub use contact_facts::{
    TilingBoundaryOwnershipReport, TilingContactFact, TilingContactReplayReport, TilingContactRole,
    TilingGeometryCounters,
};
pub use geometry_certification::{
    certify_rectangular_tiling_cell_geometry_checked, TilingGeometryCertification,
};
pub use rectangular_regions::{RectangularTileRegion, TilingColorId, TilingTileId};
pub use screening_lowering::{
    evaluate_tiling_boundary_ownership_checked, evaluate_tiling_exact_unit_contact_checked,
    evaluate_tiling_minkowski_contact_checked, evaluate_tiling_same_color_contact_checked,
    evaluate_tiling_tile_diameter_checked,
};
pub use tiling_geometry_errors::TilingGeometryError;
