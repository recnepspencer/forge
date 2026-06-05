use worth_geom::facade::{ParameterDomain, PolygonalTrimmedParameterRegion};

use crate::bindings::anchors::SpatialAnchorAuthorityError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnchorCarrierKind {
    FaceSurface,
    EdgeCurve,
    CoedgePCurve,
}

impl AnchorCarrierKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::FaceSurface => "face_surface",
            Self::EdgeCurve => "edge_curve",
            Self::CoedgePCurve => "coedge_pcurve",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnchorCarrierOwnership {
    carrier_kind: AnchorCarrierKind,
    carrier_identity: String,
    parameter_domain: ParameterDomain,
    trimmed_region: Option<PolygonalTrimmedParameterRegion>,
}

impl AnchorCarrierOwnership {
    pub fn for_face_surface(
        carrier_identity: impl Into<String>,
        parameter_domain: ParameterDomain,
    ) -> Result<Self, SpatialAnchorAuthorityError> {
        Self::new(
            AnchorCarrierKind::FaceSurface,
            carrier_identity,
            parameter_domain,
            None,
        )
    }

    pub fn for_trimmed_face_surface(
        carrier_identity: impl Into<String>,
        trimmed_region: PolygonalTrimmedParameterRegion,
    ) -> Result<Self, SpatialAnchorAuthorityError> {
        let parameter_domain = trimmed_region.domain().clone();
        Self::new(
            AnchorCarrierKind::FaceSurface,
            carrier_identity,
            parameter_domain,
            Some(trimmed_region),
        )
    }

    pub fn for_edge_curve(
        carrier_identity: impl Into<String>,
        parameter_domain: ParameterDomain,
    ) -> Result<Self, SpatialAnchorAuthorityError> {
        Self::new(
            AnchorCarrierKind::EdgeCurve,
            carrier_identity,
            parameter_domain,
            None,
        )
    }

    pub fn for_coedge_pcurve(
        carrier_identity: impl Into<String>,
        parameter_domain: ParameterDomain,
    ) -> Result<Self, SpatialAnchorAuthorityError> {
        Self::new(
            AnchorCarrierKind::CoedgePCurve,
            carrier_identity,
            parameter_domain,
            None,
        )
    }

    fn new(
        carrier_kind: AnchorCarrierKind,
        carrier_identity: impl Into<String>,
        parameter_domain: ParameterDomain,
        trimmed_region: Option<PolygonalTrimmedParameterRegion>,
    ) -> Result<Self, SpatialAnchorAuthorityError> {
        let carrier_identity = carrier_identity.into();
        if carrier_identity.is_empty() {
            return Err(SpatialAnchorAuthorityError::MissingCarrierOwnership(
                carrier_kind,
            ));
        }

        Ok(Self {
            carrier_kind,
            carrier_identity,
            parameter_domain,
            trimmed_region,
        })
    }

    pub fn carrier_kind(&self) -> AnchorCarrierKind {
        self.carrier_kind
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    pub fn parameter_domain(&self) -> &ParameterDomain {
        &self.parameter_domain
    }

    pub fn trimmed_region(&self) -> Option<&PolygonalTrimmedParameterRegion> {
        self.trimmed_region.as_ref()
    }
}
