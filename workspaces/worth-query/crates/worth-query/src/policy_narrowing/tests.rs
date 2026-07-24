use crate::authoring::{
    AspectFieldKey, AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    GuidedAuthoringPath, OrderingSelector, RawAuthoredQuery, RawAuthoredResultShape, RootEntityKey,
    TraversalSelector, WorthQueryPredicateOperand,
};
use crate::authorized_projection::{PolicyAspectMask, PolicyInfluenceSet, PolicyMaskSnapshot};
use crate::policy_basis::{
    admit_policy_tenant_context, BranchAccessGrant, PolicyEpoch, PolicyExecutionModeRequest,
    PolicyRuleSnapshot,
};
use crate::relationship_proof::{
    RelationshipProofBudget, RelationshipProofDescriptor, RelationshipProofDescriptorSet,
};
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot};
use worth_foundational::facade::{AspectKey, FieldKey};

use super::lowering::narrow_policy_query_with_budget;
use super::{
    classify_saved_policy_narrowing_reuse, narrow_policy_query,
    optimizer_input_from_narrowed_policy_query, PolicyNarrowingFailureClass,
    SavedPolicyNarrowingReuseDescriptor, SavedPolicyNarrowingReuseDisposition,
};

mod support_profile;

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

#[test]
fn narrowed_artifact_binds_policy_tenant_projection_and_proof() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let descriptors = RelationshipProofDescriptorSet::new(
        vec![RelationshipProofDescriptor::direct_edge(
            "manager",
            admitted.bundle().policy_digest(),
        )],
        RelationshipProofBudget::bounded(1, 1),
    );

    let narrowed = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        descriptors,
    )
    .expect("Phase 2 narrowing should admit bounded direct proof");

    assert_eq!(
        narrowed.canonical_query_digest(),
        canonical.query().digest().as_str()
    );
    assert_eq!(
        narrowed.authorized_projection().visible_field_paths().len(),
        2
    );
    assert_eq!(
        native_field_pairs(
            narrowed
                .authorized_projection()
                .masked_projection()
                .masked_field_paths()
        ),
        vec![native_field_pair("secret", "salary")]
    );
    assert_eq!(narrowed.relationship_proof().descriptor_count(), 1);
    assert_eq!(narrowed.counters().narrowed_artifact_count(), 1);
    assert_eq!(
        narrowed.counters().relationship_proof().truth_touch_count(),
        0
    );
    assert!(!narrowed.digest().is_empty());
}

#[test]
fn masked_predicate_denies_before_narrowed_artifact_construction() {
    let canonical = canonical_with_masked_predicate();
    let admitted = admitted(&canonical);

    let error = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .expect_err("masked predicate must deny before narrowing");

    assert_eq!(
        error.failure_class(),
        PolicyNarrowingFailureClass::AuthorizedProjectionDenied(
            crate::authorized_projection::AuthorizedProjectionFailureClass::MaskedPredicateInfluence
        )
    );
    assert_eq!(
        error
            .counters()
            .authorized_projection()
            .hidden_predicate_denial_count(),
        1
    );
}

#[test]
fn masked_ordering_denies_before_optimizer_input_exists() {
    let canonical = canonical_with_masked_ordering();
    let admitted = admitted(&canonical);

    let error = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_non_disclosing_use_only(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .expect_err("non-disclosing ordering still leaks hidden truth");

    assert_eq!(
        error.failure_class(),
        PolicyNarrowingFailureClass::AuthorizedProjectionDenied(
            crate::authorized_projection::AuthorizedProjectionFailureClass::MaskedOrderingInfluence
        )
    );
    assert_eq!(
        error
            .counters()
            .authorized_projection()
            .hidden_ordering_denial_count(),
        1
    );
}

#[test]
fn relationship_proof_host_callback_is_forbidden_before_truth_touch() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);

    let error = narrow_policy_query_with_budget(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::new(
            vec![
                RelationshipProofDescriptor::direct_edge(
                    "manager",
                    admitted.bundle().policy_digest(),
                ),
                RelationshipProofDescriptor::host_callback_forbidden("authz"),
            ],
            RelationshipProofBudget::bounded(2, 1),
        ),
        crate::policy_narrowing::PolicyNarrowingWorkBudget::bounded(16, 16, 16, 2, 1, 8, 64),
    )
    .expect_err("host callbacks must not be relationship proof authority");

    assert_eq!(
        error.failure_class(),
        PolicyNarrowingFailureClass::RelationshipProofDenied(
            crate::relationship_proof::RelationshipProofFailureClass::HostCallbackForbidden
        )
    );
    assert_eq!(
        error
            .counters()
            .relationship_proof()
            .forbidden_host_callback_proof_count(),
        1
    );
    assert_eq!(error.counters().relationship_proof().truth_touch_count(), 0);
}

#[test]
fn optimizer_input_is_derived_from_narrowed_artifact_only() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let narrowed = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        manager_relationship_proof(&admitted),
    )
    .expect("narrowing should admit before optimizer input");

    let optimizer = optimizer_input_from_narrowed_policy_query(&narrowed);

    assert_eq!(
        optimizer.source_narrowed_artifact_digest(),
        narrowed.digest()
    );
    assert_eq!(
        native_field_pairs(optimizer.visible_field_paths()),
        vec![
            native_field_pair("identity", "id"),
            native_field_pair("profile", "display_name")
        ]
    );
    assert!(!native_field_pairs(optimizer.visible_field_paths())
        .iter()
        .any(|field| field == &native_field_pair("secret", "salary")));
    assert_eq!(
        optimizer.authorized_projection_digest(),
        narrowed.authorized_projection().identity().as_str()
    );
}

#[test]
fn policy_mask_snapshot_must_match_admitted_policy_authority() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);

    let error = narrow_policy_query(
        &canonical,
        admitted,
        PolicyMaskSnapshot::synthetic_authority(
            "wrong-policy-digest",
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .expect_err("mask snapshots must be bound to the admitted policy digest");

    assert_eq!(
        error.failure_class(),
        PolicyNarrowingFailureClass::PolicyMaskAuthorityMismatch
    );
}

#[test]
fn digest_part_budget_denies_before_artifact_construction() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let budget =
        crate::policy_narrowing::PolicyNarrowingWorkBudget::bounded(16, 16, 16, 0, 0, 8, 1);

    let error = narrow_policy_query_with_budget(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
        budget,
    )
    .expect_err("declared digest-part budget must be enforced");

    assert_eq!(
        error.failure_class(),
        PolicyNarrowingFailureClass::DigestPartBudgetExceeded
    );
}

#[test]
fn validation_report_digest_binds_authorized_projection_identity() {
    let canonical = canonical_query();
    let admitted = admitted(&canonical);
    let visible = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(&admitted, PolicyAspectMask::allow_all()),
        PolicyInfluenceSet::none(),
        manager_relationship_proof(&admitted),
    )
    .expect("visible projection should narrow");
    let masked = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask_snapshot(
            &admitted,
            PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        ),
        PolicyInfluenceSet::none(),
        manager_relationship_proof(&admitted),
    )
    .expect("masked projection should narrow");

    assert_ne!(
        visible.validation_report().digest(),
        masked.validation_report().digest()
    );
}

#[test]
fn saved_policy_narrowing_reuse_requires_exact_projection_and_proof_match() {
    let exact = SavedPolicyNarrowingReuseDescriptor::new(
        "saved-a",
        "narrowed-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-a",
        "proof-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-a",
        "proof-a",
    );
    let fresh = SavedPolicyNarrowingReuseDescriptor::new(
        "saved-a",
        "narrowed-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-a",
        "proof-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-b",
        "proof-a",
    );
    let drift = SavedPolicyNarrowingReuseDescriptor::new(
        "saved-a",
        "narrowed-a",
        "policy-a",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-a",
        "proof-a",
        "policy-b",
        "tenant-truth-a",
        "tenant-schema-a",
        "projection-a",
        "proof-a",
    );

    assert_eq!(
        classify_saved_policy_narrowing_reuse(&exact),
        SavedPolicyNarrowingReuseDisposition::LegalNoSemanticChange
    );
    assert_eq!(
        classify_saved_policy_narrowing_reuse(&fresh),
        SavedPolicyNarrowingReuseDisposition::LegalRequiresFreshNarrowing
    );
    assert_eq!(
        classify_saved_policy_narrowing_reuse(&drift),
        SavedPolicyNarrowingReuseDisposition::IllegalSemanticDrift
    );
}
