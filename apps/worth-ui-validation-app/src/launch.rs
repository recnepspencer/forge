use worth_ui::facade::{
    CommandId, WorthUi, WorthUiApp, WorthUiHeaderFramePlan, WorthUiHeaderFramePlanDenial,
    WorthUiHeaderMenuPlan, WorthUiHeaderThemePlan, WorthUiRuntimeHost, WorthUiRuntimeLaunchDenial,
    WorthUiRuntimeLaunchPreparationDenial, WorthUiRuntimeSourceModule,
};

use crate::app_capabilities::{
    validation_header_menu_requests, validation_header_theme_request, validation_worth_ui_app,
};
use crate::runtime_workbench::ValidationRuntimeWorkbench;
use crate::sample_source::{VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ValidationWorkbenchLaunch;

pub struct PreparedValidationWorkbenchLaunch {
    app: WorthUiApp,
    runtime: WorthUiRuntimeHost,
    header_frame_plan: WorthUiHeaderFramePlan,
}

#[derive(Debug)]
pub enum ValidationWorkbenchLaunchError {
    HeaderFrame(WorthUiHeaderFramePlanDenial),
    RuntimePreparation(WorthUiRuntimeLaunchPreparationDenial),
    RuntimeLaunch(WorthUiRuntimeLaunchDenial),
}

impl ValidationWorkbenchLaunch {
    pub fn new() -> Self {
        Self
    }

    pub fn prepare(
        self,
    ) -> Result<PreparedValidationWorkbenchLaunch, ValidationWorkbenchLaunchError> {
        let app = validation_worth_ui_app();
        let header_frame_plan = WorthUiHeaderFramePlan::from_snapshot(
            app.capabilities(),
            validation_header_menu_requests(),
            validation_header_theme_request(),
        )
        .map_err(ValidationWorkbenchLaunchError::HeaderFrame)?;
        let runtime_launch = WorthUi::runtime_launch()
            .from_source_module(WorthUiRuntimeSourceModule::new(
                VALIDATION_SAMPLE_MODULE_PATH,
                VALIDATION_SAMPLE_SOURCE,
            ))
            .prepare_for(&app)
            .map_err(ValidationWorkbenchLaunchError::RuntimePreparation)?;
        let runtime = app
            .launch_runtime(runtime_launch)
            .map_err(ValidationWorkbenchLaunchError::RuntimeLaunch)?;

        Ok(PreparedValidationWorkbenchLaunch {
            app,
            runtime,
            header_frame_plan,
        })
    }
}

impl PreparedValidationWorkbenchLaunch {
    pub fn app(&self) -> &WorthUiApp {
        &self.app
    }

    pub fn runtime(&self) -> &WorthUiRuntimeHost {
        &self.runtime
    }

    pub fn header_plan(&self) -> &WorthUiHeaderMenuPlan {
        self.header_frame_plan.menu_plan()
    }

    pub fn header_theme_plan(&self) -> &WorthUiHeaderThemePlan {
        self.header_frame_plan.theme_plan()
    }

    pub fn header_frame_plan(&self) -> &WorthUiHeaderFramePlan {
        &self.header_frame_plan
    }

    pub fn into_runtime_workbench(self) -> ValidationRuntimeWorkbench {
        ValidationRuntimeWorkbench::new(self.app, self.runtime, self.header_frame_plan)
    }

    pub fn has_command(&self, command_id: &str) -> bool {
        let id = CommandId::new(command_id).expect("valid command id");
        self.app.capabilities().commands().get(&id).is_some()
    }
}
