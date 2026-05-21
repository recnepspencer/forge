mod anchors;
mod frames;
mod witness_catalog;
mod witnesses;

pub use anchors::{SpatialAnchorRef, SpatialAxis};
pub use frames::SpatialFrameRef;
pub use witness_catalog::{
    EmptySpatialWitnessCatalog, SpatialCatalogParameterAdmission,
    SpatialCatalogResolvedDirectionWitness, SpatialCatalogResolvedGeometricTag,
    SpatialCatalogResolvedPointWitness, SpatialCatalogTrimmedAdmissionPosture,
    SpatialCatalogWitnessResolutionClass, SpatialGeometricTagFailureClass, SpatialWitnessCatalog,
};
pub use witnesses::{
    SpatialCarrierDirectionRole, SpatialCarrierKind, SpatialCarrierPointRole,
    SpatialDirectionWitnessRef, SpatialPointWitnessRef,
};
