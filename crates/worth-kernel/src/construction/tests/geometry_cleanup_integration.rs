use crate::construction::result::prepare_primitive_construction_result;
use crate::construction::PrimitiveConstructionIntent;
use crate::construction::specs::{
    OrthotopeSpec, RegularPrismSpec, RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec,
    WireBodySpec,
};
use topology::facade::TopologyConstructionQueryFactKind;
use worth_geom::facade::{build_direct_realization_report, tetrahedron, Plane};
use worth_primitives::{
    canonical_simplex_vertices,
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    plan_primitive_construction_birth, PrimitiveConstructionBirthFamily,
    PrimitiveConstructionBirthScaffoldInput,
};

#[test]
fn geometry_digest_mutation_and_replay_parity_hostility_suite() {
    let shifted_plane =
        Plane::from_point_normal([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]).expect("shifted plane");
    let base_vertices = canonical_simplex_vertices(1.0, 0.0).local_vertices().to_vec();
    let shifted_vertices = vec![
        base_vertices[0],
        base_vertices[1],
        [base_vertices[2][0] - 0.25, base_vertices[2][1], base_vertices[2][2]],
        base_vertices[3],
    ];
    let parity_planes = tetrahedron([0.0, 0.0, 0.0], 1.0).expect("tetrahedron planes");
    let shifted_parity_planes = {
        let mut planes = parity_planes.clone();
        planes[0] = shifted_plane;
        planes
    };
    let base_bundle = geometry_bundle(&parity_planes, &base_vertices);
    let shifted_plane_bundle = geometry_bundle(&shifted_parity_planes, &base_vertices);
    let shifted_vertex_bundle = geometry_bundle(&parity_planes, &shifted_vertices);
    let realization_report =
        build_direct_realization_report("simplex_solid", &base_vertices, &parity_planes);
    let birth_contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::SimplexSolid,
    );
    let plan = plan_primitive_construction_birth(
        PrimitiveConstructionBirthScaffoldInput::new_with_realization(
            PrimitiveConstructionBirthFamily::SimplexSolid,
            birth_contract,
            "closed_simplex_body",
            "hostility-simplex".to_string(),
            parity_planes,
            realization_report.clone(),
            base_vertices.clone(),
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

    assert_eq!(
        realization_report.geometry_digest(),
        base_bundle.realization_geometry_digest().as_str()
    );
    assert_eq!(
        plan.realization_geometry_digest(),
        base_bundle.realization_geometry_digest().as_str()
    );
    assert_ne!(
        base_bundle.scaffold_geometry_digest().as_str(),
        shifted_plane_bundle.scaffold_geometry_digest().as_str()
    );
    assert_ne!(
        base_bundle.scaffold_geometry_digest().as_str(),
        shifted_vertex_bundle.scaffold_geometry_digest().as_str()
    );
}

#[test]
fn canonical_witness_and_contract_hostility_suite_survives_full_cross_crate_flow() {
    let cases = vec![
        (
            PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec::new(1.0)),
            PrimitiveWitnessDescriptor::SimplexSolid,
        ),
        (
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 2.0, 3.0],
            }),
            PrimitiveWitnessDescriptor::Orthotope,
        ),
        (
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 6,
                radius: 1.0,
                height: 2.0,
            }),
            PrimitiveWitnessDescriptor::RegularPrism { side_count: 6 },
        ),
        (
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 5,
                radius: 1.0,
                height: 2.0,
            }),
            PrimitiveWitnessDescriptor::RegularPyramid { side_count: 5 },
        ),
        (
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 }),
            PrimitiveWitnessDescriptor::WireBody { edge_count: 8 },
        ),
        (
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 6,
                hole_loop_edge_counts: vec![3, 4],
            }),
            PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 6,
                hole_loop_edge_counts: vec![3, 4],
            },
        ),
    ];

    for (intent, descriptor) in cases {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(&descriptor);
        let result = prepare_primitive_construction_result(intent.clone()).expect("result");
        let envelope = result.topology_query_handoff().topology_query_envelope();

        assert!(!result.realization_report().geometry_digest().is_empty());
        assert_eq!(
            envelope
                .row_for(TopologyConstructionQueryFactKind::VertexBirth)
                .expect("vertex row")
                .fact_count(),
            contract.topology_contract().vertex_count()
        );
        assert_eq!(
            envelope
                .row_for(TopologyConstructionQueryFactKind::EdgeBirth)
                .expect("edge row")
                .fact_count(),
            contract.topology_contract().edge_count()
        );
        assert_eq!(
            envelope
                .row_for(TopologyConstructionQueryFactKind::LoopMembership)
                .expect("loop row")
                .fact_count(),
            contract.topology_contract().loop_count()
        );
        assert_eq!(
            envelope
                .row_for(TopologyConstructionQueryFactKind::WireMembership)
                .expect("wire row")
                .fact_count(),
            contract.topology_contract().wire_count()
        );
        assert_eq!(
            envelope
                .row_for(TopologyConstructionQueryFactKind::FaceMembership)
                .expect("face row")
                .fact_count(),
            contract.topology_contract().face_count()
        );
        assert_eq!(
            envelope
                .row_for(TopologyConstructionQueryFactKind::ShellMembership)
                .expect("shell row")
                .fact_count(),
            contract.topology_contract().shell_count()
        );
        assert_eq!(
            envelope
                .row_for(TopologyConstructionQueryFactKind::BodyMembership)
                .expect("body row")
                .fact_count(),
            contract.topology_contract().body_count()
        );
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
    PrimitiveSupportPlaneIdentity::new(
        a.to_string(),
        b.to_string(),
        c.to_string(),
        d.to_string(),
    )
}
