mod digest_support;
mod replay_artifact;

use std::collections::BTreeSet;

use worth_ui::facade::{
    WorthUiCompileBoundaryCertification, WorthUiProjectionFamily, WorthUiProjectionPlanContract,
    WorthUiProjectionRebindStatus,
};

use crate::reload::{
    ValidationReloadEvidenceEntry, ValidationReloadEvidenceLog,
    ValidationRuntimeChangeCountersEvidence, ValidationRuntimeChangeEvidence,
    ValidationRuntimeChangeFamilyRowEvidence,
};
use crate::runtime_workbench::ValidationRuntimeWorkbench;
use digest_support::{combined_changed_facts, fold_texts, visible_result_digest};
pub use replay_artifact::ValidationAuthoringTruthFinalBossReplayArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationAuthoringTruthFinalBossProof {
    authored_delta_digest: u64,
    runtime_change: ValidationRuntimeChangeEvidence,
    projection_counters: ValidationAuthoringTruthProjectionCounters,
    projection_roster: ValidationAuthoringTruthProjectionRoster,
    compile_boundary: WorthUiCompileBoundaryCertification,
    visible_result_digest: u64,
    final_active_artifact_digest: u64,
    final_active_plan_digest: u64,
    final_capability_snapshot_digest: u64,
    final_authoring_snapshot_digest: Option<u64>,
    final_last_valid_artifact_digest: u64,
    final_last_valid_plan_digest: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ValidationAuthoringTruthProjectionCounters {
    inspected_projection_count: usize,
    dependency_intersection_count: usize,
    rebuild_attempt_count: usize,
    preserved_frame_count: usize,
    denied_frame_count: usize,
    rebuilt_frame_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationAuthoringTruthProjectionRoster {
    rows: Vec<ValidationAuthoringTruthProjectionRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationAuthoringTruthProjectionRow {
    surface: ValidationAuthoringTruthProjectionSurface,
    projection_identity: String,
    projection_family: WorthUiProjectionFamily,
    status: WorthUiProjectionRebindStatus,
    rebuild_attempted: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationAuthoringTruthProjectionSurface {
    Header,
    PageHost,
}

impl ValidationAuthoringTruthFinalBossProof {
    pub fn from_workbench(
        workbench: &ValidationRuntimeWorkbench,
        evidence_log: &ValidationReloadEvidenceLog,
    ) -> Option<Self> {
        let ValidationReloadEvidenceEntry::AuthoredBatchReload {
            source_evidence,
            capability_evidence,
            runtime_change,
            compile_boundary,
            header_rebind,
            page_host_rebind,
            ..
        } = evidence_log.latest()?
        else {
            return None;
        };
        let active = workbench.runtime().inspect_active();
        let last_valid = workbench.runtime().last_valid();
        let projection_counters = ValidationAuthoringTruthProjectionCounters::from_rebinds(
            header_rebind.as_ref(),
            page_host_rebind.as_ref(),
        );
        let mut projection_roster = ValidationAuthoringTruthProjectionRoster::from_rebinds(
            header_rebind.as_ref(),
            page_host_rebind.as_ref(),
        );
        let changed_facts = combined_changed_facts(source_evidence, capability_evidence);
        projection_roster.push_preserved_if_unaffected(
            ValidationAuthoringTruthProjectionSurface::Header,
            workbench.header_frame_plan().theme_plan(),
            &changed_facts,
        );
        projection_roster.push_preserved_if_unaffected(
            ValidationAuthoringTruthProjectionSurface::Header,
            workbench.header_frame_plan().appearance_plan(),
            &changed_facts,
        );
        projection_roster.push_preserved_if_unaffected(
            ValidationAuthoringTruthProjectionSurface::PageHost,
            workbench.page_host_plan(),
            &changed_facts,
        );
        Some(Self {
            authored_delta_digest: source_evidence.authored_delta_digest().unwrap_or_default(),
            runtime_change: runtime_change.clone(),
            projection_counters,
            projection_roster,
            compile_boundary: compile_boundary.clone(),
            visible_result_digest: visible_result_digest(workbench),
            final_active_artifact_digest: active.artifact_digest(),
            final_active_plan_digest: active.active_plan_digest(),
            final_capability_snapshot_digest: active.snapshot_digest(),
            final_authoring_snapshot_digest: active.authoring_snapshot_digest(),
            final_last_valid_artifact_digest: last_valid.artifact_digest(),
            final_last_valid_plan_digest: last_valid.active_plan_digest(),
        })
    }

    pub fn authored_delta_digest(&self) -> u64 {
        self.authored_delta_digest
    }
    pub fn runtime_change(&self) -> &ValidationRuntimeChangeEvidence {
        &self.runtime_change
    }
    pub fn changed_fact_rows(&self) -> &[ValidationRuntimeChangeFamilyRowEvidence] {
        self.runtime_change.rows()
    }
    pub fn counter_posture(&self) -> ValidationRuntimeChangeCountersEvidence {
        self.runtime_change.counters()
    }
    pub fn projection_counters(&self) -> ValidationAuthoringTruthProjectionCounters {
        self.projection_counters
    }
    pub fn projection_roster(&self) -> &ValidationAuthoringTruthProjectionRoster {
        &self.projection_roster
    }
    pub fn compile_boundary(&self) -> &WorthUiCompileBoundaryCertification {
        &self.compile_boundary
    }
    pub fn visible_result_digest(&self) -> u64 {
        self.visible_result_digest
    }
    pub fn replay_artifact(&self) -> ValidationAuthoringTruthFinalBossReplayArtifact {
        ValidationAuthoringTruthFinalBossReplayArtifact::new(
            self.authored_delta_digest,
            self.runtime_change.stable_digest(),
            self.compile_boundary.stable_digest(),
            self.visible_result_digest,
            self.projection_roster.digest(),
            self.final_active_artifact_digest,
            self.final_active_plan_digest,
            self.final_capability_snapshot_digest,
            self.final_authoring_snapshot_digest,
            self.final_last_valid_artifact_digest,
            self.final_last_valid_plan_digest,
        )
    }
}

impl ValidationAuthoringTruthProjectionCounters {
    fn from_rebinds(
        header: Option<&crate::reload::ValidationHeaderRebindEvidence>,
        page_host: Option<&crate::reload::ValidationPageHostRebindEvidence>,
    ) -> Self {
        let mut counters = Self::default();
        if let Some(header) = header {
            counters.absorb(
                header.inspected_projection_count(),
                header.dependency_intersection_count(),
                header.rebuild_attempt_count(),
                header.preserved_frame_count(),
                header.denied_frame_count(),
                header.rebuilt_frame_count(),
            );
        }
        if let Some(page_host) = page_host {
            counters.absorb(
                page_host.inspected_projection_count(),
                page_host.dependency_intersection_count(),
                page_host.rebuild_attempt_count(),
                page_host.preserved_frame_count(),
                page_host.denied_frame_count(),
                page_host.rebuilt_frame_count(),
            );
        }
        counters
    }

    fn absorb(
        &mut self,
        inspected: usize,
        intersections: usize,
        rebuilds: usize,
        preserved: usize,
        denied: usize,
        rebuilt: usize,
    ) {
        self.inspected_projection_count += inspected;
        self.dependency_intersection_count += intersections;
        self.rebuild_attempt_count += rebuilds;
        self.preserved_frame_count += preserved;
        self.denied_frame_count += denied;
        self.rebuilt_frame_count += rebuilt;
    }

    pub fn inspected_projection_count(self) -> usize {
        self.inspected_projection_count
    }
    pub fn dependency_intersection_count(self) -> usize {
        self.dependency_intersection_count
    }
    pub fn rebuild_attempt_count(self) -> usize {
        self.rebuild_attempt_count
    }
    pub fn preserved_frame_count(self) -> usize {
        self.preserved_frame_count
    }
    pub fn denied_frame_count(self) -> usize {
        self.denied_frame_count
    }
    pub fn rebuilt_frame_count(self) -> usize {
        self.rebuilt_frame_count
    }
}

impl ValidationAuthoringTruthProjectionRoster {
    fn from_rebinds(
        header: Option<&crate::reload::ValidationHeaderRebindEvidence>,
        page_host: Option<&crate::reload::ValidationPageHostRebindEvidence>,
    ) -> Self {
        let mut rows = Vec::new();
        if let Some(header) = header {
            for row in header.rows() {
                rows.push(ValidationAuthoringTruthProjectionRow {
                    surface: ValidationAuthoringTruthProjectionSurface::Header,
                    projection_identity: row.projection_identity().to_owned(),
                    projection_family: row.projection_family(),
                    status: row.status(),
                    rebuild_attempted: row.rebuild_attempted(),
                });
            }
        }
        if let Some(page_host) = page_host {
            for row in page_host.rows() {
                rows.push(ValidationAuthoringTruthProjectionRow {
                    surface: ValidationAuthoringTruthProjectionSurface::PageHost,
                    projection_identity: row.projection_identity().to_owned(),
                    projection_family: row.projection_family(),
                    status: row.status(),
                    rebuild_attempted: row.rebuild_attempted(),
                });
            }
        }
        rows.sort_by_cached_key(|row| {
            format!(
                "{:?}|{}|{:?}|{:?}",
                row.surface, row.projection_identity, row.projection_family, row.status
            )
        });
        rows.dedup();
        Self { rows }
    }

    pub fn rows(&self) -> &[ValidationAuthoringTruthProjectionRow] {
        &self.rows
    }
    pub fn rebuilt_projection_ids(&self) -> BTreeSet<String> {
        self.identities_matching(|row| {
            row.rebuild_attempted
                && matches!(
                    row.status,
                    WorthUiProjectionRebindStatus::EquivalentAfterActivation
                        | WorthUiProjectionRebindStatus::ReboundAfterActivation
                )
        })
    }
    pub fn preserved_projection_ids(&self) -> BTreeSet<String> {
        self.identities_matching(|row| {
            !row.rebuild_attempted
                && matches!(
                    row.status,
                    WorthUiProjectionRebindStatus::PreservedEquivalentReload
                        | WorthUiProjectionRebindStatus::EquivalentAfterActivation
                )
        })
    }
    pub fn denied_projection_ids(&self) -> BTreeSet<String> {
        self.identities_matching(|row| {
            !row.rebuild_attempted
                && matches!(
                    row.status,
                    WorthUiProjectionRebindStatus::PreservedDeniedReload
                        | WorthUiProjectionRebindStatus::DeniedReloadNotActivated
                )
        })
    }
    pub fn digest(&self) -> u64 {
        fold_texts(
            self.rows
                .iter()
                .map(ValidationAuthoringTruthProjectionRow::digest_basis),
        )
    }

    fn identities_matching(
        &self,
        include: impl Fn(&ValidationAuthoringTruthProjectionRow) -> bool,
    ) -> BTreeSet<String> {
        self.rows
            .iter()
            .filter(|row| include(row))
            .map(|row| row.projection_identity.clone())
            .collect()
    }

    fn push_if_missing(
        &mut self,
        surface: ValidationAuthoringTruthProjectionSurface,
        projection_identity: &str,
        projection_family: WorthUiProjectionFamily,
        status: WorthUiProjectionRebindStatus,
    ) {
        if self
            .rows
            .iter()
            .any(|row| row.projection_identity == projection_identity)
        {
            return;
        }
        self.rows.push(ValidationAuthoringTruthProjectionRow {
            surface,
            projection_identity: projection_identity.to_owned(),
            projection_family,
            status,
            rebuild_attempted: false,
        });
        self.rows.sort_by_cached_key(|row| {
            format!(
                "{:?}|{}|{:?}|{:?}",
                row.surface, row.projection_identity, row.projection_family, row.status
            )
        });
    }

    fn push_preserved_if_unaffected(
        &mut self,
        surface: ValidationAuthoringTruthProjectionSurface,
        contract: &impl WorthUiProjectionPlanContract,
        changed_facts: &worth_ui::facade::WorthUiRuntimeFactSet,
    ) {
        if contract
            .projection_dependency_declaration()
            .dependencies()
            .intersects(changed_facts)
        {
            return;
        }
        let identity = contract.projection_identity();
        self.push_if_missing(
            surface,
            identity.as_str(),
            contract.projection_family(),
            WorthUiProjectionRebindStatus::PreservedEquivalentReload,
        );
    }
}

impl ValidationAuthoringTruthProjectionRow {
    pub fn surface(&self) -> ValidationAuthoringTruthProjectionSurface {
        self.surface
    }
    pub fn projection_identity(&self) -> &str {
        &self.projection_identity
    }
    pub fn projection_family(&self) -> WorthUiProjectionFamily {
        self.projection_family
    }
    pub fn status(&self) -> WorthUiProjectionRebindStatus {
        self.status
    }

    pub fn rebuild_attempted(&self) -> bool {
        self.rebuild_attempted
    }

    fn digest_basis(&self) -> String {
        format!(
            "{:?}|{}|{:?}|{:?}|{}",
            self.surface,
            self.projection_identity,
            self.projection_family,
            self.status,
            self.rebuild_attempted
        )
    }
}
