use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementPolicyPosture,
};

use super::fact_test_support::{
    display_field_projection_consumption, entity_identity_projection_context,
    synthetic_declaration_identity,
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
    let (prerequisites, attempt, _) = entity_identity_projection_context("missing");

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
