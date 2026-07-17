use super::query_test_support::InstalledQueryFixture;
use crate::graph::UiGraphNodeIdentity;
use crate::runtime::tests::source_ingress_test_support::{empty_artifact, framework_from_artifact};
use crate::runtime::{
    UiAllocationFrameGatewayOutcome, UiAllocationFrameSourceFact,
    UiAllocationFrameSourceFactPosture, WorthUiDurableResizeSubmission,
    WorthUiHostMeasurementSubmission, WorthUiInteractionSubmission,
    WorthUiQueryProjectionSubmission, WorthUiRuntimeFrameworkLoop,
    WorthUiTransientInteractionState,
};

mod replay;
mod source_coordinates;

pub(super) struct TestSourceGateways {
    host: WorthUiHostMeasurementSubmission,
    query: WorthUiQueryProjectionSubmission,
    interaction: WorthUiInteractionSubmission,
    durable: WorthUiDurableResizeSubmission,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum TestFrameworkTurnPosture {
    Empty,
    Resolved,
    Denied,
}

impl TestSourceGateways {
    pub(super) fn submit_host_measurement(
        &mut self,
        source: crate::host::UiAdmittedHostMeasurement,
    ) -> UiAllocationFrameGatewayOutcome {
        self.host.submit_admitted_host_measurement(source)
    }
    pub(super) fn submit_query_projection(
        &mut self,
        source: worth_ui_query_binding::WorthUiQueryMeasurementFactSettlement,
    ) -> UiAllocationFrameGatewayOutcome {
        self.query.submit_query_projection_settlement(source)
    }
    pub(super) fn submit_interaction(
        &mut self,
        source: crate::runtime::WorthUiAdmittedTransientInteraction,
    ) -> UiAllocationFrameGatewayOutcome {
        self.interaction
            .submit_admitted_transient_interaction(source)
    }
    pub(super) fn submit_durable_resize(
        &mut self,
        source: crate::runtime::WorthUiAdmittedDurableResizeSourceFact,
    ) -> UiAllocationFrameGatewayOutcome {
        self.durable.submit_admitted_durable_resize(source)
    }
}

pub(super) fn run_framework_turn<F>(
    runtime: &mut WorthUiRuntimeFrameworkLoop,
    submit: F,
) -> TestFrameworkTurnPosture
where
    F: FnOnce(&mut TestSourceGateways),
{
    let mut gateways = TestSourceGateways {
        host: runtime.host_measurement_submission(),
        query: runtime.query_projection_submission(),
        interaction: runtime.interaction_submission(),
        durable: runtime.durable_resize_submission(),
    };
    match runtime.execute_framework_turn(|_| submit(&mut gateways)) {
        crate::runtime::WorthUiFrameworkTurnCompletion::ReadyToExecute { .. } => {
            TestFrameworkTurnPosture::Empty
        }
        crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
            ..
        }
        | crate::runtime::WorthUiFrameworkTurnCompletion::ViewportResizeResolved { .. } => {
            TestFrameworkTurnPosture::Resolved
        }
        crate::runtime::WorthUiFrameworkTurnCompletion::ResizePreviewPublished { .. }
        | crate::runtime::WorthUiFrameworkTurnCompletion::DurableResizeCommitted { .. }
        | crate::runtime::WorthUiFrameworkTurnCompletion::DragResizePreviewPending { .. } => {
            TestFrameworkTurnPosture::Resolved
        }
        crate::runtime::WorthUiFrameworkTurnCompletion::UnacceptedFrameBackpressured
        | crate::runtime::WorthUiFrameworkTurnCompletion::Phase6Backpressured
        | crate::runtime::WorthUiFrameworkTurnCompletion::AllocationFrameResolutionDenied {
            ..
        }
        | crate::runtime::WorthUiFrameworkTurnCompletion::AllocationInvalidationNarrowingDenied {
            ..
        }
        | crate::runtime::WorthUiFrameworkTurnCompletion::AllocationReplanSelectionDenied {
            ..
        }
        | crate::runtime::WorthUiFrameworkTurnCompletion::ViewportResizeDenied { .. } => {
            TestFrameworkTurnPosture::Denied
        }
        crate::runtime::WorthUiFrameworkTurnCompletion::Denied { .. } => {
            TestFrameworkTurnPosture::Denied
        }
    }
}

#[test]
fn interaction_gateway_reaches_only_the_framework_turn_capability() {
    let mut framework = framework_from_artifact(empty_artifact());
    let admitted = framework
        .interaction_admission()
        .admit(
            UiGraphNodeIdentity::new(41),
            WorthUiTransientInteractionState::DragCapture,
        )
        .expect("interaction source should admit");
    let mut submitted = None;
    let turn_outcome = run_framework_turn(&mut framework, |turn| {
        submitted = Some(turn.submit_interaction(admitted));
    });
    let submitted = submitted.expect("framework callback submits");
    assert!(submitted
        .submission()
        .is_some_and(|outcome| outcome.is_queued()));
    let evidence = submitted.evidence().expect("gateway evidence").ingress();
    assert_eq!(
        evidence.source_fact_posture(),
        UiAllocationFrameSourceFactPosture::Interaction
    );
    assert_eq!(
        evidence.key().ingress_identity().as_u64(),
        admitted.source_order()
    );

    assert_eq!(turn_outcome, TestFrameworkTurnPosture::Denied);
    assert!(framework.pending_narrowed_allocation_frame.is_none());
}

#[test]
fn gateway_admission_is_bounded_by_dispatcher_transport_backpressure() {
    let mut framework = framework_from_artifact(empty_artifact());
    let mut admission = framework.interaction_admission();
    let admitted = (0..=64)
        .map(|_| {
            admission
                .admit(
                    UiGraphNodeIdentity::new(7),
                    WorthUiTransientInteractionState::DragCapture,
                )
                .expect("interaction source should admit")
        })
        .collect::<Vec<_>>();
    drop(admission);
    let mut overflow = None;
    run_framework_turn(&mut framework, |turn| {
        for admitted in admitted.iter().copied().take(64) {
            let outcome = turn.submit_interaction(admitted);
            assert!(outcome
                .submission()
                .is_some_and(|submission| submission.is_queued()));
        }
        overflow = Some(turn.submit_interaction(admitted[64]));
    });
    let overflow = overflow.expect("overflow attempted in framework turn");
    let submission = overflow.submission().expect("typed dispatcher outcome");
    assert!(submission.is_backpressured());
    assert_eq!(submission.backpressure_watermark(), Some(64));
    assert_eq!(overflow.counters().mailbox_high_watermark(), 64);
    assert!(matches!(
        overflow.retry_source_fact(),
        Some(UiAllocationFrameSourceFact::Interaction(retry)) if *retry == admitted[64]
    ));
}

#[test]
fn duplicate_heavy_pressure_uses_dispatcher_capacity_not_attempt_count() {
    let mut runtime = framework_from_artifact(empty_artifact());
    let admitted = (0..=64)
        .map(|target| {
            runtime
                .interaction_admission()
                .admit(
                    UiGraphNodeIdentity::new(target),
                    WorthUiTransientInteractionState::DragCapture,
                )
                .expect("interaction source should admit")
        })
        .collect::<Vec<_>>();
    let outcome = run_framework_turn(&mut runtime, |turn| {
        for _ in 0..64 {
            assert!(turn
                .submit_interaction(admitted[0])
                .submission()
                .is_some_and(|submission| submission.is_queued() || submission.is_duplicate()));
        }
        assert!(turn
            .submit_interaction(admitted[64])
            .submission()
            .is_some_and(|submission| submission.is_queued()));
    });
    assert_eq!(outcome, TestFrameworkTurnPosture::Denied);
}

#[test]
fn empty_framework_turn_is_typed_and_does_not_acknowledge() {
    let mut framework = framework_from_artifact(empty_artifact());
    assert_eq!(
        run_framework_turn(&mut framework, |_| {}),
        TestFrameworkTurnPosture::Empty
    );
}

#[test]
fn query_gateway_derives_and_submits_real_projection_consumption() {
    let mut query = InstalledQueryFixture::new("runtime-gateway");
    let mut framework = Box::new(framework_from_artifact(empty_artifact()));
    framework.install_query_binding_for_test(query.binding_plan());
    let settlement = framework
        .admit_query_projection_for_test(query.project())
        .expect("Query source should admit before submission");
    let expected_source_identity = settlement
        .allocation_source_identity()
        .authority_index_key()
        .clone();
    let mut outcome = Box::new(None);
    let turn_outcome = run_framework_turn(&mut framework, |turn| {
        *outcome = Some(turn.submit_query_projection(settlement));
    });
    let outcome = outcome.take().expect("framework callback submits");
    assert!(outcome
        .submission()
        .is_some_and(|submission| submission.is_queued()));
    assert_eq!(
        outcome
            .evidence()
            .expect("gateway evidence")
            .ingress()
            .key()
            .source_identity()
            .query_authority_index_key(),
        Some(&expected_source_identity)
    );
    assert!(matches!(
        outcome
            .evidence()
            .expect("gateway evidence")
            .ingress()
            .source_fact_posture(),
        UiAllocationFrameSourceFactPosture::QueryProjection { settlement, .. }
            if settlement == super::UiAllocationFrameQuerySettlementPosture::Settled
    ));
    assert_eq!(turn_outcome, TestFrameworkTurnPosture::Denied);
}

#[test]
fn query_burst_uses_binding_order_and_transport_bound() {
    let mut query = InstalledQueryFixture::new("bounded-query-burst");
    let mut framework = framework_from_artifact(empty_artifact());
    framework.install_query_binding_for_test(query.binding_plan());
    let settlements = (0..=64)
        .map(|_| {
            framework
                .admit_query_projection_for_test(query.project())
                .expect("Query source should admit before submission")
        })
        .collect::<Vec<_>>();
    let mut overflow = None;
    run_framework_turn(&mut framework, |turn| {
        for settlement in settlements.iter().take(64).cloned() {
            let outcome = turn.submit_query_projection(settlement);
            assert!(outcome
                .submission()
                .is_some_and(|submission| submission.is_queued()));
        }
        overflow = Some(turn.submit_query_projection(settlements[64].clone()));
    });
    let overflow = overflow.expect("overflow attempted in framework turn");
    assert!(overflow
        .submission()
        .is_some_and(|submission| submission.is_backpressured()));
    assert_eq!(overflow.counters().mailbox_high_watermark(), 64);
}
