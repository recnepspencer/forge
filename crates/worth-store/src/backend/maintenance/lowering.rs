use crate::{
    maintenance::{
        AuthoritativeReclaimMaintenanceDeclaration, CompactionMaintenanceDeclaration,
        MaintenanceBatch, MaintenanceBatchClass, MaintenanceDeclaration, MaintenanceDeclarationId,
        RebuildMaintenanceDeclaration, ReclaimMaintenanceDeclaration,
        RetentionMaintenanceDeclaration,
    },
    retention::LoweredReclaimDeclaration,
};

pub(crate) fn lower_retention_maintenance_batch(
    lowered: crate::LoweredRetentionMaintenanceBatch,
) -> MaintenanceBatch {
    let mut declarations = Vec::new();
    let declared_work_count = lowered.compaction_declarations().len() as u64
        + lowered.reclaim_declarations().len() as u64
        + lowered.rebuild_declarations().len() as u64;
    declarations.push(MaintenanceDeclaration::retention(
        declaration_id("retention-root", lowered.batch_label()),
        RetentionMaintenanceDeclaration::new(
            lowered.batch_label(),
            lowered.closure_summary().closure_commit_count(),
            declared_work_count,
        ),
    ));

    declarations.extend(lowered.compaction_declarations().iter().map(|declaration| {
        MaintenanceDeclaration::compaction(
            declaration_id(
                "compaction",
                (
                    declaration.retained_basis_label(),
                    declaration.family_labels(),
                    declaration.closure_commit_ids(),
                ),
            ),
            CompactionMaintenanceDeclaration::new(
                declaration.retained_basis_label(),
                declaration.retained_head_branch_ids().to_vec(),
                declaration.stable_basis_labels().to_vec(),
                declaration.closure_commit_ids().to_vec(),
                declaration.frontier_commit_ids().to_vec(),
                declaration.family_labels().to_vec(),
                declaration.superseded_families().to_vec(),
                declaration.rewritten_range_count(),
            ),
        )
    }));

    declarations.extend(lowered.reclaim_declarations().iter().map(
        |declaration| match declaration {
            LoweredReclaimDeclaration::Derived {
                retained_basis_label,
                artifact_family,
                artifact_id,
            } => MaintenanceDeclaration::reclaim(
                declaration_id(
                    "derived-reclaim",
                    (retained_basis_label, artifact_family, artifact_id),
                ),
                ReclaimMaintenanceDeclaration::new(
                    retained_basis_label,
                    artifact_family,
                    artifact_id,
                ),
            ),
            LoweredReclaimDeclaration::Authoritative {
                branch_id,
                oldest_retained_commit_id,
                expired_commit_ids,
            } => MaintenanceDeclaration::authoritative_reclaim(
                declaration_id(
                    "authoritative-reclaim",
                    (branch_id, oldest_retained_commit_id, expired_commit_ids),
                ),
                AuthoritativeReclaimMaintenanceDeclaration::new(
                    branch_id.clone(),
                    *oldest_retained_commit_id,
                    expired_commit_ids.clone(),
                ),
            ),
        },
    ));

    declarations.extend(lowered.rebuild_declarations().iter().map(|declaration| {
        MaintenanceDeclaration::rebuild(
            declaration_id(
                "rebuild",
                (
                    declaration.retained_basis_label(),
                    declaration.family_label(),
                    declaration.rebuild_target_id(),
                ),
            ),
            RebuildMaintenanceDeclaration::new(
                declaration.retained_basis_label(),
                declaration.family_label(),
                declaration.rebuild_target_id(),
                Some(crate::backend::integrity::rebuild_debt_artifact_id(
                    declaration.family_label(),
                    declaration.retained_basis_label(),
                    declaration.rebuild_target_id(),
                )),
            ),
        )
    }));

    MaintenanceBatch::new(
        lowered.batch_label().to_string(),
        MaintenanceBatchClass::Retention,
        declarations,
    )
}

fn declaration_id(label: &str, value: impl serde::Serialize) -> MaintenanceDeclarationId {
    let json = serde_json::to_vec(&value).expect("maintenance declaration digest");
    let mut hasher = sha2::Sha256::new();
    use sha2::Digest;
    hasher.update(json);
    MaintenanceDeclarationId::new(format!("{label}:{:x}", hasher.finalize()))
}
