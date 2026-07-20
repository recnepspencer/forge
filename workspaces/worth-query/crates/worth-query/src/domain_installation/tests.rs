use crate::application::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainEntryMarker,
};
use crate::authoring::RelationName;
use crate::runtime::WorthQueryGraphReadTraversalOperator;

use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TestDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CapabilityDomain;

impl WorthQueryDomainEntryMarker for TestDomain {
    fn domain_key(&self) -> &'static str {
        "WORTH.tests.installed-domain"
    }

    fn display_name(&self) -> &'static str {
        "TestDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[]
    }
}

impl WorthQueryDomainEntryMarker for CapabilityDomain {
    fn domain_key(&self) -> &'static str {
        "WORTH.tests.capability-domain"
    }

    fn display_name(&self) -> &'static str {
        "CapabilityDomain"
    }

    fn required_capability_families(&self) -> &'static [WorthQueryCapabilityFamily] {
        &[WorthQueryCapabilityFamily::QueryRead]
    }
}

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
    let admitted = admit_domain_package(package(false).validate().unwrap()).unwrap();
    assert_eq!(admitted.graph_read_operations.len(), 2);
    assert!(!admitted.package_identity.as_str().is_empty());
    assert!(!admitted.admission_identity.as_str().is_empty());
}

#[test]
fn marker_identity_mismatch_denies_before_package_identity_is_minted() {
    let mismatched = WorthQueryDomainPackage::declare(
        TestDomain,
        WorthQueryDomainIdentityDeclaration::new(
            WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            WorthQueryDomainIdentityName::new("different-domain").unwrap(),
            WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .validate()
    .err()
    .expect("typed marker and canonical identity must describe one domain");

    assert_eq!(
        mismatched.kind(),
        WorthQueryDomainPackageValidationDenialKind::MarkerIdentityMismatch
    );
    assert!(mismatched.detail().contains(TestDomain.domain_key()));
}

#[test]
fn marker_required_capability_must_be_declared_by_the_package() {
    let denial = WorthQueryDomainPackage::declare(
        CapabilityDomain,
        WorthQueryDomainIdentityDeclaration::new(
            WorthQueryDomainIdentityNamespace::new("WORTH.tests").unwrap(),
            WorthQueryDomainIdentityName::new("capability-domain").unwrap(),
            WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .validate()
    .err()
    .expect("package cannot omit capability meaning declared by its typed marker");

    assert_eq!(
        denial.kind(),
        WorthQueryDomainPackageValidationDenialKind::MissingMarkerCapability
    );
    assert!(denial.detail().contains("query_read"));
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
