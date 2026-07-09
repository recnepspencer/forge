use worth_store_physical_backend::BackendQueueSpeculativeScope;
use worth_store_security::StoreSecurityScopeIdentity;

use super::{QueueBackpressureCause, QueueWorkClass, S6QueueDurabilityClass};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct QueueExecutionObservation {
    pub(crate) queue_depth_sample: u32,
    pub(crate) grouped_writes: u32,
    pub(crate) read_ahead_units: u64,
    pub(crate) read_ahead_scope: Option<BackendQueueSpeculativeScope>,
    pub(crate) write_back_units: u64,
    pub(crate) write_back_scope: Option<BackendQueueSpeculativeScope>,
    pub(crate) mechanical_retries: u64,
    pub(crate) partial_read_events: u64,
    pub(crate) short_write_events: u64,
    pub(crate) backpressure_cause: Option<QueueBackpressureCause>,
    pub(crate) foreground_wait_events: u64,
    pub(crate) attempted_work_class: Option<QueueWorkClass>,
    pub(crate) attempted_durability_class: Option<S6QueueDurabilityClass>,
    pub(crate) attempted_security_scope_identity: Option<StoreSecurityScopeIdentity>,
}

#[cfg(test)]
#[allow(dead_code)]
impl QueueExecutionObservation {
    pub(crate) const fn empty() -> Self {
        Self {
            queue_depth_sample: 0,
            grouped_writes: 0,
            read_ahead_units: 0,
            read_ahead_scope: None,
            write_back_units: 0,
            write_back_scope: None,
            mechanical_retries: 0,
            partial_read_events: 0,
            short_write_events: 0,
            backpressure_cause: None,
            foreground_wait_events: 0,
            attempted_work_class: None,
            attempted_durability_class: None,
            attempted_security_scope_identity: None,
        }
    }

    pub(crate) const fn with_queue_depth_sample(mut self, queue_depth_sample: u32) -> Self {
        self.queue_depth_sample = queue_depth_sample;
        self
    }

    pub(crate) const fn with_read_ahead_units(mut self, read_ahead_units: u64) -> Self {
        self.read_ahead_units = read_ahead_units;
        self
    }

    pub(crate) const fn with_read_ahead_scope(
        mut self,
        read_ahead_scope: BackendQueueSpeculativeScope,
    ) -> Self {
        self.read_ahead_scope = Some(read_ahead_scope);
        self
    }

    pub(crate) const fn with_attempted_durability_class(
        mut self,
        durability_class: S6QueueDurabilityClass,
    ) -> Self {
        self.attempted_durability_class = Some(durability_class);
        self
    }

    pub(crate) const fn with_backpressure(mut self, cause: QueueBackpressureCause) -> Self {
        self.backpressure_cause = Some(cause);
        self
    }

    pub(crate) const fn with_foreground_wait_events(mut self, foreground_wait_events: u64) -> Self {
        self.foreground_wait_events = foreground_wait_events;
        self
    }
}
