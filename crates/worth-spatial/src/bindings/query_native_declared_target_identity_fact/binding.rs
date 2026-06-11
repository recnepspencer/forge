use crate::bindings::authority::{
    evaluate_coedge_pcurve_completeness, evaluate_edge_curve_completeness,
    evaluate_face_surface_completeness, evaluate_vertex_geometry_completeness,
    CoedgePCurveBindingSpec, EdgeCurveBindingSpec, FaceSurfaceBindingSpec,
    SpatialBindingAuthorityError, SpatialBindingCompleteness, SpatialBindingIllegalityReason,
    SpatialBindingKind, SpatialBindingUnsupportedReason, VertexGeometryBindingSpec,
};
use crate::bindings::identity::{
    coedge_pcurve_basis, edge_curve_basis, face_surface_basis, vertex_geometry_basis,
    SpatialBindingIdentity,
};
use crate::bindings::query_native_binding_authoring::{
    AuthorPrimitiveBindingIntent, PrimitiveBindingAuthoringError, PrimitiveBindingDeclarationEntry,
};
use crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError;

#[derive(Clone, Debug)]
pub(crate) struct BindingDeclarationFact {
    binding_kind: SpatialBindingKind,
    binding_identity: SpatialBindingIdentity,
    site_identity: String,
    completeness: SpatialBindingCompleteness,
}

impl BindingDeclarationFact {
    pub(crate) fn binding_kind(&self) -> SpatialBindingKind {
        self.binding_kind
    }

    pub(crate) fn binding_identity(&self) -> &SpatialBindingIdentity {
        &self.binding_identity
    }

    pub(crate) fn site_identity(&self) -> &str {
        &self.site_identity
    }

    pub(crate) fn completeness(&self) -> SpatialBindingCompleteness {
        self.completeness
    }
}

pub(crate) fn binding_declaration_fact(
    declaration: &PrimitiveBindingDeclarationEntry,
) -> Result<BindingDeclarationFact, GeometryTargetIdentityFactError> {
    match declaration.intent() {
        AuthorPrimitiveBindingIntent::AttachSurfaceToFace(spec) => {
            binding_fact_from_face_surface_spec(spec)
                .map_err(GeometryTargetIdentityFactError::BindingDeclarationDenied)
        }
        AuthorPrimitiveBindingIntent::AttachCurveToEdge(spec) => {
            binding_fact_from_edge_curve_spec(spec)
                .map_err(GeometryTargetIdentityFactError::BindingDeclarationDenied)
        }
        AuthorPrimitiveBindingIntent::AttachPCurveToCoedge(spec) => {
            binding_fact_from_coedge_pcurve_spec(spec)
                .map_err(GeometryTargetIdentityFactError::BindingDeclarationDenied)
        }
        AuthorPrimitiveBindingIntent::AttachVertexGeometry(spec) => {
            binding_fact_from_vertex_geometry_spec(spec)
                .map_err(GeometryTargetIdentityFactError::BindingDeclarationDenied)
        }
    }
}

pub(super) fn binding_fact_from_face_surface_spec(
    spec: &FaceSurfaceBindingSpec,
) -> Result<BindingDeclarationFact, PrimitiveBindingAuthoringError> {
    validate_face_surface_spec(spec)?;
    Ok(BindingDeclarationFact {
        binding_kind: SpatialBindingKind::FaceSurface,
        binding_identity: SpatialBindingIdentity::from_basis(face_surface_basis(
            spec.site().topology_face_identity(),
            spec.birth_contract(),
            spec.geometry_identity(),
        )),
        site_identity: spec.site().topology_face_identity().to_string(),
        completeness: evaluate_face_surface_completeness(spec.geometry_identity()),
    })
}

pub(super) fn binding_fact_from_edge_curve_spec(
    spec: &EdgeCurveBindingSpec,
) -> Result<BindingDeclarationFact, PrimitiveBindingAuthoringError> {
    validate_edge_curve_spec(spec)?;
    Ok(BindingDeclarationFact {
        binding_kind: SpatialBindingKind::EdgeCurve,
        binding_identity: SpatialBindingIdentity::from_basis(edge_curve_basis(
            spec.site().topology_edge_identity(),
            spec.birth_contract(),
            spec.geometry_identity(),
        )),
        site_identity: spec.site().topology_edge_identity().to_string(),
        completeness: evaluate_edge_curve_completeness(spec.geometry_identity()),
    })
}

pub(super) fn binding_fact_from_coedge_pcurve_spec(
    spec: &CoedgePCurveBindingSpec,
) -> Result<BindingDeclarationFact, PrimitiveBindingAuthoringError> {
    validate_coedge_pcurve_spec(spec)?;
    Ok(BindingDeclarationFact {
        binding_kind: SpatialBindingKind::CoedgePCurve,
        binding_identity: SpatialBindingIdentity::from_basis(coedge_pcurve_basis(
            spec.site().topology_coedge_identity(),
            spec.birth_contract(),
            spec.geometry_identity(),
        )),
        site_identity: spec.site().topology_coedge_identity().to_string(),
        completeness: evaluate_coedge_pcurve_completeness(spec.geometry_identity()),
    })
}

pub(super) fn binding_fact_from_vertex_geometry_spec(
    spec: &VertexGeometryBindingSpec,
) -> Result<BindingDeclarationFact, PrimitiveBindingAuthoringError> {
    validate_vertex_geometry_spec(spec)?;
    Ok(BindingDeclarationFact {
        binding_kind: SpatialBindingKind::VertexGeometry,
        binding_identity: SpatialBindingIdentity::from_basis(vertex_geometry_basis(
            spec.site().topology_vertex_identity(),
            spec.birth_contract(),
            spec.geometry_identity(),
            spec.provenance().as_str(),
            spec.tolerance_regime().as_str(),
        )),
        site_identity: spec.site().topology_vertex_identity().to_string(),
        completeness: evaluate_vertex_geometry_completeness(spec.geometry_identity()),
    })
}

fn validate_face_surface_spec(
    spec: &FaceSurfaceBindingSpec,
) -> Result<(), PrimitiveBindingAuthoringError> {
    if spec.site().topology_face_identity().is_empty() {
        return Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::FaceSurface,
                ),
            ),
        ));
    }
    if spec.birth_contract().topology_contract().face_count() == 0 {
        return Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Unsupported(
                SpatialBindingUnsupportedReason::TopologyBirthClassDoesNotAdmitBindingKind {
                    binding_kind: SpatialBindingKind::FaceSurface,
                    topology_birth_class: spec.birth_contract().topology_birth_class(),
                },
            ),
        ));
    }
    Ok(())
}

fn validate_edge_curve_spec(
    spec: &EdgeCurveBindingSpec,
) -> Result<(), PrimitiveBindingAuthoringError> {
    if spec.site().topology_edge_identity().is_empty() {
        return Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::EdgeCurve,
                ),
            ),
        ));
    }
    if spec.birth_contract().topology_contract().edge_count() == 0 {
        return Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Unsupported(
                SpatialBindingUnsupportedReason::TopologyBirthClassDoesNotAdmitBindingKind {
                    binding_kind: SpatialBindingKind::EdgeCurve,
                    topology_birth_class: spec.birth_contract().topology_birth_class(),
                },
            ),
        ));
    }
    Ok(())
}

fn validate_coedge_pcurve_spec(
    spec: &CoedgePCurveBindingSpec,
) -> Result<(), PrimitiveBindingAuthoringError> {
    if spec.site().topology_coedge_identity().is_empty() {
        return Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::CoedgePCurve,
                ),
            ),
        ));
    }
    if spec.birth_contract().topology_contract().loop_count() == 0 {
        return Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Unsupported(
                SpatialBindingUnsupportedReason::TopologyBirthClassDoesNotAdmitBindingKind {
                    binding_kind: SpatialBindingKind::CoedgePCurve,
                    topology_birth_class: spec.birth_contract().topology_birth_class(),
                },
            ),
        ));
    }
    Ok(())
}

fn validate_vertex_geometry_spec(
    spec: &VertexGeometryBindingSpec,
) -> Result<(), PrimitiveBindingAuthoringError> {
    if spec.site().topology_vertex_identity().is_empty() {
        return Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::VertexGeometry,
                ),
            ),
        ));
    }
    if spec.birth_contract().topology_contract().vertex_count() == 0 {
        return Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Unsupported(
                SpatialBindingUnsupportedReason::TopologyBirthClassDoesNotAdmitBindingKind {
                    binding_kind: SpatialBindingKind::VertexGeometry,
                    topology_birth_class: spec.birth_contract().topology_birth_class(),
                },
            ),
        ));
    }
    Ok(())
}
