use super::{
    layout_scenario, LayoutProductionApi, LayoutScenarioDefinition, LayoutScenarioKind,
    LayoutTransitionState,
};
use crate::layout_harness::shortcut_denials::LayoutShortcutDenialKind;
use std::sync::OnceLock;

pub fn all_layout_index_layout_scenarios() -> [LayoutScenarioDefinition; 8] {
    [
        layout_scenario(LayoutScenarioKind::LayoutDeclarationInventory),
        layout_scenario(LayoutScenarioKind::AccessShapeDenial),
        layout_scenario(LayoutScenarioKind::BroadScanRejection),
        layout_scenario(LayoutScenarioKind::ExactCounter),
        layout_scenario(LayoutScenarioKind::CorruptionRebuildParity),
        layout_scenario(LayoutScenarioKind::MigrationRollbackInterruption),
        layout_scenario(LayoutScenarioKind::TrustBoundaryReadmission),
        layout_scenario(LayoutScenarioKind::MultiArtifactIntegration),
    ]
}

pub fn canonical_layout_index_layout_supported_scenarios() -> &'static [LayoutScenarioKind] {
    static SUPPORTED_SCENARIOS: OnceLock<Box<[LayoutScenarioKind]>> = OnceLock::new();
    SUPPORTED_SCENARIOS.get_or_init(|| {
        all_layout_index_layout_scenarios()
            .into_iter()
            .map(|scenario| scenario.kind())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    })
}

pub fn canonical_layout_index_layout_production_apis() -> &'static [LayoutProductionApi] {
    static PRODUCTION_APIS: OnceLock<Box<[LayoutProductionApi]>> = OnceLock::new();
    PRODUCTION_APIS.get_or_init(|| {
        collect_unique_from_scenarios(|scenario| scenario.production_apis()).into_boxed_slice()
    })
}

pub fn canonical_layout_index_layout_required_transitions() -> &'static [LayoutTransitionState] {
    static REQUIRED_TRANSITIONS: OnceLock<Box<[LayoutTransitionState]>> = OnceLock::new();
    REQUIRED_TRANSITIONS.get_or_init(|| {
        collect_unique_from_scenarios(|scenario| scenario.transitions()).into_boxed_slice()
    })
}

pub fn canonical_layout_index_layout_shortcut_denials() -> &'static [LayoutShortcutDenialKind] {
    static SHORTCUT_DENIALS: OnceLock<Box<[LayoutShortcutDenialKind]>> = OnceLock::new();
    SHORTCUT_DENIALS.get_or_init(|| {
        collect_unique_from_scenarios(|scenario| scenario.shortcut_denials()).into_boxed_slice()
    })
}

fn collect_unique_from_scenarios<T>(
    project: impl Fn(LayoutScenarioDefinition) -> &'static [T],
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
