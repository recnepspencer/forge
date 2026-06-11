use worth_primitives::PrimitiveConstructionFamilyKey;

use super::PrimitiveConstructionBirthScaffoldInput;
use crate::bindings::primitive_birth_assessment::assess_primitive_construction_birth;
use crate::bindings::primitive_birth_runtime::PrimitiveConstructionBirthRealizationFacts;
use worth_geom::facade::{
    block, prism, pyramid, realize_pyramid_support, tetrahedron, Plane,
    PrimitiveNormalizationDisposition, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
    PrimitiveSupportNormalClass,
};
use worth_primitives::{
    canonical_simplex_vertices, PrimitiveConstructionFamilyContractRegistry,
    PrimitiveWitnessDescriptor,
};

#[test]
fn primitive_birth_admits_closed_and_planar_phase_three_families() {
    let simplex =
        assess_primitive_construction_birth(PrimitiveConstructionBirthScaffoldInput::new(
            PrimitiveConstructionFamilyKey::SimplexSolid,
            "closed_simplex_body",
            "simplex".to_string(),
            tetrahedron([0.0, 0.0, 0.0], 1.0).expect("planes"),
            canonical_simplex_vertices(1.0, 0.0)
                .local_vertices()
                .to_vec(),
            4,
            6,
            4,
            0,
            4,
            1,
            1,
        ))
        .expect("simplex birth");
    let orthotope =
        assess_primitive_construction_birth(PrimitiveConstructionBirthScaffoldInput::new(
            PrimitiveConstructionFamilyKey::Orthotope,
            "closed_orthotope_body",
            "orthotope".to_string(),
            block([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]).expect("planes"),
            vec![
                [-1.0, -1.0, -1.0],
                [-1.0, -1.0, 1.0],
                [-1.0, 1.0, -1.0],
                [-1.0, 1.0, 1.0],
                [1.0, -1.0, -1.0],
                [1.0, -1.0, 1.0],
                [1.0, 1.0, -1.0],
                [1.0, 1.0, 1.0],
            ],
            8,
            12,
            6,
            0,
            6,
            1,
            1,
        ))
        .expect("orthotope birth");
    let prism = assess_primitive_construction_birth(PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionFamilyKey::RegularPrism,
        "closed_regular_prism_body",
        "prism".to_string(),
        prism([0.0, 0.0, 0.0], 6, 1.0, 2.0).expect("planes"),
        (0..6)
            .flat_map(|index| {
                let angle = std::f64::consts::TAU * index as f64 / 6.0;
                [
                    [angle.cos(), angle.sin(), -1.0],
                    [angle.cos(), angle.sin(), 1.0],
                ]
            })
            .collect(),
        12,
        18,
        8,
        0,
        8,
        1,
        1,
    ))
    .expect("prism birth");
    let pyramid =
        assess_primitive_construction_birth(PrimitiveConstructionBirthScaffoldInput::new(
            PrimitiveConstructionFamilyKey::RegularPyramid,
            "closed_regular_pyramid_body",
            "pyramid".to_string(),
            pyramid([0.0, 0.0, 0.0], 5, 1.0, 2.0).expect("planes"),
            {
                let mut vertices = (0..5)
                    .map(|index| {
                        let angle = std::f64::consts::TAU * index as f64 / 5.0;
                        [angle.cos(), angle.sin(), 0.0]
                    })
                    .collect::<Vec<_>>();
                vertices.push([0.0, 0.0, 2.0]);
                vertices
            },
            6,
            10,
            6,
            0,
            6,
            1,
            1,
        ))
        .expect("pyramid birth");
    let wire = assess_primitive_construction_birth(PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionFamilyKey::WireBody,
        "planar_wire_body",
        "wire".to_string(),
        vec![plane()],
        vec![
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
        ],
        4,
        4,
        1,
        1,
        0,
        0,
        1,
    ))
    .expect("wire birth");
    let shell = assess_primitive_construction_birth(PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionFamilyKey::ShellWithHole,
        "planar_shell_with_hole_body",
        "shell".to_string(),
        vec![plane()],
        vec![
            [3.0, 0.0, 0.0],
            [0.0, 3.0, 0.0],
            [-3.0, 0.0, 0.0],
            [0.0, -3.0, 0.0],
            [0.4, 0.0, 0.0],
            [0.0, 0.4, 0.0],
            [-0.4, 0.0, 0.0],
        ],
        7,
        7,
        2,
        0,
        1,
        1,
        1,
    ))
    .expect("shell birth");

    assert_eq!(simplex.supported_edge_count(), 6);
    assert_eq!(orthotope.supported_loop_count(), 6);
    assert_eq!(prism.supported_face_count(), 8);
    assert_eq!(pyramid.supported_edge_count(), 10);
    assert_eq!(wire.supported_wire_count(), 1);
    assert_eq!(shell.supported_loop_count(), 2);
    assert_eq!(
        prism.realization_strategy(),
        PrimitiveRealizationStrategy::DirectWorld
    );
    assert_eq!(
        pyramid.realization_strategy(),
        PrimitiveRealizationStrategy::DirectWorld
    );
    assert_eq!(
        shell.stability_class(),
        PrimitiveStabilityClass::StableDirect
    );
}

#[test]
fn primitive_birth_rejects_wrong_wire_counts() {
    let error = assess_primitive_construction_birth(PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionFamilyKey::WireBody,
        "planar_wire_body",
        "wire".to_string(),
        vec![plane()],
        vec![[0.0, 0.0, 0.0]; 4],
        4,
        5,
        1,
        1,
        0,
        0,
        1,
    ))
    .expect_err("wire mismatch should reject");

    assert!(error
        .to_string()
        .contains("admitted primitive family contract"));
}

#[test]
fn primitive_birth_preserves_escalated_realization_provenance() {
    let realization =
        realize_pyramid_support([0.0, 0.0, 0.0], 3, 1.0e-200, 1.0e-200).expect("realization");
    let input = PrimitiveConstructionBirthScaffoldInput::new_with_realization_facts(
        PrimitiveConstructionFamilyKey::RegularPyramid,
        PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::RegularPyramid { side_count: 3 },
        ),
        "closed_regular_pyramid_body",
        "tiny-pyramid".to_string(),
        realization.planes().to_vec(),
        PrimitiveConstructionBirthRealizationFacts::from_realization_report(
            realization.report().clone(),
        ),
        vec![
            [1.0e-200, 0.0, 0.0],
            [-5.0e-201, 8.660254037844386e-201, 0.0],
            [-5.0e-201, -8.660254037844386e-201, 0.0],
            [0.0, 0.0, 1.0e-200],
        ],
        4,
        6,
        4,
        0,
        4,
        1,
        1,
    );
    let plan = assess_primitive_construction_birth(input).expect("birth assessment");

    assert_eq!(
        plan.realization_strategy(),
        PrimitiveRealizationStrategy::ExactSupport
    );
    assert_eq!(
        plan.stability_class(),
        PrimitiveStabilityClass::StableAfterEscalation
    );
    assert_eq!(
        plan.support_normal_class(),
        PrimitiveSupportNormalClass::Degenerate
    );
    assert_eq!(
        plan.normalization_disposition(),
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    );
    assert!(!plan.realization_fact_digest().is_empty());
}

fn plane() -> Plane {
    Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).expect("plane")
}
