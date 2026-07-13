mod artifacts;
mod counters;
mod declarative_request;
mod errors;
mod field_path;
mod influence;
mod masks;
mod support;

pub(crate) use artifacts::derive_authorized_projection;
pub use artifacts::AuthorizedProjectionArtifact;
pub use counters::AuthorizedProjectionCounters;
pub(crate) use declarative_request::{
    reconcile_authorized_declarative_projection, AuthorizedDeclarativeProjection,
};
pub use errors::{AuthorizedProjectionError, AuthorizedProjectionFailureClass};
pub use field_path::{
    AuthorizedProjectionFieldPath, AuthorizedProjectionIdentity, MaskedProjectionArtifact,
    PolicyFieldInfluenceSet,
};
pub use influence::{PolicyInfluenceEntry, PolicyInfluencePurpose, PolicyInfluenceSet};
pub use masks::{PolicyAspectMask, PolicyMaskSnapshot, ProjectionVisibility};
pub use support::{
    runtime_backed_authorized_projection_support_profile, AuthorizedProjectionSupportProfile,
    AuthorizedProjectionSupportStatus, AuthorizedProjectionSurface,
};

#[cfg(test)]
mod tests;
