use super::super::super::support::*;
use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate, ScalarPredicateValue,
    TraversalSelector,
};
use crate::ordinary::read::declare;
use crate::runtime::WorthQueryReadDenialKind;

#[test]
fn excessive_traversal_is_denied_before_a_runnable_capability_exists() {
    let stop = declare(|read| {
        read.anchored_collection(
            "user",
            manager_schema(),
            |query| {
                query
                    .traverse(
                        TraversalSelector::bounded("manager", 2)
                            .expect("non-zero traversal should author"),
                    )
                    .project(identity_field())
            },
            |shape| shape.field(identity_result_field()),
        )
    })
    .expect_err("schema-excessive traversal must not mint a declaration");

    assert_eq!(
        stop.denial().kind(),
        &WorthQueryReadDenialKind::ValidationDenied
    );
    assert_eq!(
        stop.next_action(),
        crate::ordinary::read::WorthQueryReadNextAction::ReviseDeclaration
    );
}

#[test]
fn invalid_result_shape_is_denied_before_a_runnable_capability_exists() {
    let stop = declare(|read| {
        read.local_detail(
            "user",
            manager_schema(),
            |query| query.project(identity_field()),
            |shape| {
                shape.field(
                    AuthoredResultShapeField::new("profile", "display_name", "display_name")
                        .expect("result field should author"),
                )
            },
        )
    })
    .expect_err("shape outside the query projection must not mint a declaration");

    assert_eq!(
        stop.denial().kind(),
        &WorthQueryReadDenialKind::AuthoringDenied
    );
    assert_eq!(
        stop.next_action(),
        crate::ordinary::read::WorthQueryReadNextAction::ReviseDeclaration
    );
}

#[test]
fn unknown_field_is_denied_during_declaration() {
    let stop = declare(|read| {
        read.local_detail(
            "user",
            manager_schema(),
            |query| {
                query.project(
                    AspectFieldSelector::new("profile", "unknown")
                        .expect("field selector syntax should author"),
                )
            },
            |shape| {
                shape.field(
                    AuthoredResultShapeField::new("profile", "unknown", "unknown")
                        .expect("result field syntax should author"),
                )
            },
        )
    })
    .expect_err("unknown schema field must not mint a declaration");

    assert_eq!(
        stop.denial().kind(),
        &WorthQueryReadDenialKind::ValidationDenied
    );
    assert_eq!(
        stop.next_action(),
        crate::ordinary::read::WorthQueryReadNextAction::ReviseDeclaration
    );
}

#[test]
fn predicate_type_mismatch_is_denied_during_declaration() {
    let stop = declare(|read| {
        read.local_detail(
            "user",
            manager_schema(),
            |query| {
                query.project(identity_field()).where_equal(
                    EqualityPredicate::new(
                        "profile",
                        "display_name",
                        ScalarPredicateValue::Integer(42),
                    )
                    .expect("predicate syntax should author"),
                )
            },
            |shape| shape.field(identity_result_field()),
        )
    })
    .expect_err("predicate kind mismatch must not mint a declaration");

    assert_eq!(
        stop.denial().kind(),
        &WorthQueryReadDenialKind::ValidationDenied
    );
    assert_eq!(
        stop.next_action(),
        crate::ordinary::read::WorthQueryReadNextAction::ReviseDeclaration
    );
}

fn identity_field() -> AspectFieldSelector {
    AspectFieldSelector::new("identity", "id").expect("identity field should author")
}

fn identity_result_field() -> AuthoredResultShapeField {
    AuthoredResultShapeField::new("identity", "id", "id")
        .expect("identity result field should author")
}
