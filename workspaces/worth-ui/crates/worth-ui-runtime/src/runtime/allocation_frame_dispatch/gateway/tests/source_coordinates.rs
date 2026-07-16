use crate::graph::UiGraphNodeIdentity;
use crate::host::{
    admit_current_host_measurement_evidence, UiHostMeasurementAssumptionProfile,
    UiHostMeasurementFreshnessWitness,
};
use crate::runtime::tests::source_ingress_test_support::{empty_artifact, framework_from_artifact};
use worth_ui_host_contract::{
    UiHostObservationValue, UiMeasurementEvidenceFamily, UiMeasurementRequest,
    UiMeasurementRequestIdentity, UiViewportExtentObservation, UiViewportExtentRequest,
    WorthUiHostCapability, WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport,
    WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

#[test]
fn host_gateway_accepts_only_source_positioned_current_measurement_evidence() {
    let report =
        WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::ViewportObservation])
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(7));
    let profile = UiHostMeasurementAssumptionProfile::from_capability_report(&report, 1, 2, 3, 4);
    let generation = UiEvidenceAuthorityGeneration::new(11);
    let result = crate::host::tests::measurement_result_test_support::normalized_viewport_result(
        UiMeasurementRequestIdentity::new(91),
        &report,
        generation,
        profile,
    );
    assert!(admit_current_host_measurement_evidence(
        &result,
        UiHostMeasurementFreshnessWitness::new(UiEvidenceAuthorityGeneration::new(12), profile),
    )
    .is_err());
    let mut runtime = framework_from_artifact(empty_artifact());
    let first = admit_host_source(&result, generation, profile);
    let second = admit_host_source(&result, generation, profile);
    assert_eq!(first.source_identity(), second.source_identity());
    let mut outcomes = None;
    let turn_outcome = super::run_framework_turn(&mut runtime, |turn| {
        outcomes = Some((
            turn.submit_host_measurement(first),
            turn.submit_host_measurement(second),
        ));
    });
    let (first_outcome, second_outcome) = outcomes.expect("framework callback submits");
    assert!(first_outcome
        .submission()
        .is_some_and(|outcome| outcome.is_queued()));
    assert!(second_outcome
        .submission()
        .is_some_and(|outcome| outcome.is_duplicate()));
    assert_eq!(turn_outcome, super::TestFrameworkTurnPosture::Denied);
}

fn admit_host_source(
    result: &crate::evidence::UiMeasurementResult,
    generation: UiEvidenceAuthorityGeneration,
    profile: UiHostMeasurementAssumptionProfile,
) -> crate::host::UiAdmittedHostMeasurement {
    crate::host::UiAdmittedHostMeasurement::from_current(
        admit_current_host_measurement_evidence(
            result,
            UiHostMeasurementFreshnessWitness::new(generation, profile),
        )
        .expect("matching host generation admits"),
    )
}

struct ViewportAdapter(f32);

impl WorthUiMeasurementHostAdapter for ViewportAdapter {
    fn observe_measurement(&self, _request: &UiMeasurementRequest) -> UiHostObservationValue {
        UiHostObservationValue::ViewportExtent(UiViewportExtentObservation {
            width: self.0,
            height: 600.0,
        })
    }
}

#[test]
fn host_source_distinguishes_equal_truth_at_later_monotonic_positions() {
    let report =
        WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::ViewportObservation])
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(7));
    let profile = UiHostMeasurementAssumptionProfile::from_capability_report(&report, 1, 2, 3, 4);
    let generation = UiEvidenceAuthorityGeneration::new(11);
    let mut runtime = framework_from_artifact(empty_artifact());
    let collector = runtime.host_measurement_collector();
    let admitted = [800.0, 900.0, 800.0].map(|width| {
        collector
            .collect_admitted(
                &ViewportAdapter(width),
                UiMeasurementRequestIdentity::new(91),
                UiMeasurementEvidenceFamily::ViewportExtent,
                crate::host::UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
                &report,
                generation,
                crate::host::UiHostMeasurementNormalizationContext::viewport_logical_exact(profile),
            )
            .expect("host observation admits")
    });
    assert_eq!(
        admitted.each_ref().map(|fact| fact.source_order()),
        [1, 2, 3]
    );
    let posture = super::run_framework_turn(&mut runtime, |turn| {
        for fact in admitted {
            assert!(turn
                .submit_host_measurement(fact)
                .submission()
                .is_some_and(|submission| submission.is_queued()));
        }
    });
    assert_eq!(posture, super::TestFrameworkTurnPosture::Denied);
}

#[test]
fn durable_gateway_preserves_reconciliation_generation_and_order() {
    let (mut runtime, _, input) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_durable_resize_catalog();
    let source_fact = runtime
        .admit_durable_resize_source(input.clone())
        .expect("runtime durable source admits");
    let replay = source_fact.clone();
    let later_equal = runtime
        .admit_durable_resize_source(input.clone())
        .expect("later equal durable event admits");
    assert_eq!(
        source_fact.source_generation(),
        input.authority_generation()
    );
    assert_eq!(source_fact.source_order(), 1);
    assert_eq!(later_equal.source_order(), 2);
    let mut outcomes = None;
    let turn_outcome = super::run_framework_turn(&mut runtime, |turn| {
        outcomes = Some((
            turn.submit_durable_resize(source_fact),
            turn.submit_durable_resize(replay),
            turn.submit_durable_resize(later_equal),
        ));
    });
    let (first, duplicate, later) = outcomes.expect("framework callback submits");
    assert!(first
        .submission()
        .is_some_and(|submission| submission.is_queued()));
    assert!(duplicate
        .submission()
        .is_some_and(|submission| submission.is_duplicate()));
    assert!(later
        .submission()
        .is_some_and(|submission| submission.is_queued()));
    assert_eq!(turn_outcome, super::TestFrameworkTurnPosture::Resolved);
}

#[test]
fn framework_turn_capability_routes_all_four_admitted_sources_once() {
    let (mut runtime, _, durable_input) = crate::runtime::tests::production_catalog_activation_test_support::runtime_with_durable_resize_catalog();
    let mut query = super::super::query_test_support::InstalledQueryFixture::new(
        "four-source-tick",
    );
    runtime.install_query_binding_for_test(query.binding_plan());
    let attempt = query.project();

    let report =
        WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::ViewportObservation])
            .with_observation_generation(WorthUiHostCapabilityObservationGeneration::new(7));
    let profile = UiHostMeasurementAssumptionProfile::from_capability_report(&report, 1, 2, 3, 4);
    let mut submissions = Vec::new();
    let completion = runtime.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            submissions.push(
                source
                    .collect_and_submit(
                        &ViewportAdapter(800.0),
                        UiMeasurementRequestIdentity::new(91),
                        UiMeasurementEvidenceFamily::ViewportExtent,
                        crate::host::UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
                        &report,
                        UiEvidenceAuthorityGeneration::new(11),
                        crate::host::UiHostMeasurementNormalizationContext::viewport_logical_exact(
                            profile,
                        ),
                    )
                    .expect("host source admits"),
            );
        });
        turn.query_projection(|source| {
            submissions.push(
                source
                    .admit_and_submit(attempt)
                    .expect("partial Query source admits"),
            );
        });
        turn.resize_preview(|source| {
            submissions.push(
                source
                    .admit_and_submit(crate::runtime::UiResizePreviewSample::new(
                        UiGraphNodeIdentity::new(41),
                        crate::runtime::UiResizeLogicalExtent::try_from_logical_pixels(300.0)
                            .unwrap(),
                    ))
                    .expect("interaction source admits"),
            );
        });
        turn.durable_resize(|source| {
            submissions.push(
                source
                    .admit_and_submit(crate::runtime::UiDurableResizeCommitIntent::terminal(
                        durable_input,
                        crate::runtime::UiResizeLogicalExtent::try_from_logical_pixels(320.0)
                            .unwrap(),
                    ))
                    .expect("durable source admits"),
            );
        });
    });
    assert!(matches!(
        completion,
        crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationNarrowingDenied { .. }
            | crate::runtime::WorthUiFrameworkTurnCompletion::AllocationFrameResolutionDenied { .. }
    ));
    drop(completion);
    assert!(submissions.iter().all(|outcome| outcome
        .submission()
        .is_some_and(|submission| submission.is_queued())));
    assert!(runtime.pending_narrowed_allocation_frame.is_none());
}
