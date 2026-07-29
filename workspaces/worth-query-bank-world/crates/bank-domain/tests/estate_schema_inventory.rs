use std::collections::BTreeSet;

use bank_domain::schema::BankSchema;
use worth_query_decl::facade::application_schema::ApplicationSchemaMember;

#[test]
fn required_estate_topology_and_policy_contributions_are_present() {
    let declaration = BankSchema::declaration().unwrap();
    let members = declaration.erased().members();
    assert_required_subset_present(
        members.iter().filter_map(entity_name).collect(),
        estate_entities(),
    );
    assert_required_subset_present(
        members.iter().filter_map(relation_name).collect(),
        estate_relations(),
    );
    assert_required_subset_present(
        members.iter().filter_map(policy_name).collect(),
        estate_policies(),
    );
}

#[test]
fn estate_commands_wait_for_runtime_capability_installation_without_local_workaround() {
    let declaration = BankSchema::declaration().unwrap();
    let operations = declaration
        .erased()
        .members()
        .iter()
        .filter_map(operation_name);
    assert!(
        operations.into_iter().all(|operation| {
            !operation.contains("Estate")
                && !operation.contains("Emergency")
                && !operation.contains("Death")
        }),
        "estate commands must not become executable before Runtime Phase 7 enforcement"
    );

    let sources = [
        include_str!("../src/estate/mod.rs"),
        include_str!("../src/estate/identities.rs"),
        include_str!("../src/estate/capability.rs"),
        include_str!("../src/estate/disclosure.rs"),
        include_str!("../src/estate/emergency_access.rs"),
        include_str!("../src/estate/operations.rs"),
        include_str!("../src/estate/aftermath.rs"),
        include_str!("../src/estate/world.rs"),
        include_str!("../src/estate/oracles/mod.rs"),
        include_str!("../src/estate/oracles/accounting.rs"),
        include_str!("../src/estate/oracles/capability.rs"),
        include_str!("../src/estate/oracles/conflict.rs"),
        include_str!("../src/estate/oracles/disclosure.rs"),
        include_str!("../src/estate/oracles/integrity.rs"),
        include_str!("../src/estate/oracles/role.rs"),
        include_str!("../src/schema/estate/mod.rs"),
        include_str!("../src/schema/estate/authority_relation_installation.rs"),
        include_str!("../src/schema/estate/capability_installation.rs"),
        include_str!("../src/schema/estate/entities.rs"),
        include_str!("../src/schema/estate/estate_relation_installation.rs"),
        include_str!("../src/schema/estate/fields.rs"),
        include_str!("../src/schema/estate/member_installation.rs"),
        include_str!("../src/schema/estate/policies.rs"),
        include_str!("../src/schema/estate/policy_installation.rs"),
        include_str!("../src/schema/estate/relations.rs"),
        include_str!("../src/schema/estate/values.rs"),
    ]
    .join("\n");
    for forbidden in [
        "superuser",
        "redact",
        "HashMap",
        "dyn Fn",
        "undo_stack",
        "route_predicate",
    ] {
        assert!(
            !sources.contains(forbidden),
            "estate world contains forbidden local authority lane: {forbidden}"
        );
    }
}

fn assert_required_subset_present(actual: BTreeSet<&str>, expected: BTreeSet<&str>) {
    assert_eq!(
        actual
            .intersection(&expected)
            .copied()
            .collect::<BTreeSet<_>>(),
        expected
    );
}

fn entity_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Entity { entity } => Some(entity),
        _ => None,
    }
}

fn relation_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Relation { relation, .. } => Some(relation),
        _ => None,
    }
}

fn policy_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Policy { policy } => Some(policy),
        _ => None,
    }
}

fn operation_name(member: &ApplicationSchemaMember) -> Option<&str> {
    match member {
        ApplicationSchemaMember::Operation { operation, .. } => Some(operation),
        _ => None,
    }
}

fn estate_entities() -> BTreeSet<&'static str> {
    [
        "Branch",
        "CapabilityGrant",
        "DeathNotice",
        "EmergencyAccess",
        "EstateCase",
        "LegalAuthority",
        "MandatoryReview",
    ]
    .into_iter()
    .collect()
}

fn estate_relations() -> BTreeSet<&'static str> {
    [
        "BranchInstitution",
        "CapabilityAccount",
        "CapabilityBranch",
        "CapabilityEstate",
        "CapabilityGrantee",
        "CapabilityGrantor",
        "CapabilityInstitution",
        "CapabilityParent",
        "DeathNoticeSubject",
        "EmergencyApprover",
        "EmergencyGrant",
        "EmergencyRequester",
        "EmergencyReview",
        "EstateAccount",
        "EstateAssignment",
        "EstateAuthorizedSigner",
        "EstateBeneficiary",
        "EstateBranch",
        "EstateDeathNotice",
        "EstateDeceased",
        "EstateExecutor",
        "EstateJointOwner",
        "LegalAuthorityEstate",
        "LegalAuthorityHolder",
        "ReviewEstate",
        "ReviewPrincipal",
    ]
    .into_iter()
    .collect()
}

fn estate_policies() -> BTreeSet<&'static str> {
    [
        "EmergencyElevationPolicy",
        "EstateBeneficiaryExclusionPolicy",
        "EstateCapabilityScopePolicy",
        "EstateConflictOfInterestPolicy",
        "EstateDisclosurePolicy",
        "EstateDistinctActorPolicy",
        "EstateSeparationOfDutyPolicy",
    ]
    .into_iter()
    .collect()
}
