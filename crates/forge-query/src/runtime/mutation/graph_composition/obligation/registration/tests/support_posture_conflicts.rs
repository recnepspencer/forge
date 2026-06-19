use crate::runtime::{
    ForgeQueryGraphObligationBudgetExceededPolicy, ForgeQueryGraphObligationExecutionBudget,
    ForgeQueryGraphObligationExecutionScope, ForgeQueryGraphObligationOperatingWorldSelector,
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphObligationRegistrationCatalog,
    ForgeQueryGraphObligationRegistrationDenialKind, ForgeQueryGraphObligationRuleIdentity,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportPosture,
    ForgeQueryGraphTouchSelector,
};

#[test]
fn contradictory_support_posture_for_same_registration_slot_is_denied() {
    let left = registration_with_posture(ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::GraphComposition,
    ));
    let right =
        registration_with_posture(ForgeQueryGraphObligationSupportPosture::diagnostic_only(
            ForgeQueryGraphObligationSupportLane::GraphComposition,
        ));

    let denial =
        ForgeQueryGraphObligationRegistrationCatalog::from_registrations(vec![left, right])
            .unwrap_err();

    assert_eq!(
        denial.kind(),
        &ForgeQueryGraphObligationRegistrationDenialKind::ConflictingRegistrationForRule
    );
}

#[test]
fn contradictory_budget_for_same_registration_slot_is_denied() {
    let left = registration_with_budget(ForgeQueryGraphObligationExecutionBudget::bounded_sparse(
        ForgeQueryGraphObligationExecutionScope::TouchedRelationKind,
        ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed,
    ));
    let right = registration_with_budget(ForgeQueryGraphObligationExecutionBudget::bounded_sparse(
        ForgeQueryGraphObligationExecutionScope::TouchedRelationKind,
        ForgeQueryGraphObligationBudgetExceededPolicy::Advisory,
    ));

    let denial =
        ForgeQueryGraphObligationRegistrationCatalog::from_registrations(vec![left, right])
            .unwrap_err();

    assert_eq!(
        denial.kind(),
        &ForgeQueryGraphObligationRegistrationDenialKind::ConflictingRegistrationForRule
    );
}

#[test]
fn support_posture_and_budget_participate_in_registration_identity() {
    let supported = registration_with_posture(ForgeQueryGraphObligationSupportPosture::supported(
        ForgeQueryGraphObligationSupportLane::GraphComposition,
    ));
    let diagnostic =
        registration_with_posture(ForgeQueryGraphObligationSupportPosture::diagnostic_only(
            ForgeQueryGraphObligationSupportLane::GraphComposition,
        ));
    let capped = registration_with_budget(
        ForgeQueryGraphObligationExecutionBudget::bounded_sparse(
            ForgeQueryGraphObligationExecutionScope::TouchedRelationKind,
            ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed,
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
    support_posture: ForgeQueryGraphObligationSupportPosture,
) -> ForgeQueryGraphObligationRegistration {
    base_registration().with_support_posture(support_posture)
}

fn registration_with_budget(
    execution_budget: ForgeQueryGraphObligationExecutionBudget,
) -> ForgeQueryGraphObligationRegistration {
    base_registration().with_execution_budget(execution_budget)
}

fn base_registration() -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::blocking_invariant(
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap(),
        ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
}
