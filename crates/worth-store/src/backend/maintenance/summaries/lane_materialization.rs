use std::collections::BTreeMap;

use crate::{
    backend::records::{MaintenanceDebtSummaryRecord, MaintenanceQueueSummaryRecord, StoreState},
    maintenance::{
        MaintenanceDebtPressureClass, MaintenanceDebtSummary, MaintenanceLocalityScope,
        MaintenanceQueueSummary, MaintenanceStarvationStatus,
    },
};

use super::lane_accumulation::LaneAccumulator;

const STARVATION_DEFERRED_THRESHOLD: u64 = 2;

#[derive(Debug, Clone)]
pub(super) struct MaterializedMaintenanceLane {
    pub(super) queue_summary: MaintenanceQueueSummary,
}

pub(super) fn materialize_lane_and_debt_summaries(
    state: &mut StoreState,
    lane_accumulators: BTreeMap<String, LaneAccumulator>,
) -> Vec<MaterializedMaintenanceLane> {
    let mut materialized_lanes = Vec::new();
    for accumulator in lane_accumulators.into_values() {
        let lane_summary = accumulator.into_queue_summary();
        let lane_id = lane_summary.lane_key().artifact_id();
        let starvation_status = if lane_summary.deferred_count() >= STARVATION_DEFERRED_THRESHOLD {
            MaintenanceStarvationStatus::DeferredLanePressure
        } else {
            MaintenanceStarvationStatus::NotStarved
        };
        let debt_family = state
            .maintenance_declaration_records
            .values()
            .find(|record| record.work_descriptor.lane_key() == *lane_summary.lane_key())
            .and_then(|record| record.work_descriptor.debt_family());
        let pressure_class = if debt_family.is_none() {
            MaintenanceDebtPressureClass::None
        } else if matches!(
            starvation_status,
            MaintenanceStarvationStatus::DeferredLanePressure
        ) {
            MaintenanceDebtPressureClass::Elevated
        } else {
            MaintenanceDebtPressureClass::Active
        };
        let explicit_global_scope_debt = matches!(
            lane_summary.lane_key().locality_scope(),
            MaintenanceLocalityScope::StoreGlobalLocalityScope
        ) && (debt_family.is_some()
            || state.maintenance_execution_records.values().any(|record| {
                record.lane_key.as_ref() == Some(lane_summary.lane_key())
                    && record.explicit_global_scope_debt
            }));

        state.maintenance_queue_summary_records.insert(
            lane_id.clone(),
            MaintenanceQueueSummaryRecord {
                artifact_id: lane_id.clone(),
                family_version: 1,
                summary: lane_summary.clone(),
            },
        );
        state.maintenance_debt_summary_records.insert(
            lane_id.clone(),
            MaintenanceDebtSummaryRecord {
                artifact_id: lane_id,
                family_version: 1,
                lane_key: lane_summary.lane_key().clone(),
                summary: MaintenanceDebtSummary::new(
                    debt_family,
                    lane_summary.lane_key().locality_scope().clone(),
                    pressure_class,
                    starvation_status,
                    explicit_global_scope_debt,
                ),
            },
        );
        materialized_lanes.push(MaterializedMaintenanceLane {
            queue_summary: lane_summary,
        });
    }
    materialized_lanes
}
