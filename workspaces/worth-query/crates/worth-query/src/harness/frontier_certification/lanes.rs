use crate::execution::{
    execute_parallel_admission_route, execute_preflight_bundle, execute_serial_fallback_route,
};
use crate::frontier_planning::FrontierSurfaceDigest;
use crate::harness::fixtures::execution_preflights::{
    ordered_collection_preflight, ordered_collection_without_traversal_preflight,
};
use crate::planning::{
    admit_bounded_materialization_frontier_preflight, admit_ordered_collection_frontier_preflight,
    lower_execution_preflight_to_frontier_plan,
    lower_preflight_bundle_to_parallel_admission_routes,
    lower_preflight_bundle_to_serial_fallback_routes, lower_preflight_to_parallel_admission_route,
    lower_preflight_to_serial_fallback_route, FrontierDisjointnessClass, FrontierParityBundle,
    FrontierPredictionDriftOutcome, ParallelAdmissionBundleEvidence, ParallelAdmissionEvidence,
    SerialFallbackBundleEvidence, SerialFallbackEvidence, SerialFallbackReason,
};

use super::FrontierCertificationLane;

pub(super) fn serial_control_lane() -> FrontierCertificationLane {
    let preflight = ordered_collection_without_traversal_preflight();
    let frontier_plan =
        lower_execution_preflight_to_frontier_plan(&preflight).expect("serial control plan");
    let execution = execute_preflight_bundle(&preflight).expect("serial control execution");

    FrontierCertificationLane {
        parity_bundle: FrontierParityBundle::from_serial_control(
            &frontier_plan,
            &preflight,
            &execution,
        ),
    }
}

pub(super) fn parallel_admitted_lane() -> FrontierCertificationLane {
    let preflight = ordered_collection_without_traversal_preflight();
    let admitted =
        admit_ordered_collection_frontier_preflight(preflight.clone()).expect("ordered admitted");
    let evidence = ParallelAdmissionEvidence::from_surface(
        preflight.basis().proof().digest().as_str(),
        FrontierSurfaceDigest::from_label("frontier-certification-parallel"),
        FrontierDisjointnessClass::CollectionWindowSurface,
    );
    let route =
        lower_preflight_to_parallel_admission_route(&admitted, &evidence).expect("parallel route");
    let execution =
        execute_parallel_admission_route(&route).expect("parallel execution should succeed");
    FrontierCertificationLane {
        parity_bundle: FrontierParityBundle::from_parallel_admission(&route, &execution),
    }
}

pub(super) fn serial_fallback_lane() -> FrontierCertificationLane {
    let preflight = ordered_collection_preflight();
    let admitted = admit_bounded_materialization_frontier_preflight(preflight.clone())
        .expect("bounded materialization admitted");
    let evidence = SerialFallbackEvidence::from_surface(
        preflight.basis().proof().digest().as_str(),
        FrontierSurfaceDigest::from_label("frontier-certification-serial-fallback"),
        SerialFallbackReason::DeterministicAdmissionDenied,
        FrontierPredictionDriftOutcome::WithinBudget,
    );
    let route =
        lower_preflight_to_serial_fallback_route(&admitted, &evidence).expect("serial route");
    let execution =
        execute_serial_fallback_route(&route).expect("serial fallback execution should succeed");
    FrontierCertificationLane {
        parity_bundle: FrontierParityBundle::from_serial_fallback(&route, &execution),
    }
}

pub(super) fn serial_fallback_bundle_lane() -> FrontierCertificationLane {
    let first = admit_bounded_materialization_frontier_preflight(ordered_collection_preflight())
        .expect("first bounded preflight admitted");
    let second = admit_bounded_materialization_frontier_preflight(ordered_collection_preflight())
        .expect("second bounded preflight admitted");
    let bundle_evidence = SerialFallbackBundleEvidence::from_routes(
        FrontierSurfaceDigest::from_label("frontier-certification-bundle"),
        vec![
            SerialFallbackEvidence::from_surface(
                first.as_preflight().basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("frontier-certification-bundle-a"),
                SerialFallbackReason::SerialExecutor,
                FrontierPredictionDriftOutcome::WithinBudget,
            ),
            SerialFallbackEvidence::from_surface(
                second.as_preflight().basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("frontier-certification-bundle-b"),
                SerialFallbackReason::SerialExecutor,
                FrontierPredictionDriftOutcome::WithinBudget,
            ),
        ],
    )
    .expect("serial fallback bundle evidence should carry one shared basis");
    let bundle = lower_preflight_bundle_to_serial_fallback_routes(
        &[first.clone(), second],
        &bundle_evidence,
    )
    .expect("serial fallback bundle should lower");
    let route = &bundle.routes()[0];
    let execution = execute_serial_fallback_route(route).expect("bundle execution");
    FrontierCertificationLane {
        parity_bundle: FrontierParityBundle::from_serial_fallback_bundle(&bundle, 0, &execution)
            .expect("bundle parity bundle should resolve first route"),
    }
}

pub(super) fn parallel_admitted_bundle_lane() -> FrontierCertificationLane {
    let first = admit_ordered_collection_frontier_preflight(
        ordered_collection_without_traversal_preflight(),
    )
    .expect("first ordered frontier preflight admitted");
    let second = admit_ordered_collection_frontier_preflight(
        ordered_collection_without_traversal_preflight(),
    )
    .expect("second ordered frontier preflight admitted");
    let bundle_evidence = ParallelAdmissionBundleEvidence::from_routes(
        FrontierSurfaceDigest::from_label("frontier-certification-parallel-bundle"),
        vec![
            ParallelAdmissionEvidence::from_surface(
                first.as_preflight().basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("frontier-certification-parallel-bundle-a"),
                FrontierDisjointnessClass::CollectionWindowSurface,
            ),
            ParallelAdmissionEvidence::from_surface(
                second.as_preflight().basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("frontier-certification-parallel-bundle-b"),
                FrontierDisjointnessClass::CollectionWindowSurface,
            ),
        ],
    )
    .expect("parallel bundle evidence should carry one shared basis");
    let bundle =
        lower_preflight_bundle_to_parallel_admission_routes(&[first, second], &bundle_evidence)
            .expect("parallel bundle should lower");
    let route = &bundle.routes()[0];
    let execution = execute_parallel_admission_route(route).expect("parallel bundle execution");

    FrontierCertificationLane {
        parity_bundle: FrontierParityBundle::from_parallel_admission_bundle(&bundle, 0, &execution)
            .expect("parallel bundle parity bundle should resolve first route"),
    }
}
