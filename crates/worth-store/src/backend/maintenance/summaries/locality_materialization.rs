use std::collections::BTreeMap;

use crate::{
    backend::records::{MaintenanceLocalitySummaryRecord, StoreState},
    maintenance::{MaintenanceLocalityScope, MaintenanceLocalitySummary},
};

use super::lane_materialization::MaterializedMaintenanceLane;

#[derive(Debug, Default, Clone)]
struct LocalityAccumulator {
    lane_count: u64,
    admitted_count: u64,
    deferred_count: u64,
    active_count: u64,
}

pub(super) fn materialize_locality_summaries(
    state: &mut StoreState,
    lanes: &[MaterializedMaintenanceLane],
) {
    let mut locality_accumulators =
        BTreeMap::<MaintenanceLocalityScope, LocalityAccumulator>::new();
    for lane in lanes {
        let summary = &lane.queue_summary;
        let entry = locality_accumulators
            .entry(summary.lane_key().locality_scope().clone())
            .or_default();
        entry.lane_count += 1;
        entry.admitted_count += summary.admitted_count();
        entry.deferred_count += summary.deferred_count();
        entry.active_count += summary.reserved_count() + summary.active_quantum_count();
    }

    for (scope, accumulator) in locality_accumulators {
        let artifact_id = locality_summary_artifact_id(&scope);
        state.maintenance_locality_summary_records.insert(
            artifact_id.clone(),
            MaintenanceLocalitySummaryRecord {
                artifact_id,
                family_version: 1,
                summary: MaintenanceLocalitySummary::new(
                    scope,
                    accumulator.lane_count,
                    accumulator.admitted_count,
                    accumulator.deferred_count,
                    accumulator.active_count,
                ),
            },
        );
    }
}

fn locality_summary_artifact_id(scope: &MaintenanceLocalityScope) -> String {
    match scope {
        MaintenanceLocalityScope::BranchLocalityScope { branch_label } => {
            format!("locality:branch:{branch_label}")
        }
        MaintenanceLocalityScope::ArtifactFamilyLocalityScope { family_label } => {
            format!("locality:family:{family_label}")
        }
        MaintenanceLocalityScope::TenantLocalityScope { tenant_label } => {
            format!("locality:tenant:{tenant_label}")
        }
        MaintenanceLocalityScope::StoreGlobalLocalityScope => "locality:store:global".to_string(),
    }
}
