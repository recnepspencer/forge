use crate::harness::fixtures::schema_view::structured_content_queryable_schema_view;
use crate::harness::validation_cases::support::{
    assert_rejects_with, canonical_bundle_with_projection,
};
use crate::validation::QueryValidationError;
use worth_foundational::facade::{AspectKey, FieldKey};

#[test]
fn unknown_aspect_rejects_before_planning() {
    assert_rejects_with(
        canonical_bundle_with_projection("missing", "id"),
        QueryValidationError::UnknownAspect {
            aspect: "missing".to_string(),
        },
        "unknown aspect should reject",
    );
}

#[test]
fn unknown_field_triggers_widening_denial() {
    let error = crate::validation::validate_canonical_bundle(
        canonical_bundle_with_projection("profile", "unknown"),
        crate::harness::fixtures::schema_view::detail_schema_view(),
    )
    .expect_err("unknown field should reject");

    assert_eq!(
        error,
        QueryValidationError::ProjectionWideningDenied {
            aspect: "profile".to_string(),
            field: "unknown".to_string(),
        }
    );
    assert_eq!(
        error.failure_digest(),
        "projection-widening-denied:profile:unknown"
    );
}

#[test]
fn non_queryable_field_rejects() {
    assert_rejects_with(
        canonical_bundle_with_projection("profile", "private_note"),
        QueryValidationError::NonQueryableField {
            aspect: "profile".to_string(),
            field: "private_note".to_string(),
        },
        "non-queryable field should reject",
    );
}

#[test]
fn non_queryable_content_reference_rejects_explicitly() {
    assert_rejects_with(
        canonical_bundle_with_projection("content", "bio"),
        QueryValidationError::NonQueryableField {
            aspect: "content".to_string(),
            field: "bio".to_string(),
        },
        "non-queryable content reference should reject",
    );
}

#[test]
fn structured_content_projection_validates_when_schema_admits_it() {
    let validated = crate::validation::validate_canonical_bundle(
        crate::harness::fixtures::schema_view::legal_structured_content_bundle(),
        structured_content_queryable_schema_view(),
    )
    .expect("queryable structured content projection should validate");

    assert_eq!(validated.query().projection().len(), 2);
    let content_aspect_key = AspectKey::new("content").expect("fixture aspect key should admit");
    let bio_field_key = FieldKey::new("bio").expect("fixture field key should admit");
    let bio_binding = validated
        .result_shape()
        .bindings()
        .iter()
        .find(|binding| {
            binding.native_source_aspect_key() == &content_aspect_key
                && binding.native_source_field_key() == &bio_field_key
        })
        .expect("structured content binding should be present");
    assert_eq!(
        bio_binding.field_kind(),
        &crate::schema_view::ScalarAspectType::ContentRef
    );
}
