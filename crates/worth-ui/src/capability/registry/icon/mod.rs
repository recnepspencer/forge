mod descriptor;
mod frozen_icon_capabilities;
mod frozen_icon_entry;
mod icon_key;
mod icon_registry;
mod registration;

pub use descriptor::{
    IconAccessibilityPosture, IconColorSupport, IconDescriptor, IconFamily,
    IconNativeVectorSupport, IconSizeSupport, IconSourceDescriptor, IconSourceKind,
    IconThemePosture, RawIconAssetReference,
};
pub use frozen_icon_capabilities::FrozenIconCapabilities;
pub use frozen_icon_entry::FrozenIconEntry;
pub use icon_key::IconKey;
pub(crate) use icon_registry::IconRegistry;
pub(crate) use registration::IconAcceptedRegistrationProof;
