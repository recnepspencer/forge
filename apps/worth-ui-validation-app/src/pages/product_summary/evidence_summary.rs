use worth_ui::facade::{
    WorthUiCapabilityReloadStage, WorthUiCapabilityReloadStatus, WorthUiRuntimeFactId,
};

use crate::reload::{ValidationReloadEvidenceEntry, ValidationReloadStage, ValidationReloadStatus};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationProductSummaryEvidence {
    kind: ValidationProductSummaryEvidenceKind,
    status: ValidationProductSummaryEvidenceStatus,
    primary_digest: u64,
    secondary_digest: Option<u64>,
    header_rebind_status: Option<String>,
    touched_count: Option<usize>,
    query_bindings_compared: usize,
    query_rebind_entries: usize,
    changed_fact_count: usize,
    changed_facts: Vec<WorthUiRuntimeFactId>,
    denial_detail: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationProductSummaryEvidenceKind {
    LaunchReceipt,
    RuntimeReload,
    AuthoredBatchReload,
    ThemeReload,
    CommandReload,
    ComponentReload,
    CommandProjectionReload,
    AppearanceReload,
    DensityReload,
    Denial,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationProductSummaryEvidenceStatus {
    LaunchReceipt,
    RuntimeReload(ValidationReloadStatus),
    CapabilityReload(WorthUiCapabilityReloadStatus),
    Denial(ValidationProductSummaryDenialStatus),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationProductSummaryDenialStatus {
    ThemeDenied,
    SourceActivationDenied(ValidationReloadStage),
    ThemeActivationDenied(WorthUiCapabilityReloadStage),
    ComponentActivationDenied(WorthUiCapabilityReloadStage),
    CommandActivationDenied(WorthUiCapabilityReloadStage),
    CommandProjectionActivationDenied(WorthUiCapabilityReloadStage),
    AppearanceActivationDenied(WorthUiCapabilityReloadStage),
    DensityActivationDenied(WorthUiCapabilityReloadStage),
    InputUnreadable,
}

impl ValidationProductSummaryEvidence {
    pub fn from_latest_entry(entry: Option<&ValidationReloadEvidenceEntry>) -> Self {
        let Some(entry) = entry else {
            return Self::launch_receipt();
        };

        match entry {
            ValidationReloadEvidenceEntry::RuntimeReload {
                status,
                active_artifact_digest,
                active_plan_digest,
                header_rebind,
                query_bindings_compared,
                query_rebind_entries,
                changed_fact_count,
                changed_facts,
                ..
            } => Self {
                kind: ValidationProductSummaryEvidenceKind::RuntimeReload,
                status: ValidationProductSummaryEvidenceStatus::RuntimeReload(*status),
                primary_digest: *active_artifact_digest,
                secondary_digest: Some(*active_plan_digest),
                header_rebind_status: header_rebind
                    .as_ref()
                    .map(|rebind| format!("{:?}", rebind.status())),
                touched_count: None,
                query_bindings_compared: *query_bindings_compared,
                query_rebind_entries: *query_rebind_entries,
                changed_fact_count: *changed_fact_count,
                changed_facts: changed_facts.clone(),
                denial_detail: None,
            },
            ValidationReloadEvidenceEntry::AuthoredBatchReload {
                source_evidence,
                runtime_change,
                header_rebind,
                ..
            } => Self {
                kind: ValidationProductSummaryEvidenceKind::AuthoredBatchReload,
                status: ValidationProductSummaryEvidenceStatus::RuntimeReload(
                    source_evidence.status(),
                ),
                primary_digest: source_evidence.active_artifact_digest_after(),
                secondary_digest: Some(runtime_change.digest()),
                header_rebind_status: header_rebind
                    .as_ref()
                    .map(|rebind| format!("{:?}", rebind.status())),
                touched_count: Some(runtime_change.counters().family_row_count()),
                query_bindings_compared: source_evidence.query_bindings_compared(),
                query_rebind_entries: source_evidence.query_rebind_entries(),
                changed_fact_count: runtime_change.counters().changed_fact_count(),
                changed_facts: runtime_change
                    .rows()
                    .iter()
                    .flat_map(|row| row.changed_facts().iter().cloned())
                    .collect(),
                denial_detail: None,
            },
            ValidationReloadEvidenceEntry::ThemeReload {
                status,
                active_snapshot_digest,
                touched_theme_token_count,
                changed_fact_count,
                changed_facts,
                header_rebind,
                ..
            } => Self::capability_reload(
                ValidationProductSummaryEvidenceKind::ThemeReload,
                *status,
                *active_snapshot_digest,
                *touched_theme_token_count,
                *changed_fact_count,
                changed_facts,
                header_rebind.as_ref().map(|rebind| rebind.status()),
            ),
            ValidationReloadEvidenceEntry::CommandReload {
                status,
                active_snapshot_digest,
                touched_command_count,
                changed_fact_count,
                changed_facts,
                header_rebind,
                ..
            } => Self::capability_reload(
                ValidationProductSummaryEvidenceKind::CommandReload,
                *status,
                *active_snapshot_digest,
                *touched_command_count,
                *changed_fact_count,
                changed_facts,
                header_rebind.as_ref().map(|rebind| rebind.status()),
            ),
            ValidationReloadEvidenceEntry::ComponentReload {
                status,
                active_snapshot_digest,
                touched_component_count,
                changed_fact_count,
                changed_facts,
                header_rebind,
                ..
            } => Self::capability_reload(
                ValidationProductSummaryEvidenceKind::ComponentReload,
                *status,
                *active_snapshot_digest,
                *touched_component_count,
                *changed_fact_count,
                changed_facts,
                header_rebind.as_ref().map(|rebind| rebind.status()),
            ),
            ValidationReloadEvidenceEntry::CommandProjectionReload {
                status,
                active_snapshot_digest,
                touched_projection_count,
                changed_fact_count,
                changed_facts,
                header_rebind,
                ..
            } => Self::capability_reload(
                ValidationProductSummaryEvidenceKind::CommandProjectionReload,
                *status,
                *active_snapshot_digest,
                *touched_projection_count,
                *changed_fact_count,
                changed_facts,
                header_rebind.as_ref().map(|rebind| rebind.status()),
            ),
            ValidationReloadEvidenceEntry::AppearanceReload {
                status,
                active_snapshot_digest,
                touched_appearance_count,
                changed_fact_count,
                changed_facts,
                header_rebind,
                ..
            } => Self::capability_reload(
                ValidationProductSummaryEvidenceKind::AppearanceReload,
                *status,
                *active_snapshot_digest,
                *touched_appearance_count,
                *changed_fact_count,
                changed_facts,
                header_rebind.as_ref().map(|rebind| rebind.status()),
            ),
            ValidationReloadEvidenceEntry::DensityReload {
                status,
                active_snapshot_digest,
                touched_density_count,
                changed_fact_count,
                changed_facts,
                header_rebind,
                ..
            } => Self::capability_reload(
                ValidationProductSummaryEvidenceKind::DensityReload,
                *status,
                *active_snapshot_digest,
                *touched_density_count,
                *changed_fact_count,
                changed_facts,
                header_rebind.as_ref().map(|rebind| rebind.status()),
            ),
            ValidationReloadEvidenceEntry::ThemeDenied(denial) => Self::denial(
                ValidationProductSummaryDenialStatus::ThemeDenied,
                denial.theme_source_digest(),
                format!("{:?}", denial.reason()),
            ),
            ValidationReloadEvidenceEntry::SourceActivationDenied(stage) => Self::denial(
                ValidationProductSummaryDenialStatus::SourceActivationDenied(*stage),
                0,
                format!("{stage:?}"),
            ),
            ValidationReloadEvidenceEntry::ThemeActivationDenied(stage) => Self::denial(
                ValidationProductSummaryDenialStatus::ThemeActivationDenied(*stage),
                0,
                format!("{stage:?}"),
            ),
            ValidationReloadEvidenceEntry::ComponentActivationDenied(stage) => Self::denial(
                ValidationProductSummaryDenialStatus::ComponentActivationDenied(*stage),
                0,
                format!("{stage:?}"),
            ),
            ValidationReloadEvidenceEntry::CommandActivationDenied(stage) => Self::denial(
                ValidationProductSummaryDenialStatus::CommandActivationDenied(*stage),
                0,
                format!("{stage:?}"),
            ),
            ValidationReloadEvidenceEntry::CommandProjectionActivationDenied(stage) => {
                Self::denial(
                    ValidationProductSummaryDenialStatus::CommandProjectionActivationDenied(*stage),
                    0,
                    format!("{stage:?}"),
                )
            }
            ValidationReloadEvidenceEntry::AppearanceActivationDenied(stage) => Self::denial(
                ValidationProductSummaryDenialStatus::AppearanceActivationDenied(*stage),
                0,
                format!("{stage:?}"),
            ),
            ValidationReloadEvidenceEntry::DensityActivationDenied(stage) => Self::denial(
                ValidationProductSummaryDenialStatus::DensityActivationDenied(*stage),
                0,
                format!("{stage:?}"),
            ),
            ValidationReloadEvidenceEntry::InputUnreadable(denial) => Self::denial(
                ValidationProductSummaryDenialStatus::InputUnreadable,
                0,
                format!("{}: {}", denial.path().display(), denial.reason()),
            ),
        }
    }

    pub fn launch_receipt() -> Self {
        Self {
            kind: ValidationProductSummaryEvidenceKind::LaunchReceipt,
            status: ValidationProductSummaryEvidenceStatus::LaunchReceipt,
            primary_digest: 0,
            secondary_digest: None,
            header_rebind_status: None,
            touched_count: None,
            query_bindings_compared: 0,
            query_rebind_entries: 0,
            changed_fact_count: 0,
            changed_facts: Vec::new(),
            denial_detail: None,
        }
    }

    fn capability_reload(
        kind: ValidationProductSummaryEvidenceKind,
        status: WorthUiCapabilityReloadStatus,
        snapshot_digest: u64,
        touched_count: usize,
        changed_fact_count: usize,
        changed_facts: &[WorthUiRuntimeFactId],
        header_rebind_status: Option<worth_ui::facade::WorthUiHeaderFrameRebindStatus>,
    ) -> Self {
        Self {
            kind,
            status: ValidationProductSummaryEvidenceStatus::CapabilityReload(status),
            primary_digest: snapshot_digest,
            secondary_digest: None,
            header_rebind_status: header_rebind_status.map(|status| format!("{status:?}")),
            touched_count: Some(touched_count),
            query_bindings_compared: 0,
            query_rebind_entries: 0,
            changed_fact_count,
            changed_facts: changed_facts.to_vec(),
            denial_detail: None,
        }
    }

    fn denial(status: ValidationProductSummaryDenialStatus, digest: u64, detail: String) -> Self {
        Self {
            kind: ValidationProductSummaryEvidenceKind::Denial,
            status: ValidationProductSummaryEvidenceStatus::Denial(status),
            primary_digest: digest,
            secondary_digest: None,
            header_rebind_status: None,
            touched_count: None,
            query_bindings_compared: 0,
            query_rebind_entries: 0,
            changed_fact_count: 0,
            changed_facts: Vec::new(),
            denial_detail: Some(detail),
        }
    }

    pub fn kind(&self) -> ValidationProductSummaryEvidenceKind {
        self.kind
    }

    pub fn status(&self) -> &ValidationProductSummaryEvidenceStatus {
        &self.status
    }

    pub fn primary_digest(&self) -> u64 {
        self.primary_digest
    }

    pub fn secondary_digest(&self) -> Option<u64> {
        self.secondary_digest
    }

    pub fn header_rebind_status(&self) -> Option<&str> {
        self.header_rebind_status.as_deref()
    }

    pub fn touched_count(&self) -> Option<usize> {
        self.touched_count
    }

    pub fn query_bindings_compared(&self) -> usize {
        self.query_bindings_compared
    }

    pub fn query_rebind_entries(&self) -> usize {
        self.query_rebind_entries
    }

    pub fn changed_fact_count(&self) -> usize {
        self.changed_fact_count
    }

    pub fn changed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.changed_facts
    }

    pub fn denial_detail(&self) -> Option<&str> {
        self.denial_detail.as_deref()
    }
}
