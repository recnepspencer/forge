use worth_geom::facade::{CanonicalParameterPoint, ParameterSpacePoint};

use crate::bindings::{
    authority::{
        AdmittedCoedgePCurveBinding, AdmittedEdgeCurveBinding, AdmittedFaceSurfaceBinding,
        SpatialBindingCompleteness, SpatialBindingKind,
    },
    identity::{direction_anchor_basis, SpatialBindingIdentity},
};

use super::{
    carrier_ownership::{AnchorCarrierKind, AnchorCarrierOwnership},
    parameter_space_point::AnchorAttachableBinding,
    resolution::{
        canonicalize_parameter_point, validate_carrier_match, validate_direction_role,
        SpatialAnchorAuthorityError,
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

    pub fn admit(self) -> Result<AdmittedCarrierOwnedDirectionAnchor, SpatialAnchorAuthorityError> {
        Ok(AdmittedCarrierOwnedDirectionAnchor {
            ownership: self.ownership,
            canonical_parameter: self.canonical_parameter,
            role: self.role,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedCarrierOwnedDirectionAnchor {
    ownership: AnchorCarrierOwnership,
    canonical_parameter: CanonicalParameterPoint,
    role: AnchorDirectionRole,
}

impl AdmittedCarrierOwnedDirectionAnchor {
    pub fn ownership(&self) -> &AnchorCarrierOwnership {
        &self.ownership
    }

    pub fn canonical_parameter(&self) -> &CanonicalParameterPoint {
        &self.canonical_parameter
    }

    pub fn role(&self) -> AnchorDirectionRole {
        self.role
    }

    pub fn role_as_str(&self) -> &'static str {
        role_as_str(self.role)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedBindingDirectionAnchorAttachment<B> {
    binding: B,
    anchor: AdmittedCarrierOwnedDirectionAnchor,
    identity: SpatialBindingIdentity,
}

impl<B: AnchorAttachableBinding> AdmittedBindingDirectionAnchorAttachment<B> {
    fn new(binding: B, anchor: AdmittedCarrierOwnedDirectionAnchor) -> Self {
        let identity = SpatialBindingIdentity::from_basis(direction_anchor_basis(
            binding.identity().as_str(),
            anchor.ownership().carrier_kind().as_str(),
            anchor.ownership().carrier_identity(),
            anchor.canonical_parameter().point(),
            role_as_str(anchor.role()),
        ));
        Self {
            binding,
            anchor,
            identity,
        }
    }

    pub fn kind(&self) -> SpatialBindingKind {
        self.binding.kind()
    }

    pub fn binding(&self) -> &B {
        &self.binding
    }

    pub fn anchor(&self) -> &AdmittedCarrierOwnedDirectionAnchor {
        &self.anchor
    }

    pub fn identity(&self) -> &SpatialBindingIdentity {
        &self.identity
    }

    pub fn completeness(&self) -> &SpatialBindingCompleteness {
        self.binding.completeness()
    }
}

pub type AdmittedFaceSurfaceDirectionAnchorBinding =
    AdmittedBindingDirectionAnchorAttachment<AdmittedFaceSurfaceBinding>;
pub type AdmittedEdgeCurveDirectionAnchorBinding =
    AdmittedBindingDirectionAnchorAttachment<AdmittedEdgeCurveBinding>;
pub type AdmittedCoedgePCurveDirectionAnchorBinding =
    AdmittedBindingDirectionAnchorAttachment<AdmittedCoedgePCurveBinding>;

pub(crate) fn attach_direction_anchor<B: AnchorAttachableBinding>(
    binding: B,
    anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
    expected_kind: AnchorCarrierKind,
    expected_identity: &str,
) -> Result<AdmittedBindingDirectionAnchorAttachment<B>, SpatialAnchorAuthorityError> {
    validate_carrier_match(anchor_spec.ownership(), expected_kind, expected_identity)?;
    let anchor = anchor_spec.admit()?;
    Ok(AdmittedBindingDirectionAnchorAttachment::new(
        binding, anchor,
    ))
}

fn role_as_str(role: AnchorDirectionRole) -> &'static str {
    match role {
        AnchorDirectionRole::Tangent => "tangent",
        AnchorDirectionRole::Normal => "normal",
        AnchorDirectionRole::TangentU => "tangent_u",
        AnchorDirectionRole::TangentV => "tangent_v",
    }
}
