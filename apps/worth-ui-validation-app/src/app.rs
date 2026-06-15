use eframe::{App, Frame, NativeOptions};
use egui::Context;

use crate::{
    runtime::{PreparedValidationWorkbenchLaunch, ValidationWorkbenchSnapshot},
    workspace::ValidationWorkspaceShell,
};

pub struct ValidationWorkbenchApp {
    workspace: ValidationWorkspaceShell,
}

#[derive(Debug)]
pub enum ValidationWorkbenchRunError {
    Native(eframe::Error),
}

impl ValidationWorkbenchApp {
    pub fn new(launch: PreparedValidationWorkbenchLaunch) -> Self {
        Self {
            workspace: ValidationWorkspaceShell::from_launch(launch),
        }
    }

    pub fn run_native(launch: PreparedValidationWorkbenchLaunch) -> eframe::Result<()> {
        let options = NativeOptions::default();
        eframe::run_native(
            "Worth UI Validation App",
            options,
            Box::new(|_| Ok(Box::new(Self::new(launch)))),
        )
    }

    pub fn snapshot(&self) -> ValidationWorkbenchSnapshot {
        self.workspace.snapshot()
    }

    pub fn launch(&self) -> &PreparedValidationWorkbenchLaunch {
        self.workspace.launch()
    }

    pub fn workspace(&self) -> &ValidationWorkspaceShell {
        &self.workspace
    }

    pub fn workspace_mut(&mut self) -> &mut ValidationWorkspaceShell {
        &mut self.workspace
    }
}

impl App for ValidationWorkbenchApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.workspace.render(ctx);
    }
}
