use worth_geom::facade::{
    CanonicalParameterPoint, ParameterDomainError, ParameterSpacePoint,
    PolygonalTrimmedParameterRegion,
};

use crate::bindings::authority::SpatialBindingAuthorityError;

use super::{
    carrier_ownership::{AnchorCarrierKind, AnchorCarrierOwnership},
    parameter_space_direction::AnchorDirectionRole,
};

#[derive(Clone, Debug, PartialEq)]
pub enum SpatialAnchorAuthorityError {
    BindingAuthority(SpatialBindingAuthorityError),
    MissingCarrierOwnership(AnchorCarrierKind),
    CarrierFamilyMismatch {
        expected: AnchorCarrierKind,
        found: AnchorCarrierKind,
    },
    CarrierIdentityMismatch {
        expected: String,
        found: String,
    },
    ParameterDomainViolation(ParameterDomainError),
    UnsupportedDirectionRole {
        carrier_kind: AnchorCarrierKind,
        requested_role: AnchorDirectionRole,
    },
}

pub(crate) fn canonicalize_parameter_point(
    ownership: &AnchorCarrierOwnership,
    point: ParameterSpacePoint,
) -> Result<CanonicalParameterPoint, SpatialAnchorAuthorityError> {
    let canonical = ownership
        .parameter_domain()
        .canonicalize(point)
        .map_err(SpatialAnchorAuthorityError::ParameterDomainViolation)?;
    if let Some(trimmed_region) = ownership.trimmed_region() {
        admit_trimmed_region(trimmed_region, canonical.clone())?;
    }
    Ok(canonical)
}

pub(crate) fn validate_direction_role(
    carrier_kind: AnchorCarrierKind,
    requested_role: AnchorDirectionRole,
) -> Result<(), SpatialAnchorAuthorityError> {
    let supported = match carrier_kind {
        AnchorCarrierKind::FaceSurface => matches!(
            requested_role,
            AnchorDirectionRole::Normal
                | AnchorDirectionRole::TangentU
                | AnchorDirectionRole::TangentV
        ),
        AnchorCarrierKind::EdgeCurve | AnchorCarrierKind::CoedgePCurve => {
            matches!(requested_role, AnchorDirectionRole::Tangent)
        }
    };
    if supported {
        Ok(())
    } else {
        Err(SpatialAnchorAuthorityError::UnsupportedDirectionRole {
            carrier_kind,
            requested_role,
        })
    }
}

fn admit_trimmed_region(
    trimmed_region: &PolygonalTrimmedParameterRegion,
    canonical: CanonicalParameterPoint,
) -> Result<(), SpatialAnchorAuthorityError> {
    trimmed_region
        .admit(canonical)
        .map(|_| ())
        .map_err(SpatialAnchorAuthorityError::ParameterDomainViolation)
}
