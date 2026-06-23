use worth_ui::facade::{
    WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadStage,
    WorthUiCompileBoundaryCertification, WorthUiRebindPhaseExecutionReceipt,
};

use super::{
    ValidationAuthoredStructuralReloadEvidence, ValidationReloadEvidence, ValidationReloadInput,
    ValidationReloadInputDenial, ValidationReloadStage, ValidationRuntimeChangeEvidence,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationReloadTick {
    Unchanged(ValidationReloadObservation),
    Changed(ValidationReloadInput),
    Unreadable(ValidationReloadInputDenial),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationReloadObservation {
    source_digest: u64,
    theme_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationRuntimeReloadTickOutcome {
    Unchanged(ValidationReloadObservation),
    SourceReloaded {
        evidence: ValidationReloadEvidence,
        phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
        authored_structural: Option<ValidationAuthoredStructuralReloadEvidence>,
    },
    SourceActivationDenied(ValidationReloadStage),
    ThemeReloaded {
        evidence: WorthUiCapabilityReloadEvidence,
        phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
    },
    ComponentReloaded {
        evidence: WorthUiCapabilityReloadEvidence,
        phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
    },
    CommandReloaded {
        evidence: WorthUiCapabilityReloadEvidence,
        phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
    },
    CommandProjectionReloaded {
        evidence: WorthUiCapabilityReloadEvidence,
        phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
    },
    AppearanceReloaded {
        evidence: WorthUiCapabilityReloadEvidence,
        phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
    },
    DensityReloaded {
        evidence: WorthUiCapabilityReloadEvidence,
        phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
    },
    AppearanceAndDensityReloaded {
        evidence: WorthUiCapabilityReloadEvidence,
        phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
    },
    ThemeActivationDenied(WorthUiCapabilityReloadStage),
    ComponentActivationDenied(WorthUiCapabilityReloadStage),
    CommandActivationDenied(WorthUiCapabilityReloadStage),
    CommandProjectionActivationDenied(WorthUiCapabilityReloadStage),
    AppearanceActivationDenied(WorthUiCapabilityReloadStage),
    DensityActivationDenied(WorthUiCapabilityReloadStage),
    SourceReloadedAndThemeDenied {
        evidence: ValidationReloadEvidence,
        phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
        authored_structural: Option<ValidationAuthoredStructuralReloadEvidence>,
        theme_denial: ValidationThemeReloadDenial,
    },
    SourceReloadedAndThemeReloaded {
        evidence: ValidationReloadEvidence,
        phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
        authored_structural: Option<ValidationAuthoredStructuralReloadEvidence>,
        theme_evidence: WorthUiCapabilityReloadEvidence,
        theme_phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
    },
    AuthoredBatchReloaded {
        source_evidence: ValidationReloadEvidence,
        capability_evidence: WorthUiCapabilityReloadEvidence,
        runtime_change: ValidationRuntimeChangeEvidence,
        compile_boundary: WorthUiCompileBoundaryCertification,
        phase_execution: Option<WorthUiRebindPhaseExecutionReceipt>,
        authored_structural: Option<ValidationAuthoredStructuralReloadEvidence>,
    },
    InputUnreadable(ValidationReloadInputDenial),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationThemeReloadDenial {
    theme_source_digest: u64,
    reason: ValidationThemeReloadDenialReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationThemeReloadDenialReason {
    RuntimeCapabilitySnapshotReplacementNotAdmitted,
}

impl ValidationReloadObservation {
    pub fn new(source_digest: u64, theme_digest: u64) -> Self {
        Self {
            source_digest,
            theme_digest,
        }
    }

    pub fn source_digest(self) -> u64 {
        self.source_digest
    }

    pub fn theme_digest(self) -> u64 {
        self.theme_digest
    }
}

impl ValidationThemeReloadDenial {
    pub fn snapshot_replacement_not_admitted(theme_source_digest: u64) -> Self {
        Self {
            theme_source_digest,
            reason:
                ValidationThemeReloadDenialReason::RuntimeCapabilitySnapshotReplacementNotAdmitted,
        }
    }

    pub fn theme_source_digest(&self) -> u64 {
        self.theme_source_digest
    }

    pub fn reason(&self) -> ValidationThemeReloadDenialReason {
        self.reason
    }
}
