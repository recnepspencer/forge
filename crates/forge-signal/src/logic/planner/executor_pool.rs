#![cfg(feature = "parallel")]

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, OnceLock};

use rayon::ThreadPool;
use rayon::ThreadPoolBuilder;

use crate::data::error::SignalError;

#[derive(Debug, Clone)]
pub(super) struct PlannerExecutorPool {
    pool: Arc<ThreadPool>,
}

impl PlannerExecutorPool {
    pub(super) fn shared(worker_count: usize) -> Result<Self, SignalError> {
        let worker_count = worker_count.max(1);
        let registry = SHARED_POOLS.get_or_init(|| Mutex::new(BTreeMap::new()));
        let mut guard = registry
            .lock()
            .map_err(|_| SignalError::internal("planner executor pool registry poisoned"))?;
        if let Some(pool) = guard.get(&worker_count) {
            return Ok(Self {
                pool: Arc::clone(pool),
            });
        }
        let pool = Arc::new(
            ThreadPoolBuilder::new()
                .num_threads(worker_count)
                .thread_name(|index| format!("forge-signal-planner-{index}"))
                .build()
                .map_err(|error| {
                    SignalError::internal(format!("failed to build planner executor pool: {error}"))
                })?,
        );
        guard.insert(worker_count, Arc::clone(&pool));
        Ok(Self { pool })
    }

    pub(super) fn install<R: Send>(&self, run: impl FnOnce() -> R + Send) -> R {
        self.pool.install(run)
    }
}

static SHARED_POOLS: OnceLock<Mutex<BTreeMap<usize, Arc<ThreadPool>>>> = OnceLock::new();
