mod command_accepted_registration_proof;
mod command_category;
mod command_descriptor;
mod command_registration;
mod command_registry;
mod frozen_command_capabilities;
mod route;
mod shortcut;

pub(crate) use command_accepted_registration_proof::CommandAcceptedRegistrationProof;
pub use command_category::CommandCategory;
pub use command_descriptor::CommandDescriptor;
pub(crate) use command_registry::CommandRegistry;
pub use frozen_command_capabilities::FrozenCommandCapabilities;
pub use route::{
    UiCommandContextConsumption, UiCommandRegistrationGeneration, UiCommandRegistrationOwner,
    UiCommandRegistrationOwnerIdentity, UiCommandRepeatPolicy, UiCommandRouteDeclaration,
    UiCommandRouteDestination, UiCommandRoutePriority, UiCommandRouteScope,
    UiCommandRouteScopeIdentity, UiCommandTextInputPolicy,
};
pub use shortcut::{
    UiCommandKeyCode, UiCommandLogicalKey, UiCommandModifierSet, UiCommandPhysicalKey,
    UiCommandShortcutKey, UiCommandShortcutPlatform, UiCommandShortcutSequence,
    UiCommandShortcutStroke,
};
