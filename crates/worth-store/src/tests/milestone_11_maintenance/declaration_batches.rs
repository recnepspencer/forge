use crate::{
    MaintenanceBatch, MaintenanceBatchClass, MaintenanceDeclaration, MaintenanceDeclarationId,
};

pub(super) fn duplicate_compaction_batch(
    batch: &MaintenanceBatch,
    duplicate_id: &str,
) -> MaintenanceBatch {
    let mut declarations = batch.declarations().to_vec();
    let duplicate = declarations
        .iter()
        .find_map(|declaration| match declaration {
            MaintenanceDeclaration::Compaction { declaration, .. } => {
                Some(MaintenanceDeclaration::compaction(
                    MaintenanceDeclarationId::new(duplicate_id.to_string()),
                    declaration.clone(),
                ))
            }
            _ => None,
        })
        .expect("compaction declaration should exist");
    declarations.push(duplicate);
    MaintenanceBatch::new(
        format!("{}-duplicate", batch.batch_id()),
        MaintenanceBatchClass::Retention,
        declarations,
    )
}

pub(super) fn same_lane_distinct_compaction_batch(
    batch: &MaintenanceBatch,
    duplicate_id: &str,
) -> MaintenanceBatch {
    let mut declarations = batch.declarations().to_vec();
    let duplicate = declarations
        .iter()
        .find_map(|declaration| match declaration {
            MaintenanceDeclaration::Compaction { declaration, .. } => {
                Some(MaintenanceDeclaration::compaction(
                    MaintenanceDeclarationId::new(duplicate_id.to_string()),
                    crate::CompactionMaintenanceDeclaration::new(
                        declaration.retained_basis_label().to_string(),
                        declaration.retained_head_branch_ids().to_vec(),
                        declaration.stable_basis_labels().to_vec(),
                        declaration.closure_commit_ids().to_vec(),
                        declaration.frontier_commit_ids().to_vec(),
                        declaration.family_labels().to_vec(),
                        declaration.superseded_families().to_vec(),
                        declaration.rewritten_range_count() + 1,
                    ),
                ))
            }
            _ => None,
        })
        .expect("compaction declaration should exist");
    declarations.push(duplicate);
    MaintenanceBatch::new(
        format!("{}-same-lane-distinct", batch.batch_id()),
        MaintenanceBatchClass::Retention,
        declarations,
    )
}

pub(super) fn tier_placement_batch(batch_id: &str, declaration_id: &str) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::tier_placement_proposal(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::TierPlacementMaintenanceDeclaration::new(
                "snapshot_family",
                "family:tier-local",
                "proposal:conservative-cold",
            ),
        )],
    )
}

pub(super) fn snapshot_refresh_batch(batch_id: &str, declaration_id: &str) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::snapshot_refresh(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::SnapshotRefreshMaintenanceDeclaration::new(
                "snapshot_family",
                "family:snapshot-local",
                "refresh:publication-support",
            ),
        )],
    )
}

pub(super) fn derived_family_rebuild_batch(
    batch_id: &str,
    declaration_id: &str,
) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::derived_family_rebuild(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::DerivedFamilyRebuildMaintenanceDeclaration::new(
                "basis:derived-rebuild",
                "family:derived-local",
                "rebuild:derived-index",
            ),
        )],
    )
}

pub(super) fn replication_preparation_batch(
    batch_id: &str,
    declaration_id: &str,
) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::replication_preparation(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::ReplicationPreparationMaintenanceDeclaration::new(
                "replication_family",
                "family:replication-local",
                "prepare:capsule-handoff",
            ),
        )],
    )
}

pub(super) fn maintenance_audit_batch(batch_id: &str, declaration_id: &str) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::maintenance_audit(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::MaintenanceAuditMaintenanceDeclaration::new(
                "audit_family",
                "family:audit-local",
                "audit:queue-summary-parity",
            ),
        )],
    )
}

pub(super) fn tier_move_batch(
    batch_id: &str,
    declaration_id: &str,
    cross_locality_debt: bool,
) -> MaintenanceBatch {
    MaintenanceBatch::new(
        batch_id,
        MaintenanceBatchClass::Retention,
        vec![MaintenanceDeclaration::tier_move_execution(
            MaintenanceDeclarationId::new(declaration_id.to_string()),
            crate::TierMoveMaintenanceDeclaration::new(
                "snapshot_family",
                "family:tier-local",
                "move:cold-placement",
                cross_locality_debt,
            ),
        )],
    )
}
