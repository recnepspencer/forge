use worth_spatial::facade::planar_contracts::{
    admit_planar_contract_family, planar_admission_matrix, PlanarAdmissionClass,
    PlanarAdmissionFamily, PlanarAdmissionReason, PlanarPremetabossInputFamily,
    PlanarRuntimeConcern,
};

#[test]
fn planar_admission_matrix_covers_every_family_and_runtime_concern() {
    let matrix = planar_admission_matrix();
    let families = PlanarAdmissionFamily::all();
    let concerns = PlanarRuntimeConcern::all();

    assert_eq!(matrix.rows().len(), families.len() * concerns.len());
    assert!(!matrix.matrix_digest().is_empty());

    for family in families {
        for concern in concerns {
            let row = matrix
                .row(family, concern)
                .expect("every M6 planar family must classify every runtime concern");
            assert_eq!(row.family(), family);
            assert_eq!(row.concern(), concern);
            assert_eq!(row.reason().as_str(), row.rationale());
            assert!(!row.rationale().is_empty());
            assert!(!row.row_digest().is_empty());
            assert!(row.query_posture().configured_domain_handle_required());
            assert!(row
                .query_posture()
                .declaration_family_capability_matrix_required());
            if row.class().admits_runtime() {
                assert!(row.query_posture().canonical_declaration_required());
            }
        }
    }
}

#[test]
fn planar_admission_matrix_classifies_exact_ambiguous_unbounded_dirty_and_policy_required_classes()
{
    let matrix = planar_admission_matrix();
    let classes = matrix
        .rows()
        .iter()
        .map(|row| row.class())
        .collect::<std::collections::BTreeSet<_>>();

    for required_class in [
        PlanarAdmissionClass::Admitted,
        PlanarAdmissionClass::Denied,
        PlanarAdmissionClass::Unsupported,
        PlanarAdmissionClass::PolicyRequired,
        PlanarAdmissionClass::PredicateUncertainReserved,
    ] {
        assert!(
            classes.contains(&required_class),
            "missing required phase-one class {:?}",
            required_class
        );
    }

    assert_eq!(
        matrix
            .row(
                PlanarAdmissionFamily::ExactPlanarPredicateAuthority,
                PlanarRuntimeConcern::PredicateClassification,
            )
            .expect("predicate row")
            .class(),
        PlanarAdmissionClass::Admitted
    );
    assert_eq!(
        matrix
            .row(
                PlanarAdmissionFamily::ExactPlanarPredicateAuthority,
                PlanarRuntimeConcern::PredicateClassification,
            )
            .expect("predicate row")
            .reason(),
        PlanarAdmissionReason::ExactPlanarContractAdmitted
    );
    assert_eq!(
        matrix
            .row(
                PlanarAdmissionFamily::CoplanarOverlapContract,
                PlanarRuntimeConcern::CoplanarOverlapExtraction,
            )
            .expect("coplanar overlap row")
            .class(),
        PlanarAdmissionClass::PolicyRequired
    );
    assert_eq!(
        matrix
            .row(
                PlanarAdmissionFamily::CoplanarOverlapContract,
                PlanarRuntimeConcern::CoplanarOverlapExtraction,
            )
            .expect("coplanar overlap row")
            .reason(),
        PlanarAdmissionReason::CoplanarOverlapRequiresPolicy
    );
    assert_eq!(
        matrix
            .row(
                PlanarAdmissionFamily::DirtyPlanarInput,
                PlanarRuntimeConcern::DiagnosticsLocalization,
            )
            .expect("dirty diagnostics row")
            .class(),
        PlanarAdmissionClass::Denied
    );
    assert_eq!(
        matrix
            .row(
                PlanarAdmissionFamily::UnboundedPlanarDomain,
                PlanarRuntimeConcern::BooleanReadinessBundle,
            )
            .expect("unbounded boolean row")
            .class(),
        PlanarAdmissionClass::Unsupported
    );
    assert_eq!(
        matrix
            .row(
                PlanarAdmissionFamily::PlanarLocalFrameCertificate,
                PlanarRuntimeConcern::PredicateClassification,
            )
            .expect("ambiguous predicate row")
            .class(),
        PlanarAdmissionClass::PredicateUncertainReserved
    );
}

#[test]
fn planar_admission_receipt_exists_only_for_admitted_rows() {
    let admitted = admit_planar_contract_family(
        PlanarAdmissionFamily::PlanarLocalFrameCertificate,
        PlanarRuntimeConcern::LocalFrameDerivation,
    )
    .expect("local frame should be admitted");
    let admitted_row = planar_admission_matrix()
        .row(
            PlanarAdmissionFamily::PlanarLocalFrameCertificate,
            PlanarRuntimeConcern::LocalFrameDerivation,
        )
        .expect("local frame row")
        .clone();
    assert_eq!(
        admitted.family(),
        PlanarAdmissionFamily::PlanarLocalFrameCertificate
    );
    assert_eq!(admitted.class(), PlanarAdmissionClass::Admitted);
    assert_eq!(admitted.row_digest(), admitted_row.row_digest());
    assert!(!admitted.matrix_digest().is_empty());

    let denied = admit_planar_contract_family(
        PlanarAdmissionFamily::DirtyPlanarInput,
        PlanarRuntimeConcern::DiagnosticsLocalization,
    );
    assert!(denied.is_none());

    let policy_required = admit_planar_contract_family(
        PlanarAdmissionFamily::CoplanarOverlapContract,
        PlanarRuntimeConcern::CoplanarOverlapExtraction,
    );
    assert!(policy_required.is_none());
}

#[test]
fn mb_m6_admission_rows_cover_premetaboss_input_families() {
    let matrix = planar_admission_matrix();
    let rows = matrix.premetaboss_rows();

    assert_eq!(rows.len(), PlanarPremetabossInputFamily::all().len());

    for input_family in PlanarPremetabossInputFamily::all() {
        let row = rows
            .iter()
            .find(|row| row.input_family() == input_family)
            .unwrap_or_else(|| panic!("missing {}", input_family.as_str()));
        assert!(!row.row_digest().is_empty());
        assert!(!row.reason().as_str().is_empty());
        assert_eq!(
            row.movement_rotation_posture_class(),
            PlanarAdmissionClass::Admitted,
            "{} must stack movement/rotation posture into admission pressure",
            input_family.as_str()
        );
    }

    for (input_family, expected_class, expected_reason) in [
        (
            PlanarPremetabossInputFamily::CoplanarOverlapContractStorm,
            PlanarAdmissionClass::PolicyRequired,
            PlanarAdmissionReason::CoplanarOverlapRequiresPolicy,
        ),
        (
            PlanarPremetabossInputFamily::HighValencePlanarSingularityContract,
            PlanarAdmissionClass::Admitted,
            PlanarAdmissionReason::ExactPlanarContractAdmitted,
        ),
        (
            PlanarPremetabossInputFamily::ThinFeatureScaleSeparationContract,
            PlanarAdmissionClass::Admitted,
            PlanarAdmissionReason::ExactPlanarContractAdmitted,
        ),
        (
            PlanarPremetabossInputFamily::RetainedPlanarHistoryCancellationChain,
            PlanarAdmissionClass::Admitted,
            PlanarAdmissionReason::DownstreamContractLaneAdmitted,
        ),
        (
            PlanarPremetabossInputFamily::DirtyPlanarInputCleanFailLocalization,
            PlanarAdmissionClass::Denied,
            PlanarAdmissionReason::DirtyOrUnboundedInputDenied,
        ),
        (
            PlanarPremetabossInputFamily::UnboundedHalfSpacePlanarPosture,
            PlanarAdmissionClass::Denied,
            PlanarAdmissionReason::DirtyOrUnboundedInputDenied,
        ),
        (
            PlanarPremetabossInputFamily::ProjectionConsumedPlanarFactParity,
            PlanarAdmissionClass::Admitted,
            PlanarAdmissionReason::DownstreamContractLaneAdmitted,
        ),
        (
            PlanarPremetabossInputFamily::BooleanReadinessFinalBoss,
            PlanarAdmissionClass::Admitted,
            PlanarAdmissionReason::DownstreamContractLaneAdmitted,
        ),
    ] {
        let row = rows
            .iter()
            .find(|row| row.input_family() == input_family)
            .unwrap_or_else(|| panic!("missing {}", input_family.as_str()));
        assert_eq!(row.class(), expected_class, "{}", input_family.as_str());
        assert_eq!(row.reason(), expected_reason, "{}", input_family.as_str());
    }
}
