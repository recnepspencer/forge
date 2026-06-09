use worth_geom::facade::{build_direct_realization_report, tetrahedron, Plane};
use worth_primitives::{
    canonical_simplex_vertices, PrimitiveGeometryIdentityBundle, PrimitiveSupportPlaneIdentity,
    PrimitiveVertexIdentity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveGeometryDigestSensitivityReport {
    baseline_case_verified: bool,
    shifted_support_plane_case_verified: bool,
    shifted_embedded_vertex_case_verified: bool,
}

impl PrimitiveGeometryDigestSensitivityReport {
    pub fn covers_expected_mutation_cases(&self) -> bool {
        self.baseline_case_verified
            && self.shifted_support_plane_case_verified
            && self.shifted_embedded_vertex_case_verified
    }
}

pub fn prepare_primitive_geometry_digest_sensitivity_report(
) -> PrimitiveGeometryDigestSensitivityReport {
    let base_planes = tetrahedron([0.0, 0.0, 0.0], 1.0).expect("tetrahedron planes");
    let shifted_plane =
        Plane::from_point_normal([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]).expect("shifted plane");
    let base_vertices = canonical_simplex_vertices(1.0, 0.0)
        .local_vertices()
        .to_vec();
    let shifted_vertices = vec![
        base_vertices[0],
        base_vertices[1],
        [
            base_vertices[2][0] - 0.25,
            base_vertices[2][1],
            base_vertices[2][2],
        ],
        base_vertices[3],
    ];
    let shifted_planes = {
        let mut planes = base_planes.clone();
        planes[0] = shifted_plane;
        planes
    };
    let baseline_case_verified = geometry_digests_match(&base_planes, &base_vertices);
    let shifted_support_plane_case_verified =
        geometry_digests_match(&shifted_planes, &base_vertices);
    let shifted_embedded_vertex_case_verified =
        geometry_digests_match(&base_planes, &shifted_vertices);
    PrimitiveGeometryDigestSensitivityReport {
        baseline_case_verified,
        shifted_support_plane_case_verified,
        shifted_embedded_vertex_case_verified,
    }
}

fn geometry_digests_match(support_planes: &[Plane], vertex_positions: &[[f64; 3]]) -> bool {
    let bundle = geometry_bundle(support_planes, vertex_positions);
    let realization_report =
        build_direct_realization_report("simplex_solid", vertex_positions, support_planes);
    let realization_geometry_digest = realization_report.geometry_digest().to_string();
    realization_report.geometry_digest() == bundle.realization_geometry_digest().as_str()
        && realization_geometry_digest == bundle.realization_geometry_digest().as_str()
}

fn geometry_bundle(
    support_planes: &[Plane],
    vertex_positions: &[[f64; 3]],
) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
        support_planes.iter().map(plane_identity).collect(),
        vertex_positions
            .iter()
            .copied()
            .map(PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

fn plane_identity(plane: &Plane) -> PrimitiveSupportPlaneIdentity {
    let (a, b, c, d) = plane.exact_coefficients();
    PrimitiveSupportPlaneIdentity::new(a.to_string(), b.to_string(), c.to_string(), d.to_string())
}
