#![cfg(test)]

use forge_foundational::facade::{CanonicalFieldPath, FieldKey};
use forge_query::facade::consumer_kit::{in_memory_test_runtime, ForgeQueryTestBackendSchema};
use forge_query::facade::runtime::{
    ForgeQueryReadBuilder, ForgeQueryReadDenial, ForgeQueryReadFamily, ForgeQueryReadGraph,
    ForgeQueryWorkspace, QuerySchemaView, SchemaFieldKind, SchemaFieldView,
};
use forge_query::facade::{
    public_bridge_projection_artifacts_for_read_graph, resolve_runtime_current_snapshot_basis,
    snapshot_resolution_report, AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    ForgeQueryAspectTouch, ForgeQueryAuthoredAspectValue, ProjectMaterializedFacts,
    ProjectionFactConsumptionAttempt, ProjectionFactFieldPath, ScalarPredicateValue,
};

use crate::graph::UiGraphWorldProfile;

pub(crate) fn display_field_plus_entity_identity_projection_context(
    lane_label: &str,
) -> (
    worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence,
    ProjectionFactConsumptionAttempt,
    UiGraphWorldProfile,
) {
    let (mut workspace, family) = measurement_projection_workspace(lane_label);
    let read_result = workspace
        .execute_read_family(&family)
        .expect("query read family should execute");
    let (result_shape, authorized_projection) =
        public_bridge_projection_artifacts_for_read_graph(family.read_graph());
    let requested = ProjectMaterializedFacts::declare()
        .entity_identities()
        .display_field_path(title_value_field_path());
    let attempt = read_result
        .consume_projection_facts(&result_shape, &authorized_projection, requested)
        .expect("real query read should consume projection facts");
    let basis = resolve_runtime_current_snapshot_basis(
        workspace.snapshot_identity().evidence_identity(),
        family.read_graph().schema_basis().clone(),
    )
    .expect("runtime current snapshot basis should resolve");
    let prerequisites = worth_ui_query_binding::WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .graph_aligned(basis.clone(), snapshot_resolution_report(&basis))
        .expect("query prerequisites should admit");
    let world_profile = UiGraphWorldProfile::query_snapshot_basis(
        basis.clone(),
        snapshot_resolution_report(&basis),
    )
    .expect("query world profile should align to basis resolution");
    (prerequisites, attempt, world_profile)
}

fn measurement_projection_workspace(
    lane_label: &str,
) -> (ForgeQueryWorkspace, ForgeQueryReadFamily) {
    let schema = ForgeQueryTestBackendSchema::single_collection("task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should admit")
        .aspect("title.value", "title.value")
        .expect("title aspect should admit");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .workspace(&format!(
            "worth-ui.phase10.measurement-neighborhood.{lane_label}"
        ))
        .expect("in-memory query backend should build a workspace");
    workspace
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
        .expect("fixture insert should admit");
    let family = workspace
        .define_read_family(
            &format!("worth-ui.phase10.measurement-neighborhood.{lane_label}"),
            title_family_graph,
        )
        .expect("query read family should admit");
    (workspace, family)
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

fn title_value_field_path() -> ProjectionFactFieldPath {
    ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("title").expect("field key should admit"),
            FieldKey::new("value").expect("field key should admit"),
        ])
        .expect("canonical title.value path should admit"),
    )
}

fn field(aspect: &str, field: &str) -> AspectFieldSelector {
    AspectFieldSelector::new(aspect, field).expect("field selector should build")
}

fn result_field(aspect: &str, field: &str, delivered: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(aspect, field, delivered)
        .expect("result-shape field should build")
}

fn aspect_touch(authored_touch_text: &str) -> ForgeQueryAspectTouch {
    let mut segments = authored_touch_text.split('.');
    let aspect = segments.next().expect("touch aspect should exist");
    let fields = segments
        .map(|segment| FieldKey::new(segment).expect("touch field should admit"))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        ForgeQueryAspectTouch::whole_aspect(
            forge_foundational::facade::AspectKey::new(aspect).expect("touch aspect should admit"),
        )
    } else {
        ForgeQueryAspectTouch::aspect_field_path(
            forge_foundational::facade::AspectKey::new(aspect).expect("touch aspect should admit"),
            CanonicalFieldPath::new(fields).expect("touch field path should admit"),
        )
    }
}
