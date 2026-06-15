#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationPageId {
    SurfaceAtlas,
    ScenarioRuns,
    Evidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavigationSelection {
    page: ValidationPageId,
    selected_scenario: &'static str,
}

impl Default for NavigationSelection {
    fn default() -> Self {
        Self {
            page: ValidationPageId::SurfaceAtlas,
            selected_scenario: "validation.scenario.surface-atlas",
        }
    }
}

impl NavigationSelection {
    pub fn page(&self) -> ValidationPageId {
        self.page
    }

    pub fn selected_scenario(&self) -> &'static str {
        self.selected_scenario
    }

    pub fn select_page(&mut self, page: ValidationPageId) {
        self.page = page;
    }
}
