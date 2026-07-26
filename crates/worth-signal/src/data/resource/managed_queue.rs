use serde::{Deserialize, Serialize};

use super::{ResourceAttemptId, ResourceRequestHandle, ResourceRequestId};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ResourceQueuePressureClass {
    Available,
    Saturated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceQueuePressureObservation {
    queue_depth: u64,
    queue_capacity: u64,
    class: ResourceQueuePressureClass,
}

impl ResourceQueuePressureObservation {
    pub(crate) const fn from_managed_state(queue_depth: u64, queue_capacity: u64) -> Self {
        Self {
            queue_depth,
            queue_capacity,
            class: if queue_depth == queue_capacity {
                ResourceQueuePressureClass::Saturated
            } else {
                ResourceQueuePressureClass::Available
            },
        }
    }

    pub const fn queue_depth(self) -> u64 {
        self.queue_depth
    }

    pub const fn queue_capacity(self) -> u64 {
        self.queue_capacity
    }

    pub const fn class(self) -> ResourceQueuePressureClass {
        self.class
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ResourceManagedQueueBinding {
    request: ResourceRequestHandle,
    attempt: ResourceAttemptId,
    queue_capacity: u64,
}

impl ResourceManagedQueueBinding {
    pub(crate) const fn new(
        request: ResourceRequestHandle,
        attempt: ResourceAttemptId,
        queue_capacity: u64,
    ) -> Self {
        Self {
            request,
            attempt,
            queue_capacity,
        }
    }

    pub const fn request(&self) -> ResourceRequestHandle {
        self.request
    }

    pub const fn attempt(&self) -> ResourceAttemptId {
        self.attempt
    }

    pub const fn queue_capacity(&self) -> u64 {
        self.queue_capacity
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct ResourceManagedQueueState {
    queue_depth: u64,
    queue_capacity: u64,
}

impl ResourceManagedQueueState {
    pub(crate) fn new(queue_capacity: u64) -> Result<Self, ResourceManagedQueueDenialClass> {
        if queue_capacity == 0 {
            return Err(ResourceManagedQueueDenialClass::ZeroCapacity);
        }
        Ok(Self {
            queue_depth: 0,
            queue_capacity,
        })
    }

    pub(crate) const fn queue_capacity(self) -> u64 {
        self.queue_capacity
    }

    pub(crate) const fn queue_depth(self) -> u64 {
        self.queue_depth
    }

    pub(crate) const fn is_empty(self) -> bool {
        self.queue_depth == 0
    }

    pub(crate) const fn pressure(self) -> ResourceQueuePressureObservation {
        ResourceQueuePressureObservation::from_managed_state(self.queue_depth, self.queue_capacity)
    }

    pub(crate) fn enqueue(&mut self, width: u64) -> Result<(), ResourceManagedQueueDenialClass> {
        if width == 0 {
            return Err(ResourceManagedQueueDenialClass::ZeroMutationWidth);
        }
        let Some(next_depth) = self.queue_depth.checked_add(width) else {
            return Err(ResourceManagedQueueDenialClass::CapacityExceeded);
        };
        if next_depth > self.queue_capacity {
            return Err(ResourceManagedQueueDenialClass::CapacityExceeded);
        }
        self.queue_depth = next_depth;
        Ok(())
    }

    pub(crate) fn dequeue(&mut self, width: u64) -> Result<(), ResourceManagedQueueDenialClass> {
        if width == 0 {
            return Err(ResourceManagedQueueDenialClass::ZeroMutationWidth);
        }
        self.queue_depth = self
            .queue_depth
            .checked_sub(width)
            .ok_or(ResourceManagedQueueDenialClass::DepthUnderflow)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ResourceManagedQueueMutationKind {
    Enqueued,
    Dequeued,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceManagedQueueCounters {
    exact_request_lookup_count: usize,
    queue_state_mutation_count: usize,
}

impl ResourceManagedQueueCounters {
    pub(crate) const fn none() -> Self {
        Self {
            exact_request_lookup_count: 0,
            queue_state_mutation_count: 0,
        }
    }

    pub(crate) const fn exact_lookup(queue_state_mutation_count: usize) -> Self {
        Self {
            exact_request_lookup_count: 1,
            queue_state_mutation_count,
        }
    }

    pub const fn exact_request_lookup_count(self) -> usize {
        self.exact_request_lookup_count
    }

    pub const fn queue_state_mutation_count(self) -> usize {
        self.queue_state_mutation_count
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ResourceManagedQueueDenialClass {
    ZeroCapacity,
    RequestUnavailable,
    RequestNotActive,
    QueueAlreadyBound,
    QueueUnavailable,
    BindingMismatch,
    ZeroMutationWidth,
    CapacityExceeded,
    DepthUnderflow,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceManagedQueueDenial {
    request_id: ResourceRequestId,
    class: ResourceManagedQueueDenialClass,
    counters: ResourceManagedQueueCounters,
}

impl ResourceManagedQueueDenial {
    pub(crate) const fn new(
        request_id: ResourceRequestId,
        class: ResourceManagedQueueDenialClass,
        counters: ResourceManagedQueueCounters,
    ) -> Self {
        Self {
            request_id,
            class,
            counters,
        }
    }

    pub const fn request_id(&self) -> ResourceRequestId {
        self.request_id
    }

    pub const fn class(&self) -> ResourceManagedQueueDenialClass {
        self.class
    }

    pub const fn counters(&self) -> ResourceManagedQueueCounters {
        self.counters
    }

    pub const fn detail(&self) -> &'static str {
        match self.class {
            ResourceManagedQueueDenialClass::ZeroCapacity => {
                "managed resource queue capacity must be nonzero"
            }
            ResourceManagedQueueDenialClass::RequestUnavailable => {
                "managed resource queue request is unavailable"
            }
            ResourceManagedQueueDenialClass::RequestNotActive => {
                "managed resource queue request is not active"
            }
            ResourceManagedQueueDenialClass::QueueAlreadyBound => {
                "resource request already owns a managed queue"
            }
            ResourceManagedQueueDenialClass::QueueUnavailable => {
                "resource request has no managed queue"
            }
            ResourceManagedQueueDenialClass::BindingMismatch => {
                "managed resource queue binding does not match request state"
            }
            ResourceManagedQueueDenialClass::ZeroMutationWidth => {
                "managed resource queue mutation width must be nonzero"
            }
            ResourceManagedQueueDenialClass::CapacityExceeded => {
                "managed resource queue capacity would be exceeded"
            }
            ResourceManagedQueueDenialClass::DepthUnderflow => {
                "managed resource queue depth would underflow"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceManagedQueueMutationReport {
    request: ResourceRequestHandle,
    kind: ResourceManagedQueueMutationKind,
    pressure: ResourceQueuePressureObservation,
    counters: ResourceManagedQueueCounters,
}

impl ResourceManagedQueueMutationReport {
    pub(crate) const fn new(
        request: ResourceRequestHandle,
        kind: ResourceManagedQueueMutationKind,
        pressure: ResourceQueuePressureObservation,
        counters: ResourceManagedQueueCounters,
    ) -> Self {
        Self {
            request,
            kind,
            pressure,
            counters,
        }
    }

    pub const fn request(&self) -> ResourceRequestHandle {
        self.request
    }

    pub const fn kind(&self) -> ResourceManagedQueueMutationKind {
        self.kind
    }

    pub const fn pressure(&self) -> ResourceQueuePressureObservation {
        self.pressure
    }

    pub const fn counters(&self) -> ResourceManagedQueueCounters {
        self.counters
    }
}
