use crate::{
    backend::records::StoreState,
    failure::StoreError,
    publication::{classify_snapshot_publication, PublicationClassification},
    snapshot::{stable_snapshot_digest, SnapshotId},
};
use serde::Serialize;

use super::report::MaintenanceRecoveryDisposition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SnapshotMaintenanceRecoveryAction {
    RetainPublished,
    RequireRebuild,
    RequireQuarantine,
    DiscardUnpublished,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SnapshotMaintenanceRecoveryReport {
    snapshot_id: SnapshotId,
    publication_classification: PublicationClassification,
    action: SnapshotMaintenanceRecoveryAction,
    relation_valid: bool,
}

impl SnapshotMaintenanceRecoveryReport {
    pub fn snapshot_id(&self) -> SnapshotId {
        self.snapshot_id
    }

    pub fn publication_classification(&self) -> PublicationClassification {
        self.publication_classification
    }

    pub fn action(&self) -> SnapshotMaintenanceRecoveryAction {
        self.action
    }

    pub fn relation_valid(&self) -> bool {
        self.relation_valid
    }

    pub fn disposition(&self) -> MaintenanceRecoveryDisposition {
        match self.action {
            SnapshotMaintenanceRecoveryAction::RetainPublished => {
                MaintenanceRecoveryDisposition::RetainPublished
            }
            SnapshotMaintenanceRecoveryAction::RequireRebuild => {
                MaintenanceRecoveryDisposition::RequireRebuild
            }
            SnapshotMaintenanceRecoveryAction::RequireQuarantine => {
                MaintenanceRecoveryDisposition::RequireQuarantine
            }
            SnapshotMaintenanceRecoveryAction::DiscardUnpublished => {
                MaintenanceRecoveryDisposition::DiscardUnpublished
            }
        }
    }
}

pub(crate) fn classify_snapshot_maintenance_recovery(
    state: &StoreState,
    snapshot_id: SnapshotId,
    media_report: crate::media::DurableMediaReport,
) -> Result<SnapshotMaintenanceRecoveryReport, StoreError> {
    let basis = state.snapshot_basis_records.get(&snapshot_id.0).cloned();
    let image = state.snapshot_image_records.get(&snapshot_id.0).cloned();
    let publication = classify_snapshot_publication(media_report, basis.clone(), image.clone())?;
    let relation_valid = match (&basis, &image) {
        (Some(basis), Some(image)) => {
            stable_snapshot_digest(&image.image) == basis.snapshot_image_digest
        }
        _ => false,
    };
    let action = match publication.classification() {
        PublicationClassification::RetainTrusted if relation_valid => {
            SnapshotMaintenanceRecoveryAction::RetainPublished
        }
        PublicationClassification::RetainTrusted => {
            SnapshotMaintenanceRecoveryAction::RequireQuarantine
        }
        PublicationClassification::FinishPublication => {
            SnapshotMaintenanceRecoveryAction::RequireRebuild
        }
        PublicationClassification::RequireRebuild => {
            SnapshotMaintenanceRecoveryAction::RequireRebuild
        }
        PublicationClassification::RequireQuarantine => {
            SnapshotMaintenanceRecoveryAction::RequireQuarantine
        }
        PublicationClassification::DiscardUnpublished => {
            SnapshotMaintenanceRecoveryAction::DiscardUnpublished
        }
    };
    Ok(SnapshotMaintenanceRecoveryReport {
        snapshot_id,
        publication_classification: publication.classification(),
        action,
        relation_valid,
    })
}
