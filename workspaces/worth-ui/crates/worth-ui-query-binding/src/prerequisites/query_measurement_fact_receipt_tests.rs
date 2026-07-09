use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::runtime::{
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, WorthQueryReadBuilder, WorthQueryReadDenial,
    WorthQueryReadFamily, WorthQueryReadGraph, WorthQueryWorkspace,
};
use worth_query::facade::{
    public_bridge_projection_artifacts_for_read_graph, resolve_runtime_current_snapshot_basis,
    snapshot_resolution_report, AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    ProjectMaterializedFacts, ProjectionFactConsumptionAttempt, ProjectionFactFieldPath,
    ScalarPredicateValue, WorthQueryAspectTouch, WorthQueryAuthoredAspectValue,
};

use crate::{
    WorthUiQueryBindingSubsystem, WorthUiQueryMeasurementFactFamily,
    WorthUiQueryPrerequisiteEvidence,
};

#[test]
fn measurement_fact_receipts_follow_real_projection_consumption_and_preserve_identity() {
    let (prerequisites, attempt) = display_field_projection_consumption("receipt-identity");

    let receipt = WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .measurement_fact_receipt_from_projection_consumption(prerequisites, &attempt)
        .expect(
            "display-field projection consumption should admit a query measurement fact receipt",
        );

    assert_eq!(
        receipt.consumed_families(),
        &[WorthUiQueryMeasurementFactFamily::ScrollContentExtent]
    );
    assert_eq!(
        receipt.prerequisites().projection_contract_digest(),
        Some(receipt.projection_contract_digest())
    );
    assert_eq!(receipt.observations().len(), 1);
    assert_eq!(receipt.observations()[0].extent(), 240.0);
    assert!(!receipt
        .projection_consumption_declaration_digest()
        .is_empty());
    assert!(!receipt.projection_consumption_receipt_digest().is_empty());
    assert!(!receipt.projection_fact_set_digest().is_empty());
    assert!(!receipt.projection_source_identity().is_empty());
}

#[test]
fn equivalent_projection_consumption_paths_yield_equivalent_measurement_fact_receipts() {
    let (left_prerequisites, left_attempt) = display_field_projection_consumption("equivalent");
    let (right_prerequisites, right_attempt) = display_field_projection_consumption("equivalent");

    let left = WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .measurement_fact_receipt_from_projection_consumption(left_prerequisites, &left_attempt)
        .expect("left display consumption should admit");
    let right = WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .measurement_fact_receipt_from_projection_consumption(right_prerequisites, &right_attempt)
        .expect("right display consumption should admit");

    assert_eq!(left, right);
}

fn display_field_projection_consumption(
    lane_label: &str,
) -> (
    WorthUiQueryPrerequisiteEvidence,
    ProjectionFactConsumptionAttempt,
) {
    let (mut workspace, family) = measurement_projection_workspace(lane_label);
    let read_result = workspace
        .execute_read_family(&family)
        .expect("query read family should execute");
    let (result_shape, authorized_projection) =
        public_bridge_projection_artifacts_for_read_graph(family.read_graph());
    let attempt = read_result
        .consume_projection_facts(
            &result_shape,
            &authorized_projection,
            ProjectMaterializedFacts::declare().display_field_path(size_value_field_path()),
        )
        .expect("real query read should consume projection facts");
    let basis = resolve_runtime_current_snapshot_basis(
        workspace.snapshot_identity().evidence_identity(),
        family.read_graph().schema_basis().clone(),
    )
    .expect("runtime current snapshot basis should resolve");
    let prerequisites = WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .graph_aligned(basis.clone(), snapshot_resolution_report(&basis))
        .expect("query prerequisites should admit");
    (prerequisites, attempt)
}

fn measurement_projection_workspace(
    lane_label: &str,
) -> (WorthQueryWorkspace, WorthQueryReadFamily) {
    let schema = WorthQueryTestBackendSchema::single_collection("task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should admit")
        .aspect("size.value", "size.value")
        .expect("size aspect should admit");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .workspace(&format!("worth-ui.phase8.query-fact-receipt.{lane_label}"))
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
    let family = workspace
        .define_read_family(
            &format!("worth-ui.phase8.query-fact-receipt.{lane_label}"),
            size_family_graph,
        )
        .expect("query read family should admit");
    (workspace, family)
}

fn size_family_graph(
    read: WorthQueryReadBuilder,
) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
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
                worth_query::facade::AspectName::new("identity")
                    .expect("schema aspect should admit"),
                worth_query::facade::FieldName::new("id").expect("schema field should admit"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                worth_query::facade::AspectName::new("size").expect("schema aspect should admit"),
                worth_query::facade::FieldName::new("value").expect("schema field should admit"),
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
