use crate::execution::{
    execute_parallel_admission_route, execute_preflight_bundle, execute_serial_fallback_route,
};
use crate::frontier_signal_adapter::{SignalFrontierBundleEvidence, SignalFrontierSurfaceEvidence};
use crate::live::promote_preflight_bundle_to_live;
use crate::planning::{
    admit_bounded_materialization_frontier_preflight, admit_ordered_collection_frontier_preflight,
    lower_execution_preflight_to_frontier_plan, lower_frontier_planning_bundle,
    lower_live_plan_to_frontier_plan, lower_preflight_bundle_to_parallel_admission_routes,
    lower_preflight_bundle_to_serial_fallback_routes, lower_preflight_to_parallel_admission_route,
    lower_preflight_to_serial_fallback_route, FrontierBundleRoutePlanningError,
    FrontierDisjointnessClass, FrontierPlanFamily, FrontierPlanningError, FrontierPlanningInput,
    FrontierPredictionDriftOutcome, FrontierPreflightAdmissionError, FrontierRoutePlanningError,
    FrontierSurfaceDigest, PacketMergeContract, ParallelAdmissionBundleEvidence,
    ParallelAdmissionEvidence, PlannedWorkPacketFamily, SerialFallbackEvidence,
    SerialFallbackReason,
};
use forge_signal::facade::adapters::{
    FrontierEntryClassification, FrontierExecutionCounters, FrontierExecutionSummary,
    FrontierInclusionBasis, FrontierPlan, FrontierPredictedCounters, FrontierRouteEvidenceReason,
    FrontierRouteEvidenceReceipt, FrontierSeedCause, FrontierWaveEntryPlan,
    FrontierWaveEntrySummary, FrontierWavePlan, FrontierWaveSummary, InvalidationSeed,
    InvalidationSeedBatch, PartitionScopeSet, TouchedScopeSummary,
};
use forge_signal::facade::specialist::{EvaluationOutput, RunMode};
use forge_signal::facade::{
    Aspect, AspectVersion, NodeId, PartitionSubscription, SignalError, SignalGraph,
};

#[test]
fn ordered_collection_preflight_lowers_to_stable_frontier_plan() {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();

    let first = lower_execution_preflight_to_frontier_plan(&preflight)
        .expect("ordered collection preflight should lower");
    let second = lower_execution_preflight_to_frontier_plan(&preflight)
        .expect("repeat lowering should stay stable");

    assert_eq!(first.family(), &FrontierPlanFamily::OrderedCollection);
    assert_eq!(
        first.packet_set().packets()[0].family(),
        &PlannedWorkPacketFamily::OrderedCollectionRoot
    );
    assert_eq!(
        first.report().packet_merge_contract(),
        &PacketMergeContract::OrderedCollectionResultBoundary
    );
    assert_eq!(
        first.drift_outcome(),
        &FrontierPredictionDriftOutcome::WithinBudget
    );
    assert_eq!(
        first.bundle_basis_digest().as_str(),
        preflight.basis().proof().digest().as_str()
    );
    assert_eq!(first.packet_set().packets().len(), 1);
    assert_eq!(
        first.packet_set().packets()[0].digest(),
        second.packet_set().packets()[0].digest()
    );
    assert_eq!(
        first.report().posture_digest(),
        second.report().posture_digest()
    );
    assert_eq!(
        first.counters().frontier_planning_invocation_count(),
        1,
        "single-route lowering should emit one planning invocation"
    );
}

#[test]
fn bounded_materialization_lowers_with_distinct_packet_family_and_merge_contract() {
    let ordered =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let bounded = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();

    let ordered_plan = lower_execution_preflight_to_frontier_plan(&ordered)
        .expect("ordered collection preflight should lower");
    let bounded_plan = lower_execution_preflight_to_frontier_plan(&bounded)
        .expect("bounded materialization preflight should lower");

    assert_eq!(
        bounded_plan.family(),
        &FrontierPlanFamily::BoundedMaterialization
    );
    assert_eq!(
        bounded_plan.packet_set().packets()[0].family(),
        &PlannedWorkPacketFamily::BoundedMaterializationRoot
    );
    assert_eq!(
        bounded_plan.report().packet_merge_contract(),
        &PacketMergeContract::BoundedMaterializationResultBoundary
    );
    assert_ne!(
        ordered_plan.packet_set().packets()[0].digest(),
        bounded_plan.packet_set().packets()[0].digest()
    );
    assert_ne!(
        ordered_plan.report().packet_merge_contract(),
        bounded_plan.report().packet_merge_contract()
    );
}

#[test]
fn live_plan_lowering_preserves_descriptor_identity_and_uses_planner_packets() {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live = promote_preflight_bundle_to_live(&preflight)
        .expect("ordered collection preflight should promote to live");

    let plan = lower_live_plan_to_frontier_plan(&live).expect("live plan should lower");

    assert_eq!(
        plan.source_plan_digest(),
        live.descriptor().plan_digest(),
        "frontier lowering should preserve live descriptor plan identity"
    );
    assert_eq!(
        plan.query_digest(),
        live.descriptor().query_digest(),
        "frontier lowering should preserve live descriptor query identity"
    );
    assert_eq!(plan.family(), &FrontierPlanFamily::LiveOrderedCollection);
    assert_eq!(
        plan.packet_set().packets()[0].family(),
        &PlannedWorkPacketFamily::LiveOrderedCollectionRoot
    );
    assert_eq!(
        plan.bundle_basis_digest().as_str(),
        live.progress_basis()
            .current_basis()
            .proof()
            .digest()
            .as_str()
    );
}

#[test]
fn source_plan_digest_changes_frontier_posture_digest() {
    let ascending =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let descending =
        crate::harness::fixtures::execution_preflights::descending_collection_preflight();

    let ascending_plan = lower_execution_preflight_to_frontier_plan(&ascending)
        .expect("ascending collection preflight should lower");
    let descending_plan = lower_execution_preflight_to_frontier_plan(&descending)
        .expect("descending collection preflight should lower");

    assert_ne!(
        ascending.plan().query().plan_digest(),
        descending.plan().query().plan_digest()
    );
    assert_ne!(
        ascending_plan.report().posture_digest(),
        descending_plan.report().posture_digest()
    );
}

#[test]
fn same_basis_bundle_lowers_with_exact_bundle_basis_digest() {
    let ordered =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let bounded = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();

    let bundle = lower_frontier_planning_bundle(&[
        FrontierPlanningInput::from(ordered.clone()),
        FrontierPlanningInput::from(bounded.clone()),
    ])
    .expect("same-basis frontier bundle should lower");

    assert_eq!(bundle.route_plans().len(), 2);
    assert_eq!(
        bundle.bundle_basis_digest().as_str(),
        ordered.basis().proof().digest().as_str()
    );
    assert!(bundle
        .route_plans()
        .iter()
        .all(|route| route.bundle_basis_digest() == bundle.bundle_basis_digest()));
    assert_eq!(bundle.counters().planned_bundle_route_count(), 2);
}

#[test]
fn mixed_basis_bundle_is_denied_even_when_basis_class_matches() {
    let ordered =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let alternate =
        crate::harness::fixtures::execution_preflights::alternate_basis_ordered_collection_preflight();

    let error = lower_frontier_planning_bundle(&[
        FrontierPlanningInput::from(ordered.clone()),
        FrontierPlanningInput::from(alternate.clone()),
    ])
    .expect_err("mixed basis bundle should be rejected");

    match error {
        FrontierPlanningError::MixedBasisBundle {
            expected_basis_digest,
            found_basis_digest,
        } => {
            assert_eq!(
                expected_basis_digest.as_str(),
                ordered.basis().proof().digest().as_str()
            );
            assert_eq!(
                found_basis_digest.as_str(),
                alternate.basis().proof().digest().as_str()
            );
            assert_ne!(expected_basis_digest, found_basis_digest);
        }
        other => panic!("expected mixed basis denial, got {other:?}"),
    }
}

#[test]
fn unsupported_bundle_composition_is_distinguishable_from_mixed_basis_denial() {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let live = promote_preflight_bundle_to_live(&preflight)
        .expect("ordered collection preflight should promote to live");

    let error = lower_frontier_planning_bundle(&[
        FrontierPlanningInput::from(preflight),
        FrontierPlanningInput::from(live),
    ])
    .expect_err("mixed execution/live bundle should be rejected as unsupported composition");

    assert_eq!(error, FrontierPlanningError::UnsupportedBundleComposition);
}

#[test]
fn ordered_collection_lowers_into_parallel_route_with_typed_executor_entrypoint() {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let admitted = admit_ordered_collection_frontier_preflight(preflight.clone())
        .expect("ordered collection should admit on the ordered frontier lane");
    let evidence = ParallelAdmissionEvidence::from_surface(
        preflight.basis().proof().digest().as_str(),
        FrontierSurfaceDigest::from_label("ordered-collection-disjoint"),
        FrontierDisjointnessClass::CollectionWindowSurface,
    );

    let route = lower_preflight_to_parallel_admission_route(&admitted, &evidence)
        .expect("ordered collection should admit the parallel route");
    let typed =
        execute_parallel_admission_route(&route).expect("parallel route entrypoint should execute");
    let baseline = execute_preflight_bundle(&preflight).expect("baseline execution should succeed");

    assert_eq!(typed.rows(), baseline.rows());
    assert_eq!(
        typed.report().result_digest(),
        baseline.report().result_digest()
    );
    assert_eq!(
        route.query_digest(),
        preflight.plan().query().validated_query_digest()
    );
    assert_eq!(
        route.report().route_surface_digest(),
        evidence.surface_digest()
    );
    assert_eq!(
        route.report().disjointness_class(),
        Some(&FrontierDisjointnessClass::CollectionWindowSurface)
    );
    assert_eq!(route.counters().route_parallel_admission_count(), 1);
}

#[test]
fn same_basis_parallel_bundle_lowers_into_parallel_admission_routes() {
    let first =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let second =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let first_admitted = admit_ordered_collection_frontier_preflight(first.clone())
        .expect("first ordered collection should admit");
    let second_admitted = admit_ordered_collection_frontier_preflight(second.clone())
        .expect("second ordered collection should admit");
    let evidence = ParallelAdmissionBundleEvidence::from_routes(
        FrontierSurfaceDigest::from_label("parallel-bundle-surface"),
        vec![
            ParallelAdmissionEvidence::from_surface(
                first.basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("parallel-bundle-route-a"),
                FrontierDisjointnessClass::CollectionWindowSurface,
            ),
            ParallelAdmissionEvidence::from_surface(
                second.basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("parallel-bundle-route-b"),
                FrontierDisjointnessClass::CollectionWindowSurface,
            ),
        ],
    )
    .expect("parallel bundle evidence should carry one shared basis");

    let bundle = lower_preflight_bundle_to_parallel_admission_routes(
        &[first_admitted, second_admitted],
        &evidence,
    )
    .expect("parallel bundle should lower");

    assert_eq!(bundle.routes().len(), 2);
    assert_eq!(
        bundle.bundle_basis_digest(),
        first.basis().proof().digest().as_str()
    );
    assert!(bundle
        .routes()
        .iter()
        .all(|route| route.report().disjointness_class().is_some()));
}

#[test]
fn parallel_bundle_rejects_mixed_basis_evidence() {
    let first =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let second =
        crate::harness::fixtures::execution_preflights::alternate_basis_ordered_collection_preflight();

    let error = ParallelAdmissionBundleEvidence::from_routes(
        FrontierSurfaceDigest::from_label("parallel-bundle-mixed-basis"),
        vec![
            ParallelAdmissionEvidence::from_surface(
                first.basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("parallel-bundle-mixed-a"),
                FrontierDisjointnessClass::CollectionWindowSurface,
            ),
            ParallelAdmissionEvidence::from_surface(
                second.basis().proof().digest().as_str(),
                FrontierSurfaceDigest::from_label("parallel-bundle-mixed-b"),
                FrontierDisjointnessClass::CollectionWindowSurface,
            ),
        ],
    )
    .expect_err("mixed-basis parallel bundle evidence must reject");

    match error {
        crate::frontier_planning::ParallelAdmissionBundleEvidenceError::MixedBasisDigest {
            expected_basis_digest,
            found_basis_digest,
        } => {
            assert_eq!(
                expected_basis_digest,
                first.basis().proof().digest().as_str()
            );
            assert_eq!(found_basis_digest, second.basis().proof().digest().as_str());
        }
        other => panic!("expected mixed-basis parallel bundle denial, got {other:?}"),
    }
}

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

#[test]
fn signal_stage_record_admitted_reason_maps_to_parallel_route_evidence() {
    let preflight =
        crate::harness::fixtures::execution_preflights::ordered_collection_without_traversal_preflight();
    let admitted = admit_ordered_collection_frontier_preflight(preflight.clone())
        .expect("ordered collection should admit on the ordered frontier lane");
    let signal_surface = SignalFrontierSurfaceEvidence::from_frontier_execution_summary(
        &sample_signal_frontier_summary(),
    );
    let stage = sample_stage_execution_record(
        FrontierRouteEvidenceReason::AdmittedProofSafeGroupedConcurrent,
    );
    let evidence = signal_surface.to_parallel_admission_evidence(
        preflight.basis().proof().digest().as_str(),
        FrontierDisjointnessClass::CollectionWindowSurface,
    );
    assert!(stage.is_parallel_admitted());

    let route = lower_preflight_to_parallel_admission_route(&admitted, &evidence)
        .expect("admitted stage evidence should admit the query route");

    assert_eq!(
        route.decision().disjointness_class(),
        &FrontierDisjointnessClass::CollectionWindowSurface
    );
}

#[test]
fn signal_stage_record_serial_reason_maps_to_serial_fallback_evidence() {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let admitted = admit_bounded_materialization_frontier_preflight(preflight.clone())
        .expect("bounded materialization should admit on the serial fallback frontier lane");
    let signal_surface = SignalFrontierSurfaceEvidence::from_frontier_execution_summary(
        &sample_signal_frontier_summary(),
    );
    let evidence = signal_surface.to_serial_fallback_evidence(
        preflight.basis().proof().digest().as_str(),
        SerialFallbackReason::SerialExecutor,
        FrontierPredictionDriftOutcome::WithinBudget,
    );

    let route = lower_preflight_to_serial_fallback_route(&admitted, &evidence)
        .expect("serial stage evidence should lower into the serial fallback route");

    assert_eq!(route.reason(), &SerialFallbackReason::SerialExecutor);
    assert_eq!(
        route.report().serial_fallback_reason(),
        Some(&SerialFallbackReason::SerialExecutor)
    );
}

#[test]
fn signal_stage_record_preserves_specific_serial_admission_reasons() {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let admitted = admit_bounded_materialization_frontier_preflight(preflight.clone())
        .expect("bounded materialization should admit on the serial fallback frontier lane");
    let signal_surface = SignalFrontierSurfaceEvidence::from_frontier_execution_summary(
        &sample_signal_frontier_summary(),
    );

    let cases = [
        (
            FrontierRouteEvidenceReason::BelowMinStageWidth,
            SerialFallbackReason::BelowMinStageWidth,
        ),
        (
            FrontierRouteEvidenceReason::BelowPolicyWorkThreshold,
            SerialFallbackReason::BelowPolicyWorkThreshold,
        ),
        (
            FrontierRouteEvidenceReason::ValidationHeavyStage,
            SerialFallbackReason::ValidationHeavyStage,
        ),
        (
            FrontierRouteEvidenceReason::BelowFullParallelThreshold,
            SerialFallbackReason::BelowFullParallelThreshold,
        ),
        (
            FrontierRouteEvidenceReason::FullParallelUnsupportedByMutableEngine,
            SerialFallbackReason::FullParallelUnsupportedByMutableEngine,
        ),
    ];

    for (signal_reason, query_reason) in cases {
        let _stage = sample_stage_execution_record(signal_reason);
        let evidence = signal_surface.to_serial_fallback_evidence(
            preflight.basis().proof().digest().as_str(),
            query_reason.clone(),
            FrontierPredictionDriftOutcome::WithinBudget,
        );
        let route = lower_preflight_to_serial_fallback_route(&admitted, &evidence)
            .expect("specific serial reason should lower into fallback route");

        assert_eq!(route.reason(), &query_reason);
        assert_eq!(route.report().serial_fallback_reason(), Some(&query_reason));
        assert_eq!(
            route.report().drift_outcome(),
            &FrontierPredictionDriftOutcome::WithinBudget
        );
    }
}

#[test]
fn signal_backed_route_evidence_rejects_basis_mismatch() {
    let preflight = crate::harness::fixtures::execution_preflights::ordered_collection_preflight();
    let admitted = admit_bounded_materialization_frontier_preflight(preflight.clone())
        .expect("bounded materialization should admit on the serial fallback frontier lane");
    let signal_surface = SignalFrontierSurfaceEvidence::from_frontier_execution_summary(
        &sample_signal_frontier_summary(),
    );
    let evidence = signal_surface.to_serial_fallback_evidence(
        "wrong-basis-digest",
        SerialFallbackReason::SerialExecutor,
        FrontierPredictionDriftOutcome::WithinBudget,
    );

    let error = lower_preflight_to_serial_fallback_route(&admitted, &evidence)
        .expect_err("serial fallback route should reject frontier evidence bound to another basis");

    match error {
        FrontierRoutePlanningError::SerialFallbackUnavailable { .. } => {}
        other => panic!("expected serial fallback unavailability, got {other:?}"),
    }
}

#[test]
fn same_basis_bundle_lowers_into_signal_backed_serial_fallback_routes() {
    let first = admit_bounded_materialization_frontier_preflight(
        crate::harness::fixtures::execution_preflights::ordered_collection_preflight(),
    )
    .expect("bounded materialization should admit on the serial fallback frontier lane");
    let second = admit_bounded_materialization_frontier_preflight(
        crate::harness::fixtures::execution_preflights::ordered_collection_preflight(),
    )
    .expect("bounded materialization should admit on the serial fallback frontier lane");
    let route_surface = SignalFrontierSurfaceEvidence::from_frontier_execution_summary(
        &sample_signal_frontier_summary(),
    );
    let bundle_evidence = SignalFrontierBundleEvidence::from_route_evidences(vec![
        route_surface.to_serial_fallback_evidence(
            first.as_preflight().basis().proof().digest().as_str(),
            SerialFallbackReason::SerialExecutor,
            FrontierPredictionDriftOutcome::WithinBudget,
        ),
        route_surface.to_serial_fallback_evidence(
            second.as_preflight().basis().proof().digest().as_str(),
            SerialFallbackReason::SerialExecutor,
            FrontierPredictionDriftOutcome::WithinBudget,
        ),
    ]);

    let bundle = lower_preflight_bundle_to_serial_fallback_routes(
        &[first.clone(), second.clone()],
        &bundle_evidence
            .bind_to_basis(first.as_preflight().basis().proof().digest().as_str())
            .expect("signal bundle evidence should bind to one shared basis"),
    )
    .expect("same-basis bounded bundle should lower into serial fallback routes");

    assert_eq!(
        bundle.bundle_basis_digest(),
        first.as_preflight().basis().proof().digest().as_str()
    );
    assert_eq!(bundle.routes().len(), 2);
    assert_eq!(
        bundle.routes()[0].reason(),
        &SerialFallbackReason::SerialExecutor
    );
    assert_eq!(
        bundle.routes()[1].reason(),
        &SerialFallbackReason::SerialExecutor
    );
    assert!(!bundle.bundle_posture_digest().as_str().is_empty());
    assert!(!bundle_evidence.bundle_surface_digest().as_str().is_empty());
}

#[test]
fn signal_backed_serial_bundle_rejects_evidence_count_mismatch() {
    let preflight = admit_bounded_materialization_frontier_preflight(
        crate::harness::fixtures::execution_preflights::ordered_collection_preflight(),
    )
    .expect("bounded materialization should admit on the serial fallback frontier lane");
    let signal_surface = SignalFrontierSurfaceEvidence::from_frontier_execution_summary(
        &sample_signal_frontier_summary(),
    );
    let runtime_stage = runtime_signal_stage_execution_record();
    let bundle_evidence = SignalFrontierBundleEvidence::from_stage_records(
        preflight.as_preflight().basis().proof().digest().as_str(),
        &[signal_surface],
        &[runtime_stage],
        &[FrontierDisjointnessClass::TraversalScopeSurface],
    )
    .expect("single-route signal bundle evidence should compose");

    let error = lower_preflight_bundle_to_serial_fallback_routes(
        &[preflight.clone(), preflight],
        &bundle_evidence
            .bind_to_basis("signal-bundle-evidence")
            .expect("single-route signal bundle evidence should bind"),
    )
    .expect_err("bundle lowering should reject missing second evidence");

    assert_eq!(
        error,
        FrontierBundleRoutePlanningError::EvidenceCountMismatch {
            expected: 2,
            found: 1,
        }
    );
}

#[test]
fn signal_backed_serial_bundle_rejects_basis_mismatch() {
    let route_surface = SignalFrontierSurfaceEvidence::from_frontier_execution_summary(
        &sample_signal_frontier_summary(),
    );
    let bundle_evidence = SignalFrontierBundleEvidence::from_route_evidences(vec![
        SerialFallbackEvidence::from_surface(
            "basis-a",
            route_surface.surface_digest().clone(),
            SerialFallbackReason::SerialExecutor,
            FrontierPredictionDriftOutcome::WithinBudget,
        ),
        SerialFallbackEvidence::from_surface(
            "basis-b",
            route_surface.surface_digest().clone(),
            SerialFallbackReason::SerialExecutor,
            FrontierPredictionDriftOutcome::WithinBudget,
        ),
    ]);

    let error = crate::frontier_planning::SerialFallbackBundleEvidence::from_routes(
        bundle_evidence.bundle_surface_digest().clone(),
        vec![
            SerialFallbackEvidence::from_surface(
                "basis-a",
                route_surface.surface_digest().clone(),
                SerialFallbackReason::SerialExecutor,
                FrontierPredictionDriftOutcome::WithinBudget,
            ),
            SerialFallbackEvidence::from_surface(
                "basis-b",
                route_surface.surface_digest().clone(),
                SerialFallbackReason::SerialExecutor,
                FrontierPredictionDriftOutcome::WithinBudget,
            ),
        ],
    )
    .expect_err("mixed route bases must be rejected before bundle evidence is admitted");

    match error {
        crate::frontier_planning::SerialFallbackBundleEvidenceError::MixedBasisDigest {
            expected_basis_digest,
            found_basis_digest,
        } => {
            assert_eq!(expected_basis_digest, "basis-a");
            assert_eq!(found_basis_digest, "basis-b");
        }
        other => panic!("expected mixed basis evidence denial, got {other:?}"),
    }
}

#[test]
fn signal_bundle_surface_digest_binds_member_basis_evidence() {
    let route_surface = SignalFrontierSurfaceEvidence::from_frontier_execution_summary(
        &sample_signal_frontier_summary(),
    );
    let left = SignalFrontierBundleEvidence::from_route_evidences(vec![
        SerialFallbackEvidence::from_surface(
            "basis-a",
            route_surface.surface_digest().clone(),
            SerialFallbackReason::SerialExecutor,
            FrontierPredictionDriftOutcome::WithinBudget,
        ),
        SerialFallbackEvidence::from_surface(
            "basis-a",
            route_surface.surface_digest().clone(),
            SerialFallbackReason::SerialExecutor,
            FrontierPredictionDriftOutcome::WithinBudget,
        ),
    ]);
    let right = SignalFrontierBundleEvidence::from_route_evidences(vec![
        SerialFallbackEvidence::from_surface(
            "basis-a",
            route_surface.surface_digest().clone(),
            SerialFallbackReason::SerialExecutor,
            FrontierPredictionDriftOutcome::WithinBudget,
        ),
        SerialFallbackEvidence::from_surface(
            "basis-b",
            route_surface.surface_digest().clone(),
            SerialFallbackReason::SerialExecutor,
            FrontierPredictionDriftOutcome::WithinBudget,
        ),
    ]);

    assert_ne!(
        left.bundle_surface_digest(),
        right.bundle_surface_digest(),
        "bundle surface digest must bind member basis evidence, not only surface and fallback shape"
    );
}

fn sample_signal_frontier_plan() -> FrontierPlan {
    let seed = InvalidationSeed::new(
        NodeId::new(7, 0),
        Aspect::new(0),
        vec![PartitionSubscription::whole_partition("wing")],
        FrontierSeedCause::DirtySource,
    );
    let wave = FrontierWavePlan::new(
        0,
        Aspect::new(0),
        [FrontierWaveEntryPlan::new(
            NodeId::new(8, 0),
            FrontierEntryClassification::DirectDirty,
            FrontierInclusionBasis::PartitionScopeOverlap,
            vec![PartitionSubscription::whole_partition("wing")],
            [0],
        )],
    );

    FrontierPlan::new(
        InvalidationSeedBatch::new([seed]),
        vec![wave],
        Vec::new(),
        TouchedScopeSummary::new(
            PartitionScopeSet::new([PartitionSubscription::whole_partition("wing")]),
            vec![NodeId::new(7, 0), NodeId::new(8, 0)],
            vec![NodeId::new(7, 0)],
        ),
        FrontierPredictedCounters {
            seed_count: 1,
            group_count: 1,
            direct_wave_count: 1,
            transitive_wave_count: 0,
            direct_dirty_count: 1,
            maybe_stale_count: 0,
            partition_scoped_checks: 1,
            partition_match_count: 1,
            detail_match_count: 0,
            cycle_check_candidate_count: 0,
        },
    )
}

fn sample_signal_frontier_summary() -> FrontierExecutionSummary {
    FrontierExecutionSummary::new(
        1,
        vec![FrontierWaveSummary::new(
            0,
            Aspect::new(0),
            [FrontierWaveEntrySummary::new(
                NodeId::new(8, 0),
                FrontierEntryClassification::DirectDirty,
                FrontierInclusionBasis::PartitionScopeOverlap,
                vec![PartitionSubscription::whole_partition("wing")],
            )],
        )],
        Vec::new(),
        TouchedScopeSummary::new(
            PartitionScopeSet::new([PartitionSubscription::whole_partition("wing")]),
            vec![NodeId::new(7, 0), NodeId::new(8, 0)],
            vec![NodeId::new(7, 0)],
        ),
        FrontierExecutionCounters {
            frontier_seed_count: 1,
            frontier_group_count: 1,
            frontier_direct_wave_count: 1,
            frontier_transitive_wave_count: 0,
            frontier_partition_scoped_check_count: 1,
            frontier_direct_dirty_count: 1,
            frontier_maybe_stale_count: 0,
            frontier_partition_match_count: 1,
            frontier_detail_match_count: 0,
            frontier_cycle_check_candidate_count: 0,
            frontier_cycle_check_visited_count: 0,
            frontier_trace_retained_count: 0,
        },
    )
}

fn sample_stage_execution_record(
    reason: FrontierRouteEvidenceReason,
) -> FrontierRouteEvidenceReceipt {
    FrontierRouteEvidenceReceipt::from_reason(reason)
}

fn runtime_signal_stage_execution_record() -> FrontierRouteEvidenceReceipt {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let bootstrap = graph
        .build_evaluation_plan(&[node], RunMode::ForceOnDemand)
        .expect("force-on-demand bootstrap should build");
    graph
        .execute_prepared_plan(&bootstrap, &(), &|_ctx| {
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(signal_version(1)))
        })
        .expect("bootstrap evaluation should succeed");

    let plan = graph
        .build_evaluation_plan(&[node], RunMode::ForceOnDemand)
        .expect("second force-on-demand plan should build");
    let report = graph
        .execute_prepared_plan(&plan, &(), &|ctx| {
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(signal_version(
                ctx.node().index() as u64 + 2,
            )))
        })
        .expect("runtime plan should execute");

    FrontierRouteEvidenceReceipt::from_stage_execution_record(
        report
            .stages
            .first()
            .expect("runtime execution report should record one stage"),
    )
    .expect("runtime stage record should lower into a signal facade route receipt")
}

fn signal_version(revision: u64) -> AspectVersion {
    AspectVersion::from_updates([(Aspect::new(0), revision)])
}
