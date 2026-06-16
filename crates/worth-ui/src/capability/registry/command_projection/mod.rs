mod command_projection_key;
mod command_projection_registry;
mod descriptor;
mod frozen_command_projection_capabilities;
mod frozen_command_projection_entry;
mod registration;

pub use command_projection_key::CommandProjectionKey;
pub(crate) use command_projection_registry::CommandProjectionRegistry;
pub use descriptor::{
    CommandProjectionCommandReference, CommandProjectionDescriptor, CommandProjectionGrouping,
    CommandProjectionIconLabelPolicy, CommandProjectionMeaningOverride,
    CommandProjectionMosaicScope, CommandProjectionOrdering, CommandProjectionOverflowBehavior,
    CommandProjectionReadinessDisplayPolicy, CommandProjectionSelectionMode,
    CommandProjectionShortcutVisibility, CommandProjectionSurface,
};
pub use frozen_command_projection_capabilities::FrozenCommandProjectionCapabilities;
pub use frozen_command_projection_entry::FrozenCommandProjectionEntry;
pub(crate) use registration::CommandProjectionAcceptedRegistrationProof;
