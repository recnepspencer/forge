use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

use super::counters::{WorkloadEvidenceStageIndexCounterInput, WorkloadEvidenceStageIndexCounters};

pub(crate) fn build_stage_offsets(
    rows: &[WorkloadEvidenceRow],
) -> Result<[Option<usize>; WorkloadEvidenceStage::STAGE_COUNT], WorkloadEvidenceLedgerError> {
    let mut stage_offsets = [None; WorkloadEvidenceStage::STAGE_COUNT];
    for (row_index, row) in rows.iter().enumerate() {
        let slot = row.stage().index_slot();
        if stage_offsets[slot].is_some() {
            return Err(WorkloadEvidenceLedgerError::DuplicateEvidenceStage(
                row.stage(),
            ));
        }
        stage_offsets[slot] = Some(row_index);
    }
    Ok(stage_offsets)
}

pub(crate) fn stage_index_counters(
    rows: &[WorkloadEvidenceRow],
    stage_offsets: &[Option<usize>],
) -> WorkloadEvidenceStageIndexCounters {
    WorkloadEvidenceStageIndexCounters::new(WorkloadEvidenceStageIndexCounterInput {
        row_count: rows.len(),
        indexed_stage_count: stage_offsets
            .iter()
            .filter(|offset| offset.is_some())
            .count(),
        duplicate_stage_count: 0,
        manual_row_count: rows.iter().filter(|row| !row.is_receipt_backed()).count(),
        unadmitted_row_count: rows
            .iter()
            .filter(|row| row.is_receipt_backed() && !row.is_admitted())
            .count(),
        boolean_row_count: rows
            .iter()
            .filter(|row| row.stage().is_boolean_stage())
            .count(),
        counterless_boolean_row_count: rows
            .iter()
            .filter(|row| {
                row.stage().is_boolean_stage()
                    && !row
                        .counters()
                        .has_receipt_backed_counter_for_stage(row.stage())
            })
            .count(),
    })
}
