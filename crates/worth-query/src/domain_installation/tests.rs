use crate::application::{
    WorthQueryApplicationFacade, WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily,
};
use crate::authoring::RelationName;
use crate::runtime::WorthQueryGraphReadTraversalOperator;

use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TestDomain;

fn identity() -> WorthQueryDomainIdentityDeclaration<TestDomain> {
    WorthQueryDomainIdentityDeclaration::new(
        WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
        WorthQueryDomainIdentityName::new("installed-domain").unwrap(),
        WorthQueryDomainSemanticVersion::new(1, 0),
    )
}

fn operation(name: &str, relation: &str) -> WorthQueryDomainGraphReadOperationDefinition {
    WorthQueryDomainGraphReadOperationDefinition::new(
        WorthQueryDomainIdentityName::new(name).unwrap(),
        1,
    )
    .accepts_relation(RelationName::new(relation).unwrap())
    .lowers_to(WorthQueryGraphReadTraversalOperator::DeclarationTraversal)
}

fn package(reversed: bool) -> WorthQueryDomainPackage<TestDomain> {
    let first = operation("neighbors", "adjacent_to");
    let second = operation("incidences", "incident_to");
    let package = WorthQueryDomainPackage::declare(TestDomain, identity())
        .requires_capability(WorthQueryCapabilityFamily::QueryRead)
        .requires_configuration(WorthQueryConfigSectionFamily::Query)
        .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::Admission);
    if reversed {
        package
            .graph_read_operation(second)
            .graph_read_operation(first)
    } else {
        package
            .graph_read_operation(first)
            .graph_read_operation(second)
    }
}

#[test]
fn package_identity_is_insertion_order_independent() {
    let left = package(false).validate().unwrap();
    let right = package(true).validate().unwrap();
    assert_eq!(left.identity(), right.identity());
}

#[test]
fn conflicting_operation_slot_denies_without_validated_successor() {
    let denial = WorthQueryDomainPackage::declare(TestDomain, identity())
        .graph_read_operation(operation("neighbors", "adjacent_to"))
        .graph_read_operation(operation("neighbors", "incident_to"))
        .validate()
        .err()
        .expect("conflicting operation definitions must deny");
    assert_eq!(
        denial.kind(),
        WorthQueryDomainPackageValidationDenialKind::ConflictingGraphReadOperation
    );
}

#[test]
fn admitted_package_retains_canonical_identity_and_support_proof() {
    let admitted = package(false)
        .validate()
        .unwrap()
        .admit(&WorthQueryApplicationFacade::runtime_backed_default())
        .unwrap();
    assert_eq!(admitted.graph_read_operation_count(), 2);
    assert!(!admitted.identity().as_str().is_empty());
    assert!(!admitted.admission_identity().is_empty());
}

#[test]
fn identity_components_reject_representation_smuggling() {
    assert_eq!(
        WorthQueryDomainIdentityNamespace::new("WORTH/tests"),
        Err(WorthQueryDomainIdentityComponentError::InvalidCharacter)
    );
    assert_eq!(
        WorthQueryDomainIdentityName::new("digest:abc"),
        Err(WorthQueryDomainIdentityComponentError::InvalidCharacter)
    );
}
