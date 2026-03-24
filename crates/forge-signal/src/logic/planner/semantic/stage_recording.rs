#[cfg(not(feature = "parallel"))]
use super::super::types::ParallelAdmissionReason;
use super::super::types::{StageExecutionOutcome, StageExecutionRecord};

#[cfg(feature = "parallel")]
use super::super::precompute::admission::StageParallelAdmission;

pub(crate) fn begin_stage_record(
    stage_index: u32,
    snapshot_nanos: u128,
    precompute_nanos: u128,
    #[cfg(feature = "parallel")] parallel_admission: StageParallelAdmission,
) -> StageExecutionRecord {
    StageExecutionRecord {
        stage_index,
        outcome: {
            #[cfg(feature = "parallel")]
            {
                if parallel_admission.use_parallel {
                    StageExecutionOutcome::CompletedParallel
                } else {
                    StageExecutionOutcome::CompletedSerial
                }
            }
            #[cfg(not(feature = "parallel"))]
            {
                StageExecutionOutcome::CompletedSerial
            }
        },
        authority_policy: None,
        parallel_admission_reason: Some({
            #[cfg(feature = "parallel")]
            {
                parallel_admission.reason
            }
            #[cfg(not(feature = "parallel"))]
            {
                ParallelAdmissionReason::SerialExecutor
            }
        }),
        #[cfg(feature = "parallel")]
        parallel_kind: parallel_admission.kind,
        #[cfg(feature = "parallel")]
        apply_mode: None,
        #[cfg(feature = "parallel")]
        apply_group_count: 0,
        #[cfg(feature = "parallel")]
        serial_apply_rejection_reason: None,
        #[cfg(feature = "parallel")]
        serial_fallback_group_count: 0,
        #[cfg(feature = "parallel")]
        concurrent_apply_task_count: 0,
        #[cfg(feature = "parallel")]
        serial_apply_task_count: 0,
        snapshot_duration_nanos: snapshot_nanos,
        precompute_duration_nanos: precompute_nanos,
        apply_duration_nanos: 0,
        semantic_finalize_duration_nanos: 0,
        duration_nanos: 0,
        semantic_task_range: None,
        semantic_segment_count: 0,
        task_records: Vec::new(),
    }
}
