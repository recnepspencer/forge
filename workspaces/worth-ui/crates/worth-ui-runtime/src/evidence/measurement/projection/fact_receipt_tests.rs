use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementPolicyPosture,
};

use super::fact_test_support::{
    display_field_projection_consumption, synthetic_declaration_identity,
};
use crate::evidence::{
    consume_declared_measurement_projection_facts, UiProjectionFactReceiptDenial,
};

#[test]
fn projection_fact_receipts_preserve_declaration_dependency_identity_for_basis_assembly() {
    let (prerequisites, attempt) = display_field_projection_consumption("basis-assembly");
    let receipt = consume_declared_measurement_projection_facts(
        synthetic_declaration_identity("basis-assembly"),
        UiEvidenceAuthorityGeneration::new(17),
        &scroll_measurement_policy(true),
        prerequisites,
        &attempt,
    )
    .expect("scroll-backed measurement should consume projection facts into a typed receipt");

    assert_eq!(
        receipt.required_measurement_dependencies(),
        &[
            UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics,
            UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent,
        ]
    );
    assert_eq!(
        receipt.required_query_fact_families(),
        receipt.consumed_fact_families()
    );
    assert_eq!(receipt.observations().len(), 1);
    assert_eq!(receipt.observations()[0].extent(), 240.0);
    assert_eq!(
        receipt.required_query_fact_family_set_digest(),
        receipt.consumed_fact_family_set_digest()
    );
    assert!(!receipt.projection_contract_digest().is_empty());
    assert!(!receipt.projection_consumption_receipt_digest().is_empty());
    assert!(!receipt.projection_fact_set_digest().is_empty());
}

#[test]
fn non_query_measurement_dependencies_do_not_widen_query_projection_receipt_identity() {
    let (prerequisites, attempt) = display_field_projection_consumption("narrowing");
    let with_host_dependency = consume_declared_measurement_projection_facts(
        synthetic_declaration_identity("with-host"),
        UiEvidenceAuthorityGeneration::new(17),
        &scroll_measurement_policy(true),
        prerequisites.clone(),
        &attempt,
    )
    .expect("host-plus-query measurement should admit");
    let query_only = consume_declared_measurement_projection_facts(
        synthetic_declaration_identity("query-only"),
        UiEvidenceAuthorityGeneration::new(17),
        &scroll_measurement_policy(false),
        prerequisites,
        &attempt,
    )
    .expect("query-only measurement should admit");

    assert_eq!(
        with_host_dependency.required_query_fact_family_set_digest(),
        query_only.required_query_fact_family_set_digest()
    );
    assert_eq!(
        with_host_dependency.consumed_fact_family_set_digest(),
        query_only.consumed_fact_family_set_digest()
    );
}

#[test]
fn missing_query_fact_families_deny_before_best_effort_basis_assembly() {
    let (prerequisites, attempt) = entity_identity_projection_consumption("missing");

    let denial = consume_declared_measurement_projection_facts(
        synthetic_declaration_identity("missing"),
        UiEvidenceAuthorityGeneration::new(17),
        &scroll_measurement_policy(false),
        prerequisites,
        &attempt,
    )
    .expect_err(
        "entity-only projection facts should not satisfy scroll content extent measurement",
    );

    match denial {
        UiProjectionFactReceiptDenial::MissingRequiredFactFamilies { required, consumed } => {
            assert_eq!(
                required.as_ref(),
                &[worth_ui_query_binding::WorthUiQueryMeasurementFactFamily::ScrollContentExtent]
            );
            assert!(consumed.is_empty());
        }
        other => panic!("expected missing required fact families denial, got {other:?}"),
    }
}

fn scroll_measurement_policy(
    include_host_font_metrics: bool,
) -> UiDeclaredMeasurementPolicyPosture {
    let mut requirements = vec![UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent];
    if include_host_font_metrics {
        requirements.push(UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics);
    }
    UiDeclaredMeasurementPolicyPosture::new(
        Some(UiDeclaredMeasurementMode::HugHeight),
        Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        None,
        None,
        requirements,
    )
    .expect("scroll measurement policy should admit")
}

fn entity_identity_projection_consumption(
    lane_label: &str,
) -> (
    worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence,
    worth_ui_query_binding::WorthUiQueryAuthorityHandle,
) {
    projection_consumption(
        lane_label,
        authority_contract().require_entity_identities(),
    )
}

fn projection_consumption(
    lane_label: &str,
    contract: worth_query::facade::foundation::ProjectionAuthorityContract,
) -> (
    worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence,
    worth_ui_query_binding::WorthUiQueryAuthorityHandle,
) {
    let (mut workspace, family) = measurement_projection_workspace(lane_label);
    let read_result = workspace
        .execute_read_family(&family)
        .expect("query read family should execute");
    let (result_shape, authorized_projection) =
        worth_query::facade::certification::public_bridge_projection_artifacts_for_read_graph(family.read_graph());
    let outcome = read_result
        .consume_projection_authority(&result_shape, &authorized_projection, contract)
        .expect("real query read should consume projection authority");
    let basis = worth_query::facade::foundation::resolve_runtime_current_snapshot_basis(
        workspace.snapshot_identity().evidence_identity(),
        family.read_graph().schema_basis_authority(),
    )
    .expect("runtime current snapshot basis should resolve");
    let prerequisites = worth_ui_query_binding::WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .graph_aligned(
            basis.clone(),
            worth_query::facade::foundation::snapshot_resolution_report(&basis),
        )
        .expect("query prerequisites should admit");
    let (authority, _) =
        worth_ui_query_binding::WorthUiQueryAuthorityHandle::from_outcome(outcome)
            .expect("entity identity consumption should mint Query authority");
    (prerequisites, authority)
}

fn authority_contract() -> worth_query::facade::foundation::ProjectionAuthorityContract {
    worth_query::facade::foundation::ProjectionAuthorityContract::declare()
        .require_settled_consumption()
        .require_source_authority()
}

fn measurement_projection_workspace(
    lane_label: &str,
) -> (
    worth_query::facade::runtime::WorthQueryWorkspace,
    worth_query::facade::runtime::WorthQueryReadFamily,
) {
    let schema =
        worth_query::facade::consumer_kit::WorthQueryTestBackendSchema::single_collection("task")
            .aspect("identity.id", "identity.id")
            .expect("identity aspect should admit")
            .aspect("size.value", "size.value")
            .expect("size aspect should admit");
    let mut workspace = worth_query::facade::consumer_kit::in_memory_test_runtime()
        .with_schema(schema)
        .workspace(&format!(
            "worth-ui.phase8.projection-fact-receipt.{lane_label}"
        ))
        .expect("in-memory query backend should build a workspace");
    workspace
        .insert("task", |task| {
            task.set_aspect(
                aspect_touch("identity.id"),
                worth_query::facade::runtime::WorthQueryAuthoredAspectValue::string("task"),
            )
            .set_aspect(
                aspect_touch("size.value"),
                worth_query::facade::runtime::WorthQueryAuthoredAspectValue::string("240"),
            )
        })
        .expect("fixture insert should admit");
    let family = workspace
        .define_read_family(
            &format!("worth-ui.phase8.projection-fact-receipt.{lane_label}"),
            size_family_graph,
        )
        .expect("query read family should admit");
    (workspace, family)
}

fn size_family_graph(
    read: worth_query::facade::runtime::WorthQueryReadBuilder,
) -> Result<
    worth_query::facade::runtime::WorthQueryReadGraph,
    worth_query::facade::runtime::WorthQueryReadDenial,
> {
    read.local_detail(
        "task",
        task_query_schema(),
        |query| {
            query
                .where_equal(
                    worth_query::facade::foundation::EqualityPredicate::new(
                        "identity",
                        "id",
                        worth_query::facade::foundation::ScalarPredicateValue::String("task".to_string()),
                    )
                    .expect("identity anchor predicate should build"),
                )
                .project(field("size", "value"))
        },
        |shape| shape.field(result_field("size", "value", "size.value")),
    )
}

fn task_query_schema() -> worth_query::facade::runtime::QuerySchemaView {
    worth_query::facade::runtime::QuerySchemaView::new(
        "task",
        [
            worth_query::facade::runtime::SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("identity")
                    .expect("schema aspect should admit"),
                worth_query::facade::foundation::FieldName::new("id").expect("schema field should admit"),
                worth_query::facade::runtime::SchemaFieldKind::String,
            ),
            worth_query::facade::runtime::SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("size").expect("schema aspect should admit"),
                worth_query::facade::foundation::FieldName::new("value").expect("schema field should admit"),
                worth_query::facade::runtime::SchemaFieldKind::String,
            ),
        ],
        [],
    )
}

fn field(aspect: &str, field: &str) -> worth_query::facade::foundation::AspectFieldSelector {
    worth_query::facade::foundation::AspectFieldSelector::new(aspect, field)
        .expect("field selector should build")
}

fn result_field(
    aspect: &str,
    field: &str,
    delivered: &str,
) -> worth_query::facade::foundation::AuthoredResultShapeField {
    worth_query::facade::foundation::AuthoredResultShapeField::new(aspect, field, delivered)
        .expect("result-shape field should build")
}

fn aspect_touch(authored_touch_text: &str) -> worth_query::facade::runtime::WorthQueryAspectTouch {
    let mut segments = authored_touch_text.split('.');
    let aspect = segments.next().expect("touch aspect should exist");
    let fields = segments
        .map(|segment| {
            worth_foundational::facade::FieldKey::new(segment).expect("touch field should admit")
        })
        .collect::<Vec<_>>();
    if fields.is_empty() {
        worth_query::facade::runtime::WorthQueryAspectTouch::whole_aspect(
            worth_foundational::facade::AspectKey::new(aspect).expect("touch aspect should admit"),
        )
    } else {
        worth_query::facade::runtime::WorthQueryAspectTouch::aspect_field_path(
            worth_foundational::facade::AspectKey::new(aspect).expect("touch aspect should admit"),
            worth_foundational::facade::CanonicalFieldPath::new(fields)
                .expect("touch field path should admit"),
        )
    }
}
