use worth_runtime_bridge::facade::BridgeMixedCauseOrderingInput;
use worth_ui_query_binding::{
    UiPresentProjection, UiProjectionAvailability, UiProjectionFactStopKind,
    UiProjectionUnavailableKind, UiScalarProjectionBatchOutcome, UiScalarProjectionFactReceipt,
    UiScalarProjectionTransitionReceipt, UiScalarProjectionWorkCounters,
};

pub(super) use super::super::projection_lifecycle::support::ScalarLifecycleWorld;

pub(super) fn initial_fact(world: &mut ScalarLifecycleWorld) -> UiScalarProjectionFactReceipt {
    let pending = world.initial();
    assert_unavailable(pending.fact(), UiProjectionUnavailableKind::Pending);
    pending.into_fact_and_predecessor().0
}

pub(super) fn completion_batch(
    world: &mut ScalarLifecycleWorld,
    completion: worth_runtime_bridge::facade::AdmittedBridgeAsyncCompletion,
) -> worth_query::facade::runtime::WorthQueryAsyncResultTransitionBatch {
    world.transition_batch(BridgeMixedCauseOrderingInput::AsyncCompletion(completion))
}

pub(super) fn consume_batch(
    world: &mut ScalarLifecycleWorld,
    batch: worth_query::facade::runtime::WorthQueryAsyncResultTransitionBatch,
    predecessor: UiScalarProjectionFactReceipt,
    budget: worth_ui_query_binding::UiProjectionConsumptionBudget,
) -> UiScalarProjectionTransitionReceipt {
    match world.binding.consume_async_result_batch(
        &mut world.workspace,
        batch,
        Some(predecessor),
        budget,
    ) {
        UiScalarProjectionBatchOutcome::Advanced(receipt) => receipt,
        UiScalarProjectionBatchOutcome::Unchanged(_) => {
            panic!("QP05 authority attempt must produce one terminal receipt")
        }
    }
}

pub(super) fn assert_current(receipt: &UiScalarProjectionTransitionReceipt, expected: &str) {
    match receipt.fact().availability() {
        UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => {
            assert_eq!(value.as_str(), expected)
        }
        other => panic!("expected current native text `{expected}`, got {other:?}"),
    }
}

pub(super) fn assert_stop(
    receipt: &UiScalarProjectionTransitionReceipt,
    expected: UiProjectionFactStopKind,
) {
    match receipt.fact().availability() {
        UiProjectionAvailability::Stopped(stop) => assert_eq!(stop.kind(), expected),
        other => panic!("expected authority stop {expected:?}, got {other:?}"),
    }
}

pub(super) fn assert_unavailable(
    fact: &UiScalarProjectionFactReceipt,
    expected: UiProjectionUnavailableKind,
) {
    match fact.availability() {
        UiProjectionAvailability::Unavailable(receipt) => assert_eq!(receipt.kind(), expected),
        other => panic!("expected unavailable {expected:?}, got {other:?}"),
    }
}

pub(super) fn assert_zero_native_work(work: UiScalarProjectionWorkCounters) {
    assert_eq!(work.native_key_declaration_checks(), 0);
    assert_eq!(work.native_key_indexed_slot_lookups(), 0);
    assert_eq!(work.native_key_scan_work(), 0);
    assert_eq!(work.native_indexed_accesses(), 0);
    assert_eq!(work.native_access_scan_work(), 0);
}

pub(super) fn reporting_tuple(
    receipt: &UiScalarProjectionTransitionReceipt,
) -> (&str, &str, &str, &str) {
    let core = receipt.fact().core();
    (
        core.query_world_identity_for_reporting(),
        core.binding_identity_for_reporting(),
        core.source_generation_for_reporting(),
        core.result_generation_for_reporting(),
    )
}
