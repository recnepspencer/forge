use worth_ui::facade::{
    WorthUi, WorthUiApp, WorthUiRuntimeLaunch, WorthUiRuntimeLaunchPreparationDenial,
    WorthUiRuntimeSourceModule,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationAppPublicFacadeLaunch {
    module_path: &'static str,
    source_text: &'static str,
}

impl ValidationAppPublicFacadeLaunch {
    pub const DEFAULT: Self = Self {
        module_path: "validation/main.wui",
        source_text: "",
    };

    pub fn prepare_for(
        self,
        app: &WorthUiApp,
    ) -> Result<WorthUiRuntimeLaunch, WorthUiRuntimeLaunchPreparationDenial> {
        WorthUi::runtime_launch()
            .from_source_module(WorthUiRuntimeSourceModule::new(
                self.module_path,
                self.source_text,
            ))
            .prepare_for(app)
    }
}
