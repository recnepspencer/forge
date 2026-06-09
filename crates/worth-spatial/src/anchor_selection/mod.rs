mod family_types;
mod query_native;
mod query_native_authoring;
mod query_native_lowered_facts;
pub(crate) mod query_native_projection;

pub use crate::witness_resolution::{SpatialWitnessFailureClass, SpatialWitnessResolutionClass};
pub use family_types::{
    SpatialAnchorMatchConstraintSpec, SpatialConstraintError, SpatialLiesOnConstraintSpec,
    SpatialMotionError, SpatialMoveSpec, SpatialOffsetSpec, SpatialPointsTowardConstraintSpec,
    SpatialReorientSpec, SpatialRotateSpec,
};
pub use query_native::{
    SpatialAnchorSelectionDeclarationFamily, SpatialAnchorSelectionQueryDomain,
    SpatialAnchorSelectionQueryWorld,
};
pub use query_native_authoring::{
    AuthorSpatialAnchorSelectionIntent, SpatialAnchorSelectionDeclarationEntry,
    SpatialAnchorSelectionFailureKind, SpatialAnchorSelectionKind,
    SpatialAnchorSelectionPlacementError, SpatialAnchorSelectionRequestedInput,
    SpatialAnchorSelectionStatus, SpatialResolvedAnchorWitness,
};
pub use query_native_projection::{
    spatial_anchor_selection_projection_facts, SpatialAnchorSelectionFactProvenance,
    SpatialAnchorSelectionFactReadSurface, SpatialAnchorSelectionProjectionFactError,
    SpatialAnchorSelectionProjectionFactReceipt,
};
