use worth_primitives::PrimitiveGeometryIdentityBundle;

use super::binding_kind::SpatialBindingKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdmittedPartialBindingPosture {
    FaceSurfaceMissingVertexGeometry,
    EdgeCurveSingleVertexWitness,
    CoedgePCurveSingleVertexWitness,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialBindingCompleteness {
    Complete,
    AdmittedPartial(AdmittedPartialBindingPosture),
    DeniedIncomplete(SpatialBindingIncompleteness),
}

impl SpatialBindingCompleteness {
    pub const fn is_complete(&self) -> bool {
        matches!(self, Self::Complete)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialBindingIncompleteness {
    FaceSurfaceMissingSupportCarrier,
    EdgeCurveMissingCurveWitnessVertices,
    CoedgePCurveMissingPlanarSupport,
    CoedgePCurveMissingCurveWitnessVertices,
    VertexGeometryMissingVertexGeometry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialBindingUnsupportedReason {
    TopologyBirthClassDoesNotAdmitBindingKind {
        binding_kind: SpatialBindingKind,
        topology_birth_class: &'static str,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialBindingIllegalityReason {
    MissingTopologyIdentity(SpatialBindingKind),
}

pub(crate) fn evaluate_face_surface_completeness(
    geometry_identity: &PrimitiveGeometryIdentityBundle,
) -> SpatialBindingCompleteness {
    if !geometry_identity.has_any_support() {
        SpatialBindingCompleteness::DeniedIncomplete(
            SpatialBindingIncompleteness::FaceSurfaceMissingSupportCarrier,
        )
    } else if geometry_identity.vertices().is_empty() {
        SpatialBindingCompleteness::AdmittedPartial(
            AdmittedPartialBindingPosture::FaceSurfaceMissingVertexGeometry,
        )
    } else {
        SpatialBindingCompleteness::Complete
    }
}

pub(crate) fn evaluate_edge_curve_completeness(
    geometry_identity: &PrimitiveGeometryIdentityBundle,
) -> SpatialBindingCompleteness {
    match geometry_identity.vertices().len() {
        0 => SpatialBindingCompleteness::DeniedIncomplete(
            SpatialBindingIncompleteness::EdgeCurveMissingCurveWitnessVertices,
        ),
        1 => SpatialBindingCompleteness::AdmittedPartial(
            AdmittedPartialBindingPosture::EdgeCurveSingleVertexWitness,
        ),
        _ => SpatialBindingCompleteness::Complete,
    }
}

pub(crate) fn evaluate_coedge_pcurve_completeness(
    geometry_identity: &PrimitiveGeometryIdentityBundle,
) -> SpatialBindingCompleteness {
    if geometry_identity.support_planes().is_empty() {
        SpatialBindingCompleteness::DeniedIncomplete(
            SpatialBindingIncompleteness::CoedgePCurveMissingPlanarSupport,
        )
    } else {
        match geometry_identity.vertices().len() {
            0 => SpatialBindingCompleteness::DeniedIncomplete(
                SpatialBindingIncompleteness::CoedgePCurveMissingCurveWitnessVertices,
            ),
            1 => SpatialBindingCompleteness::AdmittedPartial(
                AdmittedPartialBindingPosture::CoedgePCurveSingleVertexWitness,
            ),
            _ => SpatialBindingCompleteness::Complete,
        }
    }
}

pub(crate) fn evaluate_vertex_geometry_completeness(
    geometry_identity: &PrimitiveGeometryIdentityBundle,
) -> SpatialBindingCompleteness {
    if geometry_identity.vertices().is_empty() {
        SpatialBindingCompleteness::DeniedIncomplete(
            SpatialBindingIncompleteness::VertexGeometryMissingVertexGeometry,
        )
    } else {
        SpatialBindingCompleteness::Complete
    }
}
