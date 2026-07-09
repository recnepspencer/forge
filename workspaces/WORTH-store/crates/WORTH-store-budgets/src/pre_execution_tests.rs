use crate::{
    pre_execution_budget_admission, S8PreExecutionBudgetAdmissionOutcome,
    S8PreExecutionBudgetDenial, S8PreExecutionBudgetEnvelope, S8PreExecutionBudgetRequest,
    S8PreExecutionBudgetScope, S8PreExecutionPlanBinding,
};

#[test]
fn pre_execution_budget_admission_accepts_foreground_request_within_envelope() {
    let request = S8PreExecutionBudgetRequest::new(
        S8PreExecutionPlanBinding::new(1, 2, 3, 4, 0),
        S8PreExecutionBudgetScope::Foreground,
        4_096,
        2,
        0,
        4,
        16_384,
    );

    let admitted = pre_execution_budget_admission()
        .admit(request, S8PreExecutionBudgetEnvelope::foreground_default());

    assert!(matches!(
        admitted,
        S8PreExecutionBudgetAdmissionOutcome::Admitted(_)
    ));
    let admitted = match admitted {
        S8PreExecutionBudgetAdmissionOutcome::Admitted(receipt) => receipt,
        S8PreExecutionBudgetAdmissionOutcome::Denied(_) => unreachable!(),
    };
    assert_eq!(
        admitted.plan_binding(),
        S8PreExecutionPlanBinding::new(1, 2, 3, 4, 0)
    );
}

#[test]
fn pre_execution_budget_admission_denies_when_range_budget_is_exceeded() {
    let request = S8PreExecutionBudgetRequest::new(
        S8PreExecutionPlanBinding::new(5, 6, 7, 8, 0),
        S8PreExecutionBudgetScope::Foreground,
        4_096,
        2,
        0,
        10_000,
        16_384,
    );

    let denied = pre_execution_budget_admission()
        .admit(request, S8PreExecutionBudgetEnvelope::foreground_default());

    assert_eq!(
        denied,
        S8PreExecutionBudgetAdmissionOutcome::Denied(
            S8PreExecutionBudgetDenial::RangeTouchesExceeded {
                estimated: 10_000,
                admitted: 16,
            },
        )
    );
}

#[test]
fn pre_execution_budget_admission_denies_when_scope_does_not_match_envelope() {
    let request = S8PreExecutionBudgetRequest::new(
        S8PreExecutionPlanBinding::new(9, 10, 11, 12, 0),
        S8PreExecutionBudgetScope::Streaming,
        4_096,
        2,
        4,
        4,
        16_384,
    );

    let denied = pre_execution_budget_admission()
        .admit(request, S8PreExecutionBudgetEnvelope::foreground_default());

    assert_eq!(
        denied,
        S8PreExecutionBudgetAdmissionOutcome::Denied(S8PreExecutionBudgetDenial::ScopeMismatch {
            requested: S8PreExecutionBudgetScope::Streaming,
            admitted: S8PreExecutionBudgetScope::Foreground,
        })
    );
}
