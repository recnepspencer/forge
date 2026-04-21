use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        records::{
            MaintenanceBatchRecord, MaintenanceDeclarationRecord, MaintenanceExecutionRecord,
        },
    },
    maintenance::{
        AdmittedMaintenanceDeclaration, MaintenanceAdmissionReceipt, MaintenanceAdmissionRejection,
        MaintenanceBatch, MaintenanceExecutionStatus,
    },
};

pub(crate) fn admit_maintenance_batch<P: StatePersistence>(
    backend: &mut StateBackedStoreBackend<P>,
    batch: MaintenanceBatch,
) -> Result<MaintenanceAdmissionReceipt, crate::StoreError> {
    let mut next = backend.state().clone();
    let mut admitted = Vec::new();
    let mut rejections = Vec::new();
    let mut declaration_ids = Vec::new();

    for declaration in batch.declarations().iter().cloned() {
        let descriptor = declaration.work_descriptor();
        let declaration_id = declaration.id().as_str().to_string();
        let duplicate_id = next
            .maintenance_declaration_records
            .contains_key(&declaration_id);
        let duplicate_equivalence = next.maintenance_declaration_records.values().any(|record| {
            record.work_descriptor.equivalence_key() == descriptor.equivalence_key()
                && (record.work_descriptor.work_class() != descriptor.work_class()
                    || record.work_descriptor.locality_scope() != descriptor.locality_scope())
        });
        if duplicate_id || duplicate_equivalence {
            rejections.push(MaintenanceAdmissionRejection::new(
                declaration.id().clone(),
                "maintenance declaration with identical identity or equivalence key was already admitted",
            ));
            continue;
        }
        let family_label = match &declaration {
            crate::MaintenanceDeclaration::Retention { .. } => None,
            crate::MaintenanceDeclaration::Compaction { declaration, .. } => {
                declaration.family_labels().first().cloned()
            }
            crate::MaintenanceDeclaration::Reclaim { declaration, .. } => {
                Some(declaration.artifact_family().to_string())
            }
            crate::MaintenanceDeclaration::AuthoritativeReclaim { .. } => {
                Some("authoritative_range".to_string())
            }
            crate::MaintenanceDeclaration::Rebuild { declaration, .. } => {
                Some(declaration.family_label().to_string())
            }
            crate::MaintenanceDeclaration::DerivedFamilyRebuild { declaration, .. } => {
                Some(declaration.family_label().to_string())
            }
            crate::MaintenanceDeclaration::SnapshotRefresh { declaration, .. } => {
                Some(declaration.snapshot_family().to_string())
            }
            crate::MaintenanceDeclaration::ReplicationPreparation { declaration, .. } => {
                Some(declaration.replication_family().to_string())
            }
            crate::MaintenanceDeclaration::MaintenanceAudit { declaration, .. } => {
                Some(declaration.audit_family().to_string())
            }
            crate::MaintenanceDeclaration::TierPlacementProposal { declaration, .. } => {
                Some(declaration.placement_family().to_string())
            }
            crate::MaintenanceDeclaration::TierMoveExecution { declaration, .. } => {
                Some(declaration.placement_family().to_string())
            }
        };
        let debt_link_artifact_id = match &declaration {
            crate::MaintenanceDeclaration::Rebuild { declaration, .. } => {
                declaration.debt_link_artifact_id().map(ToString::to_string)
            }
            _ => None,
        };
        next.next_maintenance_declaration_order += 1;
        declaration_ids.push(declaration_id.clone());
        next.maintenance_declaration_records.insert(
            declaration_id.clone(),
            MaintenanceDeclarationRecord {
                artifact_id: declaration_id.clone(),
                family_version: 1,
                batch_id: batch.batch_id().to_string(),
                declaration_class: declaration.class(),
                retained_basis_label: declaration.retained_basis_label().map(ToString::to_string),
                family_label,
                debt_link_artifact_id: debt_link_artifact_id.clone(),
                work_descriptor: descriptor.clone(),
                declaration: declaration.clone(),
                created_order: next.next_maintenance_declaration_order,
            },
        );
        next.maintenance_execution_records.insert(
            declaration_id.clone(),
            MaintenanceExecutionRecord {
                artifact_id: declaration_id.clone(),
                family_version: 1,
                declaration_id: declaration_id.clone(),
                execution_status: MaintenanceExecutionStatus::Admitted,
                lane_key: Some(descriptor.lane_key()),
                plan_family: None,
                last_completed_phase: Some("admitted".to_string()),
                pending_reason: None,
                durable_error_kind: None,
                durable_error_message: None,
                last_quantum_units: None,
                reservation_transition: None,
                execution_transition: None,
                restart_readmission_status: if descriptor.recovered_from_restart() {
                    Some(crate::MaintenanceReadmissionStatus::PendingRecoveredReadmission)
                } else {
                    None
                },
                foreground_impact: crate::MaintenanceForegroundImpact::none(),
                coalescing_decision: Some(crate::MaintenanceCoalescingDecision::NotCoalesced),
                supersession_source: None,
                resource_budget_grant: None,
                starvation_status: None,
                escalation_verdict: None,
                explicit_global_scope_debt: false,
                resume_count: 0,
            },
        );
        admitted.push(AdmittedMaintenanceDeclaration::new(declaration, descriptor));
    }

    next.maintenance_batch_records.insert(
        batch.batch_id().to_string(),
        MaintenanceBatchRecord {
            artifact_id: batch.batch_id().to_string(),
            family_version: 1,
            batch_class: batch.batch_class(),
            declaration_ids,
            declaration_count: admitted.len() as u64,
        },
    );
    super::summaries::refresh_scheduler_summaries(&mut next);
    backend.commit_replacement_state(next)?;
    backend
        .counters()
        .record_maintenance_admissions(admitted.len() as u64);
    backend
        .counters()
        .record_maintenance_locality_touches(admitted.len() as u64);
    backend
        .counters()
        .record_maintenance_rejections(rejections.len() as u64);
    backend.counters().record_maintenance_debt_links(
        admitted
            .iter()
            .filter(|declaration| match declaration.declaration() {
                crate::MaintenanceDeclaration::Rebuild { declaration, .. } => {
                    declaration.debt_link_artifact_id().is_some()
                }
                _ => false,
            })
            .count() as u64,
    );

    Ok(MaintenanceAdmissionReceipt::new(
        crate::MaintenanceBatchSummary::new(
            batch.batch_id().to_string(),
            batch.batch_class(),
            admitted.len() as u64,
        ),
        admitted,
        rejections,
    ))
}
