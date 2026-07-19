use crate::runtime::{
    WorthQueryGraphObligationBudgetExceededPolicy, WorthQueryGraphObligationExecutionBudget,
    WorthQueryGraphObligationExecutionScope, WorthQueryGraphObligationOperatingWorldSelector,
    WorthQueryGraphObligationRegistration, WorthQueryGraphObligationRegistrationCatalog,
    WorthQueryGraphObligationRegistrationDenialKind, WorthQueryGraphObligationRuleIdentity,
    WorthQueryGraphObligationSupportLane, WorthQueryGraphObligationSupportPosture,
    WorthQueryGraphTouchSelector,
};

#[test]
fn contradictory_support_posture_for_same_registration_slot_is_denied() {
    let left = registration_with_posture(WorthQueryGraphObligationSupportPosture::supported(
        WorthQueryGraphObligationSupportLane::GraphComposition,
    ));
    let right =
        registration_with_posture(WorthQueryGraphObligationSupportPosture::diagnostic_only(
            WorthQueryGraphObligationSupportLane::GraphComposition,
        ));

    let denial =
        WorthQueryGraphObligationRegistrationCatalog::from_registrations(vec![left, right])
            .unwrap_err();

    assert_eq!(
        denial.kind(),
        &WorthQueryGraphObligationRegistrationDenialKind::ConflictingRegistrationForRule
    );
}

#[test]
fn contradictory_budget_for_same_registration_slot_is_denied() {
    let left = registration_with_budget(WorthQueryGraphObligationExecutionBudget::bounded_sparse(
        WorthQueryGraphObligationExecutionScope::TouchedRelationKind,
        WorthQueryGraphObligationBudgetExceededPolicy::FailClosed,
    ));
    let right = registration_with_budget(WorthQueryGraphObligationExecutionBudget::bounded_sparse(
        WorthQueryGraphObligationExecutionScope::TouchedRelationKind,
        WorthQueryGraphObligationBudgetExceededPolicy::Advisory,
    ));

    let denial =
        WorthQueryGraphObligationRegistrationCatalog::from_registrations(vec![left, right])
            .unwrap_err();

    assert_eq!(
        denial.kind(),
        &WorthQueryGraphObligationRegistrationDenialKind::ConflictingRegistrationForRule
    );
}

#[test]
fn support_posture_and_budget_participate_in_registration_identity() {
    let supported = registration_with_posture(WorthQueryGraphObligationSupportPosture::supported(
        WorthQueryGraphObligationSupportLane::GraphComposition,
    ));
    let diagnostic =
        registration_with_posture(WorthQueryGraphObligationSupportPosture::diagnostic_only(
            WorthQueryGraphObligationSupportLane::GraphComposition,
        ));
    let capped = registration_with_budget(
        WorthQueryGraphObligationExecutionBudget::bounded_sparse(
            WorthQueryGraphObligationExecutionScope::TouchedRelationKind,
            WorthQueryGraphObligationBudgetExceededPolicy::FailClosed,
        )
        .with_max_state_scope(4),
    );

    assert_ne!(
        supported.registration_digest(),
        diagnostic.registration_digest()
    );
    assert_ne!(
        supported.registration_digest(),
        capped.registration_digest()
    );
}

fn registration_with_posture(
    support_posture: WorthQueryGraphObligationSupportPosture,
) -> WorthQueryGraphObligationRegistration {
    base_registration().with_support_posture(support_posture)
}

fn registration_with_budget(
    execution_budget: WorthQueryGraphObligationExecutionBudget,
) -> WorthQueryGraphObligationRegistration {
    base_registration().with_execution_budget(execution_budget)
}

fn base_registration() -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::blocking_invariant(
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap(),
        WorthQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
}
