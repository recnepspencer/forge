use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialGeometryEvidenceTouchCounterHonesty {
    Honest,
    Violation(SpatialGeometryEvidenceTouchCounterViolationRow),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialGeometryEvidenceTouchCounterViolationRow {
    stage: WorkloadEvidenceStage,
    observed_receipt_backed_counter_total: usize,
}

pub(super) fn spatial_touch_counter_honesty(
    stage: WorkloadEvidenceStage,
    counters: WorkloadEvidenceStageCounters,
) -> SpatialGeometryEvidenceTouchCounterHonesty {
    let observed_receipt_backed_counter_total = counters.total_receipt_backed_counters();
    if observed_receipt_backed_counter_total == 0 {
        return SpatialGeometryEvidenceTouchCounterHonesty::Violation(
            SpatialGeometryEvidenceTouchCounterViolationRow {
                stage,
                observed_receipt_backed_counter_total,
            },
        );
    }
    SpatialGeometryEvidenceTouchCounterHonesty::Honest
}

impl SpatialGeometryEvidenceTouchCounterHonesty {
    pub fn is_honest(self) -> bool {
        matches!(self, Self::Honest)
    }

    pub fn violation(self) -> Option<SpatialGeometryEvidenceTouchCounterViolationRow> {
        match self {
            Self::Honest => None,
            Self::Violation(row) => Some(row),
        }
    }

    pub(crate) fn digest_key(self) -> String {
        match self {
            Self::Honest => "counter-honesty:honest".to_string(),
            Self::Violation(row) => format!(
                "counter-honesty:violation|stage:{}|observed-total:{}",
                row.stage.human_name(),
                row.observed_receipt_backed_counter_total
            ),
        }
    }
}

impl SpatialGeometryEvidenceTouchCounterViolationRow {
    pub fn stage(self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub fn observed_receipt_backed_counter_total(self) -> usize {
        self.observed_receipt_backed_counter_total
    }
}
