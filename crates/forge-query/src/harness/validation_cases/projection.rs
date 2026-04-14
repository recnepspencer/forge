use crate::harness::validation_cases::support::{
    assert_rejects_with, canonical_bundle_with_projection,
};
use crate::validation::QueryValidationError;

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
fn structured_content_projection_rejects_explicitly() {
    assert_rejects_with(
        canonical_bundle_with_projection("content", "bio"),
        QueryValidationError::UnsupportedStructuredContentProjection {
            aspect: "content".to_string(),
            field: "bio".to_string(),
        },
        "structured content projection should reject",
    );
}
