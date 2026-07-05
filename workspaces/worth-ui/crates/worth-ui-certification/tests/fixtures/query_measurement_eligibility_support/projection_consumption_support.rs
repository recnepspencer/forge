use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::consumer_kit::{in_memory_test_runtime, ForgeQueryTestBackendSchema};
use forge_query::facade::runtime::{
    ForgeQueryReadBuilder, ForgeQueryReadDenial, ForgeQueryReadFamily, ForgeQueryReadGraph,
    ForgeQueryWorkspace, QuerySchemaView, SchemaFieldKind, SchemaFieldView,
};
use forge_query::facade::{
    public_bridge_projection_artifacts_for_read_graph, resolve_runtime_current_snapshot_basis,
    snapshot_resolution_report, AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    ForgeQueryAspectTouch, ForgeQueryAuthoredAspectValue, ForgeQueryEntityIdentity,
    ProjectMaterializedFacts, ProjectionFactConsumptionAttempt, ScalarPredicateValue,
};
use worth_ui::facade::graph::UiGraphWorldProfile;

pub(super) fn measurement_projection_workspace(
    lane_label: &str,
) -> (
    ForgeQueryWorkspace,
    ForgeQueryReadFamily,
    ForgeQueryEntityIdentity,
) {
    measurement_projection_workspace_with_graph(lane_label, title_family_graph)
}

pub(super) fn measurement_projection_workspace_with_graph(
    lane_label: &str,
    graph: fn(ForgeQueryReadBuilder) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial>,
) -> (
    ForgeQueryWorkspace,
    ForgeQueryReadFamily,
    ForgeQueryEntityIdentity,
) {
    let schema = ForgeQueryTestBackendSchema::single_collection("task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should admit")
        .aspect("title.value", "title.value")
        .expect("title aspect should admit");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .workspace(&format!("worth-ui.phase5.query-measurement.{lane_label}"))
        .expect("in-memory test backend should build a workspace");
    let write_receipt = workspace
        .insert("task", |task| {
            task.set_aspect(
                aspect_touch("identity.id"),
                ForgeQueryAuthoredAspectValue::string("task"),
            )
            .set_aspect(
                aspect_touch("title.value"),
                ForgeQueryAuthoredAspectValue::string(format!("title-{lane_label}")),
            )
        })
        .expect("test workspace should admit the query row");
    let entity_identity = write_receipt
        .target_entity_identity()
        .cloned()
        .expect("fixture insert should resolve one target entity identity");
    let family = workspace
        .define_read_family(
            &format!("worth-ui.phase5.query-measurement.{lane_label}"),
            graph,
        )
        .expect("query read family should admit");
    (workspace, family, entity_identity)
}

pub(super) fn projection_consumption_attempt(
    workspace: &mut ForgeQueryWorkspace,
    family: &ForgeQueryReadFamily,
    requested: ProjectMaterializedFacts,
) -> (UiGraphWorldProfile, ProjectionFactConsumptionAttempt) {
    let read_result = workspace
        .execute_read_family(family)
        .expect("query read family should execute");
    let basis = resolve_runtime_current_snapshot_basis(
        workspace.snapshot_identity().evidence_identity(),
        family.read_graph().schema_basis().clone(),
    )
    .expect("runtime current snapshot basis should resolve from the real read family");
    let world_profile = UiGraphWorldProfile::query_snapshot_basis(
        basis.clone(),
        snapshot_resolution_report(&basis),
    )
    .expect("query snapshot basis world should admit");
    let (result_shape, authorized_projection) =
        public_bridge_projection_artifacts_for_read_graph(family.read_graph());
    let attempt = read_result
        .consume_projection_facts(&result_shape, &authorized_projection, requested)
        .expect("real query read should consume projection facts");
    (world_profile, attempt)
}

pub(super) fn identity_only_family_graph(
    read: ForgeQueryReadBuilder,
) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
    read.local_detail(
        "task",
        task_query_schema(),
        |query| {
            query
                .where_equal(
                    EqualityPredicate::new(
                        "identity",
                        "id",
                        ScalarPredicateValue::String("task".to_string()),
                    )
                    .expect("identity anchor predicate should build"),
                )
                .project(field("identity", "id"))
        },
        |shape| shape.field(result_field("identity", "id", "identity.id")),
    )
}

pub(super) fn title_value_field_path() -> forge_query::facade::ProjectionFactFieldPath {
    forge_query::facade::ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("title").expect("field key should admit"),
            FieldKey::new("value").expect("field key should admit"),
        ])
        .expect("canonical title.value field path should admit"),
    )
}

fn title_family_graph(
    read: ForgeQueryReadBuilder,
) -> Result<ForgeQueryReadGraph, ForgeQueryReadDenial> {
    read.local_detail(
        "task",
        task_query_schema(),
        |query| {
            query
                .where_equal(
                    EqualityPredicate::new(
                        "identity",
                        "id",
                        ScalarPredicateValue::String("task".to_string()),
                    )
                    .expect("identity anchor predicate should build"),
                )
                .project(field("title", "value"))
        },
        |shape| shape.field(result_field("title", "value", "title.value")),
    )
}

fn task_query_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "task",
        [
            SchemaFieldView::new(
                forge_query::facade::AspectName::new("identity")
                    .expect("schema aspect should admit"),
                forge_query::facade::FieldName::new("id").expect("schema field should admit"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                forge_query::facade::AspectName::new("title").expect("schema aspect should admit"),
                forge_query::facade::FieldName::new("value").expect("schema field should admit"),
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

pub(super) fn aspect_touch(authored_touch_text: &str) -> ForgeQueryAspectTouch {
    let mut segments = authored_touch_text.split('.');
    let aspect = segments
        .next()
        .and_then(AspectKey::new)
        .expect("fixture authored touch aspect should admit");
    let fields = segments
        .map(|segment| FieldKey::new(segment).expect("fixture authored touch field should admit"))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        ForgeQueryAspectTouch::whole_aspect(aspect)
    } else {
        ForgeQueryAspectTouch::aspect_field_path(
            aspect,
            CanonicalFieldPath::new(fields).expect("fixture authored touch should have fields"),
        )
    }
}
