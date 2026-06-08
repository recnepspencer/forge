use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveGeometryIdentityBundle,
};

use crate::bindings::authority::SpatialBindingKind;

#[derive(Clone, Debug)]
pub(crate) enum SpatialBindingIdentityBasis {
    FaceSurface {
        site_identity: String,
        topology_birth_class: &'static str,
        geometry_digest: String,
        support_plane_count: usize,
        face_count: usize,
    },
    EdgeCurve {
        site_identity: String,
        topology_birth_class: &'static str,
        geometry_digest: String,
        edge_count: usize,
        vertex_count: usize,
    },
    CoedgePCurve {
        site_identity: String,
        topology_birth_class: &'static str,
        geometry_digest: String,
        loop_count: usize,
        support_plane_count: usize,
    },
    VertexGeometry {
        site_identity: String,
        topology_birth_class: &'static str,
        geometry_digest: String,
        vertex_count: usize,
        provenance_kind: String,
        tolerance_regime: String,
    },
}

impl SpatialBindingIdentityBasis {
    pub(crate) fn digest_parts(&self) -> Vec<String> {
        match self {
            Self::FaceSurface {
                site_identity,
                topology_birth_class,
                geometry_digest,
                support_plane_count,
                face_count,
            } => vec![
                SpatialBindingKind::FaceSurface.as_str().to_string(),
                site_identity.clone(),
                (*topology_birth_class).to_string(),
                geometry_digest.clone(),
                format!("support-plane-count:{support_plane_count}"),
                format!("face-count:{face_count}"),
            ],
            Self::EdgeCurve {
                site_identity,
                topology_birth_class,
                geometry_digest,
                edge_count,
                vertex_count,
            } => vec![
                SpatialBindingKind::EdgeCurve.as_str().to_string(),
                site_identity.clone(),
                (*topology_birth_class).to_string(),
                geometry_digest.clone(),
                format!("edge-count:{edge_count}"),
                format!("vertex-count:{vertex_count}"),
            ],
            Self::CoedgePCurve {
                site_identity,
                topology_birth_class,
                geometry_digest,
                loop_count,
                support_plane_count,
            } => vec![
                SpatialBindingKind::CoedgePCurve.as_str().to_string(),
                site_identity.clone(),
                (*topology_birth_class).to_string(),
                geometry_digest.clone(),
                format!("loop-count:{loop_count}"),
                format!("support-plane-count:{support_plane_count}"),
            ],
            Self::VertexGeometry {
                site_identity,
                topology_birth_class,
                geometry_digest,
                vertex_count,
                provenance_kind,
                tolerance_regime,
            } => vec![
                SpatialBindingKind::VertexGeometry.as_str().to_string(),
                site_identity.clone(),
                (*topology_birth_class).to_string(),
                geometry_digest.clone(),
                format!("vertex-count:{vertex_count}"),
                format!("provenance-kind:{provenance_kind}"),
                format!("tolerance-regime:{tolerance_regime}"),
            ],
        }
    }
}

pub(crate) fn face_surface_basis(
    site_identity: &str,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    geometry_identity: &PrimitiveGeometryIdentityBundle,
) -> SpatialBindingIdentityBasis {
    SpatialBindingIdentityBasis::FaceSurface {
        site_identity: site_identity.to_string(),
        topology_birth_class: birth_contract.topology_birth_class(),
        geometry_digest: geometry_identity
            .scaffold_geometry_digest()
            .as_str()
            .to_string(),
        support_plane_count: birth_contract.support_contract().support_plane_count(),
        face_count: birth_contract.topology_contract().face_count(),
    }
}

pub(crate) fn edge_curve_basis(
    site_identity: &str,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    geometry_identity: &PrimitiveGeometryIdentityBundle,
) -> SpatialBindingIdentityBasis {
    SpatialBindingIdentityBasis::EdgeCurve {
        site_identity: site_identity.to_string(),
        topology_birth_class: birth_contract.topology_birth_class(),
        geometry_digest: geometry_identity
            .scaffold_geometry_digest()
            .as_str()
            .to_string(),
        edge_count: birth_contract.topology_contract().edge_count(),
        vertex_count: birth_contract.topology_contract().vertex_count(),
    }
}

pub(crate) fn coedge_pcurve_basis(
    site_identity: &str,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    geometry_identity: &PrimitiveGeometryIdentityBundle,
) -> SpatialBindingIdentityBasis {
    SpatialBindingIdentityBasis::CoedgePCurve {
        site_identity: site_identity.to_string(),
        topology_birth_class: birth_contract.topology_birth_class(),
        geometry_digest: geometry_identity
            .scaffold_geometry_digest()
            .as_str()
            .to_string(),
        loop_count: birth_contract.topology_contract().loop_count(),
        support_plane_count: birth_contract.support_contract().support_plane_count(),
    }
}

pub(crate) fn vertex_geometry_basis(
    site_identity: &str,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    geometry_identity: &PrimitiveGeometryIdentityBundle,
    provenance_kind: &str,
    tolerance_regime: &str,
) -> SpatialBindingIdentityBasis {
    SpatialBindingIdentityBasis::VertexGeometry {
        site_identity: site_identity.to_string(),
        topology_birth_class: birth_contract.topology_birth_class(),
        geometry_digest: geometry_identity
            .scaffold_geometry_digest()
            .as_str()
            .to_string(),
        vertex_count: birth_contract.topology_contract().vertex_count(),
        provenance_kind: provenance_kind.to_string(),
        tolerance_regime: tolerance_regime.to_string(),
    }
}
