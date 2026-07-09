mod descriptors;
mod metadata;
mod runtime;
mod slots;

pub use descriptors::{IdentityBindingDescriptor, QueryBindingDescriptor};
pub use metadata::{
    BindingError, BindingFailureClass, NonIdentityBindingMetadata, NonIdentityBindingMetadataKey,
};
pub use runtime::{
    derive_binding_requirements, resolve_bindings, BindingRequirement, BindingRequirements,
    BindingResolution, BindingResolutionError, BoundBinding, BoundBindings,
};
pub use slots::{QueryBindingSlot, QueryBindingSubject};
