use egui::Context;

use crate::commands::ShellCommandRegistry;
use crate::pages::surface_atlas::SurfaceAtlasPage;
use crate::pages::ValidationPageRegistry;
use crate::runtime::PreparedValidationWorkbenchLaunch;
use crate::shell::{ShellFrameSnapshot, ShellSurfaceRenderer};
use crate::theme::ValidationWorkbenchTheme;

#[derive(Clone, Debug)]
pub struct ValidationShellFrame {
    theme: ValidationWorkbenchTheme,
    pages: ValidationPageRegistry,
    commands: ShellCommandRegistry,
    surface_atlas: SurfaceAtlasPage,
}

impl ValidationShellFrame {
    pub fn new(launch: &PreparedValidationWorkbenchLaunch) -> Self {
        Self {
            theme: launch.render_theme().clone(),
            pages: ValidationPageRegistry::DEFAULT,
            commands: ShellCommandRegistry::DEFAULT,
            surface_atlas: SurfaceAtlasPage::from_launch(launch),
        }
    }

    pub fn snapshot(&self, launch: &PreparedValidationWorkbenchLaunch) -> ShellFrameSnapshot {
        ShellFrameSnapshot::from_launch(launch, self.pages)
    }

    pub fn render(&mut self, ctx: &Context, launch: &PreparedValidationWorkbenchLaunch) {
        ctx.set_visuals(self.theme.visuals());
        let summary = launch.run_summary();
        ShellSurfaceRenderer::new(&self.theme, &self.surface_atlas, self.pages, self.commands)
            .render_manifest_surfaces(ctx, &summary);
    }
}
