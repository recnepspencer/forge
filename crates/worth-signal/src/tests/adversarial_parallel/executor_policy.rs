use std::num::NonZeroUsize;

use crate::facade::{
    ParallelAdmissionPolicy, ParallelExecutionPolicy, SignalRuntimePolicy, StageExecutor,
};

pub(super) fn hostile_executor_matrix() -> Vec<(&'static str, StageExecutor)> {
    let policies = [
        ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
            .with_worker_count(1)
            .with_chunk_size(1)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(1),
        ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
            .with_worker_count(2)
            .with_chunk_size(1)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(2),
        ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
            .with_worker_count(3)
            .with_chunk_size(2)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(2),
        ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
            .with_worker_count(4)
            .with_chunk_size(2)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(4),
    ];
    let mut executors = vec![("serial", StageExecutor::Serial)];
    for (index, policy) in policies.into_iter().enumerate() {
        executors.push((
            match index {
                0 => "staged-1x1",
                1 => "staged-2x1",
                2 => "staged-3x2",
                _ => "staged-4x2",
            },
            StageExecutor::parallel(1).with_parallel_policy(policy),
        ));
        executors.push((
            match index {
                0 => "full-1x1",
                1 => "full-2x1",
                2 => "full-3x2",
                _ => "full-4x2",
            },
            StageExecutor::full_parallel(1).with_parallel_policy(policy),
        ));
    }
    executors
}

pub(super) fn aggressive_parallel_runtime_policy() -> SignalRuntimePolicy {
    SignalRuntimePolicy::operational().with_parallel_admission(ParallelAdmissionPolicy {
        operational_min_parallel_tasks: 1,
        development_min_parallel_tasks: 1,
        forensic_min_parallel_tasks: 1,
        full_parallel_min_tasks: 1,
    })
}
