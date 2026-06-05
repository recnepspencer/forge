use worth_geom::facade::{CanonicalParameterPoint, ParameterSpacePoint};

use crate::bindings::{
    anchors::{identity_basis::point_anchor_identity_basis, SpatialAnchorIdentity},
    authority::{
        AdmittedCoedgePCurveBinding, AdmittedEdgeCurveBinding, AdmittedFaceSurfaceBinding,
        SpatialBindingCompleteness, SpatialBindingKind,
    },
    identity::SpatialBindingIdentity,
};

use super::{
    carrier_ownership::{AnchorCarrierKind, AnchorCarrierOwnership},
    resolution::{
        canonicalize_parameter_point, validate_carrier_match, SpatialAnchorAuthorityError,
    },
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

    pub fn admit(self) -> Result<AdmittedCarrierOwnedPointAnchor, SpatialAnchorAuthorityError> {
        Ok(AdmittedCarrierOwnedPointAnchor {
            ownership: self.ownership,
            canonical_parameter: self.canonical_parameter,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedCarrierOwnedPointAnchor {
    ownership: AnchorCarrierOwnership,
    canonical_parameter: CanonicalParameterPoint,
}

impl AdmittedCarrierOwnedPointAnchor {
    pub fn ownership(&self) -> &AnchorCarrierOwnership {
        &self.ownership
    }

    pub fn canonical_parameter(&self) -> &CanonicalParameterPoint {
        &self.canonical_parameter
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedBindingPointAnchorAttachment<B> {
    binding: B,
    anchor: AdmittedCarrierOwnedPointAnchor,
    identity: SpatialAnchorIdentity,
}

impl<B: AnchorAttachableBinding> AdmittedBindingPointAnchorAttachment<B> {
    fn new(binding: B, anchor: AdmittedCarrierOwnedPointAnchor) -> Self {
        let identity = SpatialAnchorIdentity::from_basis(point_anchor_identity_basis(
            binding.identity().as_str(),
            anchor.ownership().carrier_kind().as_str(),
            anchor.ownership().carrier_identity(),
            &anchor.ownership().parameter_semantics_signature(),
            anchor.canonical_parameter().point(),
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

    pub fn anchor(&self) -> &AdmittedCarrierOwnedPointAnchor {
        &self.anchor
    }

    pub fn identity(&self) -> &SpatialAnchorIdentity {
        &self.identity
    }

    pub fn completeness(&self) -> &SpatialBindingCompleteness {
        self.binding.completeness()
    }
}

pub type AdmittedFaceSurfacePointAnchorBinding =
    AdmittedBindingPointAnchorAttachment<AdmittedFaceSurfaceBinding>;
pub type AdmittedEdgeCurvePointAnchorBinding =
    AdmittedBindingPointAnchorAttachment<AdmittedEdgeCurveBinding>;
pub type AdmittedCoedgePCurvePointAnchorBinding =
    AdmittedBindingPointAnchorAttachment<AdmittedCoedgePCurveBinding>;

pub trait AnchorAttachableBinding: Clone {
    fn kind(&self) -> SpatialBindingKind;
    fn identity(&self) -> &SpatialBindingIdentity;
    fn completeness(&self) -> &SpatialBindingCompleteness;
}

impl AnchorAttachableBinding for AdmittedFaceSurfaceBinding {
    fn kind(&self) -> SpatialBindingKind {
        self.kind()
    }

    fn identity(&self) -> &SpatialBindingIdentity {
        self.identity()
    }

    fn completeness(&self) -> &SpatialBindingCompleteness {
        self.completeness()
    }
}

impl AnchorAttachableBinding for AdmittedEdgeCurveBinding {
    fn kind(&self) -> SpatialBindingKind {
        self.kind()
    }

    fn identity(&self) -> &SpatialBindingIdentity {
        self.identity()
    }

    fn completeness(&self) -> &SpatialBindingCompleteness {
        self.completeness()
    }
}

impl AnchorAttachableBinding for AdmittedCoedgePCurveBinding {
    fn kind(&self) -> SpatialBindingKind {
        self.kind()
    }

    fn identity(&self) -> &SpatialBindingIdentity {
        self.identity()
    }

    fn completeness(&self) -> &SpatialBindingCompleteness {
        self.completeness()
    }
}

pub(crate) fn attach_point_anchor<B: AnchorAttachableBinding>(
    binding: B,
    anchor_spec: CarrierOwnedParameterPointAnchorSpec,
    expected_kind: AnchorCarrierKind,
    expected_identity: &str,
) -> Result<AdmittedBindingPointAnchorAttachment<B>, SpatialAnchorAuthorityError> {
    validate_carrier_match(anchor_spec.ownership(), expected_kind, expected_identity)?;
    let anchor = anchor_spec.admit()?;
    Ok(AdmittedBindingPointAnchorAttachment::new(binding, anchor))
}
