use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiMeasurementRequestIdentity, UiViewportExtentRequest,
    WorthUiHostCapabilityReport, WorthUiHostContract,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::runtime::tests::active_application_session_test_support::source_backed_component_session;

#[test]
fn configured_host_capability_is_stable_for_the_active_session() {
    let session = source_backed_component_session();
    let first = session.host_measurement_capability();
    let second = session.host_measurement_capability();

    assert_eq!(first.session_identity(), session.host_session_identity());
    assert_eq!(second.session_identity(), first.session_identity());
    assert_eq!(
        second.observation_generation(),
        first.observation_generation()
    );
}

#[test]
fn foreign_host_capability_denies_before_observation_or_source_ingress() {
    let first = source_backed_component_session();
    let capability = first.host_measurement_capability();
    let mut second = source_backed_component_session();
    let mut denial = None;

    let completion = second
        .execute_framework_turn(|turn| {
            turn.host_measurement(|source| {
                denial = Some(
                    source
                        .collect_and_submit_capability(&capability, host_measurement_input())
                        .expect_err("foreign capability must deny"),
                );
            });
        })
        .expect("no mounted presentation lease is active");

    assert_eq!(
        denial,
        Some(crate::host::UiHostMeasurementEvidenceDenial::ForeignHostSession)
    );
    assert_eq!(
        completion
            .into_completion()
            .planning_counters()
            .expect("the framework turn reports its empty planning phase")
            .admitted_ingress_width(),
        0
    );
}

fn host_measurement_input() -> crate::facade::WorthUiHostMeasurementSessionInput {
    let report = WorthUiHostCapabilityReport::from_contract(WorthUiHostContract::headless());
    host_measurement_input_for_report(&report)
}

fn host_measurement_input_for_report(
    report: &WorthUiHostCapabilityReport,
) -> crate::facade::WorthUiHostMeasurementSessionInput {
    let profile =
        crate::host::UiHostMeasurementAssumptionProfile::from_capability_report(report, 1, 2, 3, 4);
    crate::facade::WorthUiHostMeasurementSessionInput::new(
        UiMeasurementRequestIdentity::new(1),
        UiMeasurementEvidenceFamily::ViewportExtent,
        crate::host::UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
        UiEvidenceAuthorityGeneration::new(1),
        crate::host::UiHostMeasurementNormalizationContext::viewport_logical_exact(profile),
    )
}
