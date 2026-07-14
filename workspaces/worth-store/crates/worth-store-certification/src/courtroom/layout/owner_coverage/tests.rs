use super::*;

#[test]
fn canonical_registry_contains_every_owner_family_once() {
    let declarations = LayoutOwnerCaseDeclarations::from_owner_inventories();
    assert_eq!(
        declarations.families().count(),
        LayoutOwnerFamily::all().len()
    );
    assert!(LayoutOwnerFamily::all()
        .into_iter()
        .all(|family| !declarations.cases(family).is_empty()));
}

#[test]
fn empty_observation_ledger_reports_every_declared_case_missing() {
    let declarations = LayoutOwnerCaseDeclarations::from_owner_inventories();
    let denial =
        require_exact_owner_case_coverage(&declarations, &LayoutOwnerObservationLedger::default())
            .unwrap_err();
    let expected = LayoutOwnerFamily::all()
        .into_iter()
        .map(|family| declarations.cases(family).len())
        .sum::<usize>();
    assert_eq!(denial.issues().len(), expected);
    assert!(denial
        .issues()
        .iter()
        .all(|issue| matches!(issue, LayoutOwnerCoverageIssue::Missing { .. })));
}
