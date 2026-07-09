#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum S6QueueProducerKind {
    WalCommitRecord,
    WalCheckpointRecord,
    BufferPoolReadAhead,
    BufferPoolWriteBack,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct S6QueueProducerResourceShape {
    queue_slots: u64,
    bandwidth_tokens: u64,
    flush_permits: u64,
    sync_debt: u64,
    read_ahead_windows: u64,
    write_back_windows: u64,
    dirty_page_budget: u64,
    worker_permits: u64,
    cache_residency_hints: u64,
    reclaim_permits: u64,
}

impl S6QueueProducerResourceShape {
    pub const fn new() -> Self {
        Self {
            queue_slots: 0,
            bandwidth_tokens: 0,
            flush_permits: 0,
            sync_debt: 0,
            read_ahead_windows: 0,
            write_back_windows: 0,
            dirty_page_budget: 0,
            worker_permits: 0,
            cache_residency_hints: 0,
            reclaim_permits: 0,
        }
    }

    pub const fn with_queue_slots(mut self, queue_slots: u64) -> Self {
        self.queue_slots = queue_slots;
        self
    }

    pub const fn with_bandwidth_tokens(mut self, bandwidth_tokens: u64) -> Self {
        self.bandwidth_tokens = bandwidth_tokens;
        self
    }

    pub const fn with_flush_permits(mut self, flush_permits: u64) -> Self {
        self.flush_permits = flush_permits;
        self
    }

    pub const fn with_sync_debt(mut self, sync_debt: u64) -> Self {
        self.sync_debt = sync_debt;
        self
    }

    pub const fn with_read_ahead_windows(mut self, read_ahead_windows: u64) -> Self {
        self.read_ahead_windows = read_ahead_windows;
        self
    }

    pub const fn with_write_back_windows(mut self, write_back_windows: u64) -> Self {
        self.write_back_windows = write_back_windows;
        self
    }

    pub const fn with_dirty_page_budget(mut self, dirty_page_budget: u64) -> Self {
        self.dirty_page_budget = dirty_page_budget;
        self
    }

    pub const fn with_worker_permits(mut self, worker_permits: u64) -> Self {
        self.worker_permits = worker_permits;
        self
    }

    pub const fn with_cache_residency_hints(mut self, cache_residency_hints: u64) -> Self {
        self.cache_residency_hints = cache_residency_hints;
        self
    }

    pub const fn with_reclaim_permits(mut self, reclaim_permits: u64) -> Self {
        self.reclaim_permits = reclaim_permits;
        self
    }

    pub const fn queue_slots(self) -> u64 {
        self.queue_slots
    }

    pub const fn bandwidth_tokens(self) -> u64 {
        self.bandwidth_tokens
    }

    pub const fn flush_permits(self) -> u64 {
        self.flush_permits
    }

    pub const fn sync_debt(self) -> u64 {
        self.sync_debt
    }

    pub const fn read_ahead_windows(self) -> u64 {
        self.read_ahead_windows
    }

    pub const fn write_back_windows(self) -> u64 {
        self.write_back_windows
    }

    pub const fn dirty_page_budget(self) -> u64 {
        self.dirty_page_budget
    }

    pub const fn worker_permits(self) -> u64 {
        self.worker_permits
    }

    pub const fn cache_residency_hints(self) -> u64 {
        self.cache_residency_hints
    }

    pub const fn reclaim_permits(self) -> u64 {
        self.reclaim_permits
    }
}
