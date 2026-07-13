use super::super::super::support::*;
use crate::authoring::{AspectFieldSelector, AuthoredResultShapeField, TraversalSelector};
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
        &WorthQueryReadDenialKind::CanonicalizationDenied
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
