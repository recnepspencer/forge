use super::{
    layout_scenario, S8LayoutProductionApi, S8LayoutScenarioDefinition, S8LayoutScenarioKind,
    S8LayoutTransitionState,
};
use crate::layout_harness::shortcut_denials::S8LayoutShortcutDenialKind;
use std::sync::OnceLock;

pub fn all_layout_index_layout_scenarios() -> [S8LayoutScenarioDefinition; 8] {
    [
        layout_scenario(S8LayoutScenarioKind::LayoutDeclarationInventory),
        layout_scenario(S8LayoutScenarioKind::AccessShapeDenial),
        layout_scenario(S8LayoutScenarioKind::BroadScanRejection),
        layout_scenario(S8LayoutScenarioKind::ExactCounter),
        layout_scenario(S8LayoutScenarioKind::CorruptionRebuildParity),
        layout_scenario(S8LayoutScenarioKind::MigrationRollbackInterruption),
        layout_scenario(S8LayoutScenarioKind::TrustBoundaryReadmission),
        layout_scenario(S8LayoutScenarioKind::MultiArtifactIntegration),
    ]
}

pub fn canonical_layout_index_layout_supported_scenarios() -> &'static [S8LayoutScenarioKind] {
    static SUPPORTED_SCENARIOS: OnceLock<Box<[S8LayoutScenarioKind]>> = OnceLock::new();
    SUPPORTED_SCENARIOS.get_or_init(|| {
        all_layout_index_layout_scenarios()
            .into_iter()
            .map(|scenario| scenario.kind())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    })
}

pub fn canonical_layout_index_layout_production_apis() -> &'static [S8LayoutProductionApi] {
    static PRODUCTION_APIS: OnceLock<Box<[S8LayoutProductionApi]>> = OnceLock::new();
    PRODUCTION_APIS.get_or_init(|| {
        collect_unique_from_scenarios(|scenario| scenario.production_apis()).into_boxed_slice()
    })
}

pub fn canonical_layout_index_layout_required_transitions() -> &'static [S8LayoutTransitionState] {
    static REQUIRED_TRANSITIONS: OnceLock<Box<[S8LayoutTransitionState]>> = OnceLock::new();
    REQUIRED_TRANSITIONS.get_or_init(|| {
        collect_unique_from_scenarios(|scenario| scenario.transitions()).into_boxed_slice()
    })
}

pub fn canonical_layout_index_layout_shortcut_denials() -> &'static [S8LayoutShortcutDenialKind] {
    static SHORTCUT_DENIALS: OnceLock<Box<[S8LayoutShortcutDenialKind]>> = OnceLock::new();
    SHORTCUT_DENIALS.get_or_init(|| {
        collect_unique_from_scenarios(|scenario| scenario.shortcut_denials()).into_boxed_slice()
    })
}

fn collect_unique_from_scenarios<T>(
    project: impl Fn(S8LayoutScenarioDefinition) -> &'static [T],
) -> Vec<T>
where
    T: Copy + Eq + 'static,
{
    let mut values = Vec::new();
    for scenario in all_layout_index_layout_scenarios() {
        for value in project(scenario) {
            if !values.contains(value) {
                values.push(*value);
            }
        }
    }
    values
}
