use forge_store_physical_certification::layout_harness::scenario::{
    S8LayoutScenarioKind, canonical_s8_layout_supported_scenarios,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S8LayoutScenarioBuilders {
    supported_scenarios: &'static [S8LayoutScenarioKind],
}

pub fn s8_layout_scenario_builders() -> S8LayoutScenarioBuilders {
    S8LayoutScenarioBuilders {
        supported_scenarios: canonical_s8_layout_supported_scenarios(),
    }
}

impl S8LayoutScenarioBuilders {
    pub const fn supported_scenarios(&self) -> &'static [S8LayoutScenarioKind] {
        self.supported_scenarios
    }
}
