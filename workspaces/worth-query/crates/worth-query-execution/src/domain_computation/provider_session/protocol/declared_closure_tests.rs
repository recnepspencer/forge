use worth_query_installation::facade::{
    WorthQueryAftermathPostcondition, WorthQueryDomainOperationIdentity,
    WorthQueryOperationReversalContract,
};

use super::declared_closure::{
    bind_direct_role_closure, reversal_posture, WorthQueryProviderPlanDeclaredClosure,
};

#[test]
fn read_only_roles_receive_no_effect_authority() {
    let effects = vec!["mutation".to_owned()];
    let invariants = vec!["closed-loop".to_owned()];
    let mut read_only = WorthQueryProviderPlanDeclaredClosure {
        read: vec!["read-only:observe".to_owned()],
        ..WorthQueryProviderPlanDeclaredClosure::default()
    };
    bind_direct_role_closure(&mut read_only, &effects, &invariants);
    assert!(read_only.effect.is_empty());

    let mut touched = WorthQueryProviderPlanDeclaredClosure {
        read: vec!["owner:observe".to_owned()],
        touch: vec!["owner".to_owned()],
        ..WorthQueryProviderPlanDeclaredClosure::default()
    };
    bind_direct_role_closure(&mut touched, &effects, &invariants);
    assert_eq!(touched.effect, effects);
}

#[test]
fn reversal_posture_retains_the_exact_declared_recovery_contract() {
    assert_eq!(
        reversal_posture(&WorthQueryOperationReversalContract::ExactInverse {
            lowering_family: "inverse-v2".to_owned(),
        }),
        "exact-inverse:inverse-v2"
    );
    assert_eq!(
        reversal_posture(
            &WorthQueryOperationReversalContract::CompensationWithPostcondition {
                operation: WorthQueryDomainOperationIdentity::new("undo-charge", 3),
                postcondition: WorthQueryAftermathPostcondition::InvariantRestored {
                    invariant: "balanced-ledger".to_owned(),
                },
            }
        ),
        "compensation:undo-charge:3:invariant-restored:balanced-ledger"
    );
}
