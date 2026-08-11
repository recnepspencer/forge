use super::{
    DebtPerformance, LiveMaintenanceComplexityContract, LiveMaintenanceCostClass,
    LiveMaintenanceWorkUnit, LivePerformanceReport, PatchWidthUnit, PerformanceStatus,
    RefreshAdmissionStatus,
};

#[test]
fn detail_complexity_contract_names_field_delta_units() {
    let contract = LiveMaintenanceComplexityContract::detail_patch();
    assert_eq!(
        contract.cost_class(),
        &LiveMaintenanceCostClass::DetailPatch
    );
    assert!(contract
        .work_units()
        .contains(&LiveMaintenanceWorkUnit::ProjectedFieldDeltaCount));
}

#[test]
fn performance_report_digest_includes_budget_and_status() {
    let report = LivePerformanceReport::verified_detail_family();
    let parts = report.digest_parts();

    assert!(parts
        .iter()
        .any(|part| part == "patch_width_policy:deliver_within_budget"));
    assert!(parts
        .iter()
        .any(|part| part == "performance_status:verified"));
}

#[test]
fn ordered_collection_report_uses_collection_row_budget_units() {
    let report = LivePerformanceReport::verified_ordered_collection_family();
    assert_eq!(
        report.width_budget().unit(),
        &PatchWidthUnit::CollectionRowChange
    );
}

#[test]
fn bounded_materialization_report_declares_debt_status() {
    let report = LivePerformanceReport::debt_bounded_materialization_family();
    assert_eq!(report.performance_status(), DebtPerformance::LABEL);
    assert_eq!(
        report.refresh_admission_status(),
        &RefreshAdmissionStatus::Debt
    );
}
