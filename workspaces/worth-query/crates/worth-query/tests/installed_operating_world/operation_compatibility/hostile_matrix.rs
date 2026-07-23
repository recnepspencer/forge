pub(super) mod fixture;

use worth_foundational::facade::{CanonicalBasisLocus, InternedString};
use worth_query::facade::{domain, foundation};

use super::super::installed_operation_fixture::{
    configured_runtime, support_dimension_workspace, GeometryDomain, ReadFamily, ReadVertex,
};
use fixture::{bind, no_primary_read_runtime};
type BoundVertex = domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    ReadVertex,
    ReadFamily,
    foundation::ObservationLaneWitness,
>;

#[test]
fn all_five_relationship_oracles_are_stable_across_index_rebuild() {
    let mut controlled = configured_runtime()
        .consumer_support_posture(
            domain::WorthQueryConsumerSupportDimension::Sharing,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
        .controlled_workspace("compatibility-index-oracle")
        .unwrap();
    let prior_domain = controlled.domain(GeometryDomain).unwrap();
    let subject = bind_read_vertex(&controlled, &prior_domain, observation_basis());
    let candidate_before = bind_read_vertex(&controlled, &prior_domain, observation_basis());
    let before = current_relationship_counters(&subject, &candidate_before);

    assert!(controlled
        .verify_domain_execution_index_rebuild()
        .is_equivalent());
    assert!(controlled
        .rebuild_conditional_execution_index()
        .exact_index_parity());
    let candidate_after = bind_read_vertex(&controlled, &prior_domain, observation_basis());
    let after = current_relationship_counters(&subject, &candidate_after);
    assert_eq!(after, before);
    assert_current_success_costs(before);

    controlled.advance_domain_installation_generation().unwrap();
    let (current_domain, receipt) = controlled
        .rebind_domain(prior_domain.rebind_request())
        .unwrap()
        .into_parts();
    let rebound_before = bind_read_vertex(&controlled, &current_domain, observation_basis());
    let rebind_before = subject
        .rebind_with(&rebound_before, receipt.clone())
        .unwrap()
        .counters();

    assert!(controlled
        .verify_domain_execution_index_rebuild()
        .is_equivalent());
    assert!(controlled
        .rebuild_conditional_execution_index()
        .exact_index_parity());
    let rebound_after = bind_read_vertex(&controlled, &current_domain, observation_basis());
    let rebind_after = subject
        .rebind_with(&rebound_after, receipt)
        .unwrap()
        .counters();
    assert_eq!(rebind_after, rebind_before);
    assert_eq!(rebind_after.portable_contract_comparisons, 42);
    assert_eq!(rebind_after.canonical_comparisons, 5);
    assert_eq!(rebind_after.retained_authority_checks, 8);
    assert_eq!(rebind_after.portable_conditional_nodes_submitted, 0);
    assert_eq!(rebind_after.conditional_lowerings_compared, 0);
    assert_zero_forbidden_work(rebind_after);
}

#[test]
fn same_runtime_wrong_basis_and_stale_lifecycle_deny_through_compatibility() {
    let mut controlled = no_primary_read_runtime()
        .controlled_workspace("compatibility-basis-lifecycle")
        .unwrap();
    let prior_domain = controlled.domain(GeometryDomain).unwrap();
    let current = bind(&controlled, &prior_domain, observation_basis());
    let branch = bind(&controlled, &prior_domain, branch_basis());

    let basis_denial = current.compatible_basis_with(&branch).unwrap_err();
    assert_eq!(
        basis_denial.kind(),
        domain::WorthQueryCompatibilityDenialKind::BasisMismatched
    );
    assert!(matches!(
        basis_denial
            .canonical_mismatch()
            .and_then(|mismatch| mismatch.left_locus()),
        Some(CanonicalBasisLocus::Named(InternedString::Raw(locus))) if locus == "authority"
    ));
    assert_eq!(basis_denial.counters().canonical_comparisons, 5);
    assert_eq!(basis_denial.counters().retained_authority_checks, 10);
    assert_zero_forbidden_work(basis_denial.counters());

    controlled.advance_domain_installation_generation().unwrap();
    let stale_denial = current.same_installation_with(&branch).unwrap_err();
    assert_eq!(
        stale_denial.kind(),
        domain::WorthQueryCompatibilityDenialKind::InstallationFreshness
    );
    assert_eq!(stale_denial.counters().canonical_comparisons, 0);
    assert_eq!(stale_denial.counters().retained_authority_checks, 2);
    assert_zero_forbidden_work(stale_denial.counters());
}

#[test]
fn matching_reporting_material_cannot_cross_a_foreign_runtime() {
    let owner = no_primary_read_runtime()
        .workspace("compatibility-collision")
        .unwrap();
    let foreign = no_primary_read_runtime()
        .workspace("compatibility-collision")
        .unwrap();
    let owner_bound = bind(
        &owner,
        &owner.domain(GeometryDomain).unwrap(),
        observation_basis(),
    );
    let foreign_bound = bind(
        &foreign,
        &foreign.domain(GeometryDomain).unwrap(),
        observation_basis(),
    );

    assert_eq!(
        owner_bound.definition().canonical_identity(),
        foreign_bound.definition().canonical_identity()
    );
    assert_eq!(
        owner_bound.basis().capability_digest(),
        foreign_bound.basis().capability_digest()
    );
    assert_ne!(
        owner_bound.binding_identity(),
        foreign_bound.binding_identity()
    );
    let denial = owner_bound
        .same_installation_with(&foreign_bound)
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        domain::WorthQueryCompatibilityDenialKind::RuntimeAuthority
    );
    assert_eq!(denial.counters().retained_authority_checks, 1);
    assert_zero_forbidden_work(denial.counters());
}

#[test]
fn stale_and_current_lookalikes_cannot_claim_same_installation() {
    let mut controlled = no_primary_read_runtime()
        .controlled_workspace("compatibility-stale-collision")
        .unwrap();
    let prior_domain = controlled.domain(GeometryDomain).unwrap();
    let stale = bind(&controlled, &prior_domain, observation_basis());

    controlled.advance_domain_installation_generation().unwrap();
    let (current_domain, _) = controlled
        .rebind_domain(prior_domain.rebind_request())
        .unwrap()
        .into_parts();
    let current = bind(&controlled, &current_domain, observation_basis());

    assert_eq!(
        stale.definition().canonical_identity(),
        current.definition().canonical_identity()
    );
    assert_eq!(
        stale.basis().capability_digest(),
        current.basis().capability_digest()
    );
    let denial = stale.same_installation_with(&current).unwrap_err();
    assert_eq!(
        denial.kind(),
        domain::WorthQueryCompatibilityDenialKind::InstallationFreshness
    );
    assert_eq!(denial.counters().canonical_comparisons, 0);
    assert_eq!(denial.counters().retained_authority_checks, 2);
    assert_zero_forbidden_work(denial.counters());
}

#[test]
fn execution_sharing_stops_after_the_first_unsupported_profile() {
    let workspace = no_primary_read_runtime()
        .workspace("compatibility-sharing-short-circuit")
        .unwrap();
    let domain = workspace.domain(GeometryDomain).unwrap();
    let subject = bind(&workspace, &domain, observation_basis());
    let candidate = bind(&workspace, &domain, observation_basis());

    let denial = subject.execution_sharing_with(&candidate).unwrap_err();
    assert_eq!(
        denial.kind(),
        domain::WorthQueryCompatibilityDenialKind::RelationshipRule
    );
    assert_eq!(denial.counters().retained_authority_checks, 13);
    assert_zero_forbidden_work(denial.counters());
}

#[test]
fn installation_owner_mismatch_category_survives_the_query_boundary() {
    let plain = configured_runtime()
        .workspace("compatibility-owner-category-plain")
        .unwrap();
    let sharing = support_dimension_workspace(
        "compatibility-owner-category-sharing",
        domain::WorthQueryConsumerSupportDimension::Sharing,
        domain::WorthQueryConsumerSupportPosture::Supported,
    )
    .unwrap();
    let left = bind_read_vertex(
        &plain,
        &plain.domain(GeometryDomain).unwrap(),
        observation_basis(),
    );
    let right = bind_read_vertex(
        &sharing,
        &sharing.domain(GeometryDomain).unwrap(),
        observation_basis(),
    );

    let denial = left.compatible_basis_with(&right).unwrap_err();
    assert_eq!(
        denial.kind(),
        domain::WorthQueryCompatibilityDenialKind::PortableOperationContract
    );
    assert_eq!(
        denial.portable_operation_dimension(),
        Some(&domain::WorthQueryPortableOperationDimension::Support(
            domain::WorthQueryPortableOperationSupportDimension::Sharing
        ))
    );
    assert_eq!(
        denial.portable_operation_mismatch_category(),
        Some(domain::WorthQueryPortableOperationComparisonMismatchCategory::InstallationOwner)
    );
    assert!(denial.canonical_mismatch().is_none());
    assert_eq!(denial.counters().retained_authority_checks, 0);
    assert_zero_forbidden_work(denial.counters());
}

fn current_relationship_counters(
    subject: &BoundVertex,
    candidate: &BoundVertex,
) -> [domain::WorthQueryCompatibilityCounters; 4] {
    [
        subject
            .same_installation_with(candidate)
            .unwrap()
            .counters(),
        subject.replacement_with(candidate).unwrap().counters(),
        subject.compatible_basis_with(candidate).unwrap().counters(),
        subject
            .execution_sharing_with(candidate)
            .unwrap()
            .counters(),
    ]
}

fn assert_current_success_costs(counters: [domain::WorthQueryCompatibilityCounters; 4]) {
    assert_eq!(
        counters.map(|counters| counters.retained_authority_checks),
        [10, 12, 10, 17]
    );
    assert_eq!(counters[0].portable_contract_comparisons, 0);
    assert_eq!(counters[0].portable_variable_items_submitted, 0);
    assert_eq!(counters[0].canonical_comparisons, 0);
    for counters in &counters[1..] {
        assert_eq!(counters.portable_contract_comparisons, 42);
        assert_eq!(counters.canonical_comparisons, 5);
        assert!(counters.portable_variable_items_submitted > 0);
        assert_eq!(counters.portable_conditional_nodes_submitted, 0);
        assert_eq!(counters.conditional_lowerings_compared, 0);
        assert_zero_forbidden_work(*counters);
    }
}

fn assert_zero_forbidden_work(counters: domain::WorthQueryCompatibilityCounters) {
    assert_eq!(counters.lower_runtime_contacts, 0);
    assert_eq!(counters.execution_calls, 0);
    assert_eq!(counters.maintenance_calls, 0);
}

fn bind_read_vertex(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    installed: &domain::WorthQueryInstalledDomainHandle<GeometryDomain>,
    basis: foundation::AdmittedBasisCapability<foundation::ObservationLaneWitness>,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    ReadVertex,
    ReadFamily,
    foundation::ObservationLaneWitness,
> {
    workspace
        .operating_world(basis)
        .family(ReadFamily)
        .bind(installed, ReadVertex)
        .unwrap()
}

fn observation_basis() -> foundation::AdmittedBasisCapability<foundation::ObservationLaneWitness> {
    foundation::basis_lifecycle()
        .current_head()
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .capability()
        .clone()
}

fn branch_basis() -> foundation::AdmittedBasisCapability<foundation::ObservationLaneWitness> {
    foundation::basis_lifecycle()
        .branch_head("compatibility-branch", true)
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .capability()
        .clone()
}
