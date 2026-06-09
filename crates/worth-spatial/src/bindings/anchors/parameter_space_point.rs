use worth_geom::facade::{CanonicalParameterPoint, ParameterSpacePoint};

use super::{
    carrier_ownership::AnchorCarrierOwnership,
    resolution::{canonicalize_parameter_point, SpatialAnchorAuthorityError},
};

#[derive(Clone, Debug, PartialEq)]
pub struct CarrierOwnedParameterPointAnchorSpec {
    ownership: AnchorCarrierOwnership,
    canonical_parameter: CanonicalParameterPoint,
}

impl CarrierOwnedParameterPointAnchorSpec {
    pub fn new(
        ownership: AnchorCarrierOwnership,
        parameter: ParameterSpacePoint,
    ) -> Result<Self, SpatialAnchorAuthorityError> {
        let canonical_parameter = canonicalize_parameter_point(&ownership, parameter)?;
        Ok(Self {
            ownership,
            canonical_parameter,
        })
    }

    pub fn ownership(&self) -> &AnchorCarrierOwnership {
        &self.ownership
    }

    pub fn parameter(&self) -> ParameterSpacePoint {
        self.canonical_parameter.point()
    }
}
