mod artifact_binding;
mod authority_binding;
mod budget_boundaries;
mod influence_denials;
mod optimizer_input;
mod relationship_proof;
mod saved_reuse;
mod validation_report;

use crate::authoring::{
    AspectFieldKey, AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    GuidedAuthoringPath, OrderingSelector, RawAuthoredQuery, RawAuthoredResultShape, RootEntityKey,
    TraversalSelector, WorthQueryPredicateOperand,
};
use crate::authorized_projection::{PolicyAspectMask, PolicyMaskSnapshot};
use crate::policy_basis::{
    admit_policy_tenant_context, BranchAccessGrant, PolicyEpoch, PolicyExecutionModeRequest,
    PolicyRuleSnapshot,
};
use crate::relationship_proof::{
    RelationshipProofBudget, RelationshipProofDescriptor, RelationshipProofDescriptorSet,
};
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot};
use worth_foundational::facade::{AspectKey, FieldKey};

fn canonical_query() -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("secret", "salary").unwrap())
        .traverse(TraversalSelector::bounded("manager", 1).unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();

    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

fn canonical_with_masked_predicate() -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .where_equal(
            EqualityPredicate::new("secret", "salary", WorthQueryPredicateOperand::int64(7))
                .unwrap(),
        )
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();

    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

fn canonical_with_masked_ordering() -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .order_by(OrderingSelector::ascending("secret", "salary").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();

    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

fn admitted(
    canonical: &crate::canonicalization::CanonicalQueryBundle,
) -> crate::policy_basis::AdmittedPolicyTenantContext {
    let policy = PolicyRuleSnapshot::synthetic_authority_with_projection(
        "runtime-policy",
        "rules-v1",
        PolicyEpoch::Synthetic(7),
        PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
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

fn mask_snapshot(
    admitted: &crate::policy_basis::AdmittedPolicyTenantContext,
    mask: PolicyAspectMask,
) -> PolicyMaskSnapshot {
    PolicyMaskSnapshot::synthetic_authority(admitted.bundle().policy_digest(), mask)
}

fn manager_relationship_proof(
    admitted: &crate::policy_basis::AdmittedPolicyTenantContext,
) -> RelationshipProofDescriptorSet {
    RelationshipProofDescriptorSet::new(
        vec![RelationshipProofDescriptor::direct_edge(
            "manager",
            admitted.bundle().policy_digest(),
        )],
        RelationshipProofBudget::bounded(1, 1),
    )
}

fn secret_salary_key() -> AspectFieldKey {
    AspectFieldKey::from_authoring_parts("secret", "salary").unwrap()
}

fn native_field_pairs(
    fields: &[crate::authorized_projection::AuthorizedProjectionFieldPath],
) -> Vec<(AspectKey, FieldKey)> {
    fields
        .iter()
        .filter_map(|field| {
            Some((
                field.native_aspect_key().clone(),
                field.native_field_key()?.clone(),
            ))
        })
        .collect()
}

fn native_field_pair(aspect: &str, field: &str) -> (AspectKey, FieldKey) {
    (
        AspectKey::new(aspect).expect("test aspect key should admit"),
        FieldKey::new(field).expect("test field key should admit"),
    )
}
