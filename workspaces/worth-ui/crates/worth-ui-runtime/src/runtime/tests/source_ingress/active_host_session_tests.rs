use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiMeasurementRequestIdentity, UiViewportExtentRequest,
    WorthUiHostCapabilityObservationGeneration, WorthUiHostCapabilityReport, WorthUiHostContract,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::runtime::tests::active_application_session_test_support::{
    source_backed_component_session, source_backed_component_session_with_host,
};

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
    let ingress_before = second.allocation_ingress_count_for_test();
    let mut denial = None;

    second.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            denial = Some(
                source
                    .collect_and_submit_capability(&capability, host_measurement_input())
                    .expect_err("foreign capability must deny"),
            );
        });
    });

    assert_eq!(
        denial,
        Some(crate::host::UiHostMeasurementEvidenceDenial::ForeignHostSession)
    );
    assert_eq!(second.allocation_ingress_count_for_test(), ingress_before);
}

#[test]
fn stale_host_generation_denies_before_observation_or_source_ingress() {
    let mut session = source_backed_component_session();
    let capability = session.host_measurement_capability();
    session.replace_host_observation_generation_for_test(
        WorthUiHostCapabilityObservationGeneration::new(
            capability.observation_generation().as_u64() + 1,
        ),
    );
    let ingress_before = session.allocation_ingress_count_for_test();
    let mut denial = None;

    session.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            denial = Some(
                source
                    .collect_and_submit_capability(&capability, host_measurement_input())
                    .expect_err("stale capability must deny"),
            );
        });
    });

    assert_eq!(
        denial,
        Some(crate::host::UiHostMeasurementEvidenceDenial::StaleHostObservationGeneration)
    );
    assert_eq!(session.allocation_ingress_count_for_test(), ingress_before);
}

#[test]
fn headless_and_egui_hosts_share_session_lifecycle_without_claiming_equal_capabilities() {
    let mut headless = source_backed_component_session();
    let headless_capability = headless.host_measurement_capability();
    let headless_ingress = headless.allocation_ingress_count_for_test();
    let mut headless_denied = false;
    headless.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            headless_denied = source
                .collect_and_submit_capability(
                    &headless_capability,
                    host_measurement_input_for_report(headless_capability.capability_report()),
                )
                .is_err();
        });
    });
    assert!(headless_denied);
    assert_eq!(
        headless.allocation_ingress_count_for_test(),
        headless_ingress
    );

    let context = egui::Context::default();
    context.begin_pass(egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1280.0, 720.0),
        )),
        ..Default::default()
    });
    let mut egui = source_backed_component_session_with_host(
        worth_ui_host_egui::WorthUiHostEgui::new(context.clone()),
    );
    let egui_capability = egui.host_measurement_capability();
    let egui_ingress = egui.allocation_ingress_count_for_test();
    let mut egui_admitted = false;
    egui.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            egui_admitted = source
                .collect_and_submit_capability(
                    &egui_capability,
                    host_measurement_input_for_report(egui_capability.capability_report()),
                )
                .is_ok();
        });
    });
    let _ = context.end_pass();

    assert!(egui_admitted);
    assert_eq!(egui.allocation_ingress_count_for_test(), egui_ingress + 1);
    assert_ne!(
        headless.host_session_identity(),
        egui.host_session_identity()
    );
    assert_eq!(
        egui_capability.session_identity(),
        egui.host_session_identity()
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
