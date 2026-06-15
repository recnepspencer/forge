use egui::Ui;

use crate::pages::surface_atlas::{
    SurfaceAtlasFamily, SurfaceAtlasFixtureEvidence, SurfaceAtlasModel, SurfaceAtlasRenderPlan,
    SurfaceAtlasViewport,
};
use crate::runtime::PreparedValidationWorkbenchLaunch;
use crate::shell::ValidationRunSummary;
use crate::theme::ValidationWorkbenchTheme;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceAtlasPage {
    model: SurfaceAtlasModel,
}

pub struct SurfaceAtlasRenderContext<'a> {
    pub theme: &'a ValidationWorkbenchTheme,
    pub viewport: SurfaceAtlasViewport,
}

impl SurfaceAtlasPage {
    pub fn from_launch(launch: &PreparedValidationWorkbenchLaunch) -> Self {
        Self {
            model: SurfaceAtlasModel::new(
                launch.visual_foundation().clone(),
                launch.density(),
                SurfaceAtlasFixtureEvidence::sample_only(),
            ),
        }
    }

    pub fn model(&self) -> &SurfaceAtlasModel {
        &self.model
    }

    pub fn model_mut(&mut self) -> &mut SurfaceAtlasModel {
        &mut self.model
    }

    pub fn render_plan(&self) -> SurfaceAtlasRenderPlan {
        SurfaceAtlasRenderPlan::DEFAULT
    }

    pub fn rendered_surface_families(&self) -> impl Iterator<Item = SurfaceAtlasFamily> {
        self.render_plan().families()
    }

    pub fn render(
        &self,
        ui: &mut Ui,
        context: SurfaceAtlasRenderContext<'_>,
        run_summary: &ValidationRunSummary,
    ) {
        self.render_plan().render(
            ui,
            context.theme,
            context.viewport,
            &self.model,
            run_summary,
        );
    }
}
