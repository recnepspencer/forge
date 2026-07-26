use worth_runtime_bridge::facade::{
    BridgeExecutionBasisSignalTerminal, BridgeExecutionBasisTerminalDisposition,
    BridgeManagedExecutionCancellationReason,
};

use super::yield_fixture::YieldProvider;
use super::*;

#[test]
fn direct_yield_retains_exact_authorities_and_releases_them_explicitly() {
    let (running, graph, _bridge) = managed_graph_run_with_provider_and_bridge(
        WorthQueryOperationGraphAccess::Observe,
        YieldProvider::installed(5),
    );
    let logical_run_identity = running.logical_run_identity().to_owned();
    let attempt_identity = running.identity().to_owned();
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "direct-yield",
            ),
        )
        .expect("yield provider should begin");
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("yield provider did not reach its declared safe point"),
    };

    let yielded = match paused.yield_run() {
        crate::domain_computation::WorthQueryDirectYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("eligible direct run did not yield"),
    };
    assert_eq!(yielded.logical_run_identity(), logical_run_identity);
    assert_eq!(yielded.yielded_attempt_identity(), attempt_identity);
    assert_ne!(
        yielded.logical_run_identity(),
        yielded.yielded_attempt_identity()
    );
    assert_eq!(yielded.checkpoint().retained_bytes(), 5);
    assert!(yielded.checkpoint().provider_generation() > 0);
    assert_eq!(yielded.retained_capacity_reservation_count(), 2);
    assert_eq!(
        yielded.bridge().disposition(),
        BridgeExecutionBasisTerminalDisposition::Yielded
    );
    assert_eq!(
        yielded.bridge().signal_terminal(),
        BridgeExecutionBasisSignalTerminal::Cancelled
    );
    assert!(yielded.bridge().reservation_released());
    assert!(yielded.bridge().signal_transition_performed());
    assert_eq!(yielded.provider_work().interrupted_call_count(), 1);
    assert_eq!(yielded.provider_work().completed_work_units(), 2);

    let cleanup = complete_direct_yield_cleanup(yielded);
    assert_eq!(cleanup.logical_run_identity(), logical_run_identity);
    assert_eq!(cleanup.yielded_attempt_identity(), attempt_identity);
    assert!(cleanup.relational().released());
    assert_eq!(cleanup.attempt().capacity().released_reservation_count(), 2);
    assert_eq!(cleanup.checkpoint().unwrap().retained_bytes(), 5);
}

#[test]
fn direct_yield_denials_preserve_the_paused_execution_authority() {
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        YieldProvider::without_installed_yield(),
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "direct-yield-denied",
            ),
        )
        .unwrap();
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("provider did not pause"),
    };
    let denied = match paused.yield_run() {
        crate::domain_computation::WorthQueryDirectYieldOutcome::Denied(denied) => denied,
        _ => panic!("non-yieldable contract minted yielded authority"),
    };
    assert_eq!(
        denied.kind(),
        crate::domain_computation::WorthQueryDirectYieldDenialKind::YieldNotInstalled
    );
    let completion = match denied.into_paused().advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("yield denial consumed the paused execution"),
    };
    let terminal = completion.into_running().completed().unwrap();
    assert!(terminal.cleanup().is_ok());
}

#[test]
fn checkpoint_claim_is_required_even_when_yield_is_installed() {
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        YieldProvider::without_checkpoint_evidence(),
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "direct-no-checkpoint",
            ),
        )
        .unwrap();
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("provider did not pause"),
    };
    let denied = match paused.yield_run() {
        crate::domain_computation::WorthQueryDirectYieldOutcome::Denied(denied) => denied,
        _ => panic!("missing checkpoint evidence permitted yield"),
    };
    assert_eq!(
        denied.kind(),
        crate::domain_computation::WorthQueryDirectYieldDenialKind::CheckpointUnavailable
    );
    let completion = match denied.into_paused().advance() {
        WorthQueryDirectGraphStepOutcome::Completed(completion) => completion,
        _ => panic!("checkpoint denial consumed the execution"),
    };
    assert!(completion
        .into_running()
        .completed()
        .unwrap()
        .cleanup()
        .is_ok());
}

#[test]
fn suspension_failure_terminalizes_signal_but_preserves_cleanup_authority() {
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        YieldProvider::suspension_failure(),
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "direct-suspend-failure",
            ),
        )
        .unwrap();
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("provider did not pause"),
    };
    let recovery = match paused.yield_run() {
        crate::domain_computation::WorthQueryDirectYieldOutcome::RecoveryRequired(recovery) => {
            recovery
        }
        _ => panic!("suspension failure did not return recovery authority"),
    };
    assert!(!recovery.running_attempt_recoverable());
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryYieldRecoveryKind::ProviderCheckpointSuspension(
            crate::domain_computation::WorthQueryProviderCheckpointSuspensionFailureKind::
                ProviderRejected,
        )
    );
    let cleanup = match recovery.cleanup_terminalized() {
        Ok(cleanup) => cleanup,
        Err(_) => panic!("terminalized direct recovery did not release"),
    };
    assert_eq!(
        cleanup.bridge().signal_terminal(),
        BridgeExecutionBasisSignalTerminal::Cancelled
    );
    assert!(cleanup.relational().released());
    assert_eq!(cleanup.attempt().capacity().released_reservation_count(), 2);
    assert_eq!(cleanup.provider_work().abandoned_call_count(), 1);
}

#[test]
fn suspension_panic_and_oversized_checkpoint_follow_the_same_recovery_lane() {
    for (provider, expected_kind) in [
        (
            YieldProvider::suspension_panic(),
            crate::domain_computation::WorthQueryYieldRecoveryKind::ProviderCheckpointSuspension(
                crate::domain_computation::WorthQueryProviderCheckpointSuspensionFailureKind::
                    ProviderPanicked,
            ),
        ),
        (
            YieldProvider::checkpoint_probe_panic(),
            crate::domain_computation::WorthQueryYieldRecoveryKind::ProviderCheckpointSuspension(
                crate::domain_computation::WorthQueryProviderCheckpointSuspensionFailureKind::
                    CheckpointRetention(
                        crate::domain_computation::
                            WorthQueryProviderCheckpointRetentionFailureKind::
                                RetainedByteProbePanicked,
                    ),
            ),
        ),
        (
            YieldProvider::checkpoint_probe_and_drop_panic(),
            crate::domain_computation::WorthQueryYieldRecoveryKind::ProviderCheckpointSuspension(
                crate::domain_computation::WorthQueryProviderCheckpointSuspensionFailureKind::
                    CheckpointRetention(
                        crate::domain_computation::
                            WorthQueryProviderCheckpointRetentionFailureKind::
                                RetainedByteProbePanicked,
                    ),
            ),
        ),
        (
            YieldProvider::installed(4_097),
            crate::domain_computation::WorthQueryYieldRecoveryKind::RetainedBytesExceeded,
        ),
    ] {
        let (running, graph) =
            managed_graph_run_with_provider(WorthQueryOperationGraphAccess::Observe, provider);
        let active = running
            .begin_graph_execution(
                &graph,
                WorthQueryManagedGraphCallRequest::new(
                    WorthQueryGraphProviderCallKind::Observe,
                    "direct-yield-adversarial",
                ),
            )
            .unwrap();
        let paused = match active.advance() {
            WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
            _ => panic!("provider did not pause"),
        };
        let recovery = match paused.yield_run() {
            crate::domain_computation::WorthQueryDirectYieldOutcome::RecoveryRequired(recovery) => {
                recovery
            }
            _ => panic!("invalid checkpoint transition did not require recovery"),
        };
        assert!(!recovery.running_attempt_recoverable());
        assert_eq!(recovery.kind(), expected_kind);
        match recovery.cleanup_terminalized() {
            Ok(_) => {}
            Err(_) => panic!("terminalized adversarial recovery did not release"),
        }
    }
}

#[test]
fn direct_yield_preserves_exact_applied_effect_evidence() {
    let (running, graph) =
        managed_graph_effect_run_with_provider(YieldProvider::installed_with_partial_effect(5));
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::TouchEffect,
                "direct-yield-partial-effect",
            ),
        )
        .expect("installed effect provider should begin");
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("effect provider did not reach its yield safe point"),
    };
    let yielded = match paused.yield_run() {
        crate::domain_computation::WorthQueryDirectYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("installed partial-effect posture did not admit yield"),
    };
    assert_eq!(yielded.provider_work().applied_effect_count(), 1);
    assert_eq!(yielded.provider_work().completed_work_units(), 3);
    let cleanup = complete_direct_yield_cleanup(yielded);
    assert_eq!(cleanup.provider_work().applied_effect_count(), 1);
    assert_eq!(cleanup.provider_work().abandoned_call_count(), 0);
    assert_eq!(cleanup.provider_work().interrupted_call_count(), 1);
}

#[test]
fn signal_terminalized_after_safe_point_cannot_be_relabelled_as_yielded() {
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        YieldProvider::installed(5),
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "direct-yield-signal-race",
            ),
        )
        .unwrap();
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("provider did not pause"),
    };
    paused
        .active
        .request_cancellation(BridgeManagedExecutionCancellationReason::HostRequested)
        .expect("host should terminalize the exact Signal attempt");
    let recovery = match paused.yield_run() {
        crate::domain_computation::WorthQueryDirectYieldOutcome::RecoveryRequired(recovery) => {
            recovery
        }
        _ => panic!("pre-terminalized Signal attempt minted yielded authority"),
    };
    assert!(!recovery.running_attempt_recoverable());
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryYieldRecoveryKind::SignalAttemptAlreadyTerminal(
            BridgeExecutionBasisSignalTerminal::Cancelled,
        )
    );
    let cleanup = match recovery.cleanup_terminalized() {
        Ok(cleanup) => cleanup,
        Err(_) => panic!("terminalized Signal race did not preserve cleanup authority"),
    };
    assert!(!cleanup.bridge().signal_transition_performed());
    assert_eq!(
        cleanup.bridge().signal_terminal(),
        BridgeExecutionBasisSignalTerminal::Cancelled
    );
    assert_eq!(cleanup.provider_work().interrupted_call_count(), 1);
    assert_eq!(cleanup.provider_work().abandoned_call_count(), 0);
}

#[test]
fn timeout_and_rejection_after_safe_point_cannot_mint_yielded_authority() {
    let (timed_out, timeout_bridge) = paused_direct_yield_target("direct-yield-timeout-race");
    timeout_bridge
        .advance_managed_execution_clock(1)
        .expect("host clock should advance");
    timed_out
        .active
        .admit_ready_timeout()
        .expect("ready timeout should terminalize the exact Signal attempt");
    assert_preterminalized_yield_recovery(timed_out, BridgeExecutionBasisSignalTerminal::TimedOut);

    let (rejected, _) = paused_direct_yield_target("direct-yield-rejection-race");
    rejected
        .active
        .reject_execution(
            worth_runtime_bridge::facade::BridgeManagedExecutionRejectionReason::SemanticFailure,
        )
        .expect("rejection should terminalize the exact Signal attempt");
    assert_preterminalized_yield_recovery(rejected, BridgeExecutionBasisSignalTerminal::Rejected);
}

fn paused_direct_yield_target(
    scope: &str,
) -> (
    crate::domain_computation::WorthQueryPausedDirectGraphExecution,
    RuntimeBridge,
) {
    let (running, graph, bridge) = managed_graph_run_with_provider_and_bridge(
        WorthQueryOperationGraphAccess::Observe,
        YieldProvider::installed(5),
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(WorthQueryGraphProviderCallKind::Observe, scope),
        )
        .unwrap();
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("provider did not pause"),
    };
    (paused, bridge)
}

fn assert_preterminalized_yield_recovery(
    paused: crate::domain_computation::WorthQueryPausedDirectGraphExecution,
    terminal: BridgeExecutionBasisSignalTerminal,
) {
    let recovery = match paused.yield_run() {
        crate::domain_computation::WorthQueryDirectYieldOutcome::RecoveryRequired(recovery) => {
            recovery
        }
        _ => panic!("pre-terminalized Signal attempt minted yielded authority"),
    };
    assert_eq!(
        recovery.kind(),
        crate::domain_computation::WorthQueryYieldRecoveryKind::SignalAttemptAlreadyTerminal(
            terminal,
        )
    );
    let cleanup = match recovery.cleanup_terminalized() {
        Ok(cleanup) => cleanup,
        Err(_) => panic!("pre-terminalized Signal attempt lost cleanup authority"),
    };
    assert!(!cleanup.bridge().signal_transition_performed());
    assert_eq!(cleanup.bridge().signal_terminal(), terminal);
    assert_eq!(cleanup.provider_work().interrupted_call_count(), 1);
}
