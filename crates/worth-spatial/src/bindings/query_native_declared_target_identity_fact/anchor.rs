use crate::bindings::anchors::{
    direction_anchor_identity_basis, point_anchor_identity_basis, AnchorCarrierKind,
    AnchorDirectionRole, CarrierOwnedParameterDirectionAnchorSpec,
    CarrierOwnedParameterPointAnchorSpec, SpatialAnchorAuthorityError, SpatialAnchorIdentity,
};
use crate::bindings::authority::{
    CoedgePCurveBindingSpec, EdgeCurveBindingSpec, FaceSurfaceBindingSpec,
    SpatialBindingCompleteness, SpatialBindingKind,
};
use crate::bindings::query_native_anchor_binding_authoring::{
    AuthorPrimitiveAnchorBindingIntent, PrimitiveAnchorBindingAuthoringError,
    PrimitiveAnchorBindingDeclarationEntry,
};
use crate::bindings::query_native_target_identity::{
    GeometryTargetIdentityFactError, GeometryTargetKind,
};

use super::binding::{
    binding_fact_from_coedge_pcurve_spec, binding_fact_from_edge_curve_spec,
    binding_fact_from_face_surface_spec,
};

#[derive(Clone, Debug)]
pub(crate) struct AnchorBindingDeclarationFact {
    binding_kind: SpatialBindingKind,
    binding_identity: SpatialAnchorIdentity,
    site_identity: String,
    completeness: SpatialBindingCompleteness,
    target_kind: GeometryTargetKind,
}

impl AnchorBindingDeclarationFact {
    pub(crate) fn binding_kind(&self) -> SpatialBindingKind {
        self.binding_kind
    }

    pub(crate) fn binding_identity(&self) -> &SpatialAnchorIdentity {
        &self.binding_identity
    }

    pub(crate) fn site_identity(&self) -> &str {
        &self.site_identity
    }

    pub(crate) fn completeness(&self) -> SpatialBindingCompleteness {
        self.completeness
    }

    pub(crate) fn target_kind(&self) -> GeometryTargetKind {
        self.target_kind
    }
}

pub(crate) fn anchor_binding_declaration_fact(
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
) -> Result<AnchorBindingDeclarationFact, GeometryTargetIdentityFactError> {
    match declaration.intent() {
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToFace(binding, anchor) => {
            point_anchor_fact_from_face_spec(binding, anchor)
        }
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToEdge(binding, anchor) => {
            point_anchor_fact_from_edge_spec(binding, anchor)
        }
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToCoedge(binding, anchor) => {
            point_anchor_fact_from_coedge_spec(binding, anchor)
        }
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToFace(
            binding,
            anchor,
        ) => direction_anchor_fact_from_face_spec(binding, anchor),
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToEdge(
            binding,
            anchor,
        ) => direction_anchor_fact_from_edge_spec(binding, anchor),
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToCoedge(
            binding,
            anchor,
        ) => direction_anchor_fact_from_coedge_spec(binding, anchor),
    }
}

fn point_anchor_fact_from_face_spec(
    binding_spec: &FaceSurfaceBindingSpec,
    anchor_spec: &CarrierOwnedParameterPointAnchorSpec,
) -> Result<AnchorBindingDeclarationFact, GeometryTargetIdentityFactError> {
    let binding = binding_fact_from_face_surface_spec(binding_spec)
        .map_err(GeometryTargetIdentityFactError::BindingDeclarationDenied)?;
    validate_anchor_match(
        anchor_spec.ownership().carrier_kind(),
        anchor_spec.ownership().carrier_identity(),
        AnchorCarrierKind::FaceSurface,
        binding.site_identity(),
    )
    .map_err(GeometryTargetIdentityFactError::AnchorBindingDeclarationDenied)?;
    Ok(AnchorBindingDeclarationFact {
        binding_kind: SpatialBindingKind::FaceSurface,
        binding_identity: SpatialAnchorIdentity::from_basis(point_anchor_identity_basis(
            binding.binding_identity().as_str(),
            anchor_spec.ownership().carrier_kind().as_str(),
            anchor_spec.ownership().carrier_identity(),
            &anchor_spec.ownership().parameter_semantics_signature(),
            anchor_spec.parameter(),
        )),
        site_identity: binding.site_identity().to_string(),
        completeness: binding.completeness(),
        target_kind: GeometryTargetKind::FaceSurfacePointAnchor,
    })
}

fn point_anchor_fact_from_edge_spec(
    binding_spec: &EdgeCurveBindingSpec,
    anchor_spec: &CarrierOwnedParameterPointAnchorSpec,
) -> Result<AnchorBindingDeclarationFact, GeometryTargetIdentityFactError> {
    let binding = binding_fact_from_edge_curve_spec(binding_spec)
        .map_err(GeometryTargetIdentityFactError::BindingDeclarationDenied)?;
    validate_anchor_match(
        anchor_spec.ownership().carrier_kind(),
        anchor_spec.ownership().carrier_identity(),
        AnchorCarrierKind::EdgeCurve,
        binding.site_identity(),
    )
    .map_err(GeometryTargetIdentityFactError::AnchorBindingDeclarationDenied)?;
    Ok(AnchorBindingDeclarationFact {
        binding_kind: SpatialBindingKind::EdgeCurve,
        binding_identity: SpatialAnchorIdentity::from_basis(point_anchor_identity_basis(
            binding.binding_identity().as_str(),
            anchor_spec.ownership().carrier_kind().as_str(),
            anchor_spec.ownership().carrier_identity(),
            &anchor_spec.ownership().parameter_semantics_signature(),
            anchor_spec.parameter(),
        )),
        site_identity: binding.site_identity().to_string(),
        completeness: binding.completeness(),
        target_kind: GeometryTargetKind::EdgeCurvePointAnchor,
    })
}

fn point_anchor_fact_from_coedge_spec(
    binding_spec: &CoedgePCurveBindingSpec,
    anchor_spec: &CarrierOwnedParameterPointAnchorSpec,
) -> Result<AnchorBindingDeclarationFact, GeometryTargetIdentityFactError> {
    let binding = binding_fact_from_coedge_pcurve_spec(binding_spec)
        .map_err(GeometryTargetIdentityFactError::BindingDeclarationDenied)?;
    validate_anchor_match(
        anchor_spec.ownership().carrier_kind(),
        anchor_spec.ownership().carrier_identity(),
        AnchorCarrierKind::CoedgePCurve,
        binding.site_identity(),
    )
    .map_err(GeometryTargetIdentityFactError::AnchorBindingDeclarationDenied)?;
    Ok(AnchorBindingDeclarationFact {
        binding_kind: SpatialBindingKind::CoedgePCurve,
        binding_identity: SpatialAnchorIdentity::from_basis(point_anchor_identity_basis(
            binding.binding_identity().as_str(),
            anchor_spec.ownership().carrier_kind().as_str(),
            anchor_spec.ownership().carrier_identity(),
            &anchor_spec.ownership().parameter_semantics_signature(),
            anchor_spec.parameter(),
        )),
        site_identity: binding.site_identity().to_string(),
        completeness: binding.completeness(),
        target_kind: GeometryTargetKind::CoedgePCurvePointAnchor,
    })
}

fn direction_anchor_fact_from_face_spec(
    binding_spec: &FaceSurfaceBindingSpec,
    anchor_spec: &CarrierOwnedParameterDirectionAnchorSpec,
) -> Result<AnchorBindingDeclarationFact, GeometryTargetIdentityFactError> {
    let binding = binding_fact_from_face_surface_spec(binding_spec)
        .map_err(GeometryTargetIdentityFactError::BindingDeclarationDenied)?;
    validate_anchor_match(
        anchor_spec.ownership().carrier_kind(),
        anchor_spec.ownership().carrier_identity(),
        AnchorCarrierKind::FaceSurface,
        binding.site_identity(),
    )
    .map_err(GeometryTargetIdentityFactError::AnchorBindingDeclarationDenied)?;
    Ok(AnchorBindingDeclarationFact {
        binding_kind: SpatialBindingKind::FaceSurface,
        binding_identity: SpatialAnchorIdentity::from_basis(direction_anchor_identity_basis(
            binding.binding_identity().as_str(),
            anchor_spec.ownership().carrier_kind().as_str(),
            anchor_spec.ownership().carrier_identity(),
            &anchor_spec.ownership().parameter_semantics_signature(),
            anchor_spec.parameter(),
            direction_role_as_str(anchor_spec.role()),
        )),
        site_identity: binding.site_identity().to_string(),
        completeness: binding.completeness(),
        target_kind: GeometryTargetKind::FaceSurfaceDirectionAnchor,
    })
}

fn direction_anchor_fact_from_edge_spec(
    binding_spec: &EdgeCurveBindingSpec,
    anchor_spec: &CarrierOwnedParameterDirectionAnchorSpec,
) -> Result<AnchorBindingDeclarationFact, GeometryTargetIdentityFactError> {
    let binding = binding_fact_from_edge_curve_spec(binding_spec)
        .map_err(GeometryTargetIdentityFactError::BindingDeclarationDenied)?;
    validate_anchor_match(
        anchor_spec.ownership().carrier_kind(),
        anchor_spec.ownership().carrier_identity(),
        AnchorCarrierKind::EdgeCurve,
        binding.site_identity(),
    )
    .map_err(GeometryTargetIdentityFactError::AnchorBindingDeclarationDenied)?;
    Ok(AnchorBindingDeclarationFact {
        binding_kind: SpatialBindingKind::EdgeCurve,
        binding_identity: SpatialAnchorIdentity::from_basis(direction_anchor_identity_basis(
            binding.binding_identity().as_str(),
            anchor_spec.ownership().carrier_kind().as_str(),
            anchor_spec.ownership().carrier_identity(),
            &anchor_spec.ownership().parameter_semantics_signature(),
            anchor_spec.parameter(),
            direction_role_as_str(anchor_spec.role()),
        )),
        site_identity: binding.site_identity().to_string(),
        completeness: binding.completeness(),
        target_kind: GeometryTargetKind::EdgeCurveDirectionAnchor,
    })
}

fn direction_anchor_fact_from_coedge_spec(
    binding_spec: &CoedgePCurveBindingSpec,
    anchor_spec: &CarrierOwnedParameterDirectionAnchorSpec,
) -> Result<AnchorBindingDeclarationFact, GeometryTargetIdentityFactError> {
    let binding = binding_fact_from_coedge_pcurve_spec(binding_spec)
        .map_err(GeometryTargetIdentityFactError::BindingDeclarationDenied)?;
    validate_anchor_match(
        anchor_spec.ownership().carrier_kind(),
        anchor_spec.ownership().carrier_identity(),
        AnchorCarrierKind::CoedgePCurve,
        binding.site_identity(),
    )
    .map_err(GeometryTargetIdentityFactError::AnchorBindingDeclarationDenied)?;
    Ok(AnchorBindingDeclarationFact {
        binding_kind: SpatialBindingKind::CoedgePCurve,
        binding_identity: SpatialAnchorIdentity::from_basis(direction_anchor_identity_basis(
            binding.binding_identity().as_str(),
            anchor_spec.ownership().carrier_kind().as_str(),
            anchor_spec.ownership().carrier_identity(),
            &anchor_spec.ownership().parameter_semantics_signature(),
            anchor_spec.parameter(),
            direction_role_as_str(anchor_spec.role()),
        )),
        site_identity: binding.site_identity().to_string(),
        completeness: binding.completeness(),
        target_kind: GeometryTargetKind::CoedgePCurveDirectionAnchor,
    })
}

fn validate_anchor_match(
    found_kind: AnchorCarrierKind,
    found_identity: &str,
    expected_kind: AnchorCarrierKind,
    expected_identity: &str,
) -> Result<(), PrimitiveAnchorBindingAuthoringError> {
    if found_kind != expected_kind {
        return Err(PrimitiveAnchorBindingAuthoringError::Anchor(
            SpatialAnchorAuthorityError::CarrierFamilyMismatch {
                expected: expected_kind,
                found: found_kind,
            },
        ));
    }
    if found_identity != expected_identity {
        return Err(PrimitiveAnchorBindingAuthoringError::Anchor(
            SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                expected: expected_identity.to_string(),
                found: found_identity.to_string(),
            },
        ));
    }
    Ok(())
}

fn direction_role_as_str(role: AnchorDirectionRole) -> &'static str {
    match role {
        AnchorDirectionRole::Tangent => "tangent",
        AnchorDirectionRole::Normal => "normal",
        AnchorDirectionRole::TangentU => "tangent_u",
        AnchorDirectionRole::TangentV => "tangent_v",
    }
}
