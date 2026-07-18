use super::{
    row_catalog::{
        FRONTIER_CANONICAL_ROW_SPECS, FRONTIER_REJECTION_ROW_SPECS,
        FRONTIER_REQUIRED_CANONICAL_ROW_NAMES, FRONTIER_REQUIRED_REJECTION_ROW_NAMES,
    },
    FrontierCloseoutStatus, FrontierRouteClass,
    MilestoneFivePointThreeFrontierCertificationAdapter,
};
use crate::harness::certification::{milestone_five_point_three_requirements, unmet_required_rows};
use std::collections::BTreeSet;

#[test]
fn frontier_certification_adapter_emits_named_matrix() {
    let matrix =
        MilestoneFivePointThreeFrontierCertificationAdapter::frontier_planning_and_parallel_admission_parity_test();

    assert_eq!(
        matrix.suite_name,
        "Frontier Planning And Parallel Admission Parity Test"
    );
    for spec in FRONTIER_CANONICAL_ROW_SPECS {
        assert!(matrix.rows.iter().any(|row| row.row_name == spec.row_name));
    }
    for spec in FRONTIER_REJECTION_ROW_SPECS {
        assert!(matrix
            .rejection_rows
            .iter()
            .any(|row| row.row_name == spec.row_name));
    }
}

#[test]
fn frontier_certification_matrix_meets_milestone_requirements() {
    let matrix =
        MilestoneFivePointThreeFrontierCertificationAdapter::frontier_planning_and_parallel_admission_parity_test();
    let requirements = milestone_five_point_three_requirements();
    let missing = unmet_required_rows(
        &matrix,
        FRONTIER_REQUIRED_CANONICAL_ROW_NAMES,
        FRONTIER_REQUIRED_REJECTION_ROW_NAMES,
    );

    assert!(missing.is_empty(), "missing frontier rows: {missing:?}");
    let spec_missing = unmet_required_rows(
        &matrix,
        requirements.required_canonical_rows,
        requirements.required_rejection_rows,
    );
    assert!(
        spec_missing.is_empty(),
        "missing spec frontier rows: {spec_missing:?}"
    );
    assert!(matrix
        .rows
        .iter()
        .all(|row| row.control_lane.has_required_outputs()));
    assert!(matrix
        .rows
        .iter()
        .all(|row| row.hostile_lane.has_required_outputs()));
    assert!(matrix
        .rows
        .iter()
        .all(|row| row.parity_lane.has_required_outputs()));
    assert!(matrix
        .rejection_rows
        .iter()
        .all(|row| row.hostile_lane.has_required_outputs()));
    assert!(matrix.rows.iter().all(|row| {
        row.control_lane
            .counter_snapshot()
            .executor_parallel_rediscovery_count()
            == 0
    }));
}

#[test]
fn frontier_certification_artifact_is_deterministic_and_zero_rediscovery() {
    let left =
        MilestoneFivePointThreeFrontierCertificationAdapter::frontier_planning_and_parallel_admission_parity_artifact();
    let right =
        MilestoneFivePointThreeFrontierCertificationAdapter::frontier_planning_and_parallel_admission_parity_artifact();

    assert_eq!(
        left.certification_bundle_digest,
        right.certification_bundle_digest
    );
    assert_eq!(left.coverage_matrix_digest, right.coverage_matrix_digest);
    assert_eq!(
        left.counter_snapshot.executor_parallel_rediscovery_count(),
        0
    );
    assert!(left.counter_snapshot.parallel_admission_route_count() > 0);
    assert!(left.counter_snapshot.serial_fallback_execution_count() > 0);
    assert!(left.counter_snapshot.mixed_basis_bundle_denial_count() > 0);
    assert!(left.counter_snapshot.bundle_parallel_route_count() > 0);
    assert!(left.counter_snapshot.bundle_serial_route_count() > 0);
    assert!(
        left.counter_snapshot
            .work_avoided_by_parallel_admission_count()
            > 0
    );
}

#[test]
fn frontier_certification_rows_assert_exact_bundle_counter_shapes() {
    let matrix =
        MilestoneFivePointThreeFrontierCertificationAdapter::frontier_planning_and_parallel_admission_parity_test();
    let parallel_bundle = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "bundle-route-posture-parity")
        .expect("parallel bundle row should exist");
    let serial_bundle = matrix
        .rows
        .iter()
        .find(|row| row.row_name == "exact-basis-bundle-parity")
        .expect("serial bundle row should exist");

    assert_eq!(
        parallel_bundle.control_lane.route_class(),
        FrontierRouteClass::ParallelAdmittedBundle
    );
    assert_eq!(
        parallel_bundle
            .control_lane
            .counter_snapshot()
            .bundle_parallel_route_count(),
        2
    );
    assert_eq!(
        parallel_bundle
            .control_lane
            .counter_snapshot()
            .bundle_serial_route_count(),
        0
    );
    assert_eq!(
        serial_bundle.control_lane.route_class(),
        FrontierRouteClass::SerialFallbackBundle
    );
    assert_eq!(
        serial_bundle
            .control_lane
            .counter_snapshot()
            .bundle_parallel_route_count(),
        0
    );
    assert_eq!(
        serial_bundle
            .control_lane
            .counter_snapshot()
            .bundle_serial_route_count(),
        2
    );
}

#[test]
fn frontier_closeout_artifact_is_complete_and_full_spec_ready() {
    let artifact =
        MilestoneFivePointThreeFrontierCertificationAdapter::frontier_planning_closeout_artifact();

    assert!(!artifact.closeout_matrix_digest.is_empty());
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(artifact.is_full_spec_ready());
    for requirement in artifact
        .must_ship
        .iter()
        .chain(artifact.must_preserve.iter())
        .chain(artifact.proof_obligations.iter())
        .chain(artifact.acceptance_evidence.iter())
    {
        assert_eq!(requirement.status, FrontierCloseoutStatus::Satisfied);
        assert!(
            !requirement.production_artifacts.is_empty(),
            "requirement {:?} must point to production artifacts",
            requirement.requirement_name
        );
        assert!(
            !requirement.notes.is_empty(),
            "requirement {:?} must explain the proof surface",
            requirement.requirement_name
        );
        assert!(
            !requirement.certification_rows.is_empty(),
            "requirement {:?} must map to a behavioral certification row",
            requirement.requirement_name
        );
    }

    let known_rows = FRONTIER_REQUIRED_CANONICAL_ROW_NAMES
        .iter()
        .chain(FRONTIER_REQUIRED_REJECTION_ROW_NAMES.iter())
        .copied()
        .collect::<BTreeSet<_>>();

    for requirement in artifact
        .must_ship
        .iter()
        .chain(artifact.must_preserve.iter())
        .chain(artifact.proof_obligations.iter())
        .chain(artifact.acceptance_evidence.iter())
    {
        for row_name in requirement.certification_rows {
            assert!(
                known_rows.contains(row_name),
                "requirement {:?} references unknown frontier row {:?}",
                requirement.requirement_name,
                row_name
            );
        }
    }

    let full_suite_requirement = artifact
        .acceptance_evidence
        .iter()
        .find(|requirement| {
            requirement.requirement_name
                == "frontier planning and parallel admission parity suite passes"
        })
        .expect("full suite acceptance requirement should exist");
    assert_eq!(
        full_suite_requirement.certification_rows.len(),
        FRONTIER_REQUIRED_CANONICAL_ROW_NAMES.len() + FRONTIER_REQUIRED_REJECTION_ROW_NAMES.len(),
        "full suite acceptance requirement must bind every required canonical and rejection row"
    );

    let denial_requirement = artifact
        .acceptance_evidence
        .iter()
        .find(|requirement| {
            requirement.requirement_name
                == "unsupported families and mixed-basis bundles fail typed and early"
        })
        .expect("typed denial acceptance requirement should exist");
    for row_name in [
        "unsupported-frontier-family",
        "unsupported-bundle-composition",
        "mixed-basis-bundle-denied",
        "forbidden-hidden-serial-fallback",
    ] {
        assert!(
            denial_requirement.certification_rows.contains(&row_name),
            "typed denial requirement must bind rejection row {row_name}"
        );
    }
}
