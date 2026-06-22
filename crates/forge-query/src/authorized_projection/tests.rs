use crate::authoring::{
    AspectFieldKey, AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    GuidedAuthoringPath, OrderingSelector, RawAuthoredQuery, RawAuthoredResultShape, RootEntityKey,
    ScalarPredicateValue,
};

use super::{
    derive_authorized_projection, AuthorizedProjectionFailureClass, PolicyAspectMask,
    PolicyInfluenceSet,
};

fn canonical_query() -> crate::canonicalization::CanonicalQueryBundle {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("secret", "salary").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();

    GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap()
}

fn secret_salary_key() -> AspectFieldKey {
    AspectFieldKey::new("secret", "salary").unwrap()
}

#[test]
fn masked_projection_is_excluded_from_authorized_projection() {
    let canonical = canonical_query();
    let artifact = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy-a",
        "schema-a",
        &PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        &PolicyInfluenceSet::none(),
        8,
        8,
    )
    .unwrap();

    assert_eq!(artifact.visible_field_paths().len(), 2);
    assert_eq!(
        native_field_pairs(artifact.masked_projection().masked_field_paths()),
        vec![("secret".to_string(), "salary".to_string())]
    );
    assert_eq!(artifact.counters().masked_projection_entry_count(), 1);
}

#[test]
fn non_disclosing_predicate_is_allowed_when_not_emitted() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .where_equal(
            EqualityPredicate::new("secret", "salary", ScalarPredicateValue::Integer(7)).unwrap(),
        )
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    let canonical = GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap();

    let artifact = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy-a",
        "schema-a",
        &PolicyAspectMask::allow_all().with_non_disclosing_use_only(secret_salary_key()),
        &PolicyInfluenceSet::none(),
        8,
        8,
    )
    .unwrap();

    assert_eq!(
        native_field_pairs(artifact.visible_field_paths()),
        vec![("identity".to_string(), "id".to_string())]
    );
    assert_eq!(artifact.counters().hidden_predicate_denial_count(), 0);
}

fn native_field_pairs(fields: &[super::AuthorizedProjectionFieldPath]) -> Vec<(String, String)> {
    fields
        .iter()
        .map(|field| {
            (
                field.native_aspect_key().as_str().to_string(),
                field.native_field_key().as_str().to_string(),
            )
        })
        .collect()
}

#[test]
fn masked_ordering_is_a_typed_denial() {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .order_by(OrderingSelector::ascending("secret", "salary").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .build()
        .unwrap();
    let canonical = GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap();

    let error = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy-a",
        "schema-a",
        &PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        &PolicyInfluenceSet::none(),
        8,
        8,
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        AuthorizedProjectionFailureClass::MaskedOrderingInfluence
    );
    assert_eq!(error.counters().hidden_ordering_denial_count(), 1);
}

#[test]
fn masked_grouping_influence_is_a_typed_denial() {
    let canonical = canonical_query();
    let error = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy-a",
        "schema-a",
        &PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        &PolicyInfluenceSet::none().with_grouping_field(secret_salary_key()),
        8,
        8,
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        AuthorizedProjectionFailureClass::MaskedGroupingInfluence
    );
    assert_eq!(error.counters().hidden_grouping_denial_count(), 1);
}

#[test]
fn masked_derived_field_influence_is_a_typed_denial() {
    let canonical = canonical_query();
    let error = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy-a",
        "schema-a",
        &PolicyAspectMask::allow_all().with_non_disclosing_use_only(secret_salary_key()),
        &PolicyInfluenceSet::none().with_derived_result_field(secret_salary_key()),
        8,
        8,
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        AuthorizedProjectionFailureClass::MaskedDerivedFieldInfluence
    );
    assert_eq!(error.counters().hidden_derived_field_denial_count(), 1);
}

#[test]
fn masked_aggregation_influence_is_a_typed_denial() {
    let canonical = canonical_query();
    let error = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy-a",
        "schema-a",
        &PolicyAspectMask::allow_all().with_non_disclosing_use_only(secret_salary_key()),
        &PolicyInfluenceSet::none().with_aggregation_field(secret_salary_key()),
        8,
        8,
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        AuthorizedProjectionFailureClass::MaskedAggregationInfluence
    );
    assert_eq!(error.counters().hidden_aggregation_denial_count(), 1);
}

#[test]
fn masked_cursor_influence_is_a_typed_denial() {
    let canonical = canonical_query();
    let error = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy-a",
        "schema-a",
        &PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        &PolicyInfluenceSet::none().with_cursor_field(secret_salary_key()),
        8,
        8,
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        AuthorizedProjectionFailureClass::MaskedCursorInfluence
    );
    assert_eq!(error.counters().hidden_cursor_denial_count(), 1);
}

#[test]
fn masked_view_membership_influence_is_a_typed_denial() {
    let canonical = canonical_query();
    let error = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy-a",
        "schema-a",
        &PolicyAspectMask::allow_all().with_masked(secret_salary_key()),
        &PolicyInfluenceSet::none().with_view_membership_field(secret_salary_key()),
        8,
        8,
    )
    .unwrap_err();

    assert_eq!(
        error.failure_class(),
        AuthorizedProjectionFailureClass::MaskedViewMembershipInfluence
    );
    assert_eq!(error.counters().hidden_view_membership_denial_count(), 1);
}

#[test]
fn non_disclosing_predicate_permission_does_not_admit_other_influence_purposes() {
    let canonical = canonical_query();
    let hidden = secret_salary_key();
    let mask = PolicyAspectMask::allow_all().with_non_disclosing_use_only(secret_salary_key());
    let aggregation = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy-a",
        "schema-a",
        &mask,
        &PolicyInfluenceSet::none().with_aggregation_field(hidden.clone()),
        8,
        8,
    )
    .unwrap_err();
    let cursor = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy-a",
        "schema-a",
        &mask,
        &PolicyInfluenceSet::none().with_cursor_field(hidden.clone()),
        8,
        8,
    )
    .unwrap_err();
    let view_membership = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy-a",
        "schema-a",
        &mask,
        &PolicyInfluenceSet::none().with_view_membership_field(hidden),
        8,
        8,
    )
    .unwrap_err();

    assert_eq!(
        aggregation.failure_class(),
        AuthorizedProjectionFailureClass::MaskedAggregationInfluence
    );
    assert_eq!(
        cursor.failure_class(),
        AuthorizedProjectionFailureClass::MaskedCursorInfluence
    );
    assert_eq!(
        view_membership.failure_class(),
        AuthorizedProjectionFailureClass::MaskedViewMembershipInfluence
    );
}
