use crate::backend::records::StoreState;

pub(super) fn recovered_declaration_count(state: &StoreState) -> u64 {
    state
        .maintenance_declaration_records
        .values()
        .filter(|record| record.work_descriptor.recovered_from_restart())
        .count() as u64
}

pub(super) fn recovered_maintenance_intake_report(
    state: &StoreState,
) -> crate::RecoveredMaintenanceIntakeReport {
    let mut lane_rollup = std::collections::BTreeMap::<
        crate::MaintenanceLaneKey,
        (u64, u64, u64, u64, u64, bool),
    >::new();
    for declaration in state.maintenance_declaration_records.values() {
        if !declaration.work_descriptor.recovered_from_restart() {
            continue;
        }
        let lane_key = declaration.work_descriptor.lane_key();
        let execution = state
            .maintenance_execution_records
            .get(declaration.artifact_id.as_str());
        let entry = lane_rollup
            .entry(lane_key.clone())
            .or_insert((0, 0, 0, 0, 0, false));
        match execution.and_then(|record| record.restart_readmission_status) {
            None | Some(crate::MaintenanceReadmissionStatus::PendingRecoveredReadmission) => {
                entry.0 += 1;
            }
            Some(crate::MaintenanceReadmissionStatus::ReadmittedRecoveredWork) => {
                entry.1 += 1;
            }
            Some(crate::MaintenanceReadmissionStatus::RejectedStaleRecoveredWork) => {
                entry.2 += 1;
                entry.3 += 1;
            }
            Some(crate::MaintenanceReadmissionStatus::RejectedSupersededRecoveredWork) => {
                entry.2 += 1;
            }
        }
        if matches!(
            execution.and_then(|record| record.coalescing_decision),
            Some(crate::MaintenanceCoalescingDecision::CoalescedWithEquivalentLaneMember)
        ) {
            entry.4 += 1;
        }
        entry.5 = state
            .maintenance_debt_summary_records
            .get(&lane_key.artifact_id())
            .map(|record| {
                !matches!(
                    record.summary.pressure_class(),
                    crate::MaintenanceDebtPressureClass::None
                )
            })
            .unwrap_or(false);
    }

    let lane_intake = lane_rollup
        .into_iter()
        .map(
            |(lane_key, (pending, readmitted, rejected, stale, coalesced, debt_bearing))| {
                crate::RecoveredMaintenanceLaneIntake::new(
                    lane_key,
                    pending,
                    readmitted,
                    rejected,
                    stale,
                    coalesced,
                    debt_bearing,
                )
            },
        )
        .collect::<Vec<_>>();

    crate::RecoveredMaintenanceIntakeReport::new(
        lane_intake
            .iter()
            .map(|lane| lane.pending_recovered_count())
            .sum(),
        lane_intake
            .iter()
            .map(|lane| lane.readmitted_recovered_count())
            .sum(),
        lane_intake
            .iter()
            .map(|lane| lane.rejected_recovered_count())
            .sum(),
        lane_intake
            .iter()
            .map(|lane| lane.stale_recovered_count())
            .sum(),
        lane_intake
            .iter()
            .map(|lane| lane.coalesced_recovered_count())
            .sum(),
        lane_intake,
    )
}
