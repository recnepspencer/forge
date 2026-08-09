use worth_ui::facade::app::WorthUi;
use worth_ui::facade::obligations::{
    UiObligationClosedSemanticLane, UiObligationCloseoutGuarantee, UiObligationCloseoutNonGoal,
    UiObligationCloseoutReport,
};

#[test]
fn bootstrap_app_exposes_milestone34_obligation_closeout_report() {
    let app = WorthUi::app()
        .bind_certification_host_adapter(
            worth_ui_host_contract::UiCertificationHostBindingGrant::for_certification(),
            worth_ui_host_headless::WorthUiHeadlessHost,
        )
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .freeze()
        .expect("application preparation should succeed");
    let report = app.obligation_closeout_report();

    assert_eq!(report, UiObligationCloseoutReport::milestone34());
    assert_eq!(
        report.closed_semantic_lanes(),
        &[
            UiObligationClosedSemanticLane::TouchAuthority,
            UiObligationClosedSemanticLane::SupportAuthority,
            UiObligationClosedSemanticLane::FamilyCatalog,
            UiObligationClosedSemanticLane::SelectionAuthority,
            UiObligationClosedSemanticLane::DispatchPlanning,
            UiObligationClosedSemanticLane::VerdictAuthority,
            UiObligationClosedSemanticLane::QueryBoundary,
            UiObligationClosedSemanticLane::HostBoundary,
            UiObligationClosedSemanticLane::EvidenceRetention,
            UiObligationClosedSemanticLane::BudgetEnforcement,
            UiObligationClosedSemanticLane::AdmissionAggregation,
        ],
    );
    assert_eq!(
        report.guarantees(),
        &[
            UiObligationCloseoutGuarantee::CallerForgeryDiesAtCompileAndFacadeBoundary,
            UiObligationCloseoutGuarantee::LaterRuntimeSlicesConsumeSealedAuthorityHandoffs,
            UiObligationCloseoutGuarantee::QueryAndHostTruthRemainOwnerBound,
            UiObligationCloseoutGuarantee::EquivalentTouchesConvergeUnderStableBasis,
        ],
    );
    assert_eq!(
        report.non_goals(),
        &[
            UiObligationCloseoutNonGoal::MeasurementExecution,
            UiObligationCloseoutNonGoal::QueryExecution,
            UiObligationCloseoutNonGoal::IntentExecution,
            UiObligationCloseoutNonGoal::ServiceExecution,
            UiObligationCloseoutNonGoal::RebindExecution,
            UiObligationCloseoutNonGoal::RendererLocalLegality,
        ],
    );
}
