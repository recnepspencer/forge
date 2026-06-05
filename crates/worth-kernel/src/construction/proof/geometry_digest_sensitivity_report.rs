use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use worth_geom::facade::{build_direct_realization_report, tetrahedron, Plane};
use worth_primitives::{
    canonical_simplex_vertices, PrimitiveConstructionFamilyContractRegistry,
    PrimitiveGeometryIdentityBundle, PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity,
    PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::birth::{
    plan_primitive_construction_birth, PrimitiveConstructionBirthFamily,
    PrimitiveConstructionBirthScaffoldInput,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveGeometryDigestMutationCase {
    Baseline,
    ShiftedSupportPlane,
    ShiftedEmbeddedVertex,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveGeometryDigestSensitivityRow {
    case: PrimitiveGeometryDigestMutationCase,
    scaffold_geometry_digest: String,
    realization_geometry_digest: String,
    spatial_geometry_digest: String,
}

impl PrimitiveGeometryDigestSensitivityRow {
    pub fn case(&self) -> PrimitiveGeometryDigestMutationCase {
        self.case
    }

    pub fn scaffold_geometry_digest(&self) -> &str {
        &self.scaffold_geometry_digest
    }

    pub fn realization_geometry_digest(&self) -> &str {
        &self.realization_geometry_digest
    }

    pub fn spatial_geometry_digest(&self) -> &str {
        &self.spatial_geometry_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveGeometryDigestSensitivityReport {
    rows: Vec<PrimitiveGeometryDigestSensitivityRow>,
    report_digest: String,
}

impl PrimitiveGeometryDigestSensitivityReport {
    pub fn rows(&self) -> &[PrimitiveGeometryDigestSensitivityRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
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
    let rows = vec![
        row(
            PrimitiveGeometryDigestMutationCase::Baseline,
            &base_planes,
            &base_vertices,
        ),
        row(
            PrimitiveGeometryDigestMutationCase::ShiftedSupportPlane,
            &shifted_planes,
            &base_vertices,
        ),
        row(
            PrimitiveGeometryDigestMutationCase::ShiftedEmbeddedVertex,
            &base_planes,
            &shifted_vertices,
        ),
    ];
    assert_eq!(
        rows[0].realization_geometry_digest(),
        rows[0].spatial_geometry_digest(),
        "baseline realization and spatial geometry digests drifted"
    );
    assert_eq!(
        rows[1].realization_geometry_digest(),
        rows[1].spatial_geometry_digest(),
        "shifted-plane realization and spatial geometry digests drifted"
    );
    assert_eq!(
        rows[2].realization_geometry_digest(),
        rows[2].spatial_geometry_digest(),
        "shifted-vertex realization and spatial geometry digests drifted"
    );
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &rows
            .iter()
            .flat_map(|row| {
                [
                    format!("{:?}", row.case()),
                    row.scaffold_geometry_digest().to_string(),
                    row.realization_geometry_digest().to_string(),
                    row.spatial_geometry_digest().to_string(),
                ]
            })
            .collect::<Vec<_>>(),
    );
    PrimitiveGeometryDigestSensitivityReport {
        rows,
        report_digest,
    }
}

fn row(
    case: PrimitiveGeometryDigestMutationCase,
    support_planes: &[Plane],
    vertex_positions: &[[f64; 3]],
) -> PrimitiveGeometryDigestSensitivityRow {
    let bundle = geometry_bundle(support_planes, vertex_positions);
    let realization_report =
        build_direct_realization_report("simplex_solid", vertex_positions, support_planes);
    let birth_contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::SimplexSolid,
    );
    let plan = plan_primitive_construction_birth(
        PrimitiveConstructionBirthScaffoldInput::new_with_realization(
            PrimitiveConstructionBirthFamily::SimplexSolid,
            birth_contract,
            "closed_simplex_body",
            bundle.scaffold_geometry_digest().as_str().to_string(),
            support_planes.to_vec(),
            realization_report.clone(),
            vertex_positions.to_vec(),
            4,
            6,
            4,
            0,
            4,
            1,
            1,
        ),
    )
    .expect("birth plan");
    PrimitiveGeometryDigestSensitivityRow {
        case,
        scaffold_geometry_digest: bundle.scaffold_geometry_digest().as_str().to_string(),
        realization_geometry_digest: realization_report.geometry_digest().to_string(),
        spatial_geometry_digest: plan.realization_geometry_digest().to_string(),
    }
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
