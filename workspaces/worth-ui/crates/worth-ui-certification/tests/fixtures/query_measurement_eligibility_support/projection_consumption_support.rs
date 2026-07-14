use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::certification::resolve_runtime_current_snapshot_basis_for_certification;
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::foundation::{
    snapshot_resolution_report, AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    ScalarPredicateValue, WorthQueryEntityIdentity,
};
use worth_query::facade::read::{
    current, declare, WorthQueryProjectionDeclaration, WorthQueryProjectionOutcome,
    WorthQueryReadCompletion,
};
use worth_query::facade::runtime::{
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, WorthQueryAspectTouch,
    WorthQueryAuthoredAspectValue, WorthQueryReadBuilder, WorthQueryReadDenial,
    WorthQueryWorkspace,
};
use worth_ui::facade::graph::UiGraphWorldProfile;

pub(super) fn measurement_projection_workspace(
    lane_label: &str,
) -> (
    WorthQueryWorkspace,
    worth_query::facade::foundation::QuerySchemaBasisAuthority,
    WorthQueryEntityIdentity,
) {
    let schema = WorthQueryTestBackendSchema::single_collection("task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should admit")
        .aspect("size.value", "size.value")
        .expect("size aspect should admit");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .workspace(&format!("worth-ui.phase5.query-measurement.{lane_label}"))
        .expect("in-memory test backend should build a workspace");
    let write_receipt = workspace
        .insert("task", |task| {
            task.set_aspect(
                aspect_touch("identity.id"),
                WorthQueryAuthoredAspectValue::string("task"),
            )
            .set_aspect(
                aspect_touch("size.value"),
                WorthQueryAuthoredAspectValue::string("240"),
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
    consume_completion(workspace, schema_basis_authority, completion, projection)
}

pub(super) fn identity_only_projection_consumption_attempt(
    workspace: &mut WorthQueryWorkspace,
    schema_basis_authority: worth_query::facade::foundation::QuerySchemaBasisAuthority,
    projection: WorthQueryProjectionDeclaration,
) -> (UiGraphWorldProfile, WorthQueryProjectionOutcome) {
    let completion = declare(identity_only_family_graph)
        .expect("ordinary query declaration should admit")
        .using(current())
        .run(workspace)
        .into_result()
        .expect("ordinary query read should execute");
    consume_completion(workspace, schema_basis_authority, completion, projection)
}

fn consume_completion(
    workspace: &WorthQueryWorkspace,
    schema_basis_authority: worth_query::facade::foundation::QuerySchemaBasisAuthority,
    completion: WorthQueryReadCompletion,
    projection: WorthQueryProjectionDeclaration,
) -> (UiGraphWorldProfile, WorthQueryProjectionOutcome) {
    let basis = resolve_runtime_current_snapshot_basis_for_certification(
        &workspace.snapshot_identity().evidence_identity(),
        schema_basis_authority,
    )
    .expect("runtime current snapshot basis should resolve from the ordinary read");
    let world_profile = UiGraphWorldProfile::query_snapshot_basis(
        basis.clone(),
        snapshot_resolution_report(&basis),
    )
    .expect("query snapshot basis world should admit");
    (world_profile, completion.consume_projection(projection))
}

fn identity_only_family_graph<Output>(
    read: WorthQueryReadBuilder<Output>,
) -> Result<Output, WorthQueryReadDenial> {
    read.local_detail(
        "task",
        task_query_schema(),
        |query| {
            query
                .where_equal(identity_predicate())
                .project(field("identity", "id"))
        },
        |shape| shape.field(result_field("identity", "id", "identity.id")),
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
                .where_equal(identity_predicate())
                .project(field("size", "value"))
        },
        |shape| shape.field(result_field("size", "value", "size.value")),
    )
}

fn identity_predicate() -> EqualityPredicate {
    EqualityPredicate::new(
        "identity",
        "id",
        ScalarPredicateValue::String("task".to_string()),
    )
    .expect("identity anchor predicate should build")
}

pub(super) fn title_value_field_path() -> worth_query::facade::read::ProjectionFactFieldPath {
    worth_query::facade::read::ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("size").expect("field key should admit"),
            FieldKey::new("value").expect("field key should admit"),
        ])
        .expect("canonical size.value field path should admit"),
    )
}

fn task_query_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "task",
        [
            SchemaFieldView::new(
                worth_query::facade::read::AspectName::new("identity").expect("schema aspect"),
                worth_query::facade::read::FieldName::new("id").expect("schema field"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                worth_query::facade::read::AspectName::new("size").expect("schema aspect"),
                worth_query::facade::read::FieldName::new("value").expect("schema field"),
                SchemaFieldKind::String,
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
    WorthQueryAspectTouch::from_authoring_ingress_text(authored_touch_text)
        .expect("fixture authored touch should admit")
}
