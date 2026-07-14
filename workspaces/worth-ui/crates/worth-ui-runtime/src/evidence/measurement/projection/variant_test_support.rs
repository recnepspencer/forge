#![cfg(test)]

use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::runtime::{
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, WorthQueryReadBuilder, WorthQueryReadDenial,
    WorthQueryWorkspace,
};
use worth_query::facade::read::{current, declare, project_facts};
use worth_query::facade::certification::resolve_runtime_current_snapshot_basis_for_certification;
use worth_query::facade::foundation::{
    snapshot_resolution_report,
    AspectFieldSelector,
    AuthoredResultShapeField,
    EqualityPredicate,
    ProjectionFactFieldPath,
    ScalarPredicateValue,
};
use worth_query::facade::runtime::{
    WorthQueryAspectTouch,
    WorthQueryAuthoredAspectValue,
};

use crate::graph::UiGraphWorldProfile;

pub(crate) fn display_field_plus_entity_identity_projection_context(
    lane_label: &str,
) -> (
    worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence,
    worth_ui_query_binding::WorthUiQueryAuthorityHandle,
    UiGraphWorldProfile,
) {
    let (mut workspace, schema_basis_authority) = measurement_projection_workspace(lane_label);
    let completion = declare(title_family_graph)
        .expect("ordinary query declaration should admit")
        .using(current())
        .run(&mut workspace)
        .into_result()
        .expect("ordinary query read should execute");
    let outcome = completion.consume_projection(
        project_facts()
            .entity_identities()
            .display_field(size_value_field_path()),
    );
    let basis = resolve_runtime_current_snapshot_basis_for_certification(
        &workspace.snapshot_identity().evidence_identity(),
        schema_basis_authority,
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
    let (authority, _) =
        worth_ui_query_binding::WorthUiQueryAuthorityHandle::from_outcome(outcome)
            .expect("real Query consumption should mint authority");
    (prerequisites, authority, world_profile)
}

fn measurement_projection_workspace(
    lane_label: &str,
) -> (
    WorthQueryWorkspace,
    worth_query::facade::foundation::QuerySchemaBasisAuthority,
) {
    let schema = WorthQueryTestBackendSchema::single_collection("task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should admit")
        .aspect("size.value", "size.value")
        .expect("size aspect should admit");
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
                WorthQueryAuthoredAspectValue::string("task"),
            )
            .set_aspect(
                aspect_touch("size.value"),
                WorthQueryAuthoredAspectValue::string("240"),
            )
        })
        .expect("fixture insert should admit");
    (workspace, task_query_schema().basis_authority())
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
                        ScalarPredicateValue::String("task".to_string()),
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
                worth_query::facade::foundation::FieldName::new("id").expect("schema field should admit"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("size").expect("schema aspect should admit"),
                worth_query::facade::foundation::FieldName::new("value").expect("schema field should admit"),
                SchemaFieldKind::String,
            ),
        ],
        [],
    )
}

fn size_value_field_path() -> ProjectionFactFieldPath {
    ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("size").expect("field key should admit"),
            FieldKey::new("value").expect("field key should admit"),
        ])
        .expect("canonical size.value path should admit"),
    )
}

fn field(aspect: &str, field: &str) -> AspectFieldSelector {
    AspectFieldSelector::new(aspect, field).expect("field selector should build")
}

fn result_field(aspect: &str, field: &str, delivered: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(aspect, field, delivered)
        .expect("result-shape field should build")
}

fn aspect_touch(authored_touch_text: &str) -> WorthQueryAspectTouch {
    let mut segments = authored_touch_text.split('.');
    let aspect = segments.next().expect("touch aspect should exist");
    let fields = segments
        .map(|segment| FieldKey::new(segment).expect("touch field should admit"))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        WorthQueryAspectTouch::whole_aspect(
            worth_foundational::facade::AspectKey::new(aspect).expect("touch aspect should admit"),
        )
    } else {
        WorthQueryAspectTouch::aspect_field_path(
            worth_foundational::facade::AspectKey::new(aspect).expect("touch aspect should admit"),
            CanonicalFieldPath::new(fields).expect("touch field path should admit"),
        )
    }
}
