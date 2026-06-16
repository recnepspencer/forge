use worth_ui::facade::{
    WorthUiApp, WorthUiCapabilityPreparedReload, WorthUiCapabilityReloadEvidence,
    WorthUiCapabilityReloadRequest, WorthUiCapabilityReloadStage, WorthUiHeaderFramePlan,
    WorthUiHeaderFrameRebindDenial, WorthUiHeaderFrameRebindReceipt, WorthUiRuntimeHost,
    WorthUiThemeTokenReloadPackage,
};

use crate::app_capabilities::validation_header_frame_rebind_request;
use crate::reload::{
    ValidationPreparedReload, ValidationReloadEvidence, ValidationReloadInput,
    ValidationReloadRequest, ValidationReloadStage, ValidationReloadTick,
    ValidationRuntimeReloadTickOutcome, ValidationSourcePackage, ValidationThemeSource,
};

pub struct ValidationRuntimeWorkbench {
    app: WorthUiApp,
    runtime: WorthUiRuntimeHost,
    header_frame_plan: WorthUiHeaderFramePlan,
}

impl ValidationRuntimeWorkbench {
    pub(crate) fn new(
        app: WorthUiApp,
        runtime: WorthUiRuntimeHost,
        header_frame_plan: WorthUiHeaderFramePlan,
    ) -> Self {
        Self {
            app,
            runtime,
            header_frame_plan,
        }
    }

    pub fn app(&self) -> &WorthUiApp {
        &self.app
    }

    pub fn runtime(&self) -> &WorthUiRuntimeHost {
        &self.runtime
    }

    pub fn header_frame_plan(&self) -> &WorthUiHeaderFramePlan {
        &self.header_frame_plan
    }

    pub fn prepare_reload(&self, request: ValidationReloadRequest) -> ValidationPreparedReload {
        self.runtime
            .prepare_validation_reload(self.app.capabilities(), request)
    }

    pub fn activate_reload(
        &mut self,
        prepared: ValidationPreparedReload,
    ) -> Result<ValidationReloadEvidence, ValidationReloadStage> {
        prepared.activate(&mut self.runtime)
    }

    pub fn prepare_theme_capability_reload(
        &self,
        theme: &ValidationThemeSource,
    ) -> WorthUiCapabilityPreparedReload {
        self.runtime
            .prepare_capability_reload(WorthUiCapabilityReloadRequest::from_theme_tokens(
                theme_token_reload_package(theme),
            ))
    }

    pub fn activate_theme_capability_reload(
        &mut self,
        prepared: WorthUiCapabilityPreparedReload,
    ) -> Result<WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadStage> {
        prepared.activate(&mut self.runtime)
    }

    pub fn rebind_header_after_reload(
        &mut self,
        evidence: &ValidationReloadEvidence,
    ) -> Result<WorthUiHeaderFrameRebindReceipt, WorthUiHeaderFrameRebindDenial> {
        let (next_plan, receipt) = self.runtime.rebind_header_frame_after_reload(
            self.app.capabilities(),
            &self.header_frame_plan,
            validation_header_frame_rebind_request(),
            evidence,
        )?;
        self.header_frame_plan = next_plan;
        Ok(receipt)
    }

    pub fn apply_reload_tick(
        &mut self,
        tick: ValidationReloadTick,
    ) -> ValidationRuntimeReloadTickOutcome {
        match tick {
            ValidationReloadTick::Unchanged(observation) => {
                ValidationRuntimeReloadTickOutcome::Unchanged(observation)
            }
            ValidationReloadTick::Unreadable(denial) => {
                ValidationRuntimeReloadTickOutcome::InputUnreadable(denial)
            }
            ValidationReloadTick::Changed(input) => self.apply_reload_input(input),
        }
    }

    fn apply_reload_input(
        &mut self,
        input: ValidationReloadInput,
    ) -> ValidationRuntimeReloadTickOutcome {
        match input {
            ValidationReloadInput::SourcePackage(source) => self.apply_source_reload(source),
            ValidationReloadInput::HeaderTheme(theme) => self.apply_theme_reload(theme),
            ValidationReloadInput::SourcePackageAndHeaderTheme { source, theme } => {
                let source_outcome = self.apply_source_reload(source);
                let theme_outcome = self.apply_theme_reload(theme);
                merge_source_reload_with_theme_reload(source_outcome, theme_outcome)
            }
        }
    }

    fn apply_source_reload(
        &mut self,
        source: ValidationSourcePackage,
    ) -> ValidationRuntimeReloadTickOutcome {
        let prepared = self.prepare_reload(ValidationReloadRequest::from_source_module(
            source.module_path(),
            source.source_text(),
        ));
        if prepared.is_ready() {
            return match self.activate_reload(prepared) {
                Ok(evidence) => {
                    let header_receipt = self.rebind_header_after_reload(&evidence).ok();
                    ValidationRuntimeReloadTickOutcome::SourceReloaded {
                        evidence,
                        header_receipt,
                    }
                }
                Err(stage) => ValidationRuntimeReloadTickOutcome::SourceActivationDenied(stage),
            };
        }

        let evidence = prepared.evidence().clone();
        let header_receipt = self.rebind_header_after_reload(&evidence).ok();
        ValidationRuntimeReloadTickOutcome::SourceReloaded {
            evidence,
            header_receipt,
        }
    }

    fn apply_theme_reload(
        &mut self,
        theme: ValidationThemeSource,
    ) -> ValidationRuntimeReloadTickOutcome {
        let prepared = self.prepare_theme_capability_reload(&theme);
        if prepared.is_ready() {
            return match self.activate_theme_capability_reload(prepared) {
                Ok(evidence) => {
                    let header_receipt = self.rebind_header_after_capability_reload(&evidence).ok();
                    ValidationRuntimeReloadTickOutcome::ThemeReloaded {
                        evidence,
                        header_receipt,
                    }
                }
                Err(stage) => ValidationRuntimeReloadTickOutcome::ThemeActivationDenied(stage),
            };
        }

        let evidence = prepared.evidence().clone();
        let header_receipt = self.rebind_header_after_capability_reload(&evidence).ok();
        ValidationRuntimeReloadTickOutcome::ThemeReloaded {
            evidence,
            header_receipt,
        }
    }

    fn rebind_header_after_capability_reload(
        &mut self,
        evidence: &WorthUiCapabilityReloadEvidence,
    ) -> Result<WorthUiHeaderFrameRebindReceipt, WorthUiHeaderFrameRebindDenial> {
        let (next_plan, receipt) = self.runtime.rebind_header_frame_after_capability_reload(
            &self.header_frame_plan,
            validation_header_frame_rebind_request(),
            evidence,
        )?;
        self.header_frame_plan = next_plan;
        Ok(receipt)
    }
}

fn theme_token_reload_package(theme: &ValidationThemeSource) -> WorthUiThemeTokenReloadPackage {
    WorthUiThemeTokenReloadPackage::from_source(
        "apps/worth-ui-validation-app/theme/header.theme",
        theme.source_text(),
    )
}

fn merge_source_reload_with_theme_reload(
    source_outcome: ValidationRuntimeReloadTickOutcome,
    theme_outcome: ValidationRuntimeReloadTickOutcome,
) -> ValidationRuntimeReloadTickOutcome {
    match (source_outcome, theme_outcome) {
        (
            ValidationRuntimeReloadTickOutcome::SourceReloaded {
                evidence,
                header_receipt,
            },
            ValidationRuntimeReloadTickOutcome::ThemeReloaded {
                evidence: theme_evidence,
                header_receipt: theme_header_receipt,
            },
        ) => ValidationRuntimeReloadTickOutcome::SourceReloadedAndThemeReloaded {
            evidence,
            header_receipt,
            theme_evidence,
            theme_header_receipt,
        },
        (other, _) => other,
    }
}
