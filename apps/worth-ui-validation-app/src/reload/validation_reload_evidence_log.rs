use worth_ui::facade::{
    WorthUiCapabilityReloadStage, WorthUiCapabilityReloadStatus, WorthUiHeaderFrameRebindStatus,
};

use super::{
    ValidationReloadEvidence, ValidationReloadInputDenial, ValidationReloadStage,
    ValidationReloadStatus, ValidationRuntimeReloadTickOutcome, ValidationThemeReloadDenial,
};

const MAX_VALIDATION_RELOAD_EVIDENCE_ENTRIES: usize = 32;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidationReloadEvidenceLog {
    entries: Vec<ValidationReloadEvidenceEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationReloadEvidenceEntry {
    RuntimeReload {
        status: ValidationReloadStatus,
        active_artifact_digest: u64,
        active_plan_digest: u64,
        header_rebind_status: Option<WorthUiHeaderFrameRebindStatus>,
    },
    ThemeReload {
        status: WorthUiCapabilityReloadStatus,
        active_snapshot_digest: u64,
        touched_theme_token_count: usize,
        header_rebind_status: Option<WorthUiHeaderFrameRebindStatus>,
    },
    CommandReload {
        status: WorthUiCapabilityReloadStatus,
        active_snapshot_digest: u64,
        touched_command_count: usize,
        header_rebind_status: Option<WorthUiHeaderFrameRebindStatus>,
    },
    CommandProjectionReload {
        status: WorthUiCapabilityReloadStatus,
        active_snapshot_digest: u64,
        touched_projection_count: usize,
        header_rebind_status: Option<WorthUiHeaderFrameRebindStatus>,
    },
    ThemeDenied(ValidationThemeReloadDenial),
    SourceActivationDenied(ValidationReloadStage),
    ThemeActivationDenied(WorthUiCapabilityReloadStage),
    CommandActivationDenied(WorthUiCapabilityReloadStage),
    CommandProjectionActivationDenied(WorthUiCapabilityReloadStage),
    InputUnreadable(ValidationReloadInputDenial),
}

impl ValidationReloadEvidenceLog {
    pub fn record_runtime_reload_tick_outcome(
        &mut self,
        outcome: ValidationRuntimeReloadTickOutcome,
    ) {
        match outcome {
            ValidationRuntimeReloadTickOutcome::Unchanged(_) => {}
            ValidationRuntimeReloadTickOutcome::SourceReloaded {
                evidence,
                header_receipt,
            } => self.record_runtime_reload(
                &evidence,
                header_receipt.as_ref().map(|receipt| receipt.status()),
            ),
            ValidationRuntimeReloadTickOutcome::SourceActivationDenied(stage) => {
                self.record_source_activation_denial(stage);
            }
            ValidationRuntimeReloadTickOutcome::ThemeReloaded {
                evidence,
                header_receipt,
            } => {
                self.record_theme_reload(
                    &evidence,
                    header_receipt.as_ref().map(|receipt| receipt.status()),
                );
            }
            ValidationRuntimeReloadTickOutcome::ThemeActivationDenied(stage) => {
                self.record_theme_activation_denial(stage);
            }
            ValidationRuntimeReloadTickOutcome::CommandReloaded {
                evidence,
                header_receipt,
            } => {
                self.record_command_reload(
                    &evidence,
                    header_receipt.as_ref().map(|receipt| receipt.status()),
                );
            }
            ValidationRuntimeReloadTickOutcome::CommandProjectionReloaded {
                evidence,
                header_receipt,
            } => {
                self.record_command_projection_reload(
                    &evidence,
                    header_receipt.as_ref().map(|receipt| receipt.status()),
                );
            }
            ValidationRuntimeReloadTickOutcome::CommandActivationDenied(stage) => {
                self.record_command_activation_denial(stage);
            }
            ValidationRuntimeReloadTickOutcome::CommandProjectionActivationDenied(stage) => {
                self.record_command_projection_activation_denial(stage);
            }
            ValidationRuntimeReloadTickOutcome::SourceReloadedAndThemeDenied {
                evidence,
                header_receipt,
                theme_denial,
            } => {
                self.record_runtime_reload(
                    &evidence,
                    header_receipt.as_ref().map(|receipt| receipt.status()),
                );
                self.record_theme_denial(theme_denial);
            }
            ValidationRuntimeReloadTickOutcome::SourceReloadedAndThemeReloaded {
                evidence,
                header_receipt,
                theme_evidence,
                theme_header_receipt,
            } => {
                self.record_runtime_reload(
                    &evidence,
                    header_receipt.as_ref().map(|receipt| receipt.status()),
                );
                self.record_theme_reload(
                    &theme_evidence,
                    theme_header_receipt
                        .as_ref()
                        .map(|receipt| receipt.status()),
                );
            }
            ValidationRuntimeReloadTickOutcome::InputUnreadable(denial) => {
                self.record_input_unreadable(denial);
            }
        }
    }

    pub fn record_runtime_reload(
        &mut self,
        evidence: &ValidationReloadEvidence,
        header_rebind_status: Option<WorthUiHeaderFrameRebindStatus>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::RuntimeReload {
            status: evidence.status(),
            active_artifact_digest: evidence.active_artifact_digest_after(),
            active_plan_digest: evidence.active_plan_digest_after(),
            header_rebind_status,
        });
    }

    pub fn record_theme_denial(&mut self, denial: ValidationThemeReloadDenial) {
        self.push_entry(ValidationReloadEvidenceEntry::ThemeDenied(denial));
    }

    pub fn record_theme_reload(
        &mut self,
        evidence: &worth_ui::facade::WorthUiCapabilityReloadEvidence,
        header_rebind_status: Option<WorthUiHeaderFrameRebindStatus>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::ThemeReload {
            status: evidence.status(),
            active_snapshot_digest: evidence.active_snapshot_digest_after(),
            touched_theme_token_count: evidence.touched_theme_token_count(),
            header_rebind_status,
        });
    }

    pub fn record_command_reload(
        &mut self,
        evidence: &worth_ui::facade::WorthUiCapabilityReloadEvidence,
        header_rebind_status: Option<WorthUiHeaderFrameRebindStatus>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::CommandReload {
            status: evidence.status(),
            active_snapshot_digest: evidence.active_snapshot_digest_after(),
            touched_command_count: evidence.touched_theme_token_count(),
            header_rebind_status,
        });
    }

    pub fn record_command_projection_reload(
        &mut self,
        evidence: &worth_ui::facade::WorthUiCapabilityReloadEvidence,
        header_rebind_status: Option<WorthUiHeaderFrameRebindStatus>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::CommandProjectionReload {
            status: evidence.status(),
            active_snapshot_digest: evidence.active_snapshot_digest_after(),
            touched_projection_count: evidence.touched_theme_token_count(),
            header_rebind_status,
        });
    }

    pub fn record_source_activation_denial(&mut self, stage: ValidationReloadStage) {
        self.push_entry(ValidationReloadEvidenceEntry::SourceActivationDenied(stage));
    }

    pub fn record_theme_activation_denial(&mut self, stage: WorthUiCapabilityReloadStage) {
        self.push_entry(ValidationReloadEvidenceEntry::ThemeActivationDenied(stage));
    }

    pub fn record_command_activation_denial(&mut self, stage: WorthUiCapabilityReloadStage) {
        self.push_entry(ValidationReloadEvidenceEntry::CommandActivationDenied(
            stage,
        ));
    }

    pub fn record_command_projection_activation_denial(
        &mut self,
        stage: WorthUiCapabilityReloadStage,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::CommandProjectionActivationDenied(stage));
    }

    pub fn record_input_unreadable(&mut self, denial: ValidationReloadInputDenial) {
        self.push_entry(ValidationReloadEvidenceEntry::InputUnreadable(denial));
    }

    pub fn entries(&self) -> &[ValidationReloadEvidenceEntry] {
        &self.entries
    }

    pub fn latest(&self) -> Option<&ValidationReloadEvidenceEntry> {
        self.entries.last()
    }

    fn push_entry(&mut self, entry: ValidationReloadEvidenceEntry) {
        if self.entries.len() == MAX_VALIDATION_RELOAD_EVIDENCE_ENTRIES {
            self.entries.remove(0);
        }
        self.entries.push(entry);
    }
}
