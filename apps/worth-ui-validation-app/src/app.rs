use eframe::{App, Frame, NativeOptions};
use egui::Context;

use crate::runtime::PreparedValidationWorkbenchLaunch;
use crate::shell::{ShellFrameSnapshot, ValidationShellFrame};

pub struct ValidationWorkbenchApp {
    launch: PreparedValidationWorkbenchLaunch,
    shell: ValidationShellFrame,
}

#[derive(Debug)]
pub enum ValidationWorkbenchRunError {
    Native(eframe::Error),
}

impl ValidationWorkbenchApp {
    pub fn new(launch: PreparedValidationWorkbenchLaunch) -> Self {
        let shell = ValidationShellFrame::new(&launch);
        Self { launch, shell }
    }

    pub fn run_native(launch: PreparedValidationWorkbenchLaunch) -> eframe::Result<()> {
        let options = NativeOptions::default();
        eframe::run_native(
            "Worth UI Validation App",
            options,
            Box::new(|_| Ok(Box::new(Self::new(launch)))),
        )
    }

    pub fn snapshot(&self) -> ShellFrameSnapshot {
        self.shell.snapshot(&self.launch)
    }

    pub fn launch(&self) -> &PreparedValidationWorkbenchLaunch {
        &self.launch
    }

    pub fn launch_mut(&mut self) -> &mut PreparedValidationWorkbenchLaunch {
        &mut self.launch
    }
}

impl App for ValidationWorkbenchApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.shell.render(ctx, &self.launch);
    }
}
