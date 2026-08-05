use super::*;

pub(super) fn admit_adapter_cost(
    counters: &mut CompatibilityAdmissionCounters,
    family_id: &ArtifactFamilyId,
    edge: &DeclaredCompatibilityEdge,
    path: CompatibilityAdmissionPath,
) -> Result<(), CompatibilityRejection> {
    let Some(adapter) = edge.adapter() else {
        return Ok(());
    };
    counters.record_adapter_cost_class(adapter.cost_class());
    match (path, adapter.cost_class()) {
        (_, CompatibilityAdapterCostClass::ZeroCopy)
        | (_, CompatibilityAdapterCostClass::BoundedRecordLocal) => Ok(()),
        (CompatibilityAdmissionPath::HotRead, CompatibilityAdapterCostClass::BoundedBatchLocal) => {
            counters.adapter_hot_path_rejection_count += 1;
            Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::AdapterHotPathRejected,
                family_id.clone(),
                "batch-local compatibility adapter rejected from hot read path",
            ))
        }
        (
            CompatibilityAdmissionPath::HotRead | CompatibilityAdmissionPath::BatchRead,
            CompatibilityAdapterCostClass::MaintenanceOnly,
        ) => {
            counters.adapter_maintenance_required_rejection_count += 1;
            Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::AdapterMaintenanceRequired,
                family_id.clone(),
                "maintenance-only compatibility adapter rejected from read path",
            ))
        }
        (_, CompatibilityAdapterCostClass::OutOfScope) => {
            counters.adapter_out_of_scope_rejection_count += 1;
            Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::AdapterOutOfScope,
                family_id.clone(),
                "out-of-scope compatibility adapter rejected",
            ))
        }
        (
            CompatibilityAdmissionPath::BatchRead
            | CompatibilityAdmissionPath::MaintenanceScheduled,
            CompatibilityAdapterCostClass::BoundedBatchLocal,
        )
        | (
            CompatibilityAdmissionPath::MaintenanceScheduled,
            CompatibilityAdapterCostClass::MaintenanceOnly,
        ) => Ok(()),
    }
}
