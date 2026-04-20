use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        integrity::rebuild_debt_artifact_id,
    },
    failure::{StoreError, StoreErrorKind},
    maintenance::{MaintenanceDeclaration, StartedMaintenance},
    retention::{CompactionPlan, SupersededPhysicalFamily},
};

pub(crate) fn ensure_execution_eligibility<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    declaration: &MaintenanceDeclaration,
) -> Result<(), StoreError> {
    match declaration {
        MaintenanceDeclaration::Rebuild { declaration, .. } => {
            let debt_id = rebuild_debt_artifact_id(
                declaration.family_label(),
                declaration.retained_basis_label(),
                declaration.rebuild_target_id(),
            );
            let debt_record = backend
                .state()
                .rebuild_debt_records
                .get(&debt_id)
                .ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::ReclaimEligibilityViolation,
                        format!(
                            "rebuild debt `{debt_id}` was not published before maintenance execution"
                        ),
                    )
                })?;
            if debt_record.cleared {
                return Err(StoreError::new(
                    StoreErrorKind::MaintenanceLifecycleViolation,
                    format!(
                        "rebuild maintenance declaration `{}` cannot execute because debt `{debt_id}` is already cleared",
                        declaration.rebuild_target_id()
                    ),
                ));
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

pub(crate) fn execute_started_declaration<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    started: &StartedMaintenance,
) -> Result<String, StoreError> {
    match started.declaration() {
        MaintenanceDeclaration::Retention { .. } => Ok("retention_root_observed".to_string()),
        MaintenanceDeclaration::Compaction { declaration, .. } => {
            let closure_witness = crate::RetentionClosureWitness::new(
                crate::RetainedHeadSet::new(declaration.retained_head_branch_ids().to_vec()),
                crate::StableBasisSet::new(declaration.stable_basis_labels().to_vec()),
                declaration.closure_commit_ids().to_vec(),
                declaration.frontier_commit_ids().to_vec(),
            );
            let plan = CompactionPlan::new(
                declaration.retained_basis_label(),
                closure_witness,
                declaration.family_labels().to_vec(),
                declaration
                    .superseded_families()
                    .iter()
                    .map(|(family_label, artifact_id, basis_commit_id)| {
                        SupersededPhysicalFamily::new(
                            family_label.clone(),
                            artifact_id.clone(),
                            *basis_commit_id,
                        )
                    })
                    .collect(),
                declaration.rewritten_range_count(),
            );
            let publication = backend.publish_compaction_product(plan)?;
            backend.verify_compaction_product(publication.product().clone())?;
            backend.cutover_compaction_product(publication.product().clone())?;
            Ok("compaction_cutover".to_string())
        }
        MaintenanceDeclaration::Reclaim { declaration, .. } => {
            backend.execute_derived_reclaim(crate::ReclaimEligibilityWitness::new(
                declaration.artifact_family(),
                declaration.artifact_id(),
                declaration.retained_basis_label(),
            ))?;
            Ok("derived_reclaim".to_string())
        }
        MaintenanceDeclaration::AuthoritativeReclaim { declaration, .. } => {
            backend.execute_authoritative_reclaim(crate::PolicyExpiredAuthorityRange::new(
                declaration.branch_id().clone(),
                declaration.oldest_retained_commit_id(),
                declaration.expired_commit_ids().to_vec(),
            ))?;
            Ok("authoritative_reclaim".to_string())
        }
        MaintenanceDeclaration::Rebuild { declaration, .. } => {
            backend.rebuild_reclaimed_derived_family(crate::RetainedRangeRebuildUnit::new(
                declaration.retained_basis_label(),
                declaration.family_label(),
                declaration.rebuild_target_id(),
            ))?;
            Ok("rebuild".to_string())
        }
    }
}
