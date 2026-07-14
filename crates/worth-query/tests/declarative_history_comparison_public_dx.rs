mod support;

use support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn history_transcript_executes_using_only_history_capability_vocabulary() {
    use worth_query::facade::history::{
        at, declare, AspectFieldSelector, AspectName, AuthoredResultShapeField, FieldName,
        QuerySchemaView, SchemaFieldKind, SchemaFieldView,
    };

    let declaration = declare(|read| {
        read.local_detail(
            "user",
            QuerySchemaView::new(
                "public-history-dx",
                [SchemaFieldView::new(
                    AspectName::new("identity").expect("aspect should build"),
                    FieldName::new("id").expect("field should build"),
                    SchemaFieldKind::String,
                )],
                [],
            ),
            |query| {
                query.project(
                    AspectFieldSelector::new("identity", "id").expect("projection should build"),
                )
            },
            |shape| {
                shape.field(
                    AuthoredResultShapeField::new("identity", "id", "id")
                        .expect("result field should build"),
                )
            },
        )
    })
    .expect("history should declare")
    .retained_snapshot();
    let runtime = PublicBridgeRuntimeHarness::new().bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public-history-dx")
        .expect("workspace should open");
    let context = at(&workspace);
    let outcome = declaration.using(context).run(&mut workspace);

    assert!(outcome.completed().is_some());
    assert!(outcome.stop().is_none());
}

#[test]
fn comparison_transcript_executes_using_only_comparison_capability_vocabulary() {
    use worth_query::facade::comparison::{
        current_and_retained, declare, AspectFieldSelector, AspectName, AuthoredResultShapeField,
        FieldName, QuerySchemaView, SchemaFieldKind, SchemaFieldView, WorthQueryComparisonChange,
    };

    let declaration = declare(|read| {
        read.local_detail(
            "user",
            QuerySchemaView::new(
                "public-comparison-dx",
                [SchemaFieldView::new(
                    AspectName::new("identity").expect("aspect should build"),
                    FieldName::new("id").expect("field should build"),
                    SchemaFieldKind::String,
                )],
                [],
            ),
            |query| {
                query.project(
                    AspectFieldSelector::new("identity", "id").expect("projection should build"),
                )
            },
            |shape| {
                shape.field(
                    AuthoredResultShapeField::new("identity", "id", "id")
                        .expect("result field should build"),
                )
            },
        )
    })
    .expect("comparison should declare")
    .diff();
    let runtime = PublicBridgeRuntimeHarness::new().bridge_backed_runtime();
    let mut workspace = runtime
        .workspace("public-comparison-dx")
        .expect("workspace should open");
    let context = current_and_retained(&workspace);
    let outcome = declaration.using(context).run(&mut workspace);

    assert_eq!(
        outcome.completed().expect("diff should complete").change(),
        WorthQueryComparisonChange::Unchanged
    );
}
