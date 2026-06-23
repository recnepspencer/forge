use crate::runtime::{
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryGraphObligationRegistrationDenialKind,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphTouchSelector,
};

use super::fixtures::registration;

#[test]
fn exact_duplicate_registrations_canonicalize_to_one_row() {
    let row = registration(
        "loop-wiring",
        ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
    );
    let catalog =
        ForgeQueryGraphObligationRegistrationCatalog::from_registrations(vec![row.clone(), row])
            .unwrap();

    assert_eq!(catalog.registration_count(), 1);
}

#[test]
fn conflicting_registration_for_same_rule_identity_is_denied() {
    let rule_identity =
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap();
    let left = ForgeQueryGraphObligationRegistration::schema_contract_validator(
        rule_identity.clone(),
        ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let right = ForgeQueryGraphObligationRegistration::blocking_invariant(
        rule_identity,
        ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );

    let denial =
        ForgeQueryGraphObligationRegistrationCatalog::from_registrations(vec![left, right])
            .unwrap_err();

    assert_eq!(
        denial.kind(),
        &ForgeQueryGraphObligationRegistrationDenialKind::ConflictingRegistrationForRule
    );
    assert!(denial.message().contains("conflicting registrations"));
}

#[test]
fn same_rule_identity_can_register_distinct_operating_worlds() {
    let rule_identity =
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap();
    let selector = ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap();
    let committed = ForgeQueryGraphObligationRegistration::schema_contract_validator(
        rule_identity.clone(),
        selector.clone(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let preview = ForgeQueryGraphObligationRegistration::schema_contract_validator(
        rule_identity,
        selector,
        ForgeQueryGraphObligationOperatingWorldSelector::preview(),
    );

    let catalog =
        ForgeQueryGraphObligationRegistrationCatalog::from_registrations(vec![preview, committed])
            .unwrap();

    assert_eq!(catalog.registration_count(), 2);
}

#[test]
fn same_rule_identity_can_register_distinct_touch_selectors() {
    let rule_identity =
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap();
    let loop_successor = ForgeQueryGraphObligationRegistration::schema_contract_validator(
        rule_identity.clone(),
        ForgeQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let shell_membership = ForgeQueryGraphObligationRegistration::schema_contract_validator(
        rule_identity,
        ForgeQueryGraphTouchSelector::relation_kind("topology.shell_membership").unwrap(),
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );

    let catalog = ForgeQueryGraphObligationRegistrationCatalog::from_registrations(vec![
        shell_membership,
        loop_successor,
    ])
    .unwrap();

    assert_eq!(catalog.registration_count(), 2);
}
