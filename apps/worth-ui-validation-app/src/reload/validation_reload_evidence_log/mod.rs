mod capability_entries;
mod entry;

pub use entry::ValidationReloadEvidenceEntry;

use worth_ui::facade::WorthUiRebindPhaseExecutionReceipt;
use worth_ui::facade::{WorthUiCapabilityReloadFamilyKind, WorthUiCapabilityReloadFamilyStatus};

use crate::reload::{
    ValidationHeaderRebindEvidence, ValidationPageHostRebindEvidence,
    ValidationPhaseExecutionEvidence, ValidationReloadEvidence, ValidationReloadInputDenial,
    ValidationReloadStage, ValidationRuntimeReloadTickOutcome, ValidationThemeReloadDenial,
};

const MAX_VALIDATION_RELOAD_EVIDENCE_ENTRIES: usize = 32;
use entry::header_rebind_evidence;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidationReloadEvidenceLog {
    entries: Vec<ValidationReloadEvidenceEntry>,
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
                phase_execution,
                authored_structural,
            } => {
                self.record_runtime_reload(&evidence, phase_execution.as_ref(), authored_structural)
            }
            ValidationRuntimeReloadTickOutcome::SourceActivationDenied(stage) => {
                self.record_source_activation_denial(stage);
            }
            ValidationRuntimeReloadTickOutcome::ThemeReloaded {
                evidence,
                phase_execution,
            } => self.record_theme_reload(&evidence, phase_execution.as_ref()),
            ValidationRuntimeReloadTickOutcome::ThemeActivationDenied(stage) => {
                self.record_theme_activation_denial(stage);
            }
            ValidationRuntimeReloadTickOutcome::ComponentReloaded {
                evidence,
                phase_execution,
            } => self.record_component_reload(&evidence, phase_execution.as_ref()),
            ValidationRuntimeReloadTickOutcome::ComponentActivationDenied(stage) => {
                self.record_component_activation_denial(stage);
            }
            ValidationRuntimeReloadTickOutcome::CommandReloaded {
                evidence,
                phase_execution,
            } => self.record_command_reload(&evidence, phase_execution.as_ref()),
            ValidationRuntimeReloadTickOutcome::CommandProjectionReloaded {
                evidence,
                phase_execution,
            } => {
                self.record_command_projection_reload(&evidence, phase_execution.as_ref());
            }
            ValidationRuntimeReloadTickOutcome::AppearanceReloaded {
                evidence,
                phase_execution,
            } => self.record_appearance_reload(&evidence, phase_execution.as_ref()),
            ValidationRuntimeReloadTickOutcome::DensityReloaded {
                evidence,
                phase_execution,
            } => self.record_density_reload(&evidence, phase_execution.as_ref()),
            ValidationRuntimeReloadTickOutcome::AppearanceAndDensityReloaded {
                evidence,
                phase_execution,
            } => {
                let appearance_changed =
                    family_changed(&evidence, WorthUiCapabilityReloadFamilyKind::Appearance);
                if evidence
                    .family_rows()
                    .iter()
                    .any(|row| row.family() == WorthUiCapabilityReloadFamilyKind::Appearance)
                {
                    self.record_appearance_reload(
                        &evidence,
                        if appearance_changed {
                            phase_execution.as_ref()
                        } else {
                            None
                        },
                    );
                }
                let density_changed =
                    family_changed(&evidence, WorthUiCapabilityReloadFamilyKind::Density);
                if evidence
                    .family_rows()
                    .iter()
                    .any(|row| row.family() == WorthUiCapabilityReloadFamilyKind::Density)
                {
                    self.record_density_reload(
                        &evidence,
                        if density_changed {
                            phase_execution.as_ref()
                        } else {
                            None
                        },
                    );
                }
            }
            ValidationRuntimeReloadTickOutcome::CommandActivationDenied(stage) => {
                self.record_command_activation_denial(stage);
            }
            ValidationRuntimeReloadTickOutcome::CommandProjectionActivationDenied(stage) => {
                self.record_command_projection_activation_denial(stage);
            }
            ValidationRuntimeReloadTickOutcome::AppearanceActivationDenied(stage) => {
                self.record_appearance_activation_denial(stage);
            }
            ValidationRuntimeReloadTickOutcome::DensityActivationDenied(stage) => {
                self.record_density_activation_denial(stage);
            }
            ValidationRuntimeReloadTickOutcome::SourceReloadedAndThemeDenied {
                evidence,
                phase_execution,
                authored_structural,
                theme_denial,
            } => {
                self.record_runtime_reload(
                    &evidence,
                    phase_execution.as_ref(),
                    authored_structural,
                );
                self.record_theme_denial(theme_denial);
            }
            ValidationRuntimeReloadTickOutcome::SourceReloadedAndThemeReloaded {
                evidence,
                phase_execution,
                authored_structural,
                theme_evidence,
                theme_phase_execution,
            } => {
                self.record_runtime_reload(
                    &evidence,
                    phase_execution.as_ref(),
                    authored_structural,
                );
                self.record_theme_reload(&theme_evidence, theme_phase_execution.as_ref());
            }
            ValidationRuntimeReloadTickOutcome::AuthoredBatchReloaded {
                source_evidence,
                capability_evidence,
                runtime_change,
                compile_boundary,
                phase_execution,
                authored_structural,
            } => self.record_authored_batch_reload(
                source_evidence,
                capability_evidence,
                runtime_change,
                compile_boundary,
                phase_execution.as_ref(),
                authored_structural,
            ),
            ValidationRuntimeReloadTickOutcome::InputUnreadable(denial) => {
                self.record_input_unreadable(denial);
            }
        }
    }

    pub fn record_runtime_reload(
        &mut self,
        evidence: &ValidationReloadEvidence,
        phase_execution: Option<&WorthUiRebindPhaseExecutionReceipt>,
        authored_structural: Option<crate::reload::ValidationAuthoredStructuralReloadEvidence>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::RuntimeReload {
            status: evidence.status(),
            active_artifact_digest: evidence.active_artifact_digest_after(),
            active_plan_digest: evidence.active_plan_digest_after(),
            phase_execution: phase_execution.map(ValidationPhaseExecutionEvidence::from_receipt),
            authored_structural,
            header_rebind: phase_execution.map(header_rebind_evidence_from_phase_execution),
            page_host_rebind: phase_execution.map(page_host_rebind_evidence_from_phase_execution),
            query_bindings_compared: evidence.query_bindings_compared(),
            query_rebind_entries: evidence.query_rebind_entries(),
            changed_fact_count: evidence.changed_facts().len(),
            changed_facts: evidence.changed_facts().facts().cloned().collect(),
        });
    }

    pub fn record_theme_denial(&mut self, denial: ValidationThemeReloadDenial) {
        self.push_entry(ValidationReloadEvidenceEntry::ThemeDenied(denial));
    }

    pub fn record_authored_batch_reload(
        &mut self,
        source_evidence: ValidationReloadEvidence,
        capability_evidence: worth_ui::facade::WorthUiCapabilityReloadEvidence,
        runtime_change: crate::reload::ValidationRuntimeChangeEvidence,
        compile_boundary: worth_ui::facade::WorthUiCompileBoundaryCertification,
        phase_execution: Option<&WorthUiRebindPhaseExecutionReceipt>,
        authored_structural: Option<crate::reload::ValidationAuthoredStructuralReloadEvidence>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::AuthoredBatchReload {
            header_rebind: phase_execution.map(header_rebind_evidence_from_phase_execution),
            page_host_rebind: phase_execution.map(page_host_rebind_evidence_from_phase_execution),
            phase_execution: phase_execution.map(ValidationPhaseExecutionEvidence::from_receipt),
            source_evidence,
            capability_evidence,
            runtime_change,
            compile_boundary,
            authored_structural,
        });
    }

    pub fn record_source_activation_denial(&mut self, stage: ValidationReloadStage) {
        self.push_entry(ValidationReloadEvidenceEntry::SourceActivationDenied(stage));
    }

    pub fn record_theme_activation_denial(
        &mut self,
        stage: worth_ui::facade::WorthUiCapabilityReloadStage,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::ThemeActivationDenied(stage));
    }

    pub fn record_component_activation_denial(
        &mut self,
        stage: worth_ui::facade::WorthUiCapabilityReloadStage,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::ComponentActivationDenied(
            stage,
        ));
    }

    pub fn record_command_activation_denial(
        &mut self,
        stage: worth_ui::facade::WorthUiCapabilityReloadStage,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::CommandActivationDenied(
            stage,
        ));
    }

    pub fn record_command_projection_activation_denial(
        &mut self,
        stage: worth_ui::facade::WorthUiCapabilityReloadStage,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::CommandProjectionActivationDenied(stage));
    }

    pub fn record_appearance_activation_denial(
        &mut self,
        stage: worth_ui::facade::WorthUiCapabilityReloadStage,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::AppearanceActivationDenied(
            stage,
        ));
    }

    pub fn record_density_activation_denial(
        &mut self,
        stage: worth_ui::facade::WorthUiCapabilityReloadStage,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::DensityActivationDenied(
            stage,
        ));
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

fn header_rebind_evidence_from_phase_execution(
    receipt: &WorthUiRebindPhaseExecutionReceipt,
) -> ValidationHeaderRebindEvidence {
    header_rebind_evidence(Some(receipt.header_rebind()))
        .expect("phase execution receipt always carries header rebind proof")
}

fn page_host_rebind_evidence_from_phase_execution(
    receipt: &WorthUiRebindPhaseExecutionReceipt,
) -> ValidationPageHostRebindEvidence {
    ValidationPageHostRebindEvidence::from_receipt(receipt.page_host_rebind())
}

fn family_changed(
    evidence: &worth_ui::facade::WorthUiCapabilityReloadEvidence,
    family: WorthUiCapabilityReloadFamilyKind,
) -> bool {
    evidence
        .family_rows()
        .iter()
        .find(|row| row.family() == family)
        .is_some_and(|row| row.status() == WorthUiCapabilityReloadFamilyStatus::AdmittedChanged)
}
