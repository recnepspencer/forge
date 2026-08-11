use std::collections::BTreeMap;

use crate::{
    backend::records::StoreState,
    maintenance::{
        MaintenanceCoalescingDecision, MaintenanceExecutionStatus, MaintenanceLaneKey,
        MaintenanceQueueSummary, MaintenanceQueueSummaryBasis,
    },
};

#[derive(Debug, Clone)]
pub(super) struct LaneAccumulator {
    lane_key: MaintenanceLaneKey,
    admitted_count: u64,
    reserved_count: u64,
    deferred_count: u64,
    active_quantum_count: u64,
    coalesced_count: u64,
    cancelled_superseded_count: u64,
    equivalence_member_counts: BTreeMap<String, u64>,
    equivalence_leaders: BTreeMap<String, EquivalenceLeader>,
    max_supersession_epoch_by_equivalence: BTreeMap<String, u64>,
}

#[derive(Debug, Clone)]
struct EquivalenceLeader {
    created_order: u64,
    work_identity: String,
}

pub(super) fn accumulate_lane_facts(state: &StoreState) -> BTreeMap<String, LaneAccumulator> {
    let mut lane_accumulators = BTreeMap::<String, LaneAccumulator>::new();

    for declaration in state.maintenance_declaration_records.values() {
        let lane_key = declaration.work_descriptor.lane_key();
        let lane_id = lane_key.artifact_id();
        let execution = state
            .maintenance_execution_records
            .get(declaration.artifact_id.as_str());
        let accumulator = lane_accumulators
            .entry(lane_id)
            .or_insert_with(|| LaneAccumulator::new(lane_key.clone()));
        accumulator.admitted_count += 1;

        let equivalence_key = declaration
            .work_descriptor
            .equivalence_key()
            .as_str()
            .to_string();
        *accumulator
            .equivalence_member_counts
            .entry(equivalence_key.clone())
            .or_default() += 1;
        accumulator
            .max_supersession_epoch_by_equivalence
            .entry(equivalence_key.clone())
            .and_modify(|current| {
                *current = (*current).max(declaration.work_descriptor.supersession_epoch().value())
            })
            .or_insert(declaration.work_descriptor.supersession_epoch().value());
        accumulator
            .equivalence_leaders
            .entry(equivalence_key)
            .and_modify(|leader| {
                if declaration.created_order < leader.created_order {
                    *leader = EquivalenceLeader {
                        created_order: declaration.created_order,
                        work_identity: declaration
                            .work_descriptor
                            .work_identity()
                            .as_str()
                            .to_string(),
                    };
                }
            })
            .or_insert_with(|| EquivalenceLeader {
                created_order: declaration.created_order,
                work_identity: declaration
                    .work_descriptor
                    .work_identity()
                    .as_str()
                    .to_string(),
            });

        if let Some(execution) = execution {
            match execution.execution_status {
                MaintenanceExecutionStatus::Reserved => {
                    accumulator.reserved_count += 1;
                    accumulator.active_quantum_count += execution.last_quantum_units.unwrap_or(0);
                }
                MaintenanceExecutionStatus::Started => {
                    accumulator.active_quantum_count += execution.last_quantum_units.unwrap_or(0);
                }
                MaintenanceExecutionStatus::Deferred => {
                    accumulator.deferred_count += 1;
                }
                _ => {}
            }
            if matches!(
                execution.coalescing_decision,
                Some(MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember)
            ) {
                accumulator.coalesced_count += 1;
            }
            if matches!(
                execution.coalescing_decision,
                Some(MaintenanceCoalescingDecision::CancelledAsSuperseded)
            ) || execution.supersession_source.is_some()
            {
                accumulator.cancelled_superseded_count += 1;
            }
        }
    }

    lane_accumulators
}

impl LaneAccumulator {
    fn new(lane_key: MaintenanceLaneKey) -> Self {
        Self {
            lane_key,
            admitted_count: 0,
            reserved_count: 0,
            deferred_count: 0,
            active_quantum_count: 0,
            coalesced_count: 0,
            cancelled_superseded_count: 0,
            equivalence_member_counts: BTreeMap::new(),
            equivalence_leaders: BTreeMap::new(),
            max_supersession_epoch_by_equivalence: BTreeMap::new(),
        }
    }

    pub(super) fn into_queue_summary(self) -> MaintenanceQueueSummary {
        MaintenanceQueueSummary::new(MaintenanceQueueSummaryBasis {
            lane_key: self.lane_key,
            admitted_count: self.admitted_count,
            reserved_count: self.reserved_count,
            deferred_count: self.deferred_count,
            active_quantum_count: self.active_quantum_count,
            coalesced_count: self.coalesced_count,
            cancelled_superseded_count: self.cancelled_superseded_count,
            equivalence_member_counts: self.equivalence_member_counts,
            equivalence_leader_identities: self
                .equivalence_leaders
                .into_iter()
                .map(|(key, leader)| (key, leader.work_identity))
                .collect(),
            max_supersession_epoch_by_equivalence: self.max_supersession_epoch_by_equivalence,
        })
    }
}
