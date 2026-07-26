use super::readmission_direct::yielded_direct;
use super::*;

#[test]
fn foreign_bridge_denies_before_fresh_query_or_provider_work_and_preserves_retry() {
    let (yielded, bridge, runtime) = yielded_direct();
    let checkpoint = yielded.checkpoint().identity().to_owned();
    let resource_attempt = yielded.resource_attempt_identity().to_owned();
    let foreign_bridge = super::causal_fixture::managed_admission_context().bridge;
    let denied = match yielded.readmit_same_runtime(&runtime, &foreign_bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("foreign Bridge runtime should deny during Bridge preflight"),
    };
    assert_eq!(
        denied.kind(),
        crate::domain_computation::WorthQueryDirectReadmissionDenialKind::BridgeReadmissionDenied
    );
    assert_eq!(denied.counters().preflight_check_count(), 1);
    assert_eq!(denied.counters().fresh_resource_attempt_count(), 0);
    assert_eq!(denied.counters().bridge_readmission_attempt_count(), 0);
    assert_eq!(denied.counters().provider_restore_attempt_count(), 0);
    let yielded = denied.into_yielded();
    assert_eq!(yielded.checkpoint().identity(), checkpoint);
    assert_eq!(yielded.resource_attempt_identity(), resource_attempt);

    let active = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::Readmitted(active) => active,
        _ => panic!("foreign-Bridge denial must preserve exact retry authority"),
    };
    let completion = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("restored provider should complete"),
    };
    assert!(completion
        .into_running()
        .completed()
        .unwrap()
        .cleanup()
        .is_ok());
}

#[test]
fn successor_installation_denies_stale_yield_before_any_fresh_authority() {
    let (yielded, _bridge, mut runtime) = yielded_direct();
    let checkpoint = yielded.checkpoint().identity().to_owned();
    runtime
        .commit_successor_installation(Arc::new(
            runtime.installed_packages().successor_generation(),
        ))
        .expect("successor installation should commit");
    let bridge = super::causal_fixture::managed_admission_context().bridge;
    let denied = match yielded.readmit_same_runtime(&runtime, &bridge) {
        crate::domain_computation::WorthQueryDirectReadmissionOutcome::Denied(denied) => denied,
        _ => panic!("stale yielded installation should deny"),
    };
    assert_eq!(
        denied.kind(),
        crate::domain_computation::WorthQueryDirectReadmissionDenialKind::StaleInstallationGeneration
    );
    assert_eq!(denied.counters().fresh_resource_attempt_count(), 0);
    assert_eq!(denied.counters().bridge_readmission_attempt_count(), 0);
    assert_eq!(denied.counters().provider_restore_attempt_count(), 0);
    let yielded = denied.into_yielded();
    assert_eq!(yielded.checkpoint().identity(), checkpoint);
    assert_eq!(
        complete_direct_yield_cleanup(yielded)
            .checkpoint()
            .unwrap()
            .identity(),
        checkpoint
    );
}
