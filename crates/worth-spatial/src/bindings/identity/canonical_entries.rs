use crate::bindings::authority::{
    CoedgePCurveBindingSpec, EdgeCurveBindingSpec, FaceSurfaceBindingSpec,
    VertexGeometryBindingSpec,
};
use crate::bindings::canonical_projection::SpatialCanonicalDeclarationField;

use super::identity_basis::{
    coedge_pcurve_basis, edge_curve_basis, face_surface_basis, vertex_geometry_basis,
    SpatialBindingIdentityBasis,
};

impl SpatialBindingIdentityBasis {
    pub(crate) fn canonical_declaration_entries(&self) -> Vec<SpatialCanonicalDeclarationField> {
        match self {
            Self::FaceSurface {
                site_identity,
                topology_birth_class,
                geometry_digest,
                support_plane_count,
                face_count,
            } => vec![
                SpatialCanonicalDeclarationField::new("binding_kind", "face_surface"),
                SpatialCanonicalDeclarationField::new("site_identity", site_identity),
                SpatialCanonicalDeclarationField::new(
                    "topology_birth_class",
                    *topology_birth_class,
                ),
                SpatialCanonicalDeclarationField::new("geometry_digest", geometry_digest),
                SpatialCanonicalDeclarationField::new(
                    "support_plane_count",
                    support_plane_count.to_string(),
                ),
                SpatialCanonicalDeclarationField::new("face_count", face_count.to_string()),
            ],
            Self::EdgeCurve {
                site_identity,
                topology_birth_class,
                geometry_digest,
                edge_count,
                vertex_count,
            } => vec![
                SpatialCanonicalDeclarationField::new("binding_kind", "edge_curve"),
                SpatialCanonicalDeclarationField::new("site_identity", site_identity),
                SpatialCanonicalDeclarationField::new(
                    "topology_birth_class",
                    *topology_birth_class,
                ),
                SpatialCanonicalDeclarationField::new("geometry_digest", geometry_digest),
                SpatialCanonicalDeclarationField::new("edge_count", edge_count.to_string()),
                SpatialCanonicalDeclarationField::new("vertex_count", vertex_count.to_string()),
            ],
            Self::CoedgePCurve {
                site_identity,
                topology_birth_class,
                geometry_digest,
                loop_count,
                support_plane_count,
            } => vec![
                SpatialCanonicalDeclarationField::new("binding_kind", "coedge_pcurve"),
                SpatialCanonicalDeclarationField::new("site_identity", site_identity),
                SpatialCanonicalDeclarationField::new(
                    "topology_birth_class",
                    *topology_birth_class,
                ),
                SpatialCanonicalDeclarationField::new("geometry_digest", geometry_digest),
                SpatialCanonicalDeclarationField::new("loop_count", loop_count.to_string()),
                SpatialCanonicalDeclarationField::new(
                    "support_plane_count",
                    support_plane_count.to_string(),
                ),
            ],
            Self::VertexGeometry {
                site_identity,
                topology_birth_class,
                geometry_digest,
                vertex_count,
                provenance_kind,
                tolerance_regime,
            } => vec![
                SpatialCanonicalDeclarationField::new("binding_kind", "vertex_geometry"),
                SpatialCanonicalDeclarationField::new("site_identity", site_identity),
                SpatialCanonicalDeclarationField::new(
                    "topology_birth_class",
                    *topology_birth_class,
                ),
                SpatialCanonicalDeclarationField::new("geometry_digest", geometry_digest),
                SpatialCanonicalDeclarationField::new("vertex_count", vertex_count.to_string()),
                SpatialCanonicalDeclarationField::new("provenance_kind", provenance_kind),
                SpatialCanonicalDeclarationField::new("tolerance_regime", tolerance_regime),
            ],
        }
    }
}

impl FaceSurfaceBindingSpec {
    pub fn canonical_declaration_fields(&self) -> Vec<SpatialCanonicalDeclarationField> {
        face_surface_basis(
            self.site().topology_face_identity(),
            self.birth_contract(),
            self.geometry_identity(),
        )
        .canonical_declaration_entries()
    }
}

impl EdgeCurveBindingSpec {
    pub fn canonical_declaration_fields(&self) -> Vec<SpatialCanonicalDeclarationField> {
        edge_curve_basis(
            self.site().topology_edge_identity(),
            self.birth_contract(),
            self.geometry_identity(),
        )
        .canonical_declaration_entries()
    }
}

impl CoedgePCurveBindingSpec {
    pub fn canonical_declaration_fields(&self) -> Vec<SpatialCanonicalDeclarationField> {
        coedge_pcurve_basis(
            self.site().topology_coedge_identity(),
            self.birth_contract(),
            self.geometry_identity(),
        )
        .canonical_declaration_entries()
    }
}

impl VertexGeometryBindingSpec {
    pub fn canonical_declaration_fields(&self) -> Vec<SpatialCanonicalDeclarationField> {
        vertex_geometry_basis(
            self.site().topology_vertex_identity(),
            self.birth_contract(),
            self.geometry_identity(),
            self.provenance().as_str(),
            self.tolerance_regime().as_str(),
        )
        .canonical_declaration_entries()
    }
}
