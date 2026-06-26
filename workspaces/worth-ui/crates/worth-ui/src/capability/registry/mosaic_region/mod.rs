mod descriptor;
mod frozen_mosaic_region_capabilities;
mod mosaic_region_registry;
mod registration;

pub use descriptor::{
    MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture,
    MosaicRegionKindDescriptor, MosaicRegionPersistence, MosaicRegionRole, MosaicScrollOwnership,
    MosaicSizingBehavior,
};
pub use frozen_mosaic_region_capabilities::FrozenMosaicRegionCapabilities;
pub(crate) use mosaic_region_registry::MosaicRegionRegistry;
pub(crate) use registration::MosaicRegionAcceptedRegistrationProof;
