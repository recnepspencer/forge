use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::certification::resolve_runtime_current_snapshot_basis_for_certification;
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::foundation::{
    snapshot_resolution_report, AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    WorthQueryEntityIdentity, WorthQueryPredicateOperand,
};
use worth_query::facade::read::{
    current, declare, WorthQueryProjectionDeclaration, WorthQueryProjectionOutcome,
};
use worth_query::facade::runtime::{
    QuerySchemaView, SchemaFieldView, WorthQueryReadBuilder, WorthQueryReadDenial,
    WorthQueryWorkspace,
};
use worth_query::facade::runtime::{WorthQueryAspectTouch, WorthQueryAuthoredAspectValue};
use worth_ui::facade::graph::UiGraphWorldProfile;

pub(super) fn measurement_projection_workspace(
    lane_label: &str,
) -> (
    WorthQueryWorkspace,
    worth_query::facade::foundation::QuerySchemaBasisAuthority,
    WorthQueryEntityIdentity,
) {
    let schema = WorthQueryTestBackendSchema::single_collection("task")
        .aspect_contracts(worth_ui_query_binding::worth_ui_native_aspect_contracts())
        .expect("Worth UI native aspect contracts should admit")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should admit")
        .aspect("size.value", "size.value")
        .expect("size aspect should admit");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .workspace(&format!("worth-ui.phase5.measurement-basis.{lane_label}"))
        .expect("in-memory test backend should build a workspace");
    let write_receipt = workspace
        .insert("task", |task| {
            task.set_aspect(
                aspect_touch("identity.id"),
                WorthQueryAuthoredAspectValue::string("task"),
            )
            .set_aspect(
                aspect_touch("size.value"),
                WorthQueryAuthoredAspectValue::native(
                    worth_foundational::facade::AspectValue::Float32(
                        worth_foundational::facade::CanonicalF32::from_f32(240.0),
                    ),
                ),
            )
        })
        .expect("test workspace should admit the query row");
    let entity_identity = write_receipt
        .target_entity_identity()
        .cloned()
        .expect("fixture insert should resolve one target entity identity");
    (
        workspace,
        task_query_schema().basis_authority(),
        entity_identity,
    )
}

pub(super) fn projection_consumption_attempt(
    workspace: &mut WorthQueryWorkspace,
    schema_basis_authority: worth_query::facade::foundation::QuerySchemaBasisAuthority,
    projection: WorthQueryProjectionDeclaration,
) -> (UiGraphWorldProfile, WorthQueryProjectionOutcome) {
    let completion = declare(title_family_graph)
        .expect("ordinary query declaration should admit")
        .using(current())
        .run(workspace)
        .into_result()
        .expect("ordinary query read should execute");
    let basis = resolve_runtime_current_snapshot_basis_for_certification(
        &workspace.snapshot_identity().evidence_identity(),
        schema_basis_authority,
    )
    .expect("runtime current snapshot basis should resolve from the ordinary declaration");
    let world_profile = UiGraphWorldProfile::query_snapshot_basis(
        basis.clone(),
        snapshot_resolution_report(&basis),
    )
    .expect("query snapshot basis world should admit");
    let outcome = completion.consume_projection(projection);
    (world_profile, outcome)
}

pub(super) fn title_value_field_path() -> worth_query::facade::foundation::ProjectionFactFieldPath {
    worth_query::facade::foundation::ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("size").expect("field key should admit"),
            FieldKey::new("value").expect("field key should admit"),
        ])
        .expect("canonical size.value field path should admit"),
    )
}

fn title_family_graph<Output>(
    read: WorthQueryReadBuilder<Output>,
) -> Result<Output, WorthQueryReadDenial> {
    read.local_detail(
        "task",
        task_query_schema(),
        |query| {
            query
                .where_equal(
                    EqualityPredicate::new(
                        "identity",
                        "id",
                        WorthQueryPredicateOperand::string("task".to_string()),
                    )
                    .expect("identity anchor predicate should build"),
                )
                .project(field("size", "value"))
        },
        |shape| shape.field(result_field("size", "value", "size.value")),
    )
}

fn task_query_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "task",
        [
            SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("identity")
                    .expect("schema aspect should admit"),
                worth_query::facade::foundation::FieldName::new("id")
                    .expect("schema field should admit"),
                worth_foundational::facade::ScalarAspectType::String,
            ),
            SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("size")
                    .expect("schema aspect should admit"),
                worth_query::facade::foundation::FieldName::new("value")
                    .expect("schema field should admit"),
                worth_foundational::facade::ScalarAspectType::Float32,
            ),
        ],
        [],
    )
}

fn field(aspect: &str, field: &str) -> AspectFieldSelector {
    AspectFieldSelector::new(aspect, field).expect("field selector should build")
}

fn result_field(aspect: &str, field: &str, delivered: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(aspect, field, delivered)
        .expect("result-shape field should build")
}

pub(super) fn aspect_touch(authored_touch_text: &str) -> WorthQueryAspectTouch {
    let mut segments = authored_touch_text.split('.');
    let aspect = segments
        .next()
        .and_then(AspectKey::new)
        .expect("fixture authored touch aspect should admit");
    let fields = segments
        .map(|segment| FieldKey::new(segment).expect("fixture authored touch field should admit"))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        WorthQueryAspectTouch::whole_aspect(aspect)
    } else {
        WorthQueryAspectTouch::aspect_field_path(
            aspect,
            CanonicalFieldPath::new(fields).expect("fixture authored touch should have fields"),
        )
    }
}
