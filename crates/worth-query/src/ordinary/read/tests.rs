use super::{declare, WorthQueryReadNextAction};
use crate::authoring::{AspectFieldSelector, AspectName, AuthoredResultShapeField, FieldName};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind, SchemaFieldView};

#[test]
fn equivalent_read_declarations_converge_on_one_canonical_identity() {
    let first = declare(local_identity_read).expect("first declaration should canonicalize");
    let second = declare(local_identity_read).expect("second declaration should canonicalize");

    assert_eq!(first.identity(), second.identity());
}

#[test]
fn invalid_scope_shape_stops_during_declaration_without_a_workspace() {
    let stop = declare(|read| {
        read.anchored_detail(
            "user",
            identity_schema(),
            |query| query.project(identity_selector()),
            |shape| shape.field(identity_result_field()),
        )
    })
    .expect_err("anchored reads without traversal must stop during declaration");

    assert_eq!(
        stop.next_action(),
        WorthQueryReadNextAction::ReviseDeclaration
    );
    assert_eq!(
        stop.denial().kind(),
        &crate::runtime::WorthQueryReadDenialKind::ScopeShapeDenied
    );
}

fn local_identity_read(
    read: crate::runtime::WorthQueryReadBuilder,
) -> Result<crate::runtime::WorthQueryReadGraph, crate::runtime::WorthQueryReadDenial> {
    read.local_detail(
        "user",
        identity_schema(),
        |query| query.project(identity_selector()),
        |shape| shape.field(identity_result_field()),
    )
}

fn identity_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "ordinary-read-identity",
        [SchemaFieldView::new(
            AspectName::new("identity").expect("test aspect must be valid"),
            FieldName::new("id").expect("test field must be valid"),
            SchemaFieldKind::String,
        )],
        [],
    )
}

fn identity_selector() -> AspectFieldSelector {
    AspectFieldSelector::new("identity", "id").expect("test selector must be valid")
}

fn identity_result_field() -> AuthoredResultShapeField {
    AuthoredResultShapeField::new("identity", "id", "id").expect("test result field must be valid")
}
