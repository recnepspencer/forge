use std::collections::BTreeMap;

use crate::{
    backend::records::{MaintenanceReservationSummaryRecord, StoreState},
    maintenance::{MaintenanceReservationFamily, MaintenanceReservationSummary},
};

use super::lane_materialization::MaterializedMaintenanceLane;

#[derive(Debug, Default, Clone)]
struct ReservationAccumulator {
    lane_count: u64,
    reserved_count: u64,
    deferred_count: u64,
}

pub(super) fn materialize_reservation_summaries(
    state: &mut StoreState,
    lanes: &[MaterializedMaintenanceLane],
) {
    let mut reservation_accumulators =
        BTreeMap::<MaintenanceReservationFamily, ReservationAccumulator>::new();
    for lane in lanes {
        let summary = &lane.queue_summary;
        let entry = reservation_accumulators
            .entry(summary.lane_key().reservation_family())
            .or_default();
        entry.lane_count += 1;
        entry.reserved_count += summary.reserved_count();
        entry.deferred_count += summary.deferred_count();
    }

    for (family, accumulator) in reservation_accumulators {
        let artifact_id = format!("reservation:{family:?}");
        state.maintenance_reservation_summary_records.insert(
            artifact_id.clone(),
            MaintenanceReservationSummaryRecord {
                artifact_id,
                family_version: 1,
                summary: MaintenanceReservationSummary::new(
                    family,
                    accumulator.lane_count,
                    accumulator.reserved_count,
                    accumulator.deferred_count,
                ),
            },
        );
    }
}
