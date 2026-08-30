mod component_identity;
mod copy;
mod geometry;
mod palette;
mod projection;
mod typography;

pub use component_identity::PlatformPulseProductComponent;
pub use copy::PlatformPulseStaticCopy;
pub use geometry::{
    PlatformPulseCompositionExtent, PlatformPulseCompositionLayout,
    PlatformPulseCompositionLayoutDenial, PlatformPulseLogicalRect,
};
pub use palette::{PlatformPulsePaletteRole, PlatformPulseRgba, PlatformPulseSourceSignalRole};
pub use projection::{
    PlatformPulseProductFactSource, PlatformPulseProductRegion, PlatformPulseProductRegionContract,
    PlatformPulseProductTargetContract, PlatformPulseServiceStoryGate,
};
pub use typography::{PlatformPulseTextRole, PlatformPulseTextStyleContract};
