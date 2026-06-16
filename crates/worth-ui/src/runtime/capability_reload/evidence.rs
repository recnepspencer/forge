use crate::runtime::WorthUiRuntimeFactSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCapabilityReloadStage {
    ThemeTokenSourceParse,
    ThemeTokenAdmission,
    CommandSourceParse,
    CommandAdmission,
    CommandProjectionSourceParse,
    CommandProjectionAdmission,
    ActiveSnapshotDrift,
    RuntimeInstanceMismatch,
    MissingReadyActivation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCapabilityReloadStatus {
    EquivalentNoOp,
    ReadyForFrameBoundary,
    Activated,
    Denied(WorthUiCapabilityReloadStage),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCapabilityReloadEvidence {
    runtime_instance_witness: u64,
    status: WorthUiCapabilityReloadStatus,
    denial_detail: Option<String>,
    active_snapshot_digest_before: u64,
    active_snapshot_digest_after: u64,
    candidate_snapshot_digest: Option<u64>,
    theme_source_digest: u64,
    touched_theme_token_count: usize,
    theme_token_family_entry_count: usize,
    source_parse_count: usize,
    registry_lookup_count: usize,
    artifact_tree_scan_count: usize,
    active_runtime_mutations_before_activation: usize,
    changed_facts: WorthUiRuntimeFactSet,
}

impl WorthUiCapabilityReloadEvidence {
    pub(crate) fn denied(
        runtime_instance_witness: u64,
        active_snapshot_digest: u64,
        theme_source_digest: u64,
        stage: WorthUiCapabilityReloadStage,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            runtime_instance_witness,
            status: WorthUiCapabilityReloadStatus::Denied(stage),
            denial_detail: Some(detail.into()),
            active_snapshot_digest_before: active_snapshot_digest,
            active_snapshot_digest_after: active_snapshot_digest,
            candidate_snapshot_digest: None,
            theme_source_digest,
            touched_theme_token_count: 0,
            theme_token_family_entry_count: 0,
            source_parse_count: 1,
            registry_lookup_count: 0,
            artifact_tree_scan_count: 0,
            active_runtime_mutations_before_activation: 0,
            changed_facts: WorthUiRuntimeFactSet::empty(),
        }
    }

    pub(crate) fn prepared(
        runtime_instance_witness: u64,
        status: WorthUiCapabilityReloadStatus,
        active_snapshot_digest_before: u64,
        candidate_snapshot_digest: u64,
        theme_source_digest: u64,
        touched_theme_token_count: usize,
        theme_token_family_entry_count: usize,
        registry_lookup_count: usize,
        changed_facts: WorthUiRuntimeFactSet,
    ) -> Self {
        Self {
            runtime_instance_witness,
            status,
            denial_detail: None,
            active_snapshot_digest_before,
            active_snapshot_digest_after: active_snapshot_digest_before,
            candidate_snapshot_digest: Some(candidate_snapshot_digest),
            theme_source_digest,
            touched_theme_token_count,
            theme_token_family_entry_count,
            source_parse_count: 1,
            registry_lookup_count,
            artifact_tree_scan_count: 0,
            active_runtime_mutations_before_activation: 0,
            changed_facts,
        }
    }

    pub(crate) fn mark_activated(mut self, active_snapshot_digest_after: u64) -> Self {
        self.status = WorthUiCapabilityReloadStatus::Activated;
        self.active_snapshot_digest_after = active_snapshot_digest_after;
        self
    }

    pub fn runtime_instance_witness(&self) -> u64 {
        self.runtime_instance_witness
    }

    pub fn status(&self) -> WorthUiCapabilityReloadStatus {
        self.status
    }

    pub fn denial_detail(&self) -> Option<&str> {
        self.denial_detail.as_deref()
    }

    pub fn active_snapshot_digest_before(&self) -> u64 {
        self.active_snapshot_digest_before
    }

    pub fn active_snapshot_digest_after(&self) -> u64 {
        self.active_snapshot_digest_after
    }

    pub fn candidate_snapshot_digest(&self) -> Option<u64> {
        self.candidate_snapshot_digest
    }

    pub fn theme_source_digest(&self) -> u64 {
        self.theme_source_digest
    }

    pub fn touched_theme_token_count(&self) -> usize {
        self.touched_theme_token_count
    }

    pub fn theme_token_family_entry_count(&self) -> usize {
        self.theme_token_family_entry_count
    }

    pub fn source_parse_count(&self) -> usize {
        self.source_parse_count
    }

    pub fn registry_lookup_count(&self) -> usize {
        self.registry_lookup_count
    }

    pub fn artifact_tree_scan_count(&self) -> usize {
        self.artifact_tree_scan_count
    }

    pub fn active_runtime_mutations_before_activation(&self) -> usize {
        self.active_runtime_mutations_before_activation
    }

    pub fn changed_facts(&self) -> &WorthUiRuntimeFactSet {
        &self.changed_facts
    }
}
