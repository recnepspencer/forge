use crate::bindings::anchors::AnchorDirectionRole;
use crate::bindings::authority::SpatialBindingKind;
use crate::bindings::query_native_anchor_binding_authoring::AuthorPrimitiveAnchorBindingIntent;
use crate::bindings::query_native_binding_authoring::AuthorPrimitiveBindingIntent;
use crate::bindings::query_native_declared_target_identity_fact::{
    AnchorBindingDeclarationFact, BindingDeclarationFact,
};
use crate::bindings::query_native_target_identity::GeometryTargetKind;
use crate::bindings::rebinding::binding_snapshot::{AnchorSnapshot, BindingSnapshot};
use crate::bindings::rebinding::NeighborhoodBindingFamily;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DeclaredNeighborhoodBindingFact {
    binding_kind: SpatialBindingKind,
    binding_identity: String,
    site_identity: String,
    family: NeighborhoodBindingFamily,
    snapshot: BindingSnapshot,
}

impl DeclaredNeighborhoodBindingFact {
    pub(crate) fn binding_kind(&self) -> SpatialBindingKind {
        self.binding_kind
    }

    pub(crate) fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub(crate) fn site_identity(&self) -> &str {
        &self.site_identity
    }

    pub(crate) fn family(&self) -> NeighborhoodBindingFamily {
        self.family
    }

    pub(crate) fn snapshot(&self) -> &BindingSnapshot {
        &self.snapshot
    }
}

pub(crate) fn declared_neighborhood_binding_fact_from_binding_parts(
    intent: &AuthorPrimitiveBindingIntent,
    fact: &BindingDeclarationFact,
) -> DeclaredNeighborhoodBindingFact {
    let family = family_from_binding_kind(fact.binding_kind());
    let snapshot = match intent {
        AuthorPrimitiveBindingIntent::AttachSurfaceToFace(spec) => BindingSnapshot {
            family,
            birth_class: spec.birth_contract().topology_birth_class().to_string(),
            geometry_digest: spec
                .geometry_identity()
                .scaffold_geometry_digest()
                .as_str()
                .to_string(),
            completeness: fact.completeness(),
            anchor: None,
        },
        AuthorPrimitiveBindingIntent::AttachCurveToEdge(spec) => BindingSnapshot {
            family,
            birth_class: spec.birth_contract().topology_birth_class().to_string(),
            geometry_digest: spec
                .geometry_identity()
                .scaffold_geometry_digest()
                .as_str()
                .to_string(),
            completeness: fact.completeness(),
            anchor: None,
        },
        AuthorPrimitiveBindingIntent::AttachPCurveToCoedge(spec) => BindingSnapshot {
            family,
            birth_class: spec.birth_contract().topology_birth_class().to_string(),
            geometry_digest: spec
                .geometry_identity()
                .scaffold_geometry_digest()
                .as_str()
                .to_string(),
            completeness: fact.completeness(),
            anchor: None,
        },
        AuthorPrimitiveBindingIntent::AttachVertexGeometry(spec) => BindingSnapshot {
            family,
            birth_class: spec.birth_contract().topology_birth_class().to_string(),
            geometry_digest: spec
                .geometry_identity()
                .scaffold_geometry_digest()
                .as_str()
                .to_string(),
            completeness: fact.completeness(),
            anchor: None,
        },
    };
    DeclaredNeighborhoodBindingFact {
        binding_kind: fact.binding_kind(),
        binding_identity: fact.binding_identity().as_str().to_string(),
        site_identity: fact.site_identity().to_string(),
        family,
        snapshot,
    }
}

pub(crate) fn declared_neighborhood_binding_fact_from_anchor_parts(
    intent: &AuthorPrimitiveAnchorBindingIntent,
    fact: &AnchorBindingDeclarationFact,
) -> DeclaredNeighborhoodBindingFact {
    let family = family_from_anchor_target_kind(fact.target_kind());
    let snapshot = match intent {
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToFace(binding, anchor) => {
            BindingSnapshot {
                family,
                birth_class: binding.birth_contract().topology_birth_class().to_string(),
                geometry_digest: binding
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: fact.completeness(),
                anchor: Some(AnchorSnapshot::point(
                    anchor.ownership().carrier_kind().as_str(),
                    anchor.ownership().carrier_identity(),
                    &anchor.ownership().parameter_semantics_signature(),
                    anchor.parameter(),
                )),
            }
        }
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToEdge(binding, anchor) => {
            BindingSnapshot {
                family,
                birth_class: binding.birth_contract().topology_birth_class().to_string(),
                geometry_digest: binding
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: fact.completeness(),
                anchor: Some(AnchorSnapshot::point(
                    anchor.ownership().carrier_kind().as_str(),
                    anchor.ownership().carrier_identity(),
                    &anchor.ownership().parameter_semantics_signature(),
                    anchor.parameter(),
                )),
            }
        }
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToCoedge(binding, anchor) => {
            BindingSnapshot {
                family,
                birth_class: binding.birth_contract().topology_birth_class().to_string(),
                geometry_digest: binding
                    .geometry_identity()
                    .scaffold_geometry_digest()
                    .as_str()
                    .to_string(),
                completeness: fact.completeness(),
                anchor: Some(AnchorSnapshot::point(
                    anchor.ownership().carrier_kind().as_str(),
                    anchor.ownership().carrier_identity(),
                    &anchor.ownership().parameter_semantics_signature(),
                    anchor.parameter(),
                )),
            }
        }
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToFace(
            binding,
            anchor,
        ) => BindingSnapshot {
            family,
            birth_class: binding.birth_contract().topology_birth_class().to_string(),
            geometry_digest: binding
                .geometry_identity()
                .scaffold_geometry_digest()
                .as_str()
                .to_string(),
            completeness: fact.completeness(),
            anchor: Some(AnchorSnapshot::direction(
                anchor.ownership().carrier_kind().as_str(),
                anchor.ownership().carrier_identity(),
                &anchor.ownership().parameter_semantics_signature(),
                anchor.parameter(),
                anchor_direction_role_label(anchor.role()),
            )),
        },
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToEdge(
            binding,
            anchor,
        ) => BindingSnapshot {
            family,
            birth_class: binding.birth_contract().topology_birth_class().to_string(),
            geometry_digest: binding
                .geometry_identity()
                .scaffold_geometry_digest()
                .as_str()
                .to_string(),
            completeness: fact.completeness(),
            anchor: Some(AnchorSnapshot::direction(
                anchor.ownership().carrier_kind().as_str(),
                anchor.ownership().carrier_identity(),
                &anchor.ownership().parameter_semantics_signature(),
                anchor.parameter(),
                anchor_direction_role_label(anchor.role()),
            )),
        },
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToCoedge(
            binding,
            anchor,
        ) => BindingSnapshot {
            family,
            birth_class: binding.birth_contract().topology_birth_class().to_string(),
            geometry_digest: binding
                .geometry_identity()
                .scaffold_geometry_digest()
                .as_str()
                .to_string(),
            completeness: fact.completeness(),
            anchor: Some(AnchorSnapshot::direction(
                anchor.ownership().carrier_kind().as_str(),
                anchor.ownership().carrier_identity(),
                &anchor.ownership().parameter_semantics_signature(),
                anchor.parameter(),
                anchor_direction_role_label(anchor.role()),
            )),
        },
    };
    DeclaredNeighborhoodBindingFact {
        binding_kind: fact.binding_kind(),
        binding_identity: fact.binding_identity().as_str().to_string(),
        site_identity: fact.site_identity().to_string(),
        family,
        snapshot,
    }
}

fn family_from_binding_kind(binding_kind: SpatialBindingKind) -> NeighborhoodBindingFamily {
    match binding_kind {
        SpatialBindingKind::FaceSurface => NeighborhoodBindingFamily::FaceSurface,
        SpatialBindingKind::EdgeCurve => NeighborhoodBindingFamily::EdgeCurve,
        SpatialBindingKind::CoedgePCurve => NeighborhoodBindingFamily::CoedgePCurve,
        SpatialBindingKind::VertexGeometry => NeighborhoodBindingFamily::VertexGeometry,
    }
}

fn family_from_anchor_target_kind(target_kind: GeometryTargetKind) -> NeighborhoodBindingFamily {
    match target_kind {
        GeometryTargetKind::FaceSurfacePointAnchor => {
            NeighborhoodBindingFamily::FaceSurfacePointAnchor
        }
        GeometryTargetKind::EdgeCurvePointAnchor => NeighborhoodBindingFamily::EdgeCurvePointAnchor,
        GeometryTargetKind::CoedgePCurvePointAnchor => {
            NeighborhoodBindingFamily::CoedgePCurvePointAnchor
        }
        GeometryTargetKind::FaceSurfaceDirectionAnchor => {
            NeighborhoodBindingFamily::FaceSurfaceDirectionAnchor
        }
        GeometryTargetKind::EdgeCurveDirectionAnchor => {
            NeighborhoodBindingFamily::EdgeCurveDirectionAnchor
        }
        GeometryTargetKind::CoedgePCurveDirectionAnchor => {
            NeighborhoodBindingFamily::CoedgePCurveDirectionAnchor
        }
        _ => unreachable!("anchor declaration fact always yields an anchor target kind"),
    }
}

fn anchor_direction_role_label(role: AnchorDirectionRole) -> &'static str {
    match role {
        AnchorDirectionRole::Tangent => "tangent",
        AnchorDirectionRole::Normal => "normal",
        AnchorDirectionRole::TangentU => "tangent_u",
        AnchorDirectionRole::TangentV => "tangent_v",
    }
}
