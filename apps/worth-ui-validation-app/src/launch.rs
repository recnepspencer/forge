mod authored_inputs;
mod observed_files;
mod observed_startup_evidence;
#[cfg(test)]
mod tests;

use std::io;
use std::{error::Error, fmt};

use worth_ui::facade::{
    CommandId, WorthUi, WorthUiApp, WorthUiHeaderFramePlan, WorthUiHeaderFramePlanDenial,
    WorthUiHeaderMenuPlan, WorthUiHeaderThemePlan, WorthUiPageHostPlan, WorthUiPageHostPlanDenial,
    WorthUiPageHostRequest, WorthUiRuntimeHost, WorthUiRuntimeLaunchDenial,
    WorthUiRuntimeLaunchPreparationDenial, WorthUiRuntimeSourceModule,
};

use crate::app_capabilities::{
    validation_header_appearance_request, validation_header_menu_requests,
    validation_header_theme_request, validation_worth_ui_app,
};
use crate::reload::{
    ValidationReloadInput, ValidationReloadTick, ValidationRuntimeReloadTickOutcome,
};
use crate::runtime_workbench::ValidationRuntimeWorkbench;

pub use authored_inputs::ValidationWorkbenchAuthoredInputs;
pub use observed_files::ValidationObservedWorkbenchFiles;
pub use observed_startup_evidence::{
    ValidationObservedStartupEvidence, ValidationObservedStartupFileKind,
    ValidationObservedStartupRow,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ValidationWorkbenchLaunch;

pub struct PreparedValidationWorkbenchLaunch {
    app: WorthUiApp,
    runtime: WorthUiRuntimeHost,
    header_frame_plan: WorthUiHeaderFramePlan,
    page_host_plan: WorthUiPageHostPlan,
    authored_inputs: ValidationWorkbenchAuthoredInputs,
    observed_startup: Option<ValidationObservedStartupEvidence>,
}

#[derive(Debug)]
pub enum ValidationWorkbenchLaunchError {
    ObservedStartupLoad(io::Error),
    HeaderFrame(WorthUiHeaderFramePlanDenial),
    PageHost(WorthUiPageHostPlanDenial),
    RuntimePreparation(WorthUiRuntimeLaunchPreparationDenial),
    RuntimeLaunch(WorthUiRuntimeLaunchDenial),
    AuthoredStartupRejected(&'static str),
}

impl fmt::Display for ValidationWorkbenchLaunchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ObservedStartupLoad(error) => {
                write!(
                    formatter,
                    "Could not read validation startup files: {error}"
                )
            }
            Self::HeaderFrame(error) => {
                write!(formatter, "Header frame preparation failed: {error:?}")
            }
            Self::PageHost(error) => write!(formatter, "Page host preparation failed: {error:?}"),
            Self::RuntimePreparation(error) => {
                write!(formatter, "Runtime preparation failed:\n{error}")
            }
            Self::RuntimeLaunch(error) => write!(formatter, "Runtime launch failed: {error:?}"),
            Self::AuthoredStartupRejected(lane) => {
                write!(
                    formatter,
                    "Authored startup reload was rejected in lane: {lane}"
                )
            }
        }
    }
}

impl Error for ValidationWorkbenchLaunchError {}

impl ValidationWorkbenchLaunch {
    pub fn new() -> Self {
        Self
    }

    pub fn prepare(
        self,
    ) -> Result<PreparedValidationWorkbenchLaunch, ValidationWorkbenchLaunchError> {
        let observed_files = default_observed_workbench_files()
            .map_err(ValidationWorkbenchLaunchError::ObservedStartupLoad)?;
        self.prepare_from_observed_files(&observed_files)
    }

    pub fn prepare_from_workspace_root(
        self,
        workspace_root: impl AsRef<std::path::Path>,
    ) -> Result<PreparedValidationWorkbenchLaunch, ValidationWorkbenchLaunchError> {
        let observed_files = ValidationObservedWorkbenchFiles::from_workspace_root(workspace_root);
        self.prepare_from_observed_files(&observed_files)
    }

    pub fn prepare_from_observed_files(
        self,
        observed_files: &ValidationObservedWorkbenchFiles,
    ) -> Result<PreparedValidationWorkbenchLaunch, ValidationWorkbenchLaunchError> {
        let authored_inputs =
            ValidationWorkbenchAuthoredInputs::from_observed_files(observed_files)
                .map_err(ValidationWorkbenchLaunchError::ObservedStartupLoad)?;
        let mut prepared = self.prepare_from_authored_inputs(authored_inputs.clone())?;
        prepared.observed_startup = Some(ValidationObservedStartupEvidence::from_observed_files(
            observed_files,
            &authored_inputs,
        ));
        Ok(prepared)
    }

    pub fn prepare_from_authored_inputs(
        self,
        authored_inputs: ValidationWorkbenchAuthoredInputs,
    ) -> Result<PreparedValidationWorkbenchLaunch, ValidationWorkbenchLaunchError> {
        let app = validation_worth_ui_app();
        let header_frame_plan = WorthUiHeaderFramePlan::from_snapshot(
            app.capabilities(),
            validation_header_menu_requests(),
            validation_header_theme_request(),
            validation_header_appearance_request(),
        )
        .map_err(ValidationWorkbenchLaunchError::HeaderFrame)?;
        let runtime_launch = WorthUi::runtime_launch()
            .from_source_module(WorthUiRuntimeSourceModule::new(
                authored_inputs.source().module_path(),
                authored_inputs.source().source_text(),
            ))
            .prepare_for(&app)
            .map_err(ValidationWorkbenchLaunchError::RuntimePreparation)?;
        let runtime = app
            .launch_runtime(runtime_launch)
            .map_err(ValidationWorkbenchLaunchError::RuntimeLaunch)?;
        let page_host_plan =
            WorthUiPageHostPlan::from_runtime(&runtime, validation_page_host_request())
                .map_err(ValidationWorkbenchLaunchError::PageHost)?;
        let mut workbench =
            ValidationRuntimeWorkbench::new(app, runtime, header_frame_plan, page_host_plan);
        apply_authored_startup_inputs(&mut workbench, &authored_inputs)?;

        Ok(PreparedValidationWorkbenchLaunch::from_runtime_workbench(
            workbench,
            authored_inputs,
        ))
    }
}

impl PreparedValidationWorkbenchLaunch {
    fn from_runtime_workbench(
        workbench: ValidationRuntimeWorkbench,
        authored_inputs: ValidationWorkbenchAuthoredInputs,
    ) -> Self {
        let (app, runtime, header_frame_plan, page_host_plan) = workbench.into_launch_parts();
        Self {
            app,
            runtime,
            header_frame_plan,
            page_host_plan,
            authored_inputs,
            observed_startup: None,
        }
    }

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

    pub fn header_appearance_plan(&self) -> &worth_ui::facade::WorthUiHeaderAppearancePlan {
        self.header_frame_plan.appearance_plan()
    }

    pub fn header_frame_plan(&self) -> &WorthUiHeaderFramePlan {
        &self.header_frame_plan
    }

    pub fn page_host_plan(&self) -> &WorthUiPageHostPlan {
        &self.page_host_plan
    }

    pub fn authored_inputs(&self) -> &ValidationWorkbenchAuthoredInputs {
        &self.authored_inputs
    }

    pub fn observed_startup(&self) -> Option<&ValidationObservedStartupEvidence> {
        self.observed_startup.as_ref()
    }

    pub fn into_runtime_workbench(self) -> ValidationRuntimeWorkbench {
        ValidationRuntimeWorkbench::new(
            self.app,
            self.runtime,
            self.header_frame_plan,
            self.page_host_plan,
        )
    }

    pub fn has_command(&self, command_id: &str) -> bool {
        CommandId::new(command_id)
            .ok()
            .and_then(|id| self.app.capabilities().commands().get(&id))
            .is_some()
    }
}

fn apply_authored_startup_inputs(
    workbench: &mut ValidationRuntimeWorkbench,
    authored_inputs: &ValidationWorkbenchAuthoredInputs,
) -> Result<(), ValidationWorkbenchLaunchError> {
    if let Some(theme) = authored_inputs.theme() {
        require_successful_outcome(
            workbench.apply_reload_tick(ValidationReloadTick::Changed(
                ValidationReloadInput::HeaderTheme(theme.clone()),
            )),
            "header theme",
        )?;
    }
    if let Some(commands) = authored_inputs.commands() {
        require_successful_outcome(
            workbench.apply_reload_tick(ValidationReloadTick::Changed(
                ValidationReloadInput::HeaderCommands(commands.clone()),
            )),
            "header commands",
        )?;
    }
    if let Some(command_projections) = authored_inputs.command_projections() {
        require_successful_outcome(
            workbench.apply_reload_tick(ValidationReloadTick::Changed(
                ValidationReloadInput::HeaderCommandProjections(command_projections.clone()),
            )),
            "header command projections",
        )?;
    }
    if let Some(component) = authored_inputs.component() {
        require_successful_outcome(
            workbench.apply_reload_tick(ValidationReloadTick::Changed(
                ValidationReloadInput::HeaderComponents(component.clone()),
            )),
            "header component",
        )?;
    }
    match (authored_inputs.appearance(), authored_inputs.density()) {
        (Some(appearance), Some(density)) => require_successful_outcome(
            workbench.apply_reload_tick(ValidationReloadTick::Changed(
                ValidationReloadInput::HeaderAppearanceAndDensity {
                    appearance: appearance.clone(),
                    density: density.clone(),
                },
            )),
            "header appearance+density",
        )?,
        (Some(appearance), None) => require_successful_outcome(
            workbench.apply_reload_tick(ValidationReloadTick::Changed(
                ValidationReloadInput::HeaderAppearance(appearance.clone()),
            )),
            "header appearance",
        )?,
        (None, Some(density)) => require_successful_outcome(
            workbench.apply_reload_tick(ValidationReloadTick::Changed(
                ValidationReloadInput::HeaderDensity(density.clone()),
            )),
            "header density",
        )?,
        (None, None) => {}
    }
    Ok(())
}

fn require_successful_outcome(
    outcome: ValidationRuntimeReloadTickOutcome,
    lane: &'static str,
) -> Result<(), ValidationWorkbenchLaunchError> {
    let succeeded = match outcome {
        ValidationRuntimeReloadTickOutcome::ThemeReloaded {
            evidence,
            phase_execution,
            ..
        }
        | ValidationRuntimeReloadTickOutcome::CommandReloaded {
            evidence,
            phase_execution,
            ..
        }
        | ValidationRuntimeReloadTickOutcome::CommandProjectionReloaded {
            evidence,
            phase_execution,
            ..
        }
        | ValidationRuntimeReloadTickOutcome::ComponentReloaded {
            evidence,
            phase_execution,
            ..
        }
        | ValidationRuntimeReloadTickOutcome::AppearanceReloaded {
            evidence,
            phase_execution,
            ..
        }
        | ValidationRuntimeReloadTickOutcome::DensityReloaded {
            evidence,
            phase_execution,
            ..
        }
        | ValidationRuntimeReloadTickOutcome::AppearanceAndDensityReloaded {
            evidence,
            phase_execution,
            ..
        } => startup_outcome_is_acceptable(evidence.status(), phase_execution.is_some()),
        _ => false,
    };
    if succeeded {
        Ok(())
    } else {
        Err(ValidationWorkbenchLaunchError::AuthoredStartupRejected(
            lane,
        ))
    }
}

fn startup_outcome_is_acceptable(
    status: worth_ui::facade::WorthUiCapabilityReloadStatus,
    _has_phase_execution: bool,
) -> bool {
    match status {
        worth_ui::facade::WorthUiCapabilityReloadStatus::EquivalentNoOp => true,
        worth_ui::facade::WorthUiCapabilityReloadStatus::Activated
        | worth_ui::facade::WorthUiCapabilityReloadStatus::ReadyForFrameBoundary => true,
        worth_ui::facade::WorthUiCapabilityReloadStatus::Denied(_) => false,
    }
}

fn default_observed_workbench_files() -> io::Result<ValidationObservedWorkbenchFiles> {
    Ok(
        ValidationObservedWorkbenchFiles::new(default_validation_source_path())
            .with_theme_path(default_header_theme_path())
            .with_command_path(default_header_command_path())
            .with_command_projection_path(default_header_command_projection_path())
            .with_component_path(default_header_component_path())
            .with_appearance_path(default_header_appearance_path())
            .with_density_path(default_header_density_path()),
    )
}

fn manifest_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn default_validation_source_path() -> std::path::PathBuf {
    manifest_dir().join("source/header.wui")
}

fn default_header_theme_path() -> std::path::PathBuf {
    manifest_dir().join("theme/header.theme")
}

fn default_header_command_path() -> std::path::PathBuf {
    manifest_dir().join("theme/header.commands")
}

fn default_header_command_projection_path() -> std::path::PathBuf {
    manifest_dir().join("theme/header.projections")
}

fn default_header_component_path() -> std::path::PathBuf {
    manifest_dir().join("theme/header.components")
}

fn default_header_appearance_path() -> std::path::PathBuf {
    manifest_dir().join("theme/header.appearance")
}

fn default_header_density_path() -> std::path::PathBuf {
    manifest_dir().join("theme/header.density")
}

pub(crate) fn validation_page_host_request() -> WorthUiPageHostRequest {
    WorthUiPageHostRequest::new(crate::product_preview::PREVIEW_DEFAULT_PAGE)
}
