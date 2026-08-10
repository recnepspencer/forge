use crate::{
    AuthoritativeReclaimMaintenanceDeclaration, CompactionMaintenanceDeclaration,
    DerivedFamilyRebuildMaintenanceDeclaration, MaintenanceAuditMaintenanceDeclaration,
    MaintenanceDeclaration, MaintenanceDeclarationId, RebuildMaintenanceDeclaration,
    ReclaimMaintenanceDeclaration, ReplicationPreparationMaintenanceDeclaration,
    RetentionMaintenanceDeclaration, SnapshotRefreshMaintenanceDeclaration,
    TierMoveMaintenanceDeclaration, TierPlacementMaintenanceDeclaration,
};

use super::declarations::PersistedMaintenanceDeclaration;

impl TryFrom<PersistedMaintenanceDeclaration> for MaintenanceDeclaration {
    type Error = String;

    fn try_from(declaration: PersistedMaintenanceDeclaration) -> Result<Self, Self::Error> {
        Ok(match declaration {
            PersistedMaintenanceDeclaration::Retention {
                id,
                batch_label,
                closure_commit_count,
                declaration_count,
            } => MaintenanceDeclaration::retention(
                MaintenanceDeclarationId::new(id),
                RetentionMaintenanceDeclaration::new(
                    batch_label,
                    closure_commit_count,
                    declaration_count,
                ),
            ),
            PersistedMaintenanceDeclaration::Compaction {
                id,
                retained_basis_label,
                retained_head_branch_ids,
                stable_basis_labels,
                closure_commit_ids,
                frontier_commit_ids,
                family_labels,
                superseded_families,
                rewritten_range_count,
            } => MaintenanceDeclaration::compaction(
                MaintenanceDeclarationId::new(id),
                CompactionMaintenanceDeclaration::new(
                    retained_basis_label,
                    retained_head_branch_ids,
                    stable_basis_labels,
                    closure_commit_ids,
                    frontier_commit_ids,
                    family_labels,
                    superseded_families,
                    rewritten_range_count,
                ),
            ),
            PersistedMaintenanceDeclaration::Reclaim {
                id,
                retained_basis_label,
                artifact_family,
                artifact_id,
            } => MaintenanceDeclaration::reclaim(
                MaintenanceDeclarationId::new(id),
                ReclaimMaintenanceDeclaration::new(
                    retained_basis_label,
                    artifact_family,
                    artifact_id,
                ),
            ),
            PersistedMaintenanceDeclaration::AuthoritativeReclaim {
                id,
                branch_id,
                oldest_retained_commit_id,
                expired_commit_ids,
            } => MaintenanceDeclaration::authoritative_reclaim(
                MaintenanceDeclarationId::new(id),
                AuthoritativeReclaimMaintenanceDeclaration::new(
                    branch_id,
                    oldest_retained_commit_id,
                    expired_commit_ids,
                ),
            ),
            PersistedMaintenanceDeclaration::Rebuild {
                id,
                retained_basis_label,
                family_label,
                rebuild_target_id,
                debt_link_artifact_id,
            } => MaintenanceDeclaration::rebuild(
                MaintenanceDeclarationId::new(id),
                RebuildMaintenanceDeclaration::new(
                    retained_basis_label,
                    family_label,
                    rebuild_target_id,
                    debt_link_artifact_id,
                ),
            ),
            PersistedMaintenanceDeclaration::DerivedFamilyRebuild {
                id,
                retained_basis_label,
                family_label,
                rebuild_target_id,
            } => MaintenanceDeclaration::derived_family_rebuild(
                MaintenanceDeclarationId::new(id),
                DerivedFamilyRebuildMaintenanceDeclaration::new(
                    retained_basis_label,
                    family_label,
                    rebuild_target_id,
                ),
            ),
            PersistedMaintenanceDeclaration::SnapshotRefresh {
                id,
                snapshot_family,
                locality_label,
                refresh_label,
            } => MaintenanceDeclaration::snapshot_refresh(
                MaintenanceDeclarationId::new(id),
                SnapshotRefreshMaintenanceDeclaration::new(
                    snapshot_family,
                    locality_label,
                    refresh_label,
                ),
            ),
            PersistedMaintenanceDeclaration::ReplicationPreparation {
                id,
                replication_family,
                locality_label,
                preparation_label,
            } => MaintenanceDeclaration::replication_preparation(
                MaintenanceDeclarationId::new(id),
                ReplicationPreparationMaintenanceDeclaration::new(
                    replication_family,
                    locality_label,
                    preparation_label,
                ),
            ),
            PersistedMaintenanceDeclaration::MaintenanceAudit {
                id,
                audit_family,
                locality_label,
                audit_label,
            } => MaintenanceDeclaration::maintenance_audit(
                MaintenanceDeclarationId::new(id),
                MaintenanceAuditMaintenanceDeclaration::new(
                    audit_family,
                    locality_label,
                    audit_label,
                ),
            ),
            PersistedMaintenanceDeclaration::TierPlacementProposal {
                id,
                placement_family,
                locality_label,
                proposal_label,
            } => MaintenanceDeclaration::tier_placement_proposal(
                MaintenanceDeclarationId::new(id),
                TierPlacementMaintenanceDeclaration::new(
                    placement_family,
                    locality_label,
                    proposal_label,
                ),
            ),
            PersistedMaintenanceDeclaration::TierMoveExecution {
                id,
                placement_family,
                locality_label,
                move_label,
                cross_locality_debt,
            } => MaintenanceDeclaration::tier_move_execution(
                MaintenanceDeclarationId::new(id),
                TierMoveMaintenanceDeclaration::new(
                    placement_family,
                    locality_label,
                    move_label,
                    cross_locality_debt,
                ),
            ),
        })
    }
}
