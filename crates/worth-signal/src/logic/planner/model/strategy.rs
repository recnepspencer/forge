#[cfg(feature = "parallel")]
use std::num::NonZeroUsize;
#[cfg(feature = "parallel")]
use std::thread::available_parallelism;

use serde::{Deserialize, Serialize};

#[cfg(feature = "parallel")]
use super::report::ParallelExecutionKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum StageExecutor {
    #[default]
    Serial,
    #[cfg(feature = "parallel")]
    StagedParallelPrecompute { policy: ParallelExecutionPolicy },
    #[cfg(feature = "parallel")]
    FullParallel { policy: ParallelExecutionPolicy },
}

impl StageExecutor {
    #[cfg(feature = "parallel")]
    pub fn conservative_parallel() -> Self {
        Self::full_parallel(16).with_parallel_policy(
            ParallelExecutionPolicy::new(
                NonZeroUsize::new(16).expect("constant min stage width is non-zero"),
            )
            .with_worker_count(2)
            .with_chunk_size(2)
            .with_apply_group_min_width(2)
            .with_max_concurrent_apply_groups(2),
        )
    }

    #[cfg(feature = "parallel")]
    pub fn balanced_parallel() -> Self {
        Self::full_parallel(12).with_parallel_policy(
            ParallelExecutionPolicy::new(
                NonZeroUsize::new(12).expect("constant min stage width is non-zero"),
            )
            .with_worker_count(available_parallelism().map_or(4, |count| count.get().min(4)))
            .with_chunk_size(2)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(2),
        )
    }

    #[cfg(feature = "parallel")]
    pub fn aggressive_parallel() -> Self {
        Self::full_parallel(8).with_parallel_policy(
            ParallelExecutionPolicy::new(
                NonZeroUsize::new(8).expect("constant min stage width is non-zero"),
            )
            .with_worker_count(available_parallelism().map_or(4, |count| count.get().min(8)))
            .with_chunk_size(1)
            .with_apply_group_min_width(1)
            .with_max_concurrent_apply_groups(4),
        )
    }

    #[cfg(feature = "parallel")]
    pub fn parallel(min_stage_width: usize) -> Self {
        Self::staged_parallel_precompute(min_stage_width)
    }

    #[cfg(feature = "parallel")]
    pub fn staged_parallel_precompute(min_stage_width: usize) -> Self {
        Self::StagedParallelPrecompute {
            policy: ParallelExecutionPolicy::new(non_zero_width(min_stage_width)),
        }
    }

    #[cfg(feature = "parallel")]
    pub fn full_parallel(min_stage_width: usize) -> Self {
        Self::FullParallel {
            policy: ParallelExecutionPolicy::new(non_zero_width(min_stage_width)),
        }
    }

    #[cfg(feature = "parallel")]
    pub fn with_parallel_policy(self, policy: ParallelExecutionPolicy) -> Self {
        match self {
            Self::Serial => Self::Serial,
            Self::StagedParallelPrecompute { .. } => Self::StagedParallelPrecompute { policy },
            Self::FullParallel { .. } => Self::FullParallel { policy },
        }
    }

    #[cfg(feature = "parallel")]
    pub(crate) fn parallel_kind(&self) -> Option<ParallelExecutionKind> {
        match self {
            Self::Serial => None,
            Self::StagedParallelPrecompute { .. } => {
                Some(ParallelExecutionKind::StagedParallelPrecompute)
            }
            Self::FullParallel { .. } => Some(ParallelExecutionKind::FullParallel),
        }
    }

    #[cfg(feature = "parallel")]
    pub(crate) fn parallel_policy(&self) -> Option<ParallelExecutionPolicy> {
        match self {
            Self::Serial => None,
            Self::StagedParallelPrecompute { policy } | Self::FullParallel { policy } => {
                Some(*policy)
            }
        }
    }

    #[cfg(feature = "parallel")]
    pub(crate) fn is_full_parallel(&self) -> bool {
        matches!(self, Self::FullParallel { .. })
    }
}

#[cfg(feature = "parallel")]
fn non_zero_width(width: usize) -> NonZeroUsize {
    NonZeroUsize::new(width.max(1)).expect("parallel min stage width is clamped to at least one")
}

#[cfg(feature = "parallel")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParallelExecutionPolicy {
    pub min_stage_width: NonZeroUsize,
    pub worker_count: Option<NonZeroUsize>,
    pub chunk_size: Option<NonZeroUsize>,
    pub apply_group_min_width: NonZeroUsize,
    pub max_concurrent_apply_groups: Option<NonZeroUsize>,
}

#[cfg(feature = "parallel")]
impl ParallelExecutionPolicy {
    pub fn new(min_stage_width: NonZeroUsize) -> Self {
        Self {
            min_stage_width,
            worker_count: None,
            chunk_size: None,
            apply_group_min_width: min_stage_width,
            max_concurrent_apply_groups: None,
        }
    }

    pub fn with_worker_count(mut self, worker_count: usize) -> Self {
        self.worker_count = NonZeroUsize::new(worker_count.max(1));
        self
    }

    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = NonZeroUsize::new(chunk_size.max(1));
        self
    }

    pub fn with_apply_group_min_width(mut self, min_width: usize) -> Self {
        self.apply_group_min_width = NonZeroUsize::new(min_width.max(1))
            .expect("apply group min width is clamped to at least one");
        self
    }

    pub fn with_max_concurrent_apply_groups(mut self, max_groups: usize) -> Self {
        self.max_concurrent_apply_groups = NonZeroUsize::new(max_groups.max(1));
        self
    }

    pub(crate) fn chunk_size_for(self, task_count: usize) -> usize {
        if let Some(chunk_size) = self.chunk_size {
            return chunk_size.get().min(task_count.max(1));
        }
        let workers = self
            .worker_count
            .map(|count| count.get())
            .or_else(|| available_parallelism().ok().map(|count| count.get()))
            .unwrap_or(1)
            .max(1);
        task_count.div_ceil(workers).max(1)
    }

    pub(crate) fn worker_count_for(self, task_count: usize) -> usize {
        self.worker_count
            .map(|count| count.get())
            .or_else(|| available_parallelism().ok().map(|count| count.get()))
            .unwrap_or(1)
            .min(task_count.max(1))
            .max(1)
    }

    pub(crate) fn max_apply_group_count_for(self, task_count: usize) -> usize {
        self.max_concurrent_apply_groups
            .map(|count| count.get())
            .unwrap_or_else(|| self.worker_count_for(task_count))
            .min(task_count.max(1))
            .max(1)
    }
}
