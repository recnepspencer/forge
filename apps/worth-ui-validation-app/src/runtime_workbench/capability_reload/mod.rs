mod apply;
mod packages;

/// Transitional validation-app adapters for older capability-family reload
/// proof slices.
///
/// These helpers intentionally remain separate from the ordinary authored
/// source-package ingress path so the unfinished family-local seams stay
/// visible during the Phase 23 transition.
use worth_ui::facade::{
    WorthUiCapabilityPreparedReload, WorthUiCapabilityReloadEvidence,
    WorthUiCapabilityReloadRequest, WorthUiCapabilityReloadStage,
    WorthUiRebindPhaseExecutionReceipt,
};

use crate::reload::{
    ValidationAppearanceSource, ValidationCommandProjectionSource, ValidationCommandSource,
    ValidationComponentSource, ValidationDensitySource, ValidationRuntimeReloadTickOutcome,
    ValidationThemeSource,
};

use super::ValidationRuntimeWorkbench;
use packages::{
    appearance_reload_package, command_projection_reload_package, command_reload_package,
    component_reload_package, density_reload_package, theme_token_reload_package,
};

impl ValidationRuntimeWorkbench {
    pub fn prepare_theme_capability_reload(
        &self,
        theme: &ValidationThemeSource,
    ) -> WorthUiCapabilityPreparedReload {
        self.runtime
            .prepare_capability_reload(WorthUiCapabilityReloadRequest::from_theme_tokens(
                theme_token_reload_package(theme),
            ))
    }

    pub fn prepare_command_capability_reload(
        &self,
        commands: &ValidationCommandSource,
    ) -> WorthUiCapabilityPreparedReload {
        self.runtime
            .prepare_capability_reload(WorthUiCapabilityReloadRequest::from_commands(
                command_reload_package(commands),
            ))
    }

    pub fn prepare_command_projection_capability_reload(
        &self,
        command_projections: &ValidationCommandProjectionSource,
    ) -> WorthUiCapabilityPreparedReload {
        self.runtime.prepare_capability_reload(
            WorthUiCapabilityReloadRequest::from_command_projections(
                command_projection_reload_package(command_projections),
            ),
        )
    }

    pub fn prepare_component_capability_reload(
        &self,
        component: &ValidationComponentSource,
    ) -> WorthUiCapabilityPreparedReload {
        self.runtime
            .prepare_capability_reload(WorthUiCapabilityReloadRequest::from_components(
                component_reload_package(component),
            ))
    }

    pub fn prepare_appearance_capability_reload(
        &self,
        appearance: &ValidationAppearanceSource,
    ) -> WorthUiCapabilityPreparedReload {
        self.runtime
            .prepare_capability_reload(WorthUiCapabilityReloadRequest::from_appearance(
                appearance_reload_package(appearance),
            ))
    }

    pub fn prepare_density_capability_reload(
        &self,
        density: &ValidationDensitySource,
    ) -> WorthUiCapabilityPreparedReload {
        self.runtime
            .prepare_capability_reload(WorthUiCapabilityReloadRequest::from_density(
                density_reload_package(density),
            ))
    }

    pub fn prepare_appearance_and_density_capability_reload(
        &self,
        appearance: &ValidationAppearanceSource,
        density: &ValidationDensitySource,
    ) -> WorthUiCapabilityPreparedReload {
        self.runtime
            .prepare_capability_reload(WorthUiCapabilityReloadRequest::batch([
                WorthUiCapabilityReloadRequest::from_appearance(appearance_reload_package(
                    appearance,
                )),
                WorthUiCapabilityReloadRequest::from_density(density_reload_package(density)),
            ]))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn prepare_authored_batch_capability_reload(
        &self,
        theme: Option<&ValidationThemeSource>,
        command: Option<&ValidationCommandSource>,
        command_projection: Option<&ValidationCommandProjectionSource>,
        component: Option<&ValidationComponentSource>,
        appearance: Option<&ValidationAppearanceSource>,
        density: Option<&ValidationDensitySource>,
    ) -> WorthUiCapabilityPreparedReload {
        let mut requests = Vec::new();
        if let Some(theme) = theme {
            requests.push(WorthUiCapabilityReloadRequest::from_theme_tokens(
                theme_token_reload_package(theme),
            ));
        }
        if let Some(command) = command {
            requests.push(WorthUiCapabilityReloadRequest::from_commands(
                command_reload_package(command),
            ));
        }
        if let Some(command_projection) = command_projection {
            requests.push(WorthUiCapabilityReloadRequest::from_command_projections(
                command_projection_reload_package(command_projection),
            ));
        }
        if let Some(component) = component {
            requests.push(WorthUiCapabilityReloadRequest::from_components(
                component_reload_package(component),
            ));
        }
        if let Some(appearance) = appearance {
            requests.push(WorthUiCapabilityReloadRequest::from_appearance(
                appearance_reload_package(appearance),
            ));
        }
        if let Some(density) = density {
            requests.push(WorthUiCapabilityReloadRequest::from_density(
                density_reload_package(density),
            ));
        }
        self.runtime
            .prepare_capability_reload(WorthUiCapabilityReloadRequest::batch(requests))
    }

    pub fn activate_capability_reload(
        &mut self,
        prepared: WorthUiCapabilityPreparedReload,
    ) -> Result<WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadStage> {
        prepared.activate(&mut self.runtime)
    }

    pub fn activate_theme_capability_reload(
        &mut self,
        prepared: WorthUiCapabilityPreparedReload,
    ) -> Result<WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadStage> {
        self.activate_capability_reload(prepared)
    }

    pub fn apply_command_projection_source(
        &mut self,
        command_projections: ValidationCommandProjectionSource,
    ) -> ValidationRuntimeReloadTickOutcome {
        self.apply_command_projection_reload(command_projections)
    }

    pub fn apply_appearance_and_density_capability_reload(
        &mut self,
        appearance: &ValidationAppearanceSource,
        density: &ValidationDensitySource,
    ) -> Result<
        (
            WorthUiCapabilityReloadEvidence,
            Option<WorthUiRebindPhaseExecutionReceipt>,
        ),
        WorthUiCapabilityReloadStage,
    > {
        let prepared = self.prepare_appearance_and_density_capability_reload(appearance, density);
        if prepared.is_ready() {
            let evidence = self.activate_capability_reload(prepared)?;
            let phase_execution = self.capability_rebind_receipts(&evidence);
            return Ok((evidence, phase_execution));
        }

        let evidence = prepared.evidence().clone();
        let phase_execution = self.capability_rebind_receipts(&evidence);
        Ok((evidence, phase_execution))
    }
}

pub(crate) fn merge_source_reload_with_theme_reload(
    source_outcome: ValidationRuntimeReloadTickOutcome,
    theme_outcome: ValidationRuntimeReloadTickOutcome,
) -> ValidationRuntimeReloadTickOutcome {
    match (source_outcome, theme_outcome) {
        (
            ValidationRuntimeReloadTickOutcome::SourceReloaded {
                evidence,
                phase_execution,
                authored_structural,
            },
            ValidationRuntimeReloadTickOutcome::ThemeReloaded {
                evidence: theme_evidence,
                phase_execution: theme_phase_execution,
            },
        ) => ValidationRuntimeReloadTickOutcome::SourceReloadedAndThemeReloaded {
            evidence,
            phase_execution,
            authored_structural,
            theme_evidence,
            theme_phase_execution,
        },
        (other, _) => other,
    }
}
