mod bundle;
mod declaration;
mod descriptor;
mod descriptor_set;
mod digest;
mod errors;
mod families;
mod freeze_report;
mod identity;
mod reference;
mod registration;
mod registry;

#[cfg(test)]
pub(crate) use families::built_in_policy_registrations;

pub use bundle::{LoweredResourcePolicyBundle, ResourceResolvedPolicyBundle};
pub use declaration::ValidatedResourcePolicyDeclaration;
pub use descriptor::ResourcePolicyDescriptor;
pub use descriptor_set::FrozenResourcePolicyDescriptorSet;
pub use errors::{ResourcePolicyRegistryError, ResourcePolicyResolutionError};
pub use freeze_report::ResourcePolicyRegistryFreezeReport;
pub use identity::{
    ResourcePolicyCompatibilityPosture, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicySelectionBasis, ResourcePolicyVersion,
};
pub use reference::{
    FrozenResourcePolicyDescriptor, ResourceResolvedPolicy, ValidatedResourcePolicyReference,
};
pub use registration::ResourcePolicyRegistration;
pub use registry::FrozenResourcePolicyRegistry;
