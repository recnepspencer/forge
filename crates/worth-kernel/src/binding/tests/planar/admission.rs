use worth_spatial::facade::planar_contracts::{
    admit_planar_contract_family, planar_admission_matrix, PlanarAdmissionClass,
    PlanarAdmissionFamily, PlanarPremetabossInputFamily, PlanarRuntimeConcern,
};

#[test]
fn kernel_consumes_spatial_planar_admission_without_local_upgrade() {
    let matrix = planar_admission_matrix();
    let dirty_row = matrix
        .row(
            PlanarAdmissionFamily::DirtyPlanarInput,
            PlanarRuntimeConcern::DiagnosticsLocalization,
        )
        .expect("dirty planar diagnostics row");

    assert_eq!(dirty_row.class(), PlanarAdmissionClass::Denied);
    assert!(
        admit_planar_contract_family(
            PlanarAdmissionFamily::DirtyPlanarInput,
            PlanarRuntimeConcern::DiagnosticsLocalization,
        )
        .is_none(),
        "kernel must not upgrade a visible denied planar row into admission"
    );
}

#[test]
fn kernel_cannot_upgrade_any_non_admitted_planar_posture_class() {
    let matrix = planar_admission_matrix();

    for (family, concern, expected_class) in [
        (
            PlanarAdmissionFamily::DirtyPlanarInput,
            PlanarRuntimeConcern::DiagnosticsLocalization,
            PlanarAdmissionClass::Denied,
        ),
        (
            PlanarAdmissionFamily::UnboundedPlanarDomain,
            PlanarRuntimeConcern::BooleanReadinessBundle,
            PlanarAdmissionClass::Unsupported,
        ),
        (
            PlanarAdmissionFamily::CoplanarOverlapContract,
            PlanarRuntimeConcern::CoplanarOverlapExtraction,
            PlanarAdmissionClass::PolicyRequired,
        ),
        (
            PlanarAdmissionFamily::PlanarLocalFrameCertificate,
            PlanarRuntimeConcern::PredicateClassification,
            PlanarAdmissionClass::PredicateUncertainReserved,
        ),
    ] {
        let visible_row = matrix
            .row(family, concern)
            .expect("non-admitted planar posture must remain visible");

        assert_eq!(visible_row.class(), expected_class);
        assert!(
            admit_planar_contract_family(family, concern).is_none(),
            "{:?}/{:?} must fail closed before kernel can summarize it",
            family,
            concern
        );
    }
}

#[test]
fn kernel_receives_spatial_admission_receipt_for_admitted_planar_rows() {
    let receipt = admit_planar_contract_family(
        PlanarAdmissionFamily::ExactPlanarPredicateAuthority,
        PlanarRuntimeConcern::PredicateClassification,
    )
    .expect("predicate authority should be admitted by spatial support posture");

    assert_eq!(
        receipt.family(),
        PlanarAdmissionFamily::ExactPlanarPredicateAuthority
    );
    assert_eq!(
        receipt.concern(),
        PlanarRuntimeConcern::PredicateClassification
    );
    assert_eq!(receipt.class(), PlanarAdmissionClass::Admitted);
    assert!(!receipt.row_digest().is_empty());
    assert!(!receipt.matrix_digest().is_empty());
}

#[test]
fn kernel_sees_movement_rotation_posture_on_every_premetaboss_admission_row() {
    let matrix = planar_admission_matrix();

    for input_family in PlanarPremetabossInputFamily::all() {
        let premetaboss_row = matrix
            .premetaboss_rows()
            .iter()
            .find(|row| row.input_family() == input_family)
            .unwrap_or_else(|| panic!("missing {}", input_family.as_str()));

        assert_eq!(
            premetaboss_row.movement_rotation_posture_class(),
            PlanarAdmissionClass::Admitted,
            "{} must carry admitted movement/rotation posture into kernel-facing proof",
            input_family.as_str()
        );
    }
}
