use worth_geom::facade::{CanonicalParameterPoint, ParameterSpacePoint};

use super::{
    carrier_ownership::AnchorCarrierOwnership,
    resolution::{
        canonicalize_parameter_point, validate_direction_role, SpatialAnchorAuthorityError,
    },
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnchorDirectionRole {
    Tangent,
    Normal,
    TangentU,
    TangentV,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CarrierOwnedParameterDirectionAnchorSpec {
    ownership: AnchorCarrierOwnership,
    canonical_parameter: CanonicalParameterPoint,
    role: AnchorDirectionRole,
}

impl CarrierOwnedParameterDirectionAnchorSpec {
    pub fn new(
        ownership: AnchorCarrierOwnership,
        parameter: ParameterSpacePoint,
        role: AnchorDirectionRole,
    ) -> Result<Self, SpatialAnchorAuthorityError> {
        validate_direction_role(ownership.carrier_kind(), role)?;
        let canonical_parameter = canonicalize_parameter_point(&ownership, parameter)?;
        Ok(Self {
            ownership,
            canonical_parameter,
            role,
        })
    }

    pub fn ownership(&self) -> &AnchorCarrierOwnership {
        &self.ownership
    }

    pub fn parameter(&self) -> ParameterSpacePoint {
        self.canonical_parameter.point()
    }

    pub fn role(&self) -> AnchorDirectionRole {
        self.role
    }
}
