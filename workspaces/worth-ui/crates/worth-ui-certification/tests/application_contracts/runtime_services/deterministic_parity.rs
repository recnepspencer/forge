use crate::intent::{
    run_native_runtime_service_scenario,
    runtime_services_kit::{run_headless_runtime_service_scenario, RuntimeServiceSemanticOutcome},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScenarioStep {
    PublishPortal,
    PlacePortalFocus,
    DismissCurrentTop,
    RepeatDismissal,
    ReconcilePhysicalEffect,
    Shutdown,
}

const SCENARIO: &[ScenarioStep] = &[
    ScenarioStep::PublishPortal,
    ScenarioStep::PlacePortalFocus,
    ScenarioStep::DismissCurrentTop,
    ScenarioStep::RepeatDismissal,
    ScenarioStep::ReconcilePhysicalEffect,
    ScenarioStep::Shutdown,
];

fn independent_semantic_oracle(steps: &[ScenarioStep]) -> RuntimeServiceSemanticOutcome {
    assert_eq!(steps, SCENARIO, "the qualified scenario order is fixed");
    RuntimeServiceSemanticOutcome {
        portal_was_visible: true,
        focus_was_placed: true,
        dismissal_closed_only_top: true,
        focus_restored_to_previous: true,
        duplicate_was_idempotent: true,
        proposals_are_zero: true,
        terminal_resources_are_zero: true,
    }
}

#[test]
fn production_headless_and_native_paths_share_semantic_service_outcomes() {
    let expected = independent_semantic_oracle(SCENARIO);
    let headless = run_headless_runtime_service_scenario();
    let native = run_native_runtime_service_scenario();

    assert_eq!(headless.semantic, expected);
    assert_eq!(native, expected);
    assert!(headless.hot_rebind_preserved_portal);
    assert!(headless.focus_retargeted_to_successor);
    assert!(headless.inspection_was_bounded);
}
