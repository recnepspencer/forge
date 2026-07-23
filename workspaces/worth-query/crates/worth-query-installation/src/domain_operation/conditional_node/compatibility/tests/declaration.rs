use worth_foundational::facade::{CanonicalMismatchKind, CanonicalizationRuleVersion};

use super::super::*;
use super::fixture::{dependency_node, operation_node, TriggerA, TriggerB};
use crate::domain_operation::{
    WorthQueryComparatorRequirement, WorthQueryPortableConditionalDependencyLocation,
    WorthQueryPortableConditionalDependencyPart, WorthQueryPortableConditionalDimension,
};

#[test]
fn equivalent_declarations_retain_foundational_equivalence_evidence() {
    let left =
        operation_node::<TriggerA>("gate", WorthQueryComparatorRequirement::ExactCanonicalValue);
    let right = left.clone();
    let WorthQueryPortableConditionalComparisonOutcome::Equivalent(equivalent) =
        compare_portable_conditional_node_declarations(&left, &right)
    else {
        panic!("identical typed declarations must be canonically equivalent");
    };

    assert!(equivalent.comparison_count() > 10);
    assert!(equivalent.foundational_bases().iter().all(|basis| {
        basis.equivalence_basis()
            == worth_foundational::facade::CanonicalEquivalenceBasis::ExactCanonicalBasis
    }));
}

#[test]
fn one_field_trigger_drift_reports_the_owner_dimension() {
    let left =
        operation_node::<TriggerA>("gate", WorthQueryComparatorRequirement::ExactCanonicalValue);
    let right =
        operation_node::<TriggerB>("gate", WorthQueryComparatorRequirement::ExactCanonicalValue);
    let WorthQueryPortableConditionalComparisonOutcome::Mismatched(mismatch) =
        compare_portable_conditional_node_declarations(&left, &right)
    else {
        panic!("trigger owner drift must mismatch");
    };

    assert_eq!(
        mismatch.dimension(),
        &WorthQueryPortableConditionalDimension::Trigger
    );
    assert_eq!(
        mismatch.foundational_basis().kind(),
        CanonicalMismatchKind::ValueMismatch
    );
}

#[test]
fn comparator_drift_cannot_hide_behind_equal_node_identity() {
    let left =
        operation_node::<TriggerA>("gate", WorthQueryComparatorRequirement::ExactCanonicalValue);
    let right = operation_node::<TriggerA>(
        "gate",
        WorthQueryComparatorRequirement::FoundationalContractEquivalence,
    );
    let WorthQueryPortableConditionalComparisonOutcome::Mismatched(mismatch) =
        compare_portable_conditional_node_declarations(&left, &right)
    else {
        panic!("comparator drift must mismatch");
    };
    assert_eq!(
        mismatch.dimension(),
        &WorthQueryPortableConditionalDimension::DependencyComparator
    );
}

#[test]
fn foundational_contract_drift_reports_the_exact_dependency_part() {
    let left = dependency_node(1);
    let right = dependency_node(2);
    let WorthQueryPortableConditionalComparisonOutcome::Mismatched(mismatch) =
        compare_portable_conditional_node_declarations(&left, &right)
    else {
        panic!("dependency contract revision drift must mismatch");
    };
    assert_eq!(
        mismatch.dimension(),
        &WorthQueryPortableConditionalDimension::Dependency {
            location: WorthQueryPortableConditionalDependencyLocation::Declaration(0),
            part: WorthQueryPortableConditionalDependencyPart::Contract,
        }
    );
}

#[test]
fn unsupported_foundational_rule_versions_keep_their_exact_category() {
    let declaration =
        operation_node::<TriggerA>("gate", WorthQueryComparatorRequirement::ExactCanonicalValue);
    let left = CanonicalizationRuleVersion::new("conditional-test-v1").unwrap();
    let right = CanonicalizationRuleVersion::new("conditional-test-v2").unwrap();
    let WorthQueryPortableConditionalComparisonOutcome::Unsupported(unsupported) =
        compare_with_versions(&declaration, left, &declaration, right)
    else {
        panic!("different canonical rule versions must remain unsupported");
    };
    assert_eq!(
        unsupported.foundational_basis().kind(),
        CanonicalMismatchKind::VersionMismatch
    );
}

#[test]
fn canonical_material_tracks_complete_owner_meaning() {
    let left =
        operation_node::<TriggerA>("gate", WorthQueryComparatorRequirement::ExactCanonicalValue);
    let right =
        operation_node::<TriggerB>("gate", WorthQueryComparatorRequirement::ExactCanonicalValue);
    assert_ne!(
        portable_conditional_node_canonical_material(&left),
        portable_conditional_node_canonical_material(&right)
    );
}
