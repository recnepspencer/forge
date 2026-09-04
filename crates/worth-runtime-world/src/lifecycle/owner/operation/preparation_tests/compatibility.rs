use super::super::preparation_test_support::{reservation_counts, retained_relational_plan, setup};
use crate::publication::{
    CompositeComponentIntent, NoEffectCause, RuntimeWorldCancellationSource,
    SignalComponentPlanPosture,
};

use worth_relational::facade::mvcc::RelationalTransactionIntent;

/// A plan whose per-owner posture contradicts the component intent it was
/// admitted for is refused at the reservation boundary, before any identity is
/// issued or any bounded capacity is charged.
#[test]
fn incompatible_posture_and_intent_is_rejected_before_reservation() {
    for intent in [
        CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        CompositeComponentIntent::relational_and_signal(RelationalTransactionIntent::ordinary()),
    ] {
        let (owner, expected) = setup(2);
        let changes_signal = intent.changes_signal();
        let signal = if changes_signal {
            SignalComponentPlanPosture::AdvanceExact
        } else {
            SignalComponentPlanPosture::RetainExact
        };
        // The Signal leg agrees with the intent; only the Relational leg
        // retains a component the intent declares as changing.
        let plan = retained_relational_plan(owner.as_ref(), &expected, intent, signal);
        let denied = owner
            .reserve(plan, &RuntimeWorldCancellationSource::new().token(), None)
            .expect_err("a retained Relational leg cannot carry a Relational change");
        assert_eq!(denied.cause(), NoEffectCause::PreEffectFailure);
        assert_eq!(denied.expected_head(), Some(&expected));
        assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
        assert_eq!(owner.state.operation.active(), 0);
    }
}

/// Both components retained is denied before any reservation. Compatibility is
/// decided per component against the admitted intent, and no
/// `CompositeComponentIntent` leaves both components unchanged, so a
/// Retain/Retain pair can never be reserved however it was assembled.
#[test]
fn retain_retain_publication_is_denied_before_any_reservation() {
    let (owner, expected) = setup(2);
    let plan = retained_relational_plan(
        owner.as_ref(),
        &expected,
        CompositeComponentIntent::signal_only(),
        SignalComponentPlanPosture::RetainExact,
    );
    assert_eq!(
        plan.relational().posture(),
        crate::publication::RelationalComponentPlanPosture::RetainExact
    );
    assert_eq!(
        plan.signal().posture(),
        SignalComponentPlanPosture::RetainExact
    );
    let denied = owner
        .reserve(plan, &RuntimeWorldCancellationSource::new().token(), None)
        .expect_err("a publication that moves neither owner has nothing to publish");
    assert_eq!(denied.cause(), NoEffectCause::PreEffectFailure);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
    assert_eq!(owner.state.operation.active(), 0);
}
