use std::collections::BTreeMap;

use worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration;
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::UiDeclaredMeasurementBasisSource;
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_font_metrics_policy,
    host_result_font_metrics, host_result_portal_anchor, host_result_viewport_extent,
    host_result_viewport_extent_with_value, scroll_viewport_policy, synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, consume_declared_measurement_projection_facts,
    MeasurementEvidenceInput, UiMeasurementBasisDenial, UiMeasurementDependencyLineageEntry,
    UiMeasurementDependencyLineageKind, UiMeasurementEvidenceSlot,
    UiMeasurementGenerationCompatibility, UiMeasurementNeighborhoodClassHint,
};

#[test]
fn equivalent_authority_and_evidence_inputs_converge_to_the_same_basis() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("basis-deterministic");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("basis-deterministic");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let font_metrics = host_result_font_metrics(100, &capability_report, generation);
    let viewport_extent = host_result_viewport_extent(101, &capability_report, generation);
    let extra_portal = host_result_portal_anchor(102, &capability_report, generation);

    let basis_a = admit_measurement_basis(
        declaration_identity.clone(),
        UiGraphNodeIdentity::new(91),
        world_profile.clone(),
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
            MeasurementEvidenceInput::host_measurement_result(&viewport_extent),
        ],
    );
    let basis_b = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(91),
        world_profile,
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::host_measurement_result(&extra_portal),
            MeasurementEvidenceInput::host_measurement_result(&viewport_extent),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::query_projection_fact(&receipt),
        ],
    );

    assert!(basis_a.is_admitted());
    assert!(basis_b.is_admitted());
    assert_eq!(basis_a.identity_digest(), basis_b.identity_digest());
    assert_eq!(basis_a.generation(), basis_b.generation());
    assert_eq!(basis_a.dependency_lineage(), basis_b.dependency_lineage());
    assert_eq!(basis_a.evidence_inputs().len(), 4);
    assert_eq!(
        basis_a.neighborhood_class_hint(),
        UiMeasurementNeighborhoodClassHint::ViewportDependency
    );
}

#[test]
fn stale_host_capability_input_denies_with_typed_generation_compatibility() {
    let stale_report = capability_report(55);
    let current_report = capability_report(77);
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let denial = admit_measurement_basis(
        synthetic_declaration_identity("host-only-stale"),
        UiGraphNodeIdentity::new(9),
        UiGraphWorldProfile::authoritative(),
        generation,
        &host_font_metrics_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&current_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                200,
                &stale_report,
                generation,
            )),
        ],
    );

    assert!(!denial.is_admitted());
    assert_eq!(
        denial.generation_compatibility(),
        &UiMeasurementGenerationCompatibility::StaleHostCapability {
            expected: WorthUiHostCapabilityObservationGeneration::new(77),
            observed: WorthUiHostCapabilityObservationGeneration::new(55),
        }
    );
    assert_eq!(
        denial.denial_posture(),
        Some(&UiMeasurementBasisDenial::GenerationIncompatible {
            compatibility: UiMeasurementGenerationCompatibility::StaleHostCapability {
                expected: WorthUiHostCapabilityObservationGeneration::new(77),
                observed: WorthUiHostCapabilityObservationGeneration::new(55),
            },
        })
    );
}

#[test]
fn contradictory_or_partial_inputs_deny_structurally() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("basis-structural");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("basis-structural");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let font_metrics = host_result_font_metrics(300, &capability_report, generation);

    let missing_viewport = admit_measurement_basis(
        declaration_identity.clone(),
        UiGraphNodeIdentity::new(31),
        world_profile.clone(),
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
        ],
    );
    assert!(!missing_viewport.is_admitted());
    assert_eq!(
        missing_viewport.denial_posture(),
        Some(&UiMeasurementBasisDenial::MissingBasisSourceEvidence {
            basis_source: UiDeclaredMeasurementBasisSource::ScrollViewport,
            slot: UiMeasurementEvidenceSlot::ViewportExtent,
        })
    );

    let missing_query = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(32),
        world_profile,
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
            MeasurementEvidenceInput::host_measurement_result(&host_result_viewport_extent(
                301,
                &capability_report,
                generation,
            )),
        ],
    );
    assert!(!missing_query.is_admitted());
    assert_eq!(
        missing_query.denial_posture(),
        Some(&UiMeasurementBasisDenial::MissingEvidence {
            slot: UiMeasurementEvidenceSlot::QueryProjectionFactReceipt,
        })
    );
}

#[test]
fn changed_host_measurement_values_change_basis_identity_and_lineage() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("basis-value-change");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("basis-value-change");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let font_metrics = host_result_font_metrics(400, &capability_report, generation);
    let viewport_a =
        host_result_viewport_extent_with_value(401, &capability_report, generation, 100.0, 50.0);
    let viewport_b =
        host_result_viewport_extent_with_value(401, &capability_report, generation, 120.0, 50.0);

    let basis_a = admit_measurement_basis(
        declaration_identity.clone(),
        UiGraphNodeIdentity::new(41),
        world_profile.clone(),
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
            MeasurementEvidenceInput::host_measurement_result(&viewport_a),
        ],
    );
    let basis_b = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(41),
        world_profile,
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
            MeasurementEvidenceInput::host_measurement_result(&viewport_b),
        ],
    );

    assert!(basis_a.is_admitted());
    assert!(basis_b.is_admitted());
    assert_ne!(basis_a.identity_digest(), basis_b.identity_digest());
    assert_ne!(
        basis_a.dependency_lineage().identity_digest(),
        basis_b.dependency_lineage().identity_digest()
    );
    let lineage_a = lineage_entries_by_kind(basis_a.dependency_lineage().entries());
    let lineage_b = lineage_entries_by_kind(basis_b.dependency_lineage().entries());
    assert_eq!(
        lineage_a[&UiMeasurementDependencyLineageKind::QueryScrollContentExtent],
        lineage_b[&UiMeasurementDependencyLineageKind::QueryScrollContentExtent]
    );
    assert_eq!(
        lineage_a[&UiMeasurementDependencyLineageKind::HostFontMetrics],
        lineage_b[&UiMeasurementDependencyLineageKind::HostFontMetrics]
    );
    assert_ne!(
        lineage_a[&UiMeasurementDependencyLineageKind::HostViewportExtent],
        lineage_b[&UiMeasurementDependencyLineageKind::HostViewportExtent]
    );
}

fn lineage_entries_by_kind(
    entries: &[UiMeasurementDependencyLineageEntry],
) -> BTreeMap<UiMeasurementDependencyLineageKind, UiMeasurementDependencyLineageEntry> {
    entries
        .iter()
        .copied()
        .map(|entry| (entry.kind(), entry))
        .collect()
}
