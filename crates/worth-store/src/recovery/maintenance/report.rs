use crate::{backend::records::StoreState, failure::StoreError, snapshot::SnapshotId};
pub use worth_store_contracts::MaintenanceArtifactFamily;
use serde::Serialize;
use std::collections::BTreeSet;

use super::snapshot::classify_snapshot_maintenance_recovery;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MaintenanceRecoveryDisposition {
    RetainPublished,
    RequireRebuild,
    RequireQuarantine,
    DiscardUnpublished,
    NotPresent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceRecoveryEntry {
    family: MaintenanceArtifactFamily,
    scope_identity: String,
    disposition: MaintenanceRecoveryDisposition,
    reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MaintenanceRecoveryReport {
    entries: Vec<MaintenanceRecoveryEntry>,
    active_declaration_count: u64,
    escalated_declaration_count: u64,
    recovered_backlog_count: u64,
}

impl MaintenanceRecoveryEntry {
    pub fn family(&self) -> MaintenanceArtifactFamily {
        self.family
    }

    pub fn scope_identity(&self) -> &str {
        &self.scope_identity
    }

    pub fn disposition(&self) -> MaintenanceRecoveryDisposition {
        self.disposition
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl MaintenanceRecoveryReport {
    pub fn entries(&self) -> &[MaintenanceRecoveryEntry] {
        &self.entries
    }

    pub fn active_declaration_count(&self) -> u64 {
        self.active_declaration_count
    }

    pub fn escalated_declaration_count(&self) -> u64 {
        self.escalated_declaration_count
    }

    pub fn recovered_backlog_count(&self) -> u64 {
        self.recovered_backlog_count
    }
}

pub(crate) fn build_maintenance_recovery_report(
    state: &StoreState,
    media_report: crate::media::DurableMediaReport,
) -> Result<MaintenanceRecoveryReport, StoreError> {
    let snapshot_ids = state
        .snapshot_basis_records
        .keys()
        .copied()
        .chain(state.snapshot_image_records.keys().copied())
        .collect::<BTreeSet<_>>();
    let mut entries = snapshot_ids
        .into_iter()
        .map(|snapshot_id| {
            let report = classify_snapshot_maintenance_recovery(
                state,
                SnapshotId(snapshot_id),
                media_report,
            )?;
            Ok(MaintenanceRecoveryEntry {
                family: MaintenanceArtifactFamily::Snapshot,
                scope_identity: format!("snapshot:{}", report.snapshot_id().0),
                disposition: report.disposition(),
                reason: format!(
                    "publication={:?}, relation_valid={}",
                    report.publication_classification(),
                    report.relation_valid()
                ),
            })
        })
        .collect::<Result<Vec<_>, StoreError>>()?;
    entries.extend([
        MaintenanceRecoveryEntry {
            family: MaintenanceArtifactFamily::Compaction,
            scope_identity: "compaction".to_string(),
            disposition: MaintenanceRecoveryDisposition::NotPresent,
            reason:
                "no compaction publication families are persisted in the current implementation"
                    .to_string(),
        },
        MaintenanceRecoveryEntry {
            family: MaintenanceArtifactFamily::Reclaim,
            scope_identity: "reclaim".to_string(),
            disposition: MaintenanceRecoveryDisposition::NotPresent,
            reason: "no reclaim publication families are persisted in the current implementation"
                .to_string(),
        },
        MaintenanceRecoveryEntry {
            family: MaintenanceArtifactFamily::Capsule,
            scope_identity: "capsule".to_string(),
            disposition: MaintenanceRecoveryDisposition::NotPresent,
            reason: "no capsule publication families are persisted in the current implementation"
                .to_string(),
        },
    ]);
    Ok(MaintenanceRecoveryReport {
        entries,
        active_declaration_count: state
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.execution_status,
                    crate::MaintenanceExecutionStatus::Admitted
                        | crate::MaintenanceExecutionStatus::Reserved
                        | crate::MaintenanceExecutionStatus::Started
                )
            })
            .count() as u64,
        escalated_declaration_count: state
            .maintenance_execution_records
            .values()
            .filter(|record| {
                matches!(
                    record.plan_family,
                    Some(crate::MaintenancePlanFamily::Escalated)
                )
            })
            .count() as u64,
        recovered_backlog_count: state
            .maintenance_declaration_records
            .values()
            .filter(|record| record.work_descriptor.recovered_from_restart())
            .count() as u64,
    })
}
