use egui::Ui;

use crate::pages::surface_atlas::{
    SurfaceAtlasPage, SurfaceAtlasRenderContext, SurfaceAtlasViewport,
};
use crate::shell::ValidationRunSummary;
use crate::theme::ValidationWorkbenchTheme;

pub fn render_workbench_page(
    ui: &mut Ui,
    theme: &ValidationWorkbenchTheme,
    page: &SurfaceAtlasPage,
    run_summary: &ValidationRunSummary,
) {
    page.render(
        ui,
        SurfaceAtlasRenderContext {
            theme,
            viewport: SurfaceAtlasViewport::from_available_size(ui.available_size()),
        },
        run_summary,
    );
}
