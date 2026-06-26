use worth_ui::facade::{
    WorthUiDiagnosticMaterialization, WorthUiDiagnosticProjectionHook,
    WorthUiDiagnosticRichnessPolicy, WorthUiDiagnosticRichnessTier, WorthUiDiagnosticSource,
    WorthUiDiagnosticSupportReport, WorthUiPlanDiagnostic, WorthUiReloadDiagnostic,
    WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode, WorthUiRuntimeDiagnosticCounters,
    WorthUiRuntimeDiagnosticFamily, WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeDiagnosticReport,
    WorthUiRuntimeDiagnosticRequest, WorthUiRuntimeDiagnostics, WorthUiSupportReportPolicy,
};

fn main() {
    let policy = WorthUiDiagnosticRichnessPolicy::standard();
    let _runtime_policy: WorthUiRuntimeDiagnosticPolicy = policy;
    let support_policy = WorthUiSupportReportPolicy::from_diagnostic_policy(policy);
    let hook = WorthUiDiagnosticProjectionHook::projection("workspace.diagnostics.panel");

    assert_eq!(policy.tier(), WorthUiDiagnosticRichnessTier::Standard);
    assert!(!support_policy.may_materialize_support_sections());
    assert_ne!(hook.projection_digest(), 0);

    let _: Option<WorthUiRuntimeDiagnostic> = None;
    let _: Option<WorthUiReloadDiagnostic> = None;
    let _: Option<WorthUiPlanDiagnostic> = None;
    let _: Option<WorthUiRuntimeDiagnosticReport> = None;
    let _: Option<WorthUiRuntimeDiagnosticRequest<'static>> = None;
    let _: Option<WorthUiRuntimeDiagnostics<'static>> = None;
    let _: Option<WorthUiDiagnosticMaterialization> = None;
    let _: Option<WorthUiDiagnosticSupportReport> = None;
    let _: Option<WorthUiRuntimeDiagnosticCounters> = None;
    let _: Option<WorthUiDiagnosticSource> = None;
    assert_eq!(
        WorthUiRuntimeDiagnosticFamily::Reload.as_str(),
        "reload"
    );
    assert_eq!(
        WorthUiRuntimeDiagnosticCode::ReloadFailurePreserved.as_str(),
        "reload.failure_preserved"
    );
}
