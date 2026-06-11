mod canonical_entries;
mod carrier_ownership;
mod identity;
mod identity_basis;
mod parameter_space_direction;
mod parameter_space_point;
mod resolution;

pub use carrier_ownership::{AnchorCarrierKind, AnchorCarrierOwnership};
pub use identity::SpatialAnchorIdentity;
pub(crate) use identity_basis::{direction_anchor_identity_basis, point_anchor_identity_basis};
pub use parameter_space_direction::{
    AnchorDirectionRole, CarrierOwnedParameterDirectionAnchorSpec,
};
pub use parameter_space_point::CarrierOwnedParameterPointAnchorSpec;
pub use resolution::SpatialAnchorAuthorityError;
