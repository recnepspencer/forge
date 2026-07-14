use worth_ui_host_contract::WorthUiHostCapability;
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};
use crate::host::UiHostMeasurementAssumptionProfile;

use crate::evidence::measurement::projection::fact_test_support::{
    capability_report, capability_report_with_capabilities, display_field_projection_context,
    host_font_metrics_policy, host_result_font_metrics,
    host_result_font_metrics_with_assumption_profile, host_result_viewport_extent,
    scroll_viewport_policy, synthetic_declaration_identity,
};
use crate::evidence::{
    admit_measurement_basis, consume_declared_measurement_projection_facts,
    MeasurementEvidenceInput, UiMeasurementBasisDenial, UiMeasurementGenerationCompatibility,
};

#[test]
fn declaration_identity_changes_basis_identity_even_when_evidence_stays_fixed() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_a = synthetic_declaration_identity("basis-declaration-a");
    let declaration_b = synthetic_declaration_identity("basis-declaration-b");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("basis-declaration-identity");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_a.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let font_metrics = host_result_font_metrics(500, &capability_report, generation);
    let viewport_extent = host_result_viewport_extent(501, &capability_report, generation);

    let basis_a = admit_measurement_basis(
        declaration_a,
        UiGraphNodeIdentity::new(61),
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
        declaration_b,
        UiGraphNodeIdentity::new(61),
        world_profile,
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
            MeasurementEvidenceInput::host_measurement_result(&viewport_extent),
        ],
    );

    assert!(basis_a.is_admitted());
    assert!(basis_b.is_admitted());
    assert_ne!(basis_a.identity_digest(), basis_b.identity_digest());
    assert_ne!(
        basis_a.declaration_identity(),
        basis_b.declaration_identity()
    );
    assert_eq!(basis_a.generation(), basis_b.generation());
    assert_eq!(
        basis_a.generation_compatibility(),
        basis_b.generation_compatibility()
    );
}

#[test]
fn incompatible_world_is_reported_on_the_returned_basis_artifact() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("basis-wrong-world");
    let (prerequisites, attempt, _) = display_field_projection_context("basis-wrong-world");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let capability_report = capability_report(77);
    let font_metrics = host_result_font_metrics(600, &capability_report, generation);
    let viewport_extent = host_result_viewport_extent(601, &capability_report, generation);

    let basis = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(71),
        UiGraphWorldProfile::authoritative(),
        generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
            MeasurementEvidenceInput::host_measurement_result(&viewport_extent),
        ],
    );

    let compatibility = UiMeasurementGenerationCompatibility::IncompatibleWorld {
        expected_query_basis_digest: receipt.query_basis_digest().into(),
        observed_world_basis_digest: None,
    };
    assert!(!basis.is_admitted());
    assert_eq!(basis.generation_compatibility(), &compatibility);
    assert_eq!(
        basis.denial_posture(),
        Some(&UiMeasurementBasisDenial::GenerationIncompatible { compatibility })
    );
}

#[test]
fn stale_query_fact_receipt_is_reported_on_the_returned_basis_artifact() {
    let receipt_generation = UiEvidenceAuthorityGeneration::new(17);
    let declaration_identity = synthetic_declaration_identity("basis-stale-query-receipt");
    let (prerequisites, attempt, world_profile) =
        display_field_projection_context("basis-stale-query-receipt");
    let receipt = consume_declared_measurement_projection_facts(
        declaration_identity.clone(),
        receipt_generation,
        &scroll_viewport_policy(),
        prerequisites,
        &attempt,
    )
    .expect("query receipt should admit");
    let current_generation = UiEvidenceAuthorityGeneration::new(18);
    let capability_report = capability_report(77);
    let font_metrics = host_result_font_metrics(700, &capability_report, current_generation);
    let viewport_extent = host_result_viewport_extent(701, &capability_report, current_generation);

    let basis = admit_measurement_basis(
        declaration_identity,
        UiGraphNodeIdentity::new(81),
        world_profile,
        current_generation,
        &scroll_viewport_policy(),
        &[
            MeasurementEvidenceInput::query_projection_fact(&receipt),
            MeasurementEvidenceInput::host_capability_report(&capability_report),
            MeasurementEvidenceInput::host_measurement_result(&font_metrics),
            MeasurementEvidenceInput::host_measurement_result(&viewport_extent),
        ],
    );

    let compatibility = UiMeasurementGenerationCompatibility::StaleQueryFactReceipt {
        expected: current_generation,
        observed: receipt_generation,
    };
    assert!(!basis.is_admitted());
    assert_eq!(basis.generation_compatibility(), &compatibility);
    assert_eq!(
        basis.denial_posture(),
        Some(&UiMeasurementBasisDenial::GenerationIncompatible { compatibility })
    );
}

#[test]
fn incompatible_host_profile_is_reported_on_the_returned_basis_artifact() {
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let current_report = capability_report(77);
    let mismatched_profile_report =
        capability_report_with_capabilities(77, vec![WorthUiHostCapability::FontMetrics]);
    let mismatched_assumption_profile = UiHostMeasurementAssumptionProfile::from_capability_report(
        &mismatched_profile_report,
        11,
        22,
        33,
        44,
    );
    let basis = admit_measurement_basis(
        synthetic_declaration_identity("basis-host-profile"),
        UiGraphNodeIdentity::new(91),
        UiGraphWorldProfile::authoritative(),
        generation,
        &host_font_metrics_policy(),
        &[
            MeasurementEvidenceInput::host_capability_report(&current_report),
            MeasurementEvidenceInput::host_measurement_result(
                &host_result_font_metrics_with_assumption_profile(
                    800,
                    &current_report,
                    generation,
                    mismatched_assumption_profile,
                ),
            ),
        ],
    );

    let compatibility = UiMeasurementGenerationCompatibility::IncompatibleHostProfile {
        expected_profile_digest: current_report.profile_identity_digest(),
        observed_profile_digest: mismatched_assumption_profile.capability_profile_digest(),
    };
    assert!(!basis.is_admitted());
    assert_eq!(basis.generation_compatibility(), &compatibility);
    assert_eq!(
        basis.denial_posture(),
        Some(&UiMeasurementBasisDenial::GenerationIncompatible { compatibility })
    );
}
