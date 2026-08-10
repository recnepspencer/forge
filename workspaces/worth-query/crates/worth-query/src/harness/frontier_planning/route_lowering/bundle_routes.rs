use crate::execution::{execute_preflight_bundle, execute_serial_fallback_route};
use crate::frontier_signal_adapter::SignalFrontierSurfaceEvidence;
use crate::planning::{
    admit_bounded_materialization_frontier_preflight, admit_ordered_collection_frontier_preflight,
    lower_execution_preflight_to_frontier_plan, lower_preflight_to_parallel_admission_route,
    lower_preflight_to_serial_fallback_route, FrontierDisjointnessClass,
    FrontierPredictionDriftOutcome, FrontierPreflightAdmissionError, FrontierRoutePlanningError,
    FrontierSurfaceDigest, ParallelAdmissionEvidence, SerialFallbackEvidence, SerialFallbackReason,
};

use super::super::fixtures::{sample_signal_frontier_plan, sample_signal_frontier_summary};

#[test]
fn denied_by_drift_blocks_parallel_and_serial_route_lowering() {
    let ordered =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let ordered_admitted = admit_ordered_collection_frontier_preflight(ordered.clone())
        .expect("ordered collection should admit");
    let parallel_error = lower_preflight_to_parallel_admission_route(
        &ordered_admitted,
        &ParallelAdmissionEvidence::from_surface_with_drift_for_test(
            ordered.basis().proof().digest().as_str(),
            FrontierSurfaceDigest::from_label("denied-by-drift-parallel"),
            FrontierDisjointnessClass::CollectionWindowSurface,
            FrontierPredictionDriftOutcome::DeniedByDrift,
        ),
    )
    .expect_err("denied-by-drift must block parallel route lowering");
    match parallel_error {
        FrontierRoutePlanningError::PredictionDriftDenied { .. } => {}
        other => panic!("expected prediction drift denial, got {other:?}"),
    }

    let bounded = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let bounded_admitted = admit_bounded_materialization_frontier_preflight(bounded.clone())
        .expect("bounded materialization should admit");
    let denied = lower_preflight_to_serial_fallback_route(
        &bounded_admitted,
        &SerialFallbackEvidence::from_surface(
            bounded.basis().proof().digest().as_str(),
            FrontierSurfaceDigest::from_label("denied-by-drift-serial"),
            SerialFallbackReason::PredictionDriftRequiresSerialRoute,
            FrontierPredictionDriftOutcome::DeniedByDrift,
        ),
    )
    .expect_err("denied-by-drift must block serial fallback route lowering");

    match denied {
        FrontierRoutePlanningError::PredictionDriftDenied { .. } => {}
        other => panic!("expected prediction drift denial, got {other:?}"),
    }
}

#[test]
fn serial_fallback_required_denies_parallel_but_allows_serial_fallback() {
    let ordered =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let ordered_admitted = admit_ordered_collection_frontier_preflight(ordered.clone())
        .expect("ordered collection should admit");
    let denied = lower_preflight_to_parallel_admission_route(
        &ordered_admitted,
        &ParallelAdmissionEvidence::from_surface_with_drift_for_test(
            ordered.basis().proof().digest().as_str(),
            FrontierSurfaceDigest::from_label("serial-fallback-required-parallel"),
            FrontierDisjointnessClass::CollectionWindowSurface,
            FrontierPredictionDriftOutcome::SerialFallbackRequired,
        ),
    )
    .expect_err("serial-fallback-required must deny the parallel route");
    match denied {
        FrontierRoutePlanningError::ParallelAdmissionDenied { reason, .. } => {
            assert_eq!(
                reason,
                SerialFallbackReason::PredictionDriftRequiresSerialRoute
            );
        }
        other => panic!("expected parallel admission denial, got {other:?}"),
    }

    let bounded = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let bounded_admitted = admit_bounded_materialization_frontier_preflight(bounded.clone())
        .expect("bounded materialization should admit");
    let route = lower_preflight_to_serial_fallback_route(
        &bounded_admitted,
        &SerialFallbackEvidence::from_surface(
            bounded.basis().proof().digest().as_str(),
            FrontierSurfaceDigest::from_label("serial-fallback-required-serial"),
            SerialFallbackReason::PredictionDriftRequiresSerialRoute,
            FrontierPredictionDriftOutcome::SerialFallbackRequired,
        ),
    )
    .expect("serial fallback required should still allow serial route");
    assert_eq!(
        route.report().drift_outcome(),
        &FrontierPredictionDriftOutcome::SerialFallbackRequired
    );
}

#[test]
fn bounded_materialization_lowers_into_serial_fallback_route_with_typed_executor_entrypoint() {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let admitted = admit_bounded_materialization_frontier_preflight(preflight.clone())
        .expect("bounded materialization should admit on the serial fallback frontier lane");
    let evidence = SerialFallbackEvidence::from_surface(
        preflight.basis().proof().digest().as_str(),
        FrontierSurfaceDigest::from_label("bounded-materialization-overlap"),
        SerialFallbackReason::DeterministicAdmissionDenied,
        FrontierPredictionDriftOutcome::WithinBudget,
    );

    let route = lower_preflight_to_serial_fallback_route(&admitted, &evidence)
        .expect("bounded materialization should lower into the serial fallback route");
    let typed = execute_serial_fallback_route(&route)
        .expect("serial fallback route entrypoint should execute");
    let baseline = execute_preflight_bundle(&preflight).expect("baseline execution should succeed");

    assert_eq!(typed.rows(), baseline.rows());
    assert_eq!(
        typed.report().result_digest(),
        baseline.report().result_digest()
    );
    assert_eq!(
        route.reason(),
        &SerialFallbackReason::DeterministicAdmissionDenied
    );
    assert_eq!(
        route.report().serial_fallback_reason(),
        Some(&SerialFallbackReason::DeterministicAdmissionDenied)
    );
    assert_eq!(
        route.report().route_surface_digest(),
        evidence.surface_digest()
    );
    assert_eq!(route.counters().route_serial_fallback_count(), 1);
}

#[test]
fn bounded_materialization_parallel_route_is_typed_denial() {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let error = admit_ordered_collection_frontier_preflight(preflight).expect_err(
        "bounded materialization should fail before the parallel lane is even callable",
    );

    assert_eq!(
        error,
        FrontierPreflightAdmissionError::OrderedCollectionRequired
    );
}

#[test]
fn route_posture_digest_binds_frontier_evidence() {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let admitted = admit_ordered_collection_frontier_preflight(preflight.clone())
        .expect("ordered collection should admit on the ordered frontier lane");
    let first_evidence = ParallelAdmissionEvidence::from_surface(
        preflight.basis().proof().digest().as_str(),
        FrontierSurfaceDigest::from_label("frontier-a"),
        FrontierDisjointnessClass::CollectionWindowSurface,
    );
    let second_evidence = ParallelAdmissionEvidence::from_surface(
        preflight.basis().proof().digest().as_str(),
        FrontierSurfaceDigest::from_label("frontier-b"),
        FrontierDisjointnessClass::CollectionWindowSurface,
    );

    let first = lower_preflight_to_parallel_admission_route(&admitted, &first_evidence)
        .expect("first frontier surface should admit");
    let second = lower_preflight_to_parallel_admission_route(&admitted, &second_evidence)
        .expect("second frontier surface should admit");

    assert_ne!(
        first.posture_digest(),
        second.posture_digest(),
        "route posture must bind explicit frontier evidence instead of family-only heuristics"
    );
}

#[test]
fn signal_frontier_plan_adapter_produces_stable_surface_digest() {
    let plan = sample_signal_frontier_plan();

    let first = SignalFrontierSurfaceEvidence::from_frontier_plan(&plan);
    let second = SignalFrontierSurfaceEvidence::from_frontier_plan(&plan);

    assert_eq!(first.surface_digest(), second.surface_digest());
    assert_eq!(first.predicted_breadth(), second.predicted_breadth());
    assert_eq!(first.realized_breadth(), None);
}

#[test]
fn signal_frontier_surface_evidence_lowers_into_route_evidence() {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let admitted = admit_ordered_collection_frontier_preflight(preflight.clone())
        .expect("ordered collection should admit on the ordered frontier lane");
    let signal_surface = SignalFrontierSurfaceEvidence::from_frontier_execution_summary(
        &sample_signal_frontier_summary(),
    );
    let evidence = signal_surface.to_parallel_admission_evidence(
        preflight.basis().proof().digest().as_str(),
        FrontierDisjointnessClass::CollectionWindowSurface,
    );

    let route = lower_preflight_to_parallel_admission_route(&admitted, &evidence)
        .expect("signal-backed frontier evidence should admit the ordered collection route");
    let frontier_plan = lower_execution_preflight_to_frontier_plan(&preflight)
        .expect("preflight should lower into a frontier plan");

    assert_eq!(
        route.posture_digest().as_str(),
        evidence
            .route_posture_digest_for_test(&frontier_plan)
            .as_str(),
        "route posture should bind the lowered signal frontier evidence"
    );
    assert!(signal_surface.realized_breadth().is_some());
}
