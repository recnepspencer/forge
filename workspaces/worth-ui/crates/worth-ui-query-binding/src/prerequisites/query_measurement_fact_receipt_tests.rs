use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::runtime::{
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, WorthQueryReadBuilder, WorthQueryReadDenial,
    WorthQueryReadFamily, WorthQueryReadGraph, WorthQueryWorkspace,
};
use worth_query::facade::certification::public_bridge_projection_artifacts_for_read_graph;
use worth_query::facade::foundation::{
    resolve_runtime_current_snapshot_basis,
    snapshot_resolution_report,
    AspectFieldSelector,
    AuthoredResultShapeField,
    EqualityPredicate,
    ProjectionAuthorityContract,
    ProjectionAuthorityOutcome,
    ProjectionFactFieldPath,
    ScalarPredicateValue,
};
use worth_query::facade::runtime::{
    WorthQueryAspectTouch,
    WorthQueryAuthoredAspectValue,
};

use crate::{
    WorthUiQueryBindingSubsystem, WorthUiQueryMeasurementFactFamily,
    WorthUiQueryPrerequisiteEvidence,
};

#[test]
fn ordinary_projection_consumption_preserves_settled_posture() {
    let (prerequisites, attempt) = display_field_projection_consumption("settled-posture");
    let mut binding = WorthUiQueryBindingSubsystem::bootstrap();
    let settlement = binding
        .allocation_admission()
        .admit(prerequisites, attempt)
        .expect("ordinary projection consumption should settle");

    assert!(!settlement.is_partial());
    assert!(!settlement.allocation_source_identity().as_str().is_empty());
    assert_ne!(settlement.allocation_source_generation().as_u64(), 0);
    assert_ne!(settlement.allocation_source_order().as_u64(), 0);
    let invalidation_basis = settlement.allocation_invalidation_basis();
    assert!(settlement
        .receipt()
        .query_authority()
        .shares_authority_with(invalidation_basis.query_authority()));
}

#[test]
fn allocation_source_authority_separates_stable_identity_order_and_basis_generation() {
    let (first_prerequisites, first_attempt) = display_field_projection_consumption("order-one");
    let (second_prerequisites, second_attempt) = display_field_projection_consumption("order-two");
    let mut binding = WorthUiQueryBindingSubsystem::bootstrap();
    let first = binding
        .allocation_admission()
        .admit(first_prerequisites, first_attempt)
        .expect("first Query source should admit");
    let second = binding
        .allocation_admission()
        .admit(second_prerequisites, second_attempt)
        .expect("second Query source should admit");

    assert_eq!(first.allocation_source_generation().as_u64(), 1);
    assert_eq!(first.allocation_source_order().as_u64(), 1);
    assert_eq!(second.allocation_source_generation().as_u64(), 1);
    assert_eq!(second.allocation_source_order().as_u64(), 2);
    assert_eq!(
        first.allocation_source_identity(),
        second.allocation_source_identity()
    );
}

#[test]
fn equivalent_query_source_reuses_identity_without_reusing_order() {
    let (first_prerequisites, first_attempt) =
        display_field_projection_consumption("stable-source");
    let (second_prerequisites, second_attempt) =
        display_field_projection_consumption("stable-source");
    let mut binding = WorthUiQueryBindingSubsystem::bootstrap();
    let first = binding
        .allocation_admission()
        .admit(first_prerequisites, first_attempt)
        .unwrap();
    let second = binding
        .allocation_admission()
        .admit(second_prerequisites, second_attempt)
        .unwrap();

    assert_eq!(
        first.allocation_source_identity(),
        second.allocation_source_identity()
    );
    assert_eq!(
        first.allocation_source_generation(),
        second.allocation_source_generation()
    );
    assert_eq!(first.allocation_source_order().as_u64(), 1);
    assert_eq!(second.allocation_source_order().as_u64(), 2);
    assert_eq!(
        first.receipt().authority_index_key(),
        second.receipt().authority_index_key(),
        "equivalent Query measurement consumption retains one nominal receipt authority"
    );
    assert_ne!(
        first.allocation_invalidation_basis().consumption_identity(),
        second
            .allocation_invalidation_basis()
            .consumption_identity(),
        "allocation settlement order remains part of the richer Query authority"
    );
    assert!(first
        .allocation_source_identity()
        .shares_storage_with(second.allocation_source_identity()));
}

#[test]
fn repeated_settlements_share_query_owned_identity_storage() {
    let mut binding = WorthUiQueryBindingSubsystem::bootstrap();
    let settlements = (1..=64)
        .map(|expected_order| {
            let (prerequisites, attempt) =
                display_field_projection_consumption("shared-source-identity");
            let settlement = binding
                .allocation_admission()
                .admit(prerequisites, attempt)
                .expect("Query burst fact should admit");
            assert_eq!(
                settlement.allocation_source_order().as_u64(),
                expected_order
            );
            settlement
        })
        .collect::<Vec<_>>();
    let canonical_identity = settlements[0].allocation_source_identity();

    assert!(settlements.iter().all(|settlement| canonical_identity
        .shares_storage_with(settlement.allocation_source_identity())));
}

#[test]
fn measurement_fact_receipts_follow_real_projection_consumption_and_preserve_identity() {
    let (prerequisites, attempt) = display_field_projection_consumption("receipt-identity");

    let (authority, _) = attempt
        .into_admitted()
        .expect("Query authority should admit");
    let receipt = WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .measurement_fact_receipt_from_query_authority(
            prerequisites,
            super::WorthUiQueryAuthorityHandle::retain(authority),
        )
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
    assert_eq!(
        receipt
            .query_authority()
            .authority()
            .receipt()
            .receipt_digest(),
        receipt.projection_consumption_receipt_digest()
    );
}

#[test]
fn equivalent_projection_consumption_paths_yield_equivalent_measurement_fact_receipts() {
    let (left_prerequisites, left_attempt) = display_field_projection_consumption("equivalent");
    let (right_prerequisites, right_attempt) = display_field_projection_consumption("equivalent");

    let (left_authority, _) = left_attempt
        .into_admitted()
        .expect("left authority should admit");
    let left = WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .measurement_fact_receipt_from_query_authority(
            left_prerequisites,
            super::WorthUiQueryAuthorityHandle::retain(left_authority),
        )
        .expect("left display consumption should admit");
    let (right_authority, _) = right_attempt
        .into_admitted()
        .expect("right authority should admit");
    let right = WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .measurement_fact_receipt_from_query_authority(
            right_prerequisites,
            super::WorthUiQueryAuthorityHandle::retain(right_authority),
        )
        .expect("right display consumption should admit");

    assert_eq!(left, right);
}

#[test]
fn foreign_query_basis_denies_before_worth_ui_settlement_exists() {
    let (first_prerequisites, _) =
        display_field_projection_consumption_with_extent("foreign-left", "240");
    let (_, foreign_authority) =
        display_field_projection_consumption_with_extent("foreign-right", "241");

    let denial = WorthUiQueryBindingSubsystem::bootstrap()
        .allocation_admission()
        .admit(first_prerequisites, foreign_authority)
        .expect_err("cross-basis authority must deny before settlement");

    assert_eq!(
        denial,
        super::WorthUiQueryMeasurementFactSettlementDenial::Receipt(
            super::WorthUiQueryMeasurementFactReceiptError::BasisDigestMismatch,
        )
    );
}

fn display_field_projection_consumption(
    lane_label: &str,
) -> (WorthUiQueryPrerequisiteEvidence, ProjectionAuthorityOutcome) {
    display_field_projection_consumption_with_extent(lane_label, "240")
}

fn display_field_projection_consumption_with_extent(
    lane_label: &str,
    extent: &str,
) -> (WorthUiQueryPrerequisiteEvidence, ProjectionAuthorityOutcome) {
    let (mut workspace, family) = measurement_projection_workspace(lane_label, extent);
    let read_result = workspace
        .execute_read_family(&family)
        .expect("query read family should execute");
    let (result_shape, authorized_projection) =
        public_bridge_projection_artifacts_for_read_graph(family.read_graph());
    let outcome = read_result
        .consume_projection_authority(
            &result_shape,
            &authorized_projection,
            ProjectionAuthorityContract::declare()
                .require_settled_consumption()
                .require_source_authority()
                .require_display_field(size_value_field_path()),
        )
        .expect("real query read should consume projection authority");
    let basis = resolve_runtime_current_snapshot_basis(
        workspace.snapshot_identity().evidence_identity(),
        family.read_graph().schema_basis_authority(),
    )
    .expect("runtime current snapshot basis should resolve");
    let prerequisites = WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .graph_aligned(basis.clone(), snapshot_resolution_report(&basis))
        .expect("query prerequisites should admit");
    (prerequisites, outcome)
}

fn measurement_projection_workspace(
    lane_label: &str,
    extent: &str,
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
                WorthQueryAuthoredAspectValue::string(extent),
            )
        })
        .expect("fixture insert should admit");
    if extent != "240" {
        workspace
            .insert("task", |task| {
                task.set_aspect(
                    aspect_touch("identity.id"),
                    WorthQueryAuthoredAspectValue::string("unrelated"),
                )
                .set_aspect(
                    aspect_touch("size.value"),
                    WorthQueryAuthoredAspectValue::string("0"),
                )
            })
            .expect("generation-drift fixture insert should admit");
    }
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
