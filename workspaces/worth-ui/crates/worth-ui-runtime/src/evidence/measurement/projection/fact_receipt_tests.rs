use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementPolicyPosture,
};

use super::fact_test_support::{
    display_field_projection_consumption, synthetic_declaration_identity,
};
use crate::evidence::consume_declared_measurement_projection_facts;

#[test]
fn projection_fact_receipts_preserve_declaration_dependency_identity_for_basis_assembly() {
    let (view_binding_id, fact) = display_field_projection_consumption("basis-assembly");
    let receipt = consume_declared_measurement_projection_facts(
        synthetic_declaration_identity("basis-assembly"),
        UiEvidenceAuthorityGeneration::new(17),
        &scroll_measurement_policy(true),
        view_binding_id.clone(),
        &fact,
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
    assert_eq!(
        receipt.observations()[0].extent(),
        worth_foundational::facade::CanonicalF32::from_f32(240.0)
    );
    assert_eq!(
        receipt.required_query_fact_family_set_digest(),
        receipt.consumed_fact_family_set_digest()
    );
    assert_eq!(receipt.view_binding_id(), &view_binding_id);
    assert_eq!(receipt.binding_reference(), fact.binding_reference());
    assert_eq!(receipt.settlement_reference(), fact.settlement_reference());
    assert_ne!(receipt.observation_identity_digest(), 0);
}

#[test]
fn non_query_measurement_dependencies_do_not_widen_query_projection_receipt_identity() {
    let (view_binding_id, fact) = display_field_projection_consumption("narrowing");
    let with_host_dependency = consume_declared_measurement_projection_facts(
        synthetic_declaration_identity("with-host"),
        UiEvidenceAuthorityGeneration::new(17),
        &scroll_measurement_policy(true),
        view_binding_id.clone(),
        &fact,
    )
    .expect("host-plus-query measurement should admit");
    let query_only = consume_declared_measurement_projection_facts(
        synthetic_declaration_identity("query-only"),
        UiEvidenceAuthorityGeneration::new(17),
        &scroll_measurement_policy(false),
        view_binding_id,
        &fact,
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
