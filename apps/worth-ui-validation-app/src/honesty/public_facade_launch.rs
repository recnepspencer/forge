use worth_ui::facade::{
    WorthUi, WorthUiApp, WorthUiPreparedRuntimeAuthoring, WorthUiRuntimeLaunch,
    WorthUiRuntimeLaunchPreparationDenial, WorthUiRuntimeSourceModule,
};

use crate::sample::{VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationAppPublicFacadeLaunch {
    module_path: &'static str,
    source_text: &'static str,
}

impl ValidationAppPublicFacadeLaunch {
    pub const DEFAULT: Self = Self {
        module_path: VALIDATION_SAMPLE_MODULE_PATH,
        source_text: VALIDATION_SAMPLE_SOURCE,
    };

    pub fn prepare_for(
        self,
        app: &WorthUiApp,
    ) -> Result<WorthUiRuntimeLaunch, WorthUiRuntimeLaunchPreparationDenial> {
        self.prepare_authoring_for(app)
            .map(WorthUiPreparedRuntimeAuthoring::into_runtime_launch)
    }

    pub fn prepare_authoring_for(
        self,
        app: &WorthUiApp,
    ) -> Result<WorthUiPreparedRuntimeAuthoring, WorthUiRuntimeLaunchPreparationDenial> {
        WorthUi::runtime_launch()
            .from_source_module(WorthUiRuntimeSourceModule::new(
                self.module_path,
                self.source_text,
            ))
            .prepare_authoring_for(app)
    }
}
