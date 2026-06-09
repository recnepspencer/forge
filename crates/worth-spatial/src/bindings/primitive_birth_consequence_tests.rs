use worth_geom::facade::Plane;
use worth_primitives::PrimitiveConstructionFamilyKey;

use super::{
    admit_primitive_construction_birth_consequence,
    reject_mismatched_primitive_construction_birth_consequence,
    PrimitiveConstructionBirthScaffoldInput, SpatialConstructionBirthMappingKind,
};

#[test]
fn primitive_birth_consequence_admits_shell_mapping_truth() {
    let input = PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionFamilyKey::ShellWithHole,
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
    let admitted = admit_primitive_construction_birth_consequence(&input).expect("consequence");
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

#[test]
fn primitive_birth_consequence_returns_typed_rejection() {
    let input = PrimitiveConstructionBirthScaffoldInput::new(
        PrimitiveConstructionFamilyKey::WireBody,
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
        PrimitiveConstructionFamilyKey::WireBody,
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
    let rejected = reject_mismatched_primitive_construction_birth_consequence(&input, &mismatched)
        .expect("rejection");
    assert_eq!(rejected.topology_birth_class(), "bad_birth_class");
    assert!(rejected.reason().contains("topology birth class"));
    assert!(!rejected.consequence_digest().is_empty());
}

fn plane() -> Plane {
    Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).expect("plane")
}
