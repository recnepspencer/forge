use egui::Ui;

use crate::pages::surface_atlas::{
    regions, SurfaceAtlasFamily, SurfaceAtlasModel, SurfaceAtlasViewport,
};
use crate::shell::ValidationRunSummary;
use crate::theme::ValidationWorkbenchTheme;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SurfaceAtlasRenderPlan {
    steps: &'static [SurfaceAtlasRenderStep],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SurfaceAtlasRenderStep {
    families: &'static [SurfaceAtlasFamily],
    renderer: SurfaceAtlasRegionRenderer,
    separator_before: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SurfaceAtlasRegionRenderer {
    ThemeDensityControls,
    ActivityNavigation,
    ScenarioList,
    CommandProjectionSurface,
    WorkbenchCanvas,
    PinnedSidebar,
    StackedScrollPanes,
    TabbedEditor,
    EvidenceInspector,
    RunTimeline,
    OverlayPreview,
    StatusSurface,
}

impl SurfaceAtlasRenderPlan {
    pub const DEFAULT: Self = Self {
        steps: &[
            SurfaceAtlasRenderStep::new(
                &[
                    SurfaceAtlasFamily::ThemeControls,
                    SurfaceAtlasFamily::DensityControls,
                ],
                SurfaceAtlasRegionRenderer::ThemeDensityControls,
            ),
            SurfaceAtlasRenderStep::after_separator(
                &[SurfaceAtlasFamily::ActivityNavigation],
                SurfaceAtlasRegionRenderer::ActivityNavigation,
            ),
            SurfaceAtlasRenderStep::new(
                &[SurfaceAtlasFamily::ScenarioList],
                SurfaceAtlasRegionRenderer::ScenarioList,
            ),
            SurfaceAtlasRenderStep::new(
                &[SurfaceAtlasFamily::CommandProjectionSurface],
                SurfaceAtlasRegionRenderer::CommandProjectionSurface,
            ),
            SurfaceAtlasRenderStep::after_separator(
                &[SurfaceAtlasFamily::WorkbenchCanvas],
                SurfaceAtlasRegionRenderer::WorkbenchCanvas,
            ),
            SurfaceAtlasRenderStep::after_separator(
                &[SurfaceAtlasFamily::PinnedSidebar],
                SurfaceAtlasRegionRenderer::PinnedSidebar,
            ),
            SurfaceAtlasRenderStep::new(
                &[SurfaceAtlasFamily::StackedScrollPane],
                SurfaceAtlasRegionRenderer::StackedScrollPanes,
            ),
            SurfaceAtlasRenderStep::new(
                &[SurfaceAtlasFamily::TabbedEditor],
                SurfaceAtlasRegionRenderer::TabbedEditor,
            ),
            SurfaceAtlasRenderStep::new(
                &[SurfaceAtlasFamily::EvidenceInspector],
                SurfaceAtlasRegionRenderer::EvidenceInspector,
            ),
            SurfaceAtlasRenderStep::new(
                &[SurfaceAtlasFamily::BottomTimeline],
                SurfaceAtlasRegionRenderer::RunTimeline,
            ),
            SurfaceAtlasRenderStep::new(
                &[SurfaceAtlasFamily::OverlayPreview],
                SurfaceAtlasRegionRenderer::OverlayPreview,
            ),
            SurfaceAtlasRenderStep::new(
                &[SurfaceAtlasFamily::StatusSurface],
                SurfaceAtlasRegionRenderer::StatusSurface,
            ),
        ],
    };

    pub fn steps(self) -> &'static [SurfaceAtlasRenderStep] {
        self.steps
    }

    pub fn families(self) -> impl Iterator<Item = SurfaceAtlasFamily> {
        self.steps.iter().flat_map(|step| step.families())
    }

    pub fn render(
        self,
        ui: &mut Ui,
        theme: &ValidationWorkbenchTheme,
        viewport: SurfaceAtlasViewport,
        model: &SurfaceAtlasModel,
        run_summary: &ValidationRunSummary,
    ) {
        for step in self.steps {
            step.render(ui, theme, viewport, model, run_summary);
        }
    }
}

impl SurfaceAtlasRenderStep {
    const fn new(
        families: &'static [SurfaceAtlasFamily],
        renderer: SurfaceAtlasRegionRenderer,
    ) -> Self {
        Self {
            families,
            renderer,
            separator_before: false,
        }
    }

    const fn after_separator(
        families: &'static [SurfaceAtlasFamily],
        renderer: SurfaceAtlasRegionRenderer,
    ) -> Self {
        Self {
            families,
            renderer,
            separator_before: true,
        }
    }

    pub fn families(self) -> impl Iterator<Item = SurfaceAtlasFamily> {
        self.families.iter().copied()
    }

    pub fn render(
        self,
        ui: &mut Ui,
        theme: &ValidationWorkbenchTheme,
        viewport: SurfaceAtlasViewport,
        model: &SurfaceAtlasModel,
        run_summary: &ValidationRunSummary,
    ) {
        if self.separator_before {
            ui.separator();
        }
        match self.renderer {
            SurfaceAtlasRegionRenderer::ThemeDensityControls => {
                regions::theme_density_controls::render(ui, theme, model.controls());
            }
            SurfaceAtlasRegionRenderer::ActivityNavigation => {
                regions::activity_navigation::render(ui);
            }
            SurfaceAtlasRegionRenderer::ScenarioList => {
                regions::scenario_list::render(ui, run_summary);
            }
            SurfaceAtlasRegionRenderer::CommandProjectionSurface => {
                regions::command_projection_surface::render(ui);
            }
            SurfaceAtlasRegionRenderer::WorkbenchCanvas => {
                regions::workbench_canvas::render(ui, theme, run_summary);
            }
            SurfaceAtlasRegionRenderer::PinnedSidebar => {
                regions::pinned_sidebar::render(ui, model.topology());
            }
            SurfaceAtlasRegionRenderer::StackedScrollPanes => {
                regions::stacked_scroll_panes::render(ui, viewport);
            }
            SurfaceAtlasRegionRenderer::TabbedEditor => {
                regions::tabbed_editor::render(ui);
            }
            SurfaceAtlasRegionRenderer::EvidenceInspector => {
                regions::evidence_inspector::render(ui, model.fixture_evidence());
            }
            SurfaceAtlasRegionRenderer::RunTimeline => {
                regions::run_timeline::render(ui, model.fixture_evidence());
            }
            SurfaceAtlasRegionRenderer::OverlayPreview => {
                regions::overlay_preview::render(ui);
            }
            SurfaceAtlasRegionRenderer::StatusSurface => {
                regions::status_surface::render(ui, run_summary);
            }
        }
    }
}
