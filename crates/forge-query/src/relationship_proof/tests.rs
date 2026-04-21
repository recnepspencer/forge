use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RawAuthoredQuery,
    RawAuthoredResultShape, RootEntityKey,
};
use crate::policy_basis::{
    admit_policy_tenant_context, BranchAccessGrant, PolicyEpoch, PolicyExecutionModeRequest,
    PolicyRuleSnapshot,
};
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot};

use super::{
    admit_relationship_proofs, RelationshipProofBudget, RelationshipProofDescriptor,
    RelationshipProofDescriptorSet, RelationshipProofFailureClass, RelationshipProofTopologyClass,
};

fn canonical_query() -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

fn admitted(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
) -> crate::policy_basis::AdmittedPolicyTenantContext {
    let policy = PolicyRuleSnapshot::synthetic_authority(
        "runtime-policy",
        "rules-v1",
        PolicyEpoch::Synthetic(7),
    );
    let tenant = TenantBindingSnapshot::synthetic_direct(
        "tenant-a",
        "branch-a",
        "schema-a",
        TenantBasisEpoch::Synthetic(3),
    );
    let branch = BranchAccessGrant::synthetic_granted("branch-a", &policy);
    let schema = SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "compatible");
    admit_policy_tenant_context(
        canonical.query(),
        policy,
        tenant,
        branch,
        schema,
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap()
}

#[test]
fn direct_edge_descriptor_admits_without_truth_touch() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let descriptors = RelationshipProofDescriptorSet::new(
        vec![RelationshipProofDescriptor::direct_edge(
            "manager",
            admitted.bundle().policy_digest(),
        )],
        RelationshipProofBudget::bounded(1, 1),
    );

    let (admission, counters) =
        admit_relationship_proofs(canonical.query(), &admitted, &descriptors).unwrap();

    assert_eq!(admission.descriptor_count(), 1);
    assert_eq!(counters.relationship_proof_admission_count(), 1);
    assert_eq!(counters.truth_touch_count(), 0);
}

#[test]
fn unbounded_recursive_descriptor_denies() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let descriptors = RelationshipProofDescriptorSet::new(
        vec![RelationshipProofDescriptor::unbounded_recursive_walk_for_test("manager")],
        RelationshipProofBudget::bounded(1, 1),
    );

    let error = admit_relationship_proofs(canonical.query(), &admitted, &descriptors).unwrap_err();

    assert_eq!(
        error.failure_class(),
        RelationshipProofFailureClass::UnboundedRecursiveWalk
    );
    assert_eq!(
        error
            .counters()
            .relationship_proof_recursive_broadening_denial_count(),
        1
    );
}

#[test]
fn tenant_membership_descriptor_binds_tenant_schema_basis() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let descriptors = RelationshipProofDescriptorSet::new(
        vec![RelationshipProofDescriptor::tenant_membership(
            admitted.bundle().tenant_schema_basis_digest(),
        )],
        RelationshipProofBudget::bounded(1, 1),
    );

    let (admission, counters) =
        admit_relationship_proofs(canonical.query(), &admitted, &descriptors).unwrap();

    assert_eq!(
        admission.topology_classes(),
        &[RelationshipProofTopologyClass::TenantMembership]
    );
    assert_eq!(counters.relationship_proof_admission_count(), 1);
    assert_eq!(counters.truth_touch_count(), 0);
}

#[test]
fn bounded_ancestor_descriptor_requires_nonzero_bound() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let descriptors = RelationshipProofDescriptorSet::new(
        vec![RelationshipProofDescriptor::bounded_ancestor(
            "manager",
            0,
            admitted.bundle().policy_digest(),
        )],
        RelationshipProofBudget::bounded(1, 1),
    );

    let error = admit_relationship_proofs(canonical.query(), &admitted, &descriptors).unwrap_err();

    assert_eq!(
        error.failure_class(),
        RelationshipProofFailureClass::UnboundedProofTopology
    );
    assert_eq!(
        error
            .counters()
            .relationship_proof_recursive_broadening_denial_count(),
        1
    );
}

#[test]
fn query_shape_mismatch_denies_before_truth_touch() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let descriptors = RelationshipProofDescriptorSet::new(
        vec![RelationshipProofDescriptor::query_shape_mismatch_for_test(
            "different-query-digest",
        )],
        RelationshipProofBudget::bounded(1, 1),
    );

    let error = admit_relationship_proofs(canonical.query(), &admitted, &descriptors).unwrap_err();

    assert_eq!(
        error.failure_class(),
        RelationshipProofFailureClass::QueryShapeMismatch
    );
    assert_eq!(error.counters().truth_touch_count(), 0);
}
