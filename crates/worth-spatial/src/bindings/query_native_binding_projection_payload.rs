use crate::bindings::authority::{SpatialBindingCompleteness, SpatialBindingKind};
use crate::bindings::query_native_declared_target_identity_fact::{
    AnchorBindingDeclarationFact, BindingDeclarationFact,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalGeometryTargetKind {
    FaceSurface,
    EdgeCurve,
    CoedgePCurve,
    VertexGeometry,
    FaceSurfacePointAnchor,
    EdgeCurvePointAnchor,
    CoedgePCurvePointAnchor,
    FaceSurfaceDirectionAnchor,
    EdgeCurveDirectionAnchor,
    CoedgePCurveDirectionAnchor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveBindingProjectionPayload {
    binding_kind: SpatialBindingKind,
    binding_identity: String,
    site_identity: String,
    completeness: SpatialBindingCompleteness,
}

impl PrimitiveBindingProjectionPayload {
    pub fn from_binding_fact(fact: &BindingDeclarationFact) -> Self {
        Self {
            binding_kind: fact.binding_kind(),
            binding_identity: fact.binding_identity().as_str().to_string(),
            site_identity: fact.site_identity().to_string(),
            completeness: fact.completeness(),
        }
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        self.binding_kind
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn site_identity(&self) -> &str {
        &self.site_identity
    }

    pub fn completeness(&self) -> SpatialBindingCompleteness {
        self.completeness
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveBindingTargetIdentityPayload {
    target_identity: String,
    target_kind: CanonicalGeometryTargetKind,
    alias_identities: Vec<String>,
}

impl PrimitiveBindingTargetIdentityPayload {
    pub fn from_binding_fact(fact: &BindingDeclarationFact) -> Self {
        Self {
            target_identity: fact.site_identity().to_string(),
            target_kind: match fact.binding_kind() {
                SpatialBindingKind::FaceSurface => CanonicalGeometryTargetKind::FaceSurface,
                SpatialBindingKind::EdgeCurve => CanonicalGeometryTargetKind::EdgeCurve,
                SpatialBindingKind::CoedgePCurve => CanonicalGeometryTargetKind::CoedgePCurve,
                SpatialBindingKind::VertexGeometry => CanonicalGeometryTargetKind::VertexGeometry,
            },
            alias_identities: vec![fact.binding_identity().as_str().to_string()],
        }
    }

    pub fn target_identity(&self) -> &str {
        &self.target_identity
    }

    pub fn target_kind(&self) -> CanonicalGeometryTargetKind {
        self.target_kind
    }

    pub fn alias_identities(&self) -> &[String] {
        &self.alias_identities
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveAnchorBindingProjectionPayload {
    binding_kind: SpatialBindingKind,
    binding_identity: String,
    site_identity: String,
    completeness: SpatialBindingCompleteness,
}

impl PrimitiveAnchorBindingProjectionPayload {
    pub fn from_binding_fact(fact: &AnchorBindingDeclarationFact) -> Self {
        Self {
            binding_kind: fact.binding_kind(),
            binding_identity: fact.binding_identity().as_str().to_string(),
            site_identity: fact.site_identity().to_string(),
            completeness: fact.completeness(),
        }
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        self.binding_kind
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn site_identity(&self) -> &str {
        &self.site_identity
    }

    pub fn completeness(&self) -> SpatialBindingCompleteness {
        self.completeness
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveAnchorBindingTargetIdentityPayload {
    target_identity: String,
    target_kind: CanonicalGeometryTargetKind,
    alias_identities: Vec<String>,
}

impl PrimitiveAnchorBindingTargetIdentityPayload {
    pub fn from_binding_fact(fact: &AnchorBindingDeclarationFact) -> Self {
        let target_kind = match fact.target_kind() {
            crate::bindings::query_native_target_identity::GeometryTargetKind::FaceSurface => {
                CanonicalGeometryTargetKind::FaceSurface
            }
            crate::bindings::query_native_target_identity::GeometryTargetKind::EdgeCurve => {
                CanonicalGeometryTargetKind::EdgeCurve
            }
            crate::bindings::query_native_target_identity::GeometryTargetKind::CoedgePCurve => {
                CanonicalGeometryTargetKind::CoedgePCurve
            }
            crate::bindings::query_native_target_identity::GeometryTargetKind::VertexGeometry => {
                CanonicalGeometryTargetKind::VertexGeometry
            }
            crate::bindings::query_native_target_identity::GeometryTargetKind::FaceSurfacePointAnchor => {
                CanonicalGeometryTargetKind::FaceSurfacePointAnchor
            }
            crate::bindings::query_native_target_identity::GeometryTargetKind::EdgeCurvePointAnchor => {
                CanonicalGeometryTargetKind::EdgeCurvePointAnchor
            }
            crate::bindings::query_native_target_identity::GeometryTargetKind::CoedgePCurvePointAnchor => {
                CanonicalGeometryTargetKind::CoedgePCurvePointAnchor
            }
            crate::bindings::query_native_target_identity::GeometryTargetKind::FaceSurfaceDirectionAnchor => {
                CanonicalGeometryTargetKind::FaceSurfaceDirectionAnchor
            }
            crate::bindings::query_native_target_identity::GeometryTargetKind::EdgeCurveDirectionAnchor => {
                CanonicalGeometryTargetKind::EdgeCurveDirectionAnchor
            }
            crate::bindings::query_native_target_identity::GeometryTargetKind::CoedgePCurveDirectionAnchor => {
                CanonicalGeometryTargetKind::CoedgePCurveDirectionAnchor
            }
        };
        Self {
            target_identity: fact.site_identity().to_string(),
            target_kind,
            alias_identities: vec![fact.binding_identity().as_str().to_string()],
        }
    }

    pub fn target_identity(&self) -> &str {
        &self.target_identity
    }

    pub fn target_kind(&self) -> CanonicalGeometryTargetKind {
        self.target_kind
    }

    pub fn alias_identities(&self) -> &[String] {
        &self.alias_identities
    }
}
