use crate::runtime::{
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRegistrationCatalog, WorthQueryGraphObligationRegistrationDenialKind,
    WorthQueryGraphObligationRuleIdentity, WorthQueryGraphTouchSelector,
};

use super::fixtures::registration;

#[test]
fn exact_duplicate_registrations_canonicalize_to_one_row() {
    let row = registration(
        "loop-wiring",
        WorthQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
    );
    let catalog =
        WorthQueryGraphObligationRegistrationCatalog::from_registrations(vec![row.clone(), row])
            .unwrap();

    assert_eq!(catalog.registration_count(), 1);
}

#[test]
fn conflicting_registration_for_same_rule_identity_is_denied() {
    let rule_identity =
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap();
    let left = WorthQueryGraphObligationRegistration::schema_contract_validator(
        rule_identity.clone(),
        WorthQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let right = WorthQueryGraphObligationRegistration::blocking_invariant(
        rule_identity,
        WorthQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );

    let denial =
        WorthQueryGraphObligationRegistrationCatalog::from_registrations(vec![left, right])
            .unwrap_err();

    assert_eq!(
        denial.kind(),
        &WorthQueryGraphObligationRegistrationDenialKind::ConflictingRegistrationForRule
    );
    assert!(denial.message().contains("conflicting registrations"));
}

#[test]
fn same_rule_identity_can_register_distinct_operating_worlds() {
    let rule_identity =
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap();
    let selector = WorthQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap();
    let committed = WorthQueryGraphObligationRegistration::schema_contract_validator(
        rule_identity.clone(),
        selector.clone(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let preview = WorthQueryGraphObligationRegistration::schema_contract_validator(
        rule_identity,
        selector,
        WorthQueryGraphObligationOperatingWorldSelector::preview(),
    );

    let catalog =
        WorthQueryGraphObligationRegistrationCatalog::from_registrations(vec![preview, committed])
            .unwrap();

    assert_eq!(catalog.registration_count(), 2);
}

#[test]
fn same_rule_identity_can_register_distinct_touch_selectors() {
    let rule_identity =
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", "loop-wiring", "v1")
            .unwrap();
    let loop_successor = WorthQueryGraphObligationRegistration::schema_contract_validator(
        rule_identity.clone(),
        WorthQueryGraphTouchSelector::relation_kind("topology.loop_successor").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );
    let shell_membership = WorthQueryGraphObligationRegistration::schema_contract_validator(
        rule_identity,
        WorthQueryGraphTouchSelector::relation_kind("topology.shell_membership").unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    );

    let catalog = WorthQueryGraphObligationRegistrationCatalog::from_registrations(vec![
        shell_membership,
        loop_successor,
    ])
    .unwrap();

    assert_eq!(catalog.registration_count(), 2);
}
