use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, display_field_projection_context, host_font_metrics_policy,
    host_result_font_metrics, host_result_viewport_extent, host_result_viewport_extent_with_value,
    scroll_viewport_policy, synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, certify_measurement_basis_determinism,
    consume_declared_measurement_projection_facts, MeasurementEvidenceInput,
    UiMeasurementBasisDeterminismPosture, UiMeasurementGenerationCompatibility,
};

#[test]
fn certification_reports_equivalent_basis_pairs_as_deterministic() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("basis-certification-equivalent");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("basis-certification-equivalent");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let font_metrics = host_result_font_metrics(900, &capability_report, generation);
    let viewport_extent = host_result_viewport_extent(901, &capability_report, generation);

    let first = admit_measurement_basis(
        declaration_identity.clone(),
        UiGraphNodeIdentity::new(501),
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
    let second = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(501),
        world_profile,
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::host_measurement_result(&viewport_extent),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::query_projection_fact(&receipt),
        ],
    );
    let report = certify_measurement_basis_determinism(&first, &second);

    assert_eq!(
        report.determinism_posture(),
        UiMeasurementBasisDeterminismPosture::Equivalent
    );
    assert_eq!(
        report.first_compatibility(),
        &UiMeasurementGenerationCompatibility::Compatible
    );
    assert_eq!(
        report.second_compatibility(),
        &UiMeasurementGenerationCompatibility::Compatible
    );
    assert!(report.lineage_is_narrow());
    assert!(report.neighborhoods_are_narrow());
}

#[test]
fn certification_reports_value_drift_as_divergent() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("basis-certification-divergent");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("basis-certification-divergent");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let font_metrics = host_result_font_metrics(910, &capability_report, generation);
    let viewport_a =
        host_result_viewport_extent_with_value(911, &capability_report, generation, 100.0, 50.0);
    let viewport_b =
        host_result_viewport_extent_with_value(911, &capability_report, generation, 120.0, 50.0);

    let first = admit_measurement_basis(
        declaration_identity.clone(),
        UiGraphNodeIdentity::new(601),
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
    let second = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(601),
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
    let report = certify_measurement_basis_determinism(&first, &second);

    assert_eq!(
        report.determinism_posture(),
        UiMeasurementBasisDeterminismPosture::Divergent
    );
    assert!(report.lineage_is_narrow());
}

#[test]
fn certification_preserves_typed_compatibility_on_denied_bases() {
    let stale_report = capability_report(55);
    let current_report = capability_report(77);
    let generation = UiEvidenceAuthorityGeneration::new(17);

    let first = admit_measurement_basis(
        synthetic_declaration_identity("basis-certification-stale-a"),
        UiGraphNodeIdentity::new(701),
        UiGraphWorldProfile::authoritative(),
        generation,
        &host_font_metrics_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&current_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                920,
                &stale_report,
                generation,
            )),
        ],
    );
    let second = admit_measurement_basis(
        synthetic_declaration_identity("basis-certification-stale-b"),
        UiGraphNodeIdentity::new(702),
        UiGraphWorldProfile::authoritative(),
        generation,
        &host_font_metrics_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&current_report),
            MeasurementEvidenceInput::host_measurement_result(&host_result_font_metrics(
                921,
                &stale_report,
                generation,
            )),
        ],
    );
    let report = certify_measurement_basis_determinism(&first, &second);

    assert_eq!(
        report.first_compatibility(),
        &UiMeasurementGenerationCompatibility::StaleHostCapability {
            expected: current_report.observation_generation(),
            observed: stale_report.observation_generation(),
        }
    );
    assert_eq!(report.first_compatibility(), report.second_compatibility());
    assert!(report.lineage_is_narrow());
}

#[test]
fn certification_treats_child_intrinsic_evidence_as_real_lineage_support() {
    let generation = UiEvidenceAuthorityGeneration::new(27);
    let declaration_identity = synthetic_declaration_identity("basis-certification-child");
    let child_node = UiGraphNodeIdentity::new(801);
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("basis-certification-child");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");

    let first = admit_measurement_basis(
        declaration_identity.clone(),
        UiGraphNodeIdentity::new(800),
        world_profile.clone(),
        generation,
        &scroll_viewport_policy(),
        &[MeasurementEvidenceInput::child_query_projection_fact(
            child_node, &receipt,
        )],
    );
    let second = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(800),
        world_profile,
        generation,
        &scroll_viewport_policy(),
        &[MeasurementEvidenceInput::child_query_projection_fact(
            child_node, &receipt,
        )],
    );
    let report = certify_measurement_basis_determinism(&first, &second);

    assert_eq!(
        report.determinism_posture(),
        UiMeasurementBasisDeterminismPosture::Equivalent
    );
    assert!(report.lineage_is_narrow());
}
