use crate::pages::ValidationPageRegistry;
use crate::runtime::PreparedValidationWorkbenchLaunch;
use crate::shell::{StableShellSurface, StableShellSurfaceId, StableShellSurfaceManifest};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShellFrameSnapshot {
    rendered_surfaces: &'static [StableShellSurface],
    selected_scenario: &'static str,
    active_page_label: &'static str,
    active_plan_digest: u64,
}

impl ShellFrameSnapshot {
    pub fn from_launch(
        launch: &PreparedValidationWorkbenchLaunch,
        pages: ValidationPageRegistry,
    ) -> Self {
        let summary = launch.run_summary();
        Self {
            rendered_surfaces: StableShellSurfaceManifest::REQUIRED.surfaces(),
            selected_scenario: summary.selected_scenario(),
            active_page_label: active_page_label(launch, pages),
            active_plan_digest: summary.runtime_observation().active_plan_digest(),
        }
    }

    pub fn contains_surface(&self, id: StableShellSurfaceId) -> bool {
        self.rendered_surfaces
            .iter()
            .any(|surface| surface.id() == id)
    }

    pub fn rendered_surface_ids(&self) -> Vec<StableShellSurfaceId> {
        self.rendered_surfaces
            .iter()
            .map(|surface| surface.id())
            .collect()
    }

    pub fn selected_scenario(&self) -> &'static str {
        self.selected_scenario
    }

    pub fn active_page_label(&self) -> &'static str {
        self.active_page_label
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }
}

fn active_page_label(
    launch: &PreparedValidationWorkbenchLaunch,
    pages: ValidationPageRegistry,
) -> &'static str {
    pages
        .pages()
        .iter()
        .find(|page| page.id() == launch.navigation().page())
        .map_or("Unknown", |page| page.label())
}
