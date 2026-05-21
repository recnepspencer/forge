mod anchors;
mod frames;
mod point_witnesses;
mod witness_catalog;
mod witnesses;

pub use anchors::{SpatialAnchorRef, SpatialAxis};
pub use frames::{
    admit_spatial_frame, AdmittedSpatialFrameRef, SpatialFrameBasis, SpatialFrameError,
    SpatialFrameRef,
};
pub use point_witnesses::{SpatialCarrierPointRole, SpatialPointWitnessRef};
pub use witness_catalog::{
    EmptySpatialWitnessCatalog, SpatialCatalogResolvedDirectionWitness,
    SpatialCatalogResolvedGeometricTag, SpatialCatalogResolvedPointWitness,
    SpatialCatalogWitnessResolutionClass, SpatialFixtureWitnessCatalog,
    SpatialGeometricTagFailureClass, SpatialWitnessCatalog,
};
pub use witnesses::{SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialDirectionWitnessRef};
