use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey, InternedString};

use super::{
    WorthQueryGraphCallReadBinding, WorthQueryGraphCallScope, WorthQueryGraphProviderCallKind,
    WorthQueryGraphProviderCallSpec, WorthQueryGraphReadMaterial, WorthQueryGraphReadRow,
    WorthQueryGraphReceiptAdmissionDenial,
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
fn graph_product_digest_is_map_order_canonical_but_row_order_sensitive() {
    let attempt = attempt();
    let first = call(&attempt, "canonical-a");
    let second = call(&attempt, "canonical-b");
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
                .projected("second", material_with_rows(["b", "a"]))
                .unwrap(),
        )
        .unwrap();

    assert_ne!(
        first_product.graph_read_product().unwrap().result_digest(),
        second_product.graph_read_product().unwrap().result_digest()
    );
}

fn call(
    attempt: &WorthQueryDirectExecutionResourceAttempt,
    scope: &str,
) -> super::WorthQueryGraphProviderCall {
    attempt
        .provider_session()
        .bind_graph_provider_call(
            WorthQueryGraphProviderCallSpec::new(
                WorthQueryGraphProviderCallKind::Project,
                WorthQueryGraphCallScope::new(scope, "operation", "binding"),
                WorthQueryGraphCallReadBinding::new("remote", "query", "basis", "snapshot"),
            ),
            attempt.evidence(),
            attempt.resources().shared_envelope(),
        )
        .unwrap()
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

fn attempt() -> WorthQueryDirectExecutionResourceAttempt {
    WorthQueryDirectExecutionResourceAttempt::start(admitted_plan("graph-provider", 8))
}
