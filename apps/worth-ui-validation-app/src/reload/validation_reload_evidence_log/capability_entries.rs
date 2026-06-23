use worth_ui::facade::{
    WorthUiCapabilityReloadEvidence, WorthUiCapabilityReloadFamilyKind,
    WorthUiCapabilityReloadFamilyStatus, WorthUiCapabilityReloadStatus,
    WorthUiRebindPhaseExecutionReceipt,
};
use worth_ui::facade::{WorthUiRuntimeFactFamily, WorthUiRuntimeFactId};

use crate::reload::{
    ValidationPhaseExecutionEvidence, ValidationReloadEvidenceEntry, ValidationReloadEvidenceLog,
};

use super::{
    header_rebind_evidence_from_phase_execution, page_host_rebind_evidence_from_phase_execution,
};

impl ValidationReloadEvidenceLog {
    pub fn record_theme_reload(
        &mut self,
        evidence: &WorthUiCapabilityReloadEvidence,
        phase_execution: Option<&WorthUiRebindPhaseExecutionReceipt>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::ThemeReload {
            status: family_status(evidence, WorthUiCapabilityReloadFamilyKind::ThemeTokens),
            active_snapshot_digest: evidence.active_snapshot_digest_after(),
            touched_theme_token_count: evidence.touched_theme_token_count(),
            changed_fact_count: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::ThemeTokens,
            )
            .len(),
            changed_facts: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::ThemeTokens,
            ),
            phase_execution: phase_execution.map(ValidationPhaseExecutionEvidence::from_receipt),
            header_rebind: phase_execution.map(header_rebind_evidence_from_phase_execution),
            page_host_rebind: phase_execution.map(page_host_rebind_evidence_from_phase_execution),
        });
    }

    pub fn record_command_reload(
        &mut self,
        evidence: &WorthUiCapabilityReloadEvidence,
        phase_execution: Option<&WorthUiRebindPhaseExecutionReceipt>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::CommandReload {
            status: family_status(evidence, WorthUiCapabilityReloadFamilyKind::Commands),
            active_snapshot_digest: evidence.active_snapshot_digest_after(),
            touched_command_count: evidence.touched_command_count(),
            changed_fact_count: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::Commands,
            )
            .len(),
            changed_facts: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::Commands,
            ),
            phase_execution: phase_execution.map(ValidationPhaseExecutionEvidence::from_receipt),
            header_rebind: phase_execution.map(header_rebind_evidence_from_phase_execution),
            page_host_rebind: phase_execution.map(page_host_rebind_evidence_from_phase_execution),
        });
    }

    pub fn record_component_reload(
        &mut self,
        evidence: &WorthUiCapabilityReloadEvidence,
        phase_execution: Option<&WorthUiRebindPhaseExecutionReceipt>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::ComponentReload {
            status: family_status(evidence, WorthUiCapabilityReloadFamilyKind::Components),
            active_snapshot_digest: evidence.active_snapshot_digest_after(),
            touched_component_count: evidence.touched_component_count(),
            changed_component_count: evidence.changed_component_count(),
            family_rebuild_breadth: evidence
                .family_rebuild_breadth_for(WorthUiCapabilityReloadFamilyKind::Components),
            changed_fact_count: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::Components,
            )
            .len(),
            changed_facts: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::Components,
            ),
            phase_execution: phase_execution.map(ValidationPhaseExecutionEvidence::from_receipt),
            header_rebind: phase_execution.map(header_rebind_evidence_from_phase_execution),
            page_host_rebind: phase_execution.map(page_host_rebind_evidence_from_phase_execution),
        });
    }

    pub fn record_command_projection_reload(
        &mut self,
        evidence: &WorthUiCapabilityReloadEvidence,
        phase_execution: Option<&WorthUiRebindPhaseExecutionReceipt>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::CommandProjectionReload {
            status: family_status(
                evidence,
                WorthUiCapabilityReloadFamilyKind::CommandProjections,
            ),
            active_snapshot_digest: evidence.active_snapshot_digest_after(),
            touched_projection_count: evidence.touched_command_projection_count(),
            changed_fact_count: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::CommandProjections,
            )
            .len(),
            changed_facts: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::CommandProjections,
            ),
            phase_execution: phase_execution.map(ValidationPhaseExecutionEvidence::from_receipt),
            header_rebind: phase_execution.map(header_rebind_evidence_from_phase_execution),
            page_host_rebind: phase_execution.map(page_host_rebind_evidence_from_phase_execution),
        });
    }

    pub fn record_appearance_reload(
        &mut self,
        evidence: &WorthUiCapabilityReloadEvidence,
        phase_execution: Option<&WorthUiRebindPhaseExecutionReceipt>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::AppearanceReload {
            status: family_status(evidence, WorthUiCapabilityReloadFamilyKind::Appearance),
            active_snapshot_digest: evidence.active_snapshot_digest_after(),
            touched_appearance_count: evidence.touched_appearance_count(),
            changed_appearance_count: evidence.changed_appearance_count(),
            canonicalization_count: evidence.canonicalization_count(),
            descriptor_lookup_count: evidence.registry_lookup_count(),
            family_rebuild_breadth: evidence
                .family_rebuild_breadth_for(WorthUiCapabilityReloadFamilyKind::Appearance),
            changed_fact_count: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::Appearance,
            )
            .len(),
            changed_facts: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::Appearance,
            ),
            phase_execution: phase_execution.map(ValidationPhaseExecutionEvidence::from_receipt),
            header_rebind: phase_execution.map(header_rebind_evidence_from_phase_execution),
            page_host_rebind: phase_execution.map(page_host_rebind_evidence_from_phase_execution),
        });
    }

    pub fn record_density_reload(
        &mut self,
        evidence: &WorthUiCapabilityReloadEvidence,
        phase_execution: Option<&WorthUiRebindPhaseExecutionReceipt>,
    ) {
        self.push_entry(ValidationReloadEvidenceEntry::DensityReload {
            status: family_status(evidence, WorthUiCapabilityReloadFamilyKind::Density),
            active_snapshot_digest: evidence.active_snapshot_digest_after(),
            touched_density_count: evidence.touched_density_count(),
            changed_density_count: evidence.changed_density_count(),
            canonicalization_count: evidence.canonicalization_count(),
            descriptor_lookup_count: evidence.registry_lookup_count(),
            family_rebuild_breadth: evidence
                .family_rebuild_breadth_for(WorthUiCapabilityReloadFamilyKind::Density),
            changed_fact_count: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::Density,
            )
            .len(),
            changed_facts: family_changed_facts(
                evidence,
                WorthUiCapabilityReloadFamilyKind::Density,
            ),
            phase_execution: phase_execution.map(ValidationPhaseExecutionEvidence::from_receipt),
            header_rebind: phase_execution.map(header_rebind_evidence_from_phase_execution),
            page_host_rebind: phase_execution.map(page_host_rebind_evidence_from_phase_execution),
        });
    }
}

fn family_status(
    evidence: &WorthUiCapabilityReloadEvidence,
    family: WorthUiCapabilityReloadFamilyKind,
) -> WorthUiCapabilityReloadStatus {
    let Some(row) = evidence
        .family_rows()
        .iter()
        .find(|row| row.family() == family)
    else {
        return evidence.status();
    };
    match row.status() {
        WorthUiCapabilityReloadFamilyStatus::AdmittedChanged => {
            WorthUiCapabilityReloadStatus::Activated
        }
        WorthUiCapabilityReloadFamilyStatus::EquivalentNoOp => {
            WorthUiCapabilityReloadStatus::EquivalentNoOp
        }
        WorthUiCapabilityReloadFamilyStatus::Denied => evidence.status(),
    }
}

fn family_changed_facts(
    evidence: &WorthUiCapabilityReloadEvidence,
    family: WorthUiCapabilityReloadFamilyKind,
) -> Vec<WorthUiRuntimeFactId> {
    evidence
        .changed_facts()
        .facts()
        .filter(|fact| fact_matches_family(fact, family))
        .cloned()
        .collect()
}

fn fact_matches_family(
    fact: &WorthUiRuntimeFactId,
    family: WorthUiCapabilityReloadFamilyKind,
) -> bool {
    matches!(
        (family, fact.family()),
        (
            WorthUiCapabilityReloadFamilyKind::ThemeTokens,
            WorthUiRuntimeFactFamily::ThemeToken
        ) | (
            WorthUiCapabilityReloadFamilyKind::Commands,
            WorthUiRuntimeFactFamily::Command
        ) | (
            WorthUiCapabilityReloadFamilyKind::CommandProjections,
            WorthUiRuntimeFactFamily::CommandProjection
                | WorthUiRuntimeFactFamily::InteractionPolicy
        ) | (
            WorthUiCapabilityReloadFamilyKind::Components,
            WorthUiRuntimeFactFamily::Component
        ) | (
            WorthUiCapabilityReloadFamilyKind::Appearance,
            WorthUiRuntimeFactFamily::Appearance
        ) | (
            WorthUiCapabilityReloadFamilyKind::Density,
            WorthUiRuntimeFactFamily::DensityToken
        )
    )
}
