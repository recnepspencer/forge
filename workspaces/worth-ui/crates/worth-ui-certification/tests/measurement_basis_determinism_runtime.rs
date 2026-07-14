#[path = "fixtures/measurement_basis_certification_support.rs"]
mod measurement_basis_certification_support;

use worth_ui_inspection::UiEvidenceAuthorityGeneration;
use worth_ui_runtime::facade::evidence::certify_measurement_basis_determinism_for_scenarios;
use worth_ui_runtime::facade::evidence::{
    UiMeasurementBasisDeterminismPosture, UiMeasurementGenerationCompatibility,
};

use self::measurement_basis_certification_support::{
    divergent_admitted_scenarios, equivalent_scroll_scenarios, stale_query_scenarios,
};

#[test]
fn equivalent_public_inputs_converge_to_one_basis_and_narrow_lineage() {
    let (first_adapter, first, second_adapter, second) = equivalent_scroll_scenarios();
    let outcome = certify_measurement_basis_determinism_for_scenarios(
        &first,
        &first_adapter,
        &second,
        &second_adapter,
    )
    .expect("equivalent measurement certification scenarios should admit");
    let report = outcome.report();

    assert!(outcome.first_basis().is_admitted());
    assert!(outcome.second_basis().is_admitted());
    assert_eq!(
        report.determinism_posture(),
        UiMeasurementBasisDeterminismPosture::Equivalent
    );
    assert_eq!(
        report.first_compatibility(),
        &UiMeasurementGenerationCompatibility::Compatible
    );
    assert_eq!(report.first_compatibility(), report.second_compatibility());
    assert!(report.basis_postures_match());
    assert!(report.evidence_inputs_match());
    assert!(report.dependency_maps_match());
    assert!(report.lineage_is_narrow());
    assert!(report.neighborhoods_are_narrow());
}

#[test]
fn stale_query_generation_stays_typed_on_the_certified_basis() {
    let (first_adapter, first, second_adapter, second, stale_generation) = stale_query_scenarios();
    let outcome = certify_measurement_basis_determinism_for_scenarios(
        &first,
        &first_adapter,
        &second,
        &second_adapter,
    )
    .expect("stale-query measurement certification scenarios should admit");
    let report = outcome.report();

    assert_eq!(
        report.first_compatibility(),
        &UiMeasurementGenerationCompatibility::StaleQueryFactReceipt {
            expected: UiEvidenceAuthorityGeneration::new(18),
            observed: stale_generation,
        }
    );
    assert!(report.first_lineage_is_narrow());
}

#[test]
fn compatible_but_distinct_admitted_bases_are_divergent() {
    let (first_adapter, first, second_adapter, second) = divergent_admitted_scenarios();
    let outcome = certify_measurement_basis_determinism_for_scenarios(
        &first,
        &first_adapter,
        &second,
        &second_adapter,
    )
    .expect("divergent admitted measurement scenarios should still admit");
    let report = outcome.report();

    assert!(outcome.first_basis().is_admitted());
    assert!(outcome.second_basis().is_admitted());
    assert_eq!(
        report.first_compatibility(),
        &UiMeasurementGenerationCompatibility::Compatible
    );
    assert_eq!(report.first_compatibility(), report.second_compatibility());
    assert_eq!(
        report.determinism_posture(),
        UiMeasurementBasisDeterminismPosture::Divergent
    );
    assert!(!report.basis_postures_match());
    assert!(!report.dependency_maps_match());
    assert!(report.lineage_is_narrow());
    assert!(report.neighborhoods_are_narrow());
}
