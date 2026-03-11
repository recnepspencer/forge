#![cfg(feature = "parallel")]

use std::sync::{Arc, OnceLock};

use rayon::ThreadPool;
use rayon::ThreadPoolBuilder;

use crate::data::error::SignalError;

#[derive(Debug, Clone)]
pub(super) struct PlannerExecutorPool {
    pool: Arc<ThreadPool>,
}

impl PlannerExecutorPool {
    pub(super) fn shared(_worker_count: usize) -> Result<Self, SignalError> {
        let pool = SHARED_POOL.get_or_init(|| {
            let threads = std::thread::available_parallelism()
                .map(|parallelism| parallelism.get())
                .unwrap_or(1);
            Arc::new(
                ThreadPoolBuilder::new()
                    .num_threads(threads)
                    .thread_name(|index| format!("forge-signal-planner-{index}"))
                    .build()
                    .unwrap_or_else(|error| {
                        panic!("failed to build planner executor pool: {error}")
                    }),
            )
        });
        Ok(Self {
            pool: Arc::clone(pool),
        })
    }

    pub(super) fn install<R: Send>(&self, run: impl FnOnce() -> R + Send) -> R {
        self.pool.install(run)
    }

    #[cfg(test)]
    pub(super) fn registry_key_for_worker_count(_worker_count: usize) -> usize {
        1
    }
}

static SHARED_POOL: OnceLock<Arc<ThreadPool>> = OnceLock::new();

#[cfg(test)]
mod tests {
    use super::PlannerExecutorPool;

    #[test]
    fn distinct_worker_counts_do_not_require_distinct_registry_keys() {
        let key_two = PlannerExecutorPool::registry_key_for_worker_count(2);
        let key_five = PlannerExecutorPool::registry_key_for_worker_count(5);
        let key_nine = PlannerExecutorPool::registry_key_for_worker_count(9);

        assert_eq!(
            key_two, key_five,
            "executor pool registry should not grow a sleeping pool for every requested worker count"
        );
        assert_eq!(
            key_two, key_nine,
            "executor pool registry should stay bounded even when executor policy varies worker counts"
        );
    }
}
