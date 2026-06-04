use crate::facade::bindings::{
    evaluate_primitive_construction_birth_consequence, plan_primitive_construction_birth,
    PrimitiveConstructionBirthFamily, PrimitiveConstructionBirthScaffoldInput,
};
use worth_geom::facade::Plane;

use super::{SpatialConstructionBirthConsequence, SpatialConstructionBirthMappingKind};

#[test]
fn primitive_birth_consequence_admits_shell_mapping_truth() {
    let input = PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionBirthFamily::ShellWithHole,
        "planar_shell_with_hole_body",
        "shell-scaffold".to_string(),
        vec![plane()],
        vec![
            [2.0, 0.0, 0.0],
            [0.0, 2.0, 0.0],
            [-2.0, 0.0, 0.0],
            [0.0, -2.0, 0.0],
            [0.5, 0.0, 0.0],
            [0.0, 0.5, 0.0],
            [-0.5, 0.0, 0.0],
        ],
        7,
        7,
        2,
        0,
        1,
        1,
        1,
    );
    let plan = plan_primitive_construction_birth(input.clone()).expect("birth plan");
    let consequence = evaluate_primitive_construction_birth_consequence(&input, &plan);

    match consequence {
        SpatialConstructionBirthConsequence::Admitted(admitted) => {
            assert_eq!(
                admitted.family(),
                PrimitiveConstructionBirthFamily::ShellWithHole
            );
            assert_eq!(
                admitted.topology_birth_class(),
                "planar_shell_with_hole_body"
            );
            assert_eq!(admitted.birth_digest(), plan.birth_digest());
            assert_eq!(admitted.rows().len(), 7);
            assert_eq!(
                admitted
                    .row_for(SpatialConstructionBirthMappingKind::Loop)
                    .expect("loop row")
                    .mapped_count(),
                2
            );
            assert!(!admitted.consequence_digest().is_empty());
        }
        SpatialConstructionBirthConsequence::Rejected(_) => {
            panic!("expected admitted consequence")
        }
    }
}

#[test]
fn primitive_birth_consequence_returns_typed_rejection() {
    let input = PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionBirthFamily::WireBody,
        "planar_wire_body",
        "wire-scaffold".to_string(),
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
    );
    let mismatched = PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionBirthFamily::WireBody,
        "bad_birth_class",
        "wire-scaffold".to_string(),
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
    );
    let plan = plan_primitive_construction_birth(input).expect("birth plan");
    let consequence = evaluate_primitive_construction_birth_consequence(&mismatched, &plan);

    match consequence {
        SpatialConstructionBirthConsequence::Rejected(rejected) => {
            assert_eq!(rejected.topology_birth_class(), "bad_birth_class");
            assert!(rejected.reason().contains("topology birth class"));
            assert!(!rejected.consequence_digest().is_empty());
        }
        SpatialConstructionBirthConsequence::Admitted(_) => {
            panic!("expected rejected consequence")
        }
    }
}

fn plane() -> Plane {
    Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).expect("plane")
}
