use super::*;
use worth_ui_runtime::facade::mounted::{
    UiMountedDiagnosticInspection, UiMountedDiagnosticInspectionOmission,
    UiMountedRetentionEvictionPosture,
};

#[test]
fn diagnostic_pressure_omits_new_richness_while_a_real_lease_pins_the_budget() {
    let diagnostic_budget = UiMountedRetentionClassBudget::new(1, 1024 * 1024);
    let budget = UiMountedFrameRetentionBudget::new(UiMountedFrameRetentionBudgetInput {
        current: large_budget(),
        in_flight: large_budget(),
        observation_basis: large_budget(),
        predecessor_inspection: large_budget(),
        diagnostic: diagnostic_budget,
        future_snapshot: UiMountedRetentionClassBudget::new(0, 0),
        expired_identity_limit: 64,
    });
    let (mut session, host, _, instance) = retention_world("mounted-diagnostic-pressure", budget);

    let first = publish(&mut session, &host, instance);
    let first_inspection = inspected_with_diagnostics(&session);
    let first_diagnostics = available_diagnostics(&first_inspection);
    assert_eq!(first_diagnostics.frame(), first.frame);
    assert!(!first_diagnostics.rows().is_empty());

    let second = publish(&mut session, &host, instance);
    let second_inspection = inspected_with_diagnostics(&session);
    assert!(matches!(
        second_inspection.diagnostics(),
        UiMountedDiagnosticInspection::Omitted(UiMountedDiagnosticInspectionOmission::NotRetained)
    ));
    assert_eq!(second_inspection.frame(), second.frame);
    let saturated = session.mounted_retention_report();
    let diagnostic = saturated.class(UiMountedRetentionClass::Diagnostic);
    assert_eq!(
        diagnostic.posture(),
        UiMountedRetentionEvictionPosture::EvictableUnlessLeased
    );
    assert_eq!(diagnostic.retained_items(), 1);
    assert_eq!(diagnostic.active_leases(), 1);
    assert!(diagnostic.retained_structural_bytes() <= diagnostic_budget.structural_byte_limit());
    assert!(
        diagnostic.lease_charged_structural_bytes() <= diagnostic_budget.structural_byte_limit()
    );

    drop(second_inspection);
    drop(first_inspection);
    let third = publish(&mut session, &host, instance);
    let recovered = inspected_with_diagnostics(&session);
    assert_eq!(available_diagnostics(&recovered).frame(), third.frame);
    let recovered_report = session.mounted_retention_report();
    let diagnostic = recovered_report.class(UiMountedRetentionClass::Diagnostic);
    assert_eq!(diagnostic.retained_items(), 1);
    assert_eq!(diagnostic.active_leases(), 1);
}

fn inspected_with_diagnostics(
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> Box<worth_ui_runtime::facade::mounted::UiMountedInspectedFrame> {
    match session.inspect_mounted_frame(UiMountedInspectionRequest::current().with_diagnostics()) {
        UiMountedInspectionReceipt::Available(inspection) => inspection,
        other => panic!("current diagnostic inspection must retain its core frame: {other:?}"),
    }
}

fn available_diagnostics(
    inspection: &worth_ui_runtime::facade::mounted::UiMountedInspectedFrame,
) -> &worth_ui_runtime::facade::mounted::UiMountedInspectedDiagnostics {
    match inspection.diagnostics() {
        UiMountedDiagnosticInspection::Available(diagnostics) => diagnostics,
        other => panic!("diagnostic richness must be retained: {other:?}"),
    }
}
