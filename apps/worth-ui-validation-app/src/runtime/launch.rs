use worth_ui::facade::{
    WorthUi, WorthUiApp, WorthUiRuntimeHost, WorthUiRuntimeLaunchDenial,
    WorthUiRuntimeLaunchPreparationDenial, WorthUiRuntimeSourceModule,
};
use worth_ui_harness::facade::{
    HarnessDensity, HarnessVisualFoundationBundle, HarnessVisualFoundationDenial,
    HarnessVisualFoundationReceipt, HarnessVisualFoundationRegistration,
};

use crate::shell::{NavigationSelection, ValidationPageId, ValidationRunSummary};
use crate::theme::{ValidationWorkbenchTheme, ValidationWorkbenchThemeError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationWorkbenchLaunch {
    initial_page: ValidationPageId,
}

pub struct PreparedValidationWorkbenchLaunch {
    app: WorthUiApp,
    runtime: WorthUiRuntimeHost,
    visual_foundation: HarnessVisualFoundationReceipt,
    render_theme: ValidationWorkbenchTheme,
    density: HarnessDensity,
    latest_run_receipt: Option<&'static str>,
    navigation: NavigationSelection,
}

#[derive(Debug)]
pub enum ValidationWorkbenchLaunchError {
    VisualFoundation(HarnessVisualFoundationDenial),
    RenderTheme(ValidationWorkbenchThemeError),
    RuntimePreparation(WorthUiRuntimeLaunchPreparationDenial),
    RuntimeLaunch(WorthUiRuntimeLaunchDenial),
}

impl ValidationWorkbenchLaunch {
    pub fn new() -> Self {
        Self {
            initial_page: ValidationPageId::SurfaceAtlas,
        }
    }

    pub fn with_initial_page(mut self, initial_page: ValidationPageId) -> Self {
        self.initial_page = initial_page;
        self
    }

    pub fn with_default_vscode_dark_theme(self) -> Self {
        self
    }

    pub fn prepare(
        self,
    ) -> Result<PreparedValidationWorkbenchLaunch, ValidationWorkbenchLaunchError> {
        let foundation = HarnessVisualFoundationBundle::vscode_like_dark()
            .prepare()
            .map_err(ValidationWorkbenchLaunchError::VisualFoundation)?;
        let foundation_receipt = foundation.receipt().clone();
        let render_theme = ValidationWorkbenchTheme::from_theme_tokens(foundation.theme_tokens())
            .map_err(ValidationWorkbenchLaunchError::RenderTheme)?;
        let app = WorthUi::app()
            .install_harness_visual_foundation(foundation)
            .freeze();
        let runtime_launch = WorthUi::runtime_launch()
            .from_source_module(WorthUiRuntimeSourceModule::new("validation/main.wui", ""))
            .prepare_for(&app)
            .map_err(ValidationWorkbenchLaunchError::RuntimePreparation)?;
        let runtime = app
            .launch_runtime(runtime_launch)
            .map_err(ValidationWorkbenchLaunchError::RuntimeLaunch)?;
        let mut navigation = NavigationSelection::default();
        navigation.select_page(self.initial_page);
        Ok(PreparedValidationWorkbenchLaunch {
            app,
            runtime,
            visual_foundation: foundation_receipt,
            render_theme,
            density: HarnessDensity::DEFAULT,
            latest_run_receipt: None,
            navigation,
        })
    }
}

impl Default for ValidationWorkbenchLaunch {
    fn default() -> Self {
        Self::new()
    }
}

impl PreparedValidationWorkbenchLaunch {
    pub fn runtime(&self) -> &WorthUiRuntimeHost {
        &self.runtime
    }

    pub fn app(&self) -> &WorthUiApp {
        &self.app
    }

    pub fn navigation(&self) -> &NavigationSelection {
        &self.navigation
    }

    pub fn navigation_mut(&mut self) -> &mut NavigationSelection {
        &mut self.navigation
    }

    pub fn visual_foundation(&self) -> &HarnessVisualFoundationReceipt {
        &self.visual_foundation
    }

    pub fn render_theme(&self) -> &ValidationWorkbenchTheme {
        &self.render_theme
    }

    pub fn density(&self) -> HarnessDensity {
        self.density
    }

    pub fn latest_run_receipt(&self) -> Option<&'static str> {
        self.latest_run_receipt
    }

    pub fn run_summary(&self) -> ValidationRunSummary {
        ValidationRunSummary::new(
            self.navigation.selected_scenario(),
            self.latest_run_receipt,
            self.runtime.inspect_active(),
        )
    }
}
