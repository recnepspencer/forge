use worth_spatial::facade::planar_contracts::{planar_admission_matrix, PlanarAdmissionClass};

#[test]
fn movement_rotation_stack_is_present_in_every_premetaboss_family() {
    for row in planar_admission_matrix().premetaboss_rows() {
        assert_eq!(
            row.movement_rotation_posture_class(),
            PlanarAdmissionClass::Admitted,
            "{} must carry explicit movement/rotation posture",
            row.input_family().as_str()
        );
    }
}
