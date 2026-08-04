use crate::facade::{DiagnosticsTier, SignalDeploymentPreset, SignalRuntimePolicy};

#[test]
fn market_runtime_policy_presets_expose_distinct_operational_shapes() {
    let kernel = SignalRuntimePolicy::kernel();
    let fintech = SignalRuntimePolicy::fintech();
    let game = SignalRuntimePolicy::game_engine();
    let web = SignalRuntimePolicy::web_development();
    let fintech_plan = SignalDeploymentPreset::Fintech.recommended();

    assert_eq!(kernel.tier, DiagnosticsTier::Forensic);
    assert_eq!(fintech.tier, DiagnosticsTier::Development);
    assert_eq!(game.tier, DiagnosticsTier::Operational);
    assert_eq!(web.tier, DiagnosticsTier::Operational);
    assert!(
        kernel.parallel_admission.full_parallel_min_tasks
            >= fintech.parallel_admission.full_parallel_min_tasks
    );
    assert!(fintech.retention_budget.retain_flow_explanation);
    assert!(!game.retains_explanation_facts());
    assert_eq!(fintech_plan.runtime_policy, fintech);
}
