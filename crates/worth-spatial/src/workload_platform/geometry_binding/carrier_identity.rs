use crate::bindings::identity::SpatialBindingIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryCarrierFamily {
    PlanarFace,
    PlanarEdge,
    PlanarLoop,
}

impl GeometryCarrierFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlanarFace => "planar face",
            Self::PlanarEdge => "planar edge",
            Self::PlanarLoop => "planar loop",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedGeometryCarrierFamily {
    NonPlanarFace,
    FreeformSurface,
    VolumetricFeature,
    Unknown,
}

impl UnsupportedGeometryCarrierFamily {
    pub fn human_label(self) -> &'static str {
        match self {
            Self::NonPlanarFace => "non-planar face carrier",
            Self::FreeformSurface => "freeform surface carrier",
            Self::VolumetricFeature => "volumetric feature carrier",
            Self::Unknown => "unknown geometry carrier family",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryCarrierIdentity {
    family: GeometryCarrierFamily,
    target_topology_identity: String,
    carrier_identity: String,
}

impl GeometryCarrierIdentity {
    pub(crate) fn from_spatial_binding(
        family: GeometryCarrierFamily,
        target_topology_identity: impl Into<String>,
        binding_identity: &SpatialBindingIdentity,
    ) -> Self {
        Self {
            family,
            target_topology_identity: target_topology_identity.into(),
            carrier_identity: format!(
                "geometry-carrier:{}:{}",
                family.as_str(),
                binding_identity.as_str()
            ),
        }
    }

    pub fn family(&self) -> GeometryCarrierFamily {
        self.family
    }

    pub fn target_topology_identity(&self) -> &str {
        &self.target_topology_identity
    }

    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }

    pub fn is_distinct_from_topology_identity(&self) -> bool {
        self.carrier_identity != self.target_topology_identity
    }
}
