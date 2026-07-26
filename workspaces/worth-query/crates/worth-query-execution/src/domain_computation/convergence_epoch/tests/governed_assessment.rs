use super::fixture::{direct_admission_fixture, FixtureDisposition};
use super::terminal_fixture::{indeterminate_terminal, workflow_indeterminate_terminal};
use crate::domain_computation::{
    WorthQueryConvergenceDomainInvocationFailureKind as FailureKind,
    WorthQueryConvergenceDomainPhase as Phase, WorthQueryConvergenceEpochDenialKind,
    WorthQueryConvergenceIndeterminateCause, WorthQueryManagedRunCleanupDisposition,
    WorthQueryManagedRunTerminalKind, WorthQueryWorkflowConvergenceCleanupOutcome,
};

#[test]
fn query_counts_each_governed_domain_port_and_contains_rejection_or_panic() {
    let cases = [
        (
            FixtureDisposition::ComparatorFailure,
            Phase::Comparator,
            FailureKind::Rejected,
            [1, 0, 0],
        ),
        (
            FixtureDisposition::ComparatorPanic,
            Phase::Comparator,
            FailureKind::Panicked,
            [1, 0, 0],
        ),
        (
            FixtureDisposition::ProgressFailure,
            Phase::ProgressMeasure,
            FailureKind::Rejected,
            [1, 1, 0],
        ),
        (
            FixtureDisposition::ProgressPanic,
            Phase::ProgressMeasure,
            FailureKind::Panicked,
            [1, 1, 0],
        ),
        (
            FixtureDisposition::RepeatedStateFailure,
            Phase::RepeatedStateDetector,
            FailureKind::Rejected,
            [1, 1, 1],
        ),
        (
            FixtureDisposition::RepeatedStatePanic,
            Phase::RepeatedStateDetector,
            FailureKind::Panicked,
            [1, 1, 1],
        ),
    ];

    for (disposition, expected_phase, expected_kind, expected_work) in cases {
        let terminal = indeterminate_terminal(disposition);
        assert_domain_invocation_cause(
            terminal.indeterminate_cause(),
            expected_phase,
            expected_kind,
            expected_work,
        );
        assert_eq!(
            [
                terminal.counters().comparator_call_count(),
                terminal.counters().progress_check_count(),
                terminal.counters().repeated_state_probe_count(),
            ],
            expected_work
        );
        let cleanup = match terminal.cleanup() {
            Ok(cleanup) => cleanup,
            Err(_) => panic!("contained domain failure must retain direct cleanup authority"),
        };
        assert_domain_invocation_cause(
            cleanup.indeterminate_cause(),
            expected_phase,
            expected_kind,
            expected_work,
        );
        assert_eq!(
            cleanup.managed_receipt().disposition(),
            WorthQueryManagedRunCleanupDisposition::CleanupComplete
        );
    }
}

#[test]
fn workflow_cleanup_preserves_a_typed_governed_domain_panic() {
    let terminal = workflow_indeterminate_terminal(FixtureDisposition::ProgressPanic);
    assert_domain_invocation_cause(
        terminal.indeterminate_cause(),
        Phase::ProgressMeasure,
        FailureKind::Panicked,
        [1, 1, 0],
    );

    let WorthQueryWorkflowConvergenceCleanupOutcome::Complete(cleanup) = terminal.cleanup() else {
        panic!("contained domain panic must retain workflow cleanup authority");
    };
    assert_domain_invocation_cause(
        cleanup.indeterminate_cause(),
        Phase::ProgressMeasure,
        FailureKind::Panicked,
        [1, 1, 0],
    );
}

#[test]
fn provider_family_inspection_panic_denies_and_returns_every_admission_authority() {
    let fixture = direct_admission_fixture(FixtureDisposition::FamilyInspectionPanic);
    let rejection = match fixture.runtime.admit_direct_convergence_epoch(
        &fixture.operation,
        fixture.contract,
        fixture.managed,
        fixture.graph,
    ) {
        Ok(_) => panic!("provider family panic must deny convergence admission"),
        Err(rejection) => rejection,
    };
    assert_eq!(
        rejection.denial().kind(),
        WorthQueryConvergenceEpochDenialKind::ConvergenceProviderFamilyInspectionPanicked
    );
    assert_eq!(
        rejection.denial().counters().graph_authority_check_count(),
        1
    );
    let (_, managed, _) = rejection.into_parts();
    let terminal = managed
        .start()
        .terminate_for_convergence(WorthQueryManagedRunTerminalKind::Failed);
    assert_eq!(terminal.provider_work().provider_step_attempt_count(), 0);
    let cleanup = match terminal.cleanup() {
        Ok(cleanup) => cleanup,
        Err(_) => panic!("rejected convergence admission must return managed cleanup authority"),
    };
    assert_eq!(
        cleanup.disposition(),
        WorthQueryManagedRunCleanupDisposition::CleanupComplete
    );
}

fn assert_domain_invocation_cause(
    cause: Option<&WorthQueryConvergenceIndeterminateCause>,
    expected_phase: Phase,
    expected_kind: FailureKind,
    expected_work: [usize; 3],
) {
    let Some(WorthQueryConvergenceIndeterminateCause::DomainInvocation(failure)) = cause else {
        panic!("indeterminate terminal must carry typed domain invocation failure");
    };
    assert_eq!(failure.phase(), expected_phase);
    assert_eq!(failure.kind(), expected_kind);
    assert_eq!(
        [
            failure.work().comparator_call_count(),
            failure.work().progress_check_count(),
            failure.work().repeated_state_probe_count(),
        ],
        expected_work
    );
}
