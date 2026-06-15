mod frozen_surface_capabilities;
mod surface_accepted_registration_proof;
mod surface_descriptor;
mod surface_kind;
mod surface_placement_class;
mod surface_registration;
mod surface_registry;
mod surface_state_class;

pub use frozen_surface_capabilities::FrozenSurfaceCapabilities;
pub(crate) use surface_accepted_registration_proof::SurfaceAcceptedRegistrationProof;
pub use surface_descriptor::SurfaceDescriptor;
pub use surface_kind::SurfaceKind;
pub use surface_placement_class::SurfacePlacementClass;
pub(crate) use surface_registry::SurfaceRegistry;
pub use surface_state_class::SurfaceStateClass;
