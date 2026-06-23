use worth_ui::facade::{WorthUiCapabilityReloadEvidence, WorthUiRebindPhaseExecutionReceipt};

use crate::reload::{
    ValidationAppearanceSource, ValidationCommandProjectionSource, ValidationCommandSource,
    ValidationComponentSource, ValidationDensitySource, ValidationRuntimeReloadTickOutcome,
    ValidationThemeSource,
};

use super::ValidationRuntimeWorkbench;

impl ValidationRuntimeWorkbench {
    pub(crate) fn apply_theme_reload(
        &mut self,
        theme: ValidationThemeSource,
    ) -> ValidationRuntimeReloadTickOutcome {
        let prepared = self.prepare_theme_capability_reload(&theme);
        if prepared.is_ready() {
            return match self.activate_theme_capability_reload(prepared) {
                Ok(evidence) => self.theme_reloaded_outcome(evidence),
                Err(stage) => ValidationRuntimeReloadTickOutcome::ThemeActivationDenied(stage),
            };
        }

        self.theme_reloaded_outcome(prepared.evidence().clone())
    }

    pub(crate) fn apply_command_reload(
        &mut self,
        commands: ValidationCommandSource,
    ) -> ValidationRuntimeReloadTickOutcome {
        let prepared = self.prepare_command_capability_reload(&commands);
        if prepared.is_ready() {
            return match self.activate_capability_reload(prepared) {
                Ok(evidence) => self.command_reloaded_outcome(evidence),
                Err(stage) => ValidationRuntimeReloadTickOutcome::CommandActivationDenied(stage),
            };
        }

        self.command_reloaded_outcome(prepared.evidence().clone())
    }

    pub(crate) fn apply_command_projection_reload(
        &mut self,
        command_projections: ValidationCommandProjectionSource,
    ) -> ValidationRuntimeReloadTickOutcome {
        let prepared = self.prepare_command_projection_capability_reload(&command_projections);
        if prepared.is_ready() {
            return match self.activate_capability_reload(prepared) {
                Ok(evidence) => self.command_projection_reloaded_outcome(evidence),
                Err(stage) => {
                    ValidationRuntimeReloadTickOutcome::CommandProjectionActivationDenied(stage)
                }
            };
        }

        self.command_projection_reloaded_outcome(prepared.evidence().clone())
    }

    pub(crate) fn apply_component_reload(
        &mut self,
        component: ValidationComponentSource,
    ) -> ValidationRuntimeReloadTickOutcome {
        let prepared = self.prepare_component_capability_reload(&component);
        if prepared.is_ready() {
            return match self.activate_capability_reload(prepared) {
                Ok(evidence) => self.component_reloaded_outcome(evidence),
                Err(stage) => ValidationRuntimeReloadTickOutcome::ComponentActivationDenied(stage),
            };
        }

        self.component_reloaded_outcome(prepared.evidence().clone())
    }

    pub(crate) fn apply_appearance_reload(
        &mut self,
        appearance: ValidationAppearanceSource,
    ) -> ValidationRuntimeReloadTickOutcome {
        let prepared = self.prepare_appearance_capability_reload(&appearance);
        if prepared.is_ready() {
            return match self.activate_capability_reload(prepared) {
                Ok(evidence) => self.appearance_reloaded_outcome(evidence),
                Err(stage) => ValidationRuntimeReloadTickOutcome::AppearanceActivationDenied(stage),
            };
        }

        self.appearance_reloaded_outcome(prepared.evidence().clone())
    }

    pub(crate) fn apply_density_reload(
        &mut self,
        density: ValidationDensitySource,
    ) -> ValidationRuntimeReloadTickOutcome {
        let prepared = self.prepare_density_capability_reload(&density);
        if prepared.is_ready() {
            return match self.activate_capability_reload(prepared) {
                Ok(evidence) => self.density_reloaded_outcome(evidence),
                Err(stage) => ValidationRuntimeReloadTickOutcome::DensityActivationDenied(stage),
            };
        }

        self.density_reloaded_outcome(prepared.evidence().clone())
    }

    fn theme_reloaded_outcome(
        &mut self,
        evidence: WorthUiCapabilityReloadEvidence,
    ) -> ValidationRuntimeReloadTickOutcome {
        let phase_execution = self.capability_rebind_receipts(&evidence);
        ValidationRuntimeReloadTickOutcome::ThemeReloaded {
            evidence,
            phase_execution,
        }
    }

    fn command_reloaded_outcome(
        &mut self,
        evidence: WorthUiCapabilityReloadEvidence,
    ) -> ValidationRuntimeReloadTickOutcome {
        let phase_execution = self.capability_rebind_receipts(&evidence);
        ValidationRuntimeReloadTickOutcome::CommandReloaded {
            evidence,
            phase_execution,
        }
    }

    fn command_projection_reloaded_outcome(
        &mut self,
        evidence: WorthUiCapabilityReloadEvidence,
    ) -> ValidationRuntimeReloadTickOutcome {
        let phase_execution = self.capability_rebind_receipts(&evidence);
        ValidationRuntimeReloadTickOutcome::CommandProjectionReloaded {
            evidence,
            phase_execution,
        }
    }

    fn component_reloaded_outcome(
        &mut self,
        evidence: WorthUiCapabilityReloadEvidence,
    ) -> ValidationRuntimeReloadTickOutcome {
        let phase_execution = self.capability_rebind_receipts(&evidence);
        ValidationRuntimeReloadTickOutcome::ComponentReloaded {
            evidence,
            phase_execution,
        }
    }

    fn appearance_reloaded_outcome(
        &mut self,
        evidence: WorthUiCapabilityReloadEvidence,
    ) -> ValidationRuntimeReloadTickOutcome {
        let phase_execution = self.capability_rebind_receipts(&evidence);
        ValidationRuntimeReloadTickOutcome::AppearanceReloaded {
            evidence,
            phase_execution,
        }
    }

    fn density_reloaded_outcome(
        &mut self,
        evidence: WorthUiCapabilityReloadEvidence,
    ) -> ValidationRuntimeReloadTickOutcome {
        let phase_execution = self.capability_rebind_receipts(&evidence);
        ValidationRuntimeReloadTickOutcome::DensityReloaded {
            evidence,
            phase_execution,
        }
    }

    pub(super) fn capability_rebind_receipts(
        &mut self,
        evidence: &WorthUiCapabilityReloadEvidence,
    ) -> Option<WorthUiRebindPhaseExecutionReceipt> {
        let admitted_change = self
            .runtime
            .admit_capability_runtime_change(evidence)
            .expect("capability reload evidence should admit a common runtime change");
        self.runtime_change_rebind_receipts(&admitted_change)
    }
}
