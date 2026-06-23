use crate::runtime::{WorthUiCapabilityChangedFacts, WorthUiRuntimeFactSet};

use super::{
    WorthUiCapabilityReloadDenialCode, WorthUiCapabilityReloadFamilyCounters,
    WorthUiCapabilityReloadFamilyKind, WorthUiCapabilityReloadFamilyRow,
    WorthUiComponentCompatibility, WorthUiComponentReloadReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCapabilityReloadStage {
    ThemeTokenSourceParse,
    ThemeTokenAdmission,
    CommandSourceParse,
    CommandAdmission,
    CommandProjectionSourceParse,
    CommandProjectionAdmission,
    ComponentSourceParse,
    ComponentAdmission,
    AppearanceSourceParse,
    AppearanceAdmission,
    DensitySourceParse,
    DensityAdmission,
    DuplicateCapabilityFamily,
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
    request_digest: u64,
    family_rows: Vec<WorthUiCapabilityReloadFamilyRow>,
    edited_delta_width: usize,
    family_rebuild_breadth: usize,
    source_parse_count: usize,
    registry_lookup_count: usize,
    artifact_tree_scan_count: usize,
    active_runtime_mutations_before_activation: usize,
    changed_facts: WorthUiCapabilityChangedFacts,
}

impl WorthUiCapabilityReloadEvidence {
    #[cfg(test)]
    pub(crate) fn denied(
        runtime_instance_witness: u64,
        active_snapshot_digest: u64,
        request_digest: u64,
        stage: WorthUiCapabilityReloadStage,
        detail: impl Into<String>,
    ) -> Self {
        Self::denied_for_family(
            runtime_instance_witness,
            active_snapshot_digest,
            request_digest,
            family_for_stage(stage),
            stage,
            detail,
            None,
            WorthUiCapabilityReloadFamilyCounters::new(1, 0, 0, 0, 0, 0),
        )
    }

    pub(crate) fn denied_for_family(
        runtime_instance_witness: u64,
        active_snapshot_digest: u64,
        request_digest: u64,
        family: WorthUiCapabilityReloadFamilyKind,
        stage: WorthUiCapabilityReloadStage,
        detail: impl Into<String>,
        denial_code: Option<WorthUiCapabilityReloadDenialCode>,
        counters: WorthUiCapabilityReloadFamilyCounters,
    ) -> Self {
        let detail = detail.into();
        Self {
            runtime_instance_witness,
            status: WorthUiCapabilityReloadStatus::Denied(stage),
            denial_detail: Some(detail.clone()),
            active_snapshot_digest_before: active_snapshot_digest,
            active_snapshot_digest_after: active_snapshot_digest,
            candidate_snapshot_digest: None,
            request_digest,
            family_rows: vec![WorthUiCapabilityReloadFamilyRow::denied_with_counters(
                family,
                request_digest,
                counters,
                denial_code,
                detail,
            )],
            edited_delta_width: counters.edited_delta_width(),
            family_rebuild_breadth: counters.family_rebuild_breadth(),
            source_parse_count: counters.source_parse_count(),
            registry_lookup_count: counters.registry_lookup_count(),
            artifact_tree_scan_count: 0,
            active_runtime_mutations_before_activation: 0,
            changed_facts: WorthUiCapabilityChangedFacts::from_admitted_capability_reload(
                WorthUiRuntimeFactSet::empty(),
                active_snapshot_digest,
                active_snapshot_digest,
            ),
        }
    }

    #[cfg(test)]
    pub(crate) fn prepared(
        runtime_instance_witness: u64,
        status: WorthUiCapabilityReloadStatus,
        active_snapshot_digest_before: u64,
        candidate_snapshot_digest: u64,
        request_digest: u64,
        edited_delta_width: usize,
        family_rebuild_breadth: usize,
        registry_lookup_count: usize,
        changed_facts: WorthUiRuntimeFactSet,
    ) -> Self {
        let counters = WorthUiCapabilityReloadFamilyCounters::new(
            1,
            edited_delta_width,
            edited_delta_width,
            changed_facts.len(),
            family_rebuild_breadth,
            registry_lookup_count,
        );
        Self {
            runtime_instance_witness,
            status,
            denial_detail: None,
            active_snapshot_digest_before,
            active_snapshot_digest_after: active_snapshot_digest_before,
            candidate_snapshot_digest: Some(candidate_snapshot_digest),
            request_digest,
            family_rows: vec![WorthUiCapabilityReloadFamilyRow::admitted(
                WorthUiCapabilityReloadFamilyKind::ThemeTokens,
                request_digest,
                counters,
                candidate_snapshot_digest != active_snapshot_digest_before,
            )],
            edited_delta_width,
            family_rebuild_breadth,
            source_parse_count: 1,
            registry_lookup_count,
            artifact_tree_scan_count: 0,
            active_runtime_mutations_before_activation: 0,
            changed_facts: WorthUiCapabilityChangedFacts::from_admitted_capability_reload(
                changed_facts,
                active_snapshot_digest_before,
                candidate_snapshot_digest,
            ),
        }
    }

    pub(crate) fn from_family_rows(
        runtime_instance_witness: u64,
        status: WorthUiCapabilityReloadStatus,
        active_snapshot_digest_before: u64,
        candidate_snapshot_digest: Option<u64>,
        request_digest: u64,
        family_rows: Vec<WorthUiCapabilityReloadFamilyRow>,
        changed_facts: WorthUiCapabilityChangedFacts,
    ) -> Self {
        let source_parse_count = family_rows.len();
        let counters = family_rows.iter().fold(
            WorthUiCapabilityReloadFamilyCounters::default(),
            |total, row| total.add(row.counters()),
        );
        Self {
            runtime_instance_witness,
            status,
            denial_detail: family_rows
                .iter()
                .filter_map(WorthUiCapabilityReloadFamilyRow::denial_detail)
                .next()
                .map(str::to_owned),
            active_snapshot_digest_before,
            active_snapshot_digest_after: active_snapshot_digest_before,
            candidate_snapshot_digest,
            request_digest,
            family_rows,
            edited_delta_width: counters.edited_delta_width(),
            family_rebuild_breadth: counters.family_rebuild_breadth(),
            source_parse_count: counters.source_parse_count().max(source_parse_count),
            registry_lookup_count: counters.registry_lookup_count(),
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

    pub fn denial_code(&self) -> Option<WorthUiCapabilityReloadDenialCode> {
        self.family_rows
            .iter()
            .find_map(WorthUiCapabilityReloadFamilyRow::denial_code)
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

    pub fn request_digest(&self) -> u64 {
        self.request_digest
    }

    pub fn theme_source_digest(&self) -> u64 {
        self.request_digest
    }

    pub fn family_rows(&self) -> &[WorthUiCapabilityReloadFamilyRow] {
        &self.family_rows
    }

    pub fn component_compatibility(&self) -> Option<&WorthUiComponentCompatibility> {
        self.family_rows
            .iter()
            .find_map(WorthUiCapabilityReloadFamilyRow::component_compatibility)
    }

    pub fn component_reload_receipt(&self) -> Option<&WorthUiComponentReloadReceipt> {
        self.family_rows
            .iter()
            .find_map(WorthUiCapabilityReloadFamilyRow::component_reload_receipt)
    }

    pub fn edited_delta_width(&self) -> usize {
        self.edited_delta_width
    }

    pub fn family_rebuild_breadth(&self) -> usize {
        self.family_rebuild_breadth
    }

    pub fn family_rebuild_breadth_for(&self, family: WorthUiCapabilityReloadFamilyKind) -> usize {
        self.family_rows
            .iter()
            .filter(|row| row.family() == family)
            .map(|row| row.counters().family_rebuild_breadth())
            .sum()
    }

    pub fn touched_theme_token_count(&self) -> usize {
        self.touched_family_count(WorthUiCapabilityReloadFamilyKind::ThemeTokens)
    }

    pub fn touched_command_count(&self) -> usize {
        self.touched_family_count(WorthUiCapabilityReloadFamilyKind::Commands)
    }

    pub fn touched_command_projection_count(&self) -> usize {
        self.touched_family_count(WorthUiCapabilityReloadFamilyKind::CommandProjections)
    }

    pub fn touched_appearance_count(&self) -> usize {
        self.touched_family_count(WorthUiCapabilityReloadFamilyKind::Appearance)
    }

    pub fn touched_component_count(&self) -> usize {
        self.touched_family_count(WorthUiCapabilityReloadFamilyKind::Components)
    }

    pub fn touched_density_count(&self) -> usize {
        self.touched_family_count(WorthUiCapabilityReloadFamilyKind::Density)
    }

    pub fn theme_token_family_entry_count(&self) -> usize {
        self.family_rebuild_breadth
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
        self.changed_facts.changed_facts().facts()
    }

    pub fn capability_changed_facts(&self) -> &WorthUiCapabilityChangedFacts {
        &self.changed_facts
    }

    pub fn canonicalization_count(&self) -> usize {
        self.family_rows
            .iter()
            .map(|row| row.counters().canonicalization_count())
            .sum()
    }

    pub fn changed_descriptor_count(&self) -> usize {
        self.family_rows
            .iter()
            .map(|row| row.counters().changed_descriptor_count())
            .sum()
    }

    pub fn changed_appearance_count(&self) -> usize {
        self.changed_family_count(WorthUiCapabilityReloadFamilyKind::Appearance)
    }

    pub fn changed_component_count(&self) -> usize {
        self.changed_family_count(WorthUiCapabilityReloadFamilyKind::Components)
    }

    pub fn changed_density_count(&self) -> usize {
        self.changed_family_count(WorthUiCapabilityReloadFamilyKind::Density)
    }

    fn touched_family_count(&self, family: WorthUiCapabilityReloadFamilyKind) -> usize {
        self.family_rows
            .iter()
            .filter(|row| row.family() == family)
            .map(|row| row.counters().edited_delta_width())
            .sum()
    }

    fn changed_family_count(&self, family: WorthUiCapabilityReloadFamilyKind) -> usize {
        self.family_rows
            .iter()
            .filter(|row| row.family() == family)
            .map(|row| row.counters().changed_descriptor_count())
            .sum()
    }
}

#[cfg(test)]
fn family_for_stage(stage: WorthUiCapabilityReloadStage) -> WorthUiCapabilityReloadFamilyKind {
    match stage {
        WorthUiCapabilityReloadStage::ThemeTokenSourceParse
        | WorthUiCapabilityReloadStage::ThemeTokenAdmission => {
            WorthUiCapabilityReloadFamilyKind::ThemeTokens
        }
        WorthUiCapabilityReloadStage::CommandSourceParse
        | WorthUiCapabilityReloadStage::CommandAdmission => {
            WorthUiCapabilityReloadFamilyKind::Commands
        }
        WorthUiCapabilityReloadStage::CommandProjectionSourceParse
        | WorthUiCapabilityReloadStage::CommandProjectionAdmission => {
            WorthUiCapabilityReloadFamilyKind::CommandProjections
        }
        WorthUiCapabilityReloadStage::ComponentSourceParse
        | WorthUiCapabilityReloadStage::ComponentAdmission => {
            WorthUiCapabilityReloadFamilyKind::Components
        }
        WorthUiCapabilityReloadStage::AppearanceSourceParse
        | WorthUiCapabilityReloadStage::AppearanceAdmission => {
            WorthUiCapabilityReloadFamilyKind::Appearance
        }
        WorthUiCapabilityReloadStage::DensitySourceParse
        | WorthUiCapabilityReloadStage::DensityAdmission => {
            WorthUiCapabilityReloadFamilyKind::Density
        }
        WorthUiCapabilityReloadStage::DuplicateCapabilityFamily
        | WorthUiCapabilityReloadStage::ActiveSnapshotDrift
        | WorthUiCapabilityReloadStage::RuntimeInstanceMismatch
        | WorthUiCapabilityReloadStage::MissingReadyActivation => {
            panic!("test-only denied constructor requires a single capability family stage")
        }
    }
}
