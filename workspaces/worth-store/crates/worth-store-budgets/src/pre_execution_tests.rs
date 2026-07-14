use crate::{
    pre_execution_budget_admission, PreExecutionBudgetAdmissionView, PreExecutionBudgetDenial,
    PreExecutionBudgetEnvelope, PreExecutionBudgetRequest, PreExecutionBudgetScope,
};

fn request(scope: PreExecutionBudgetScope) -> PreExecutionBudgetRequest {
    PreExecutionBudgetRequest::new(scope, 1_024, 1, 0, 1, 4_096)
}

#[test]
fn budget_admission_issues_resource_authority_without_plan_identity() {
    let outcome = pre_execution_budget_admission().admit(
        request(PreExecutionBudgetScope::Foreground),
        PreExecutionBudgetEnvelope::foreground_default(),
    );
    let receipt = outcome
        .into_result()
        .expect("bounded foreground work must be admitted");
    assert_eq!(receipt.scope(), PreExecutionBudgetScope::Foreground);
    assert_eq!(
        receipt.request(),
        request(PreExecutionBudgetScope::Foreground)
    );
    assert_eq!(
        receipt.admitted_envelope(),
        PreExecutionBudgetEnvelope::foreground_default()
    );
}

#[test]
fn budget_admission_rejects_scope_and_exact_resource_overages() {
    let mismatch = pre_execution_budget_admission().admit(
        request(PreExecutionBudgetScope::Streaming),
        PreExecutionBudgetEnvelope::foreground_default(),
    );
    assert!(matches!(
        mismatch.view(),
        PreExecutionBudgetAdmissionView::Denied(PreExecutionBudgetDenial::ScopeMismatch { .. })
    ));

    let excessive =
        PreExecutionBudgetRequest::new(PreExecutionBudgetScope::Foreground, 1_024, 9, 0, 1, 4_096);
    assert!(matches!(
        pre_execution_budget_admission()
            .admit(excessive, PreExecutionBudgetEnvelope::foreground_default(),)
            .view(),
        PreExecutionBudgetAdmissionView::Denied(PreExecutionBudgetDenial::PageReadsExceeded {
            estimated: 9,
            admitted: 8
        })
    ));
}

#[test]
fn admitted_resource_grants_preserve_scope_without_claiming_operation_authority() {
    let foreground = pre_execution_budget_admission()
        .admit(
            request(PreExecutionBudgetScope::Foreground),
            PreExecutionBudgetEnvelope::foreground_default(),
        )
        .into_result()
        .expect("foreground budget");
    assert_eq!(foreground.scope(), PreExecutionBudgetScope::Foreground);

    let maintenance = pre_execution_budget_admission()
        .admit(
            request(PreExecutionBudgetScope::Maintenance),
            PreExecutionBudgetEnvelope::maintenance_default(),
        )
        .into_result()
        .expect("maintenance budget");
    assert_eq!(maintenance.scope(), PreExecutionBudgetScope::Maintenance);
}

#[test]
fn budget_receipt_retains_the_exact_admitted_demand() {
    let exact =
        PreExecutionBudgetRequest::new(PreExecutionBudgetScope::Foreground, 2_048, 2, 0, 2, 8_192);
    let different =
        PreExecutionBudgetRequest::new(PreExecutionBudgetScope::Foreground, 1_024, 1, 0, 1, 4_096);
    let receipt = pre_execution_budget_admission()
        .admit(exact, PreExecutionBudgetEnvelope::foreground_default())
        .into_result()
        .expect("exact demand must fit the foreground envelope");

    assert_eq!(receipt.request(), exact);
    assert_ne!(receipt.request(), different);
}
