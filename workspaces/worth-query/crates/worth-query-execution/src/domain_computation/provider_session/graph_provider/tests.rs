use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey, InternedString};
use worth_query_installation::facade::{
    WorthQueryInstallationGeneration, WorthQueryInstalledGraphParticipationAuthority,
};

use super::{
    WorthQueryGraphCallBindingDenial, WorthQueryGraphProviderCallKind,
    WorthQueryGraphProviderCallRequest, WorthQueryGraphReadMaterial, WorthQueryGraphReadRow,
    WorthQueryGraphReceiptAdmissionDenial, WorthQueryProviderWorkReport,
};
use crate::domain_computation::execution_runtime::WorthQueryExecutionRuntimeInstaller;
use crate::domain_computation::operation_binding::direct_authority_with_graph;
use crate::domain_computation::provider_session::tests::admitted_plan;
use crate::domain_computation::provider_session::WorthQueryDirectExecutionResourceAttempt;

struct GraphAttempt {
    attempt: WorthQueryDirectExecutionResourceAttempt,
    graph: WorthQueryInstalledGraphParticipationAuthority,
    foreign_graph: WorthQueryInstalledGraphParticipationAuthority,
}

#[test]
fn retained_graph_call_cannot_bind_a_later_call_receipt() {
    let attempt = attempt();
    let first = call(&attempt, "first");
    let second = call(&attempt, "second");
    let foreign_receipt = first
        .projected("first", material("first"), projected_work_report())
        .unwrap();

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
                .projected(
                    "first",
                    material_with_rows(["a", "b"]),
                    projected_work_report(),
                )
                .unwrap(),
        )
        .unwrap();
    let second_product = second
        .admit_receipt(
            second
                .projected(
                    "second",
                    material_with_rows(["a", "b"]),
                    projected_work_report(),
                )
                .unwrap(),
        )
        .unwrap();

    let first_product = first_product.graph_read_product().unwrap();
    let second_product = second_product.graph_read_product().unwrap();
    assert_eq!(first_product.rows(), second_product.rows());
    assert_ne!(
        first_product.call_identity(),
        second_product.call_identity()
    );
    assert_ne!(first_product.identity(), second_product.identity());
}

#[test]
fn graph_product_rows_are_canonical_across_field_insertion_order() {
    let attempt = attempt();
    let first = call(&attempt, "field-order-a");
    let second = call(&attempt, "field-order-b");
    let first_receipt = first
        .admit_receipt(
            first
                .projected(
                    "first",
                    material_with_field_order(false),
                    projected_work_report(),
                )
                .unwrap(),
        )
        .unwrap();
    let second_receipt = second
        .admit_receipt(
            second
                .projected(
                    "second",
                    material_with_field_order(true),
                    projected_work_report(),
                )
                .unwrap(),
        )
        .unwrap();

    assert_eq!(
        first_receipt.graph_read_product().unwrap().rows(),
        second_receipt.graph_read_product().unwrap().rows()
    );
}

#[test]
fn graph_product_preserves_provider_row_order_without_hashing_rows() {
    let attempt = attempt();
    let first = call(&attempt, "row-order-a");
    let second = call(&attempt, "row-order-b");
    let first_receipt = first
        .admit_receipt(
            first
                .projected(
                    "first",
                    material_with_rows(["a", "b"]),
                    projected_work_report(),
                )
                .unwrap(),
        )
        .unwrap();
    let second_receipt = second
        .admit_receipt(
            second
                .projected(
                    "second",
                    material_with_rows(["b", "a"]),
                    projected_work_report(),
                )
                .unwrap(),
        )
        .unwrap();

    assert_ne!(
        first_receipt.graph_read_product().unwrap().rows(),
        second_receipt.graph_read_product().unwrap().rows()
    );
}

#[test]
fn graph_product_retains_changed_field_values_without_hashing_rows() {
    let first_attempt = attempt();
    let second_attempt = attempt();
    let first = call(&first_attempt, "field-value-a");
    let second = call(&second_attempt, "field-value-b");
    let first_receipt = first
        .admit_receipt(
            first
                .projected(
                    "first",
                    material_with_identity_value("vertex-a"),
                    projected_work_report(),
                )
                .unwrap(),
        )
        .unwrap();
    let second_receipt = second
        .admit_receipt(
            second
                .projected(
                    "second",
                    material_with_identity_value("vertex-b"),
                    projected_work_report(),
                )
                .unwrap(),
        )
        .unwrap();

    assert_ne!(
        first_receipt.graph_read_product().unwrap().rows(),
        second_receipt.graph_read_product().unwrap().rows()
    );
}

#[test]
fn non_projection_call_cannot_seal_projection_material() {
    let attempt = attempt_with_access(
        worth_query_installation::facade::WorthQueryOperationGraphAccess::Observe,
    );
    let call = call_with_kind(
        &attempt,
        "observe-cannot-project",
        WorthQueryGraphProviderCallKind::Observe,
    );

    assert!(call
        .projected(
            "unexpected",
            material("unexpected"),
            projected_work_report(),
        )
        .is_err());
}

fn projected_work_report() -> WorthQueryProviderWorkReport {
    WorthQueryProviderWorkReport::new(1, 0, 64, 64)
}

#[test]
fn provider_session_rejects_resources_admitted_for_another_session() {
    let owner_attempt = attempt();
    let foreign_attempt = attempt();
    let denial = owner_attempt
        .attempt
        .provider_session()
        .bind_graph_provider_call(
            &owner_attempt.graph,
            call_spec(
                "foreign-resource-attempt",
                WorthQueryGraphProviderCallKind::Project,
            ),
            foreign_attempt.attempt.evidence(),
            foreign_attempt.attempt.resources().shared_envelope(),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        WorthQueryGraphCallBindingDenial::ForeignResourceAttempt
    );
}

#[test]
fn provider_session_rejects_an_installed_but_undeclared_graph_authority() {
    let attempt = attempt();
    let denial = attempt
        .attempt
        .provider_session()
        .bind_graph_provider_call(
            &attempt.foreign_graph,
            WorthQueryGraphProviderCallRequest::direct(
                WorthQueryGraphProviderCallKind::Project,
                "foreign-graph",
            )
            .bind_execution_snapshot("snapshot"),
            attempt.attempt.evidence(),
            attempt.attempt.resources().shared_envelope(),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        WorthQueryGraphCallBindingDenial::BoundOperationAuthorityMismatch
    );
}

#[test]
fn direct_provider_session_rejects_a_caller_authored_workflow_stage() {
    let attempt = attempt();
    let denial = attempt
        .attempt
        .provider_session()
        .bind_graph_provider_call(
            &attempt.graph,
            WorthQueryGraphProviderCallRequest::workflow_stage(
                WorthQueryGraphProviderCallKind::Project,
                "invented-stage",
                "invented",
            )
            .bind_execution_snapshot("snapshot"),
            attempt.attempt.evidence(),
            attempt.attempt.resources().shared_envelope(),
        )
        .unwrap_err();

    assert_eq!(
        denial,
        WorthQueryGraphCallBindingDenial::BoundOperationAuthorityMismatch
    );
}

fn call(attempt: &GraphAttempt, scope: &str) -> super::WorthQueryGraphProviderCall {
    call_with_kind(attempt, scope, WorthQueryGraphProviderCallKind::Project)
}

fn call_with_kind(
    attempt: &GraphAttempt,
    scope: &str,
    kind: WorthQueryGraphProviderCallKind,
) -> super::WorthQueryGraphProviderCall {
    attempt
        .attempt
        .provider_session()
        .bind_graph_provider_call(
            &attempt.graph,
            call_spec(scope, kind),
            attempt.attempt.evidence(),
            attempt.attempt.resources().shared_envelope(),
        )
        .unwrap()
}

fn call_spec(
    scope: &str,
    kind: WorthQueryGraphProviderCallKind,
) -> WorthQueryGraphProviderCallRequest {
    WorthQueryGraphProviderCallRequest::direct(kind, scope).bind_execution_snapshot("snapshot")
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

fn attempt() -> GraphAttempt {
    attempt_with_access(worth_query_installation::facade::WorthQueryOperationGraphAccess::Project)
}

fn attempt_with_access(
    access: worth_query_installation::facade::WorthQueryOperationGraphAccess,
) -> GraphAttempt {
    let installer = WorthQueryExecutionRuntimeInstaller::new();
    let graph = WorthQueryInstalledGraphParticipationAuthority::install(
        installer.installation_runtime(),
        "remote",
        "test-graph-provider",
        false,
        Option::<String>::None,
        std::sync::Arc::new(()),
    )
    .unwrap();
    let foreign_graph = WorthQueryInstalledGraphParticipationAuthority::install(
        installer.installation_runtime(),
        "foreign",
        "foreign-test-graph-provider",
        false,
        Option::<String>::None,
        std::sync::Arc::new(()),
    )
    .unwrap();
    let runtime = installer
        .install(
            WorthQueryInstallationGeneration::initial(),
            std::iter::empty(),
        )
        .unwrap()
        .into_parts()
        .0;
    let resources = admitted_plan("binding", 8);
    let authority = direct_authority_with_graph(&runtime, &resources, &graph, access);
    let reserved =
        worth_query_admission::integration::reserve_execution_resource_plan(resources).unwrap();
    let attempt = WorthQueryDirectExecutionResourceAttempt::start(reserved, &authority);
    GraphAttempt {
        attempt,
        graph,
        foreign_graph,
    }
}
