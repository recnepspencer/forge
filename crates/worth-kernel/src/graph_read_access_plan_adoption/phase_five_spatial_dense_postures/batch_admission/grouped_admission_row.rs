use super::super::query_posture_projection::WorthGraphReadAccessSpatialDensePostureProjection;
use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessGroupedAdmissionRow {
    query_family_digest_seed: String,
    row_count: usize,
    grouped_admission_preserved: bool,
    caller_work_measurement_status: WorthGraphReadAccessGroupedAdmissionMeasurementStatus,
    scalarized_caller_loop_count: usize,
    row_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessGroupedAdmissionMeasurementStatus {
    NoGraphReadExecutionClaimed,
    ExecutionCountersRequired,
}

impl WorthGraphReadAccessGroupedAdmissionRow {
    pub(crate) fn from_projection_group(
        query_family_digest_seed: String,
        projections: &[&WorthGraphReadAccessSpatialDensePostureProjection],
    ) -> Self {
        let row_count = projections.len();
        let grouped_admission_preserved = row_count > 1;
        let caller_work_measurement_status = if projections
            .iter()
            .any(|projection| projection.claims_graph_read_receipt())
        {
            WorthGraphReadAccessGroupedAdmissionMeasurementStatus::ExecutionCountersRequired
        } else {
            WorthGraphReadAccessGroupedAdmissionMeasurementStatus::NoGraphReadExecutionClaimed
        };
        let scalarized_caller_loop_count =
            scalarized_caller_loop_count_for_measurement_status(caller_work_measurement_status);
        let row_digest = stable_digest(&[
            "worth_graph_read_access_grouped_admission_row_v1".to_string(),
            format!("query_family:{query_family_digest_seed}"),
            format!("row_count:{row_count}"),
            format!("grouped:{grouped_admission_preserved}"),
            format!("measurement:{}", caller_work_measurement_status.as_str()),
            format!("scalarized:{scalarized_caller_loop_count}"),
        ]);
        Self {
            query_family_digest_seed,
            row_count,
            grouped_admission_preserved,
            caller_work_measurement_status,
            scalarized_caller_loop_count,
            row_digest,
        }
    }

    pub fn query_family_digest_seed(&self) -> &str {
        &self.query_family_digest_seed
    }

    pub const fn row_count(&self) -> usize {
        self.row_count
    }

    pub const fn grouped_admission_preserved(&self) -> bool {
        self.grouped_admission_preserved
    }

    pub const fn caller_work_measurement_status(
        &self,
    ) -> WorthGraphReadAccessGroupedAdmissionMeasurementStatus {
        self.caller_work_measurement_status
    }

    pub const fn scalarized_caller_loop_count(&self) -> usize {
        self.scalarized_caller_loop_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

impl WorthGraphReadAccessGroupedAdmissionMeasurementStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoGraphReadExecutionClaimed => "no_graph_read_execution_claimed",
            Self::ExecutionCountersRequired => "execution_counters_required",
        }
    }
}

const fn scalarized_caller_loop_count_for_measurement_status(
    status: WorthGraphReadAccessGroupedAdmissionMeasurementStatus,
) -> usize {
    match status {
        WorthGraphReadAccessGroupedAdmissionMeasurementStatus::NoGraphReadExecutionClaimed => 0,
        WorthGraphReadAccessGroupedAdmissionMeasurementStatus::ExecutionCountersRequired => 1,
    }
}
