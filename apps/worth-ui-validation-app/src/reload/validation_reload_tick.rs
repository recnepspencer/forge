use worth_ui::facade::{
    WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadStage, WorthUiHeaderFrameRebindReceipt,
};

use super::{
    ValidationReloadEvidence, ValidationReloadInput, ValidationReloadInputDenial,
    ValidationReloadStage,
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
        header_receipt: Option<WorthUiHeaderFrameRebindReceipt>,
    },
    SourceActivationDenied(ValidationReloadStage),
    ThemeReloaded {
        evidence: WorthUiCapabilityReloadEvidence,
        header_receipt: Option<WorthUiHeaderFrameRebindReceipt>,
    },
    ThemeActivationDenied(WorthUiCapabilityReloadStage),
    SourceReloadedAndThemeDenied {
        evidence: ValidationReloadEvidence,
        header_receipt: Option<WorthUiHeaderFrameRebindReceipt>,
        theme_denial: ValidationThemeReloadDenial,
    },
    SourceReloadedAndThemeReloaded {
        evidence: ValidationReloadEvidence,
        header_receipt: Option<WorthUiHeaderFrameRebindReceipt>,
        theme_evidence: WorthUiCapabilityReloadEvidence,
        theme_header_receipt: Option<WorthUiHeaderFrameRebindReceipt>,
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
