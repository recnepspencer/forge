use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey, InternedString};

use super::{
    WorthQueryGraphCallBindingDenial, WorthQueryGraphCallReadBinding, WorthQueryGraphCallScope,
    WorthQueryGraphProviderCallKind, WorthQueryGraphProviderCallSpec, WorthQueryGraphReadMaterial,
    WorthQueryGraphReadRow, WorthQueryGraphReceiptAdmissionDenial,
};
use crate::domain_computation::provider_session::tests::admitted_plan;
use crate::domain_computation::provider_session::WorthQueryDirectExecutionResourceAttempt;

#[test]
fn retained_graph_call_cannot_bind_a_later_call_receipt() {
    let attempt = attempt();
    let first = call(&attempt, "first");
    let second = call(&attempt, "second");
    let foreign_receipt = first.projected("first", material("first")).unwrap();

    assert_eq!(
        second.admit_receipt(foreign_receipt).unwrap_err(),
        WorthQueryGraphReceiptAdmissionDenial::ForeignCall
    );
}

#[test]
fn equal_semantic_results_keep_distinct_call_and_product_occurrences() {
    let first_attempt = attempt();
    let second_attempt = attempt();
    let first = call(&first_attempt, "canonical-a");
    let second = call(&second_attempt, "canonical-b");
    let first_product = first
        .admit_receipt(
            first
                .projected("first", material_with_rows(["a", "b"]))
                .unwrap(),
        )
        .unwrap();
    let second_product = second
        .admit_receipt(
            second
                .projected("second", material_with_rows(["a", "b"]))
                .unwrap(),
        )
        .unwrap();

    let first_product = first_product.graph_read_product().unwrap();
    let second_product = second_product.graph_read_product().unwrap();
    assert_eq!(
        first_product.result_digest(),
        second_product.result_digest()
    );
    assert_ne!(
        first_product.call_identity(),
        second_product.call_identity()
    );
    assert_ne!(first_product.identity(), second_product.identity());
}

#[test]
fn graph_product_digest_is_canonical_across_field_insertion_order() {
    let attempt = attempt();
    let first = call(&attempt, "field-order-a");
    let second = call(&attempt, "field-order-b");
    let first_receipt = first
        .admit_receipt(
            first
                .projected("first", material_with_field_order(false))
                .unwrap(),
        )
        .unwrap();
    let second_receipt = second
        .admit_receipt(
            second
                .projected("second", material_with_field_order(true))
                .unwrap(),
        )
        .unwrap();

    assert_eq!(
        first_receipt.graph_read_product().unwrap().result_digest(),
        second_receipt.graph_read_product().unwrap().result_digest()
    );
}

#[test]
fn graph_product_digest_preserves_provider_row_order() {
    let attempt = attempt();
    let first = call(&attempt, "row-order-a");
    let second = call(&attempt, "row-order-b");
    let first_receipt = first
        .admit_receipt(
            first
                .projected("first", material_with_rows(["a", "b"]))
                .unwrap(),
        )
        .unwrap();
    let second_receipt = second
        .admit_receipt(
            second
                .projected("second", material_with_rows(["b", "a"]))
                .unwrap(),
        )
        .unwrap();

    assert_ne!(
        first_receipt.graph_read_product().unwrap().result_digest(),
        second_receipt.graph_read_product().unwrap().result_digest()
    );
}

#[test]
fn graph_product_digest_changes_when_a_field_value_changes() {
    let first_attempt = attempt();
    let second_attempt = attempt();
    let first = call(&first_attempt, "field-value-a");
    let second = call(&second_attempt, "field-value-b");
    let first_receipt = first
        .admit_receipt(
            first
                .projected("first", material_with_identity_value("vertex-a"))
                .unwrap(),
        )
        .unwrap();
    let second_receipt = second
        .admit_receipt(
            second
                .projected("second", material_with_identity_value("vertex-b"))
                .unwrap(),
        )
        .unwrap();

    assert_ne!(
        first_receipt.graph_read_product().unwrap().result_digest(),
        second_receipt.graph_read_product().unwrap().result_digest()
    );
}

#[test]
fn non_projection_call_cannot_seal_projection_material() {
    let attempt = attempt();
    let call = call_with_kind(
        &attempt,
        "observe-cannot-project",
        WorthQueryGraphProviderCallKind::Observe,
    );

    assert!(call
        .projected("unexpected", material("unexpected"))
        .is_err());
}

#[test]
fn provider_session_rejects_resources_admitted_for_another_session() {
    let owner_attempt = attempt();
    let foreign_attempt = attempt();
    let denial = owner_attempt
        .provider_session()
        .bind_graph_provider_call(
            call_spec(
                "foreign-resource-attempt",
                WorthQueryGraphProviderCallKind::Project,
            ),
            foreign_attempt.evidence(),
            foreign_attempt.resources().shared_envelope(),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        WorthQueryGraphCallBindingDenial::ForeignResourceAttempt
    );
}

fn call(
    attempt: &WorthQueryDirectExecutionResourceAttempt,
    scope: &str,
) -> super::WorthQueryGraphProviderCall {
    call_with_kind(attempt, scope, WorthQueryGraphProviderCallKind::Project)
}

fn call_with_kind(
    attempt: &WorthQueryDirectExecutionResourceAttempt,
    scope: &str,
    kind: WorthQueryGraphProviderCallKind,
) -> super::WorthQueryGraphProviderCall {
    attempt
        .provider_session()
        .bind_graph_provider_call(
            call_spec(scope, kind),
            attempt.evidence(),
            attempt.resources().shared_envelope(),
        )
        .unwrap()
}

fn call_spec(
    scope: &str,
    kind: WorthQueryGraphProviderCallKind,
) -> WorthQueryGraphProviderCallSpec {
    WorthQueryGraphProviderCallSpec::new(
        kind,
        WorthQueryGraphCallScope::new(scope, "operation", "binding"),
        WorthQueryGraphCallReadBinding::new("remote", "query", "basis", "snapshot"),
    )
}

fn material(label: &str) -> WorthQueryGraphReadMaterial {
    material_with_rows([label])
}

fn material_with_rows<const N: usize>(labels: [&str; N]) -> WorthQueryGraphReadMaterial {
    WorthQueryGraphReadMaterial::new(labels.into_iter().map(|label| {
        let field = CanonicalFieldPath::single(FieldKey::new("id").unwrap());
        let values = BTreeMap::from([(field, AspectValue::String(InternedString::from(label)))]);
        WorthQueryGraphReadRow::from_native_fields(label, values).unwrap()
    }))
}

fn material_with_field_order(reverse: bool) -> WorthQueryGraphReadMaterial {
    let identity_path = CanonicalFieldPath::single(FieldKey::new("id").unwrap());
    let kind_path = CanonicalFieldPath::single(FieldKey::new("kind").unwrap());
    let mut values = BTreeMap::new();
    let identity = AspectValue::String(InternedString::from("vertex-a"));
    let kind = AspectValue::String(InternedString::from("vertex"));
    if reverse {
        values.insert(kind_path, kind);
        values.insert(identity_path, identity);
    } else {
        values.insert(identity_path, identity);
        values.insert(kind_path, kind);
    }
    WorthQueryGraphReadMaterial::new([WorthQueryGraphReadRow::from_native_fields(
        "vertex-a", values,
    )
    .unwrap()])
}

fn material_with_identity_value(value: &str) -> WorthQueryGraphReadMaterial {
    let identity_path = CanonicalFieldPath::single(FieldKey::new("id").unwrap());
    let values = BTreeMap::from([(
        identity_path,
        AspectValue::String(InternedString::from(value)),
    )]);
    WorthQueryGraphReadMaterial::new([WorthQueryGraphReadRow::from_native_fields(
        "stable-entity",
        values,
    )
    .unwrap()])
}

fn attempt() -> WorthQueryDirectExecutionResourceAttempt {
    WorthQueryDirectExecutionResourceAttempt::start(admitted_plan("graph-provider", 8))
}
