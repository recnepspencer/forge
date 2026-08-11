use crate::MaintenanceDeclaration;

use serde::{Deserialize, Serialize};

use worth_relational::facade::history::{BranchId, CommitId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(in crate::backend::records::maintenance::declaration) enum PersistedMaintenanceDeclaration {
    Retention {
        id: String,
        batch_label: String,
        closure_commit_count: u64,
        declaration_count: u64,
    },
    Compaction {
        id: String,
        retained_basis_label: String,
        retained_head_branch_ids: Vec<BranchId>,
        stable_basis_labels: Vec<String>,
        closure_commit_ids: Vec<CommitId>,
        frontier_commit_ids: Vec<CommitId>,
        family_labels: Vec<String>,
        superseded_families: Vec<(String, String, Option<CommitId>)>,
        rewritten_range_count: u64,
    },
    Reclaim {
        id: String,
        retained_basis_label: String,
        artifact_family: String,
        artifact_id: String,
    },
    AuthoritativeReclaim {
        id: String,
        branch_id: BranchId,
        oldest_retained_commit_id: Option<CommitId>,
        expired_commit_ids: Vec<CommitId>,
    },
    Rebuild {
        id: String,
        retained_basis_label: String,
        family_label: String,
        rebuild_target_id: String,
        debt_link_artifact_id: Option<String>,
    },
    DerivedFamilyRebuild {
        id: String,
        retained_basis_label: String,
        family_label: String,
        rebuild_target_id: String,
    },
    SnapshotRefresh {
        id: String,
        snapshot_family: String,
        locality_label: String,
        refresh_label: String,
    },
    ReplicationPreparation {
        id: String,
        replication_family: String,
        locality_label: String,
        preparation_label: String,
    },
    MaintenanceAudit {
        id: String,
        audit_family: String,
        locality_label: String,
        audit_label: String,
    },
    TierPlacementProposal {
        id: String,
        placement_family: String,
        locality_label: String,
        proposal_label: String,
    },
    TierMoveExecution {
        id: String,
        placement_family: String,
        locality_label: String,
        move_label: String,
        cross_locality_debt: bool,
    },
}

impl From<&MaintenanceDeclaration> for PersistedMaintenanceDeclaration {
    fn from(declaration: &MaintenanceDeclaration) -> Self {
        match declaration {
            MaintenanceDeclaration::Retention { id, declaration } => Self::Retention {
                id: id.as_str().to_string(),
                batch_label: declaration.batch_label().to_string(),
                closure_commit_count: declaration.closure_commit_count(),
                declaration_count: declaration.declaration_count(),
            },
            MaintenanceDeclaration::Compaction { id, declaration } => Self::Compaction {
                id: id.as_str().to_string(),
                retained_basis_label: declaration.retained_basis_label().to_string(),
                retained_head_branch_ids: declaration.retained_head_branch_ids().to_vec(),
                stable_basis_labels: declaration.stable_basis_labels().to_vec(),
                closure_commit_ids: declaration.closure_commit_ids().to_vec(),
                frontier_commit_ids: declaration.frontier_commit_ids().to_vec(),
                family_labels: declaration.family_labels().to_vec(),
                superseded_families: declaration.superseded_families().to_vec(),
                rewritten_range_count: declaration.rewritten_range_count(),
            },
            MaintenanceDeclaration::Reclaim { id, declaration } => Self::Reclaim {
                id: id.as_str().to_string(),
                retained_basis_label: declaration.retained_basis_label().to_string(),
                artifact_family: declaration.artifact_family().to_string(),
                artifact_id: declaration.artifact_id().to_string(),
            },
            MaintenanceDeclaration::AuthoritativeReclaim { id, declaration } => {
                Self::AuthoritativeReclaim {
                    id: id.as_str().to_string(),
                    branch_id: declaration.branch_id().clone(),
                    oldest_retained_commit_id: declaration.oldest_retained_commit_id(),
                    expired_commit_ids: declaration.expired_commit_ids().to_vec(),
                }
            }
            MaintenanceDeclaration::Rebuild { id, declaration } => Self::Rebuild {
                id: id.as_str().to_string(),
                retained_basis_label: declaration.retained_basis_label().to_string(),
                family_label: declaration.family_label().to_string(),
                rebuild_target_id: declaration.rebuild_target_id().to_string(),
                debt_link_artifact_id: declaration.debt_link_artifact_id().map(ToString::to_string),
            },
            MaintenanceDeclaration::DerivedFamilyRebuild { id, declaration } => {
                Self::DerivedFamilyRebuild {
                    id: id.as_str().to_string(),
                    retained_basis_label: declaration.retained_basis_label().to_string(),
                    family_label: declaration.family_label().to_string(),
                    rebuild_target_id: declaration.rebuild_target_id().to_string(),
                }
            }
            MaintenanceDeclaration::SnapshotRefresh { id, declaration } => Self::SnapshotRefresh {
                id: id.as_str().to_string(),
                snapshot_family: declaration.snapshot_family().to_string(),
                locality_label: declaration.locality_label().to_string(),
                refresh_label: declaration.refresh_label().to_string(),
            },
            MaintenanceDeclaration::ReplicationPreparation { id, declaration } => {
                Self::ReplicationPreparation {
                    id: id.as_str().to_string(),
                    replication_family: declaration.replication_family().to_string(),
                    locality_label: declaration.locality_label().to_string(),
                    preparation_label: declaration.preparation_label().to_string(),
                }
            }
            MaintenanceDeclaration::MaintenanceAudit { id, declaration } => {
                Self::MaintenanceAudit {
                    id: id.as_str().to_string(),
                    audit_family: declaration.audit_family().to_string(),
                    locality_label: declaration.locality_label().to_string(),
                    audit_label: declaration.audit_label().to_string(),
                }
            }
            MaintenanceDeclaration::TierPlacementProposal { id, declaration } => {
                Self::TierPlacementProposal {
                    id: id.as_str().to_string(),
                    placement_family: declaration.placement_family().to_string(),
                    locality_label: declaration.locality_label().to_string(),
                    proposal_label: declaration.proposal_label().to_string(),
                }
            }
            MaintenanceDeclaration::TierMoveExecution { id, declaration } => {
                Self::TierMoveExecution {
                    id: id.as_str().to_string(),
                    placement_family: declaration.placement_family().to_string(),
                    locality_label: declaration.locality_label().to_string(),
                    move_label: declaration.move_label().to_string(),
                    cross_locality_debt: declaration.cross_locality_debt(),
                }
            }
        }
    }
}
