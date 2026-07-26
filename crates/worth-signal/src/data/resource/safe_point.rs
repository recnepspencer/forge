use serde::{Deserialize, Serialize};

use crate::data::temporal::TemporalWakeId;

use super::{
    ResourceInFlightStatus, ResourceLifecycleOrdinal, ResourceQueuePressureObservation,
    ResourceRequestHandle, ResourceRequestId,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceSafePointObservationCounters {
    exact_request_lookup_count: usize,
    pressure_classification_count: usize,
}

impl ResourceSafePointObservationCounters {
    pub(crate) const fn exact_request_and_pressure() -> Self {
        Self {
            exact_request_lookup_count: 1,
            pressure_classification_count: 1,
        }
    }

    pub const fn exact_request_lookup_count(self) -> usize {
        self.exact_request_lookup_count
    }

    pub const fn pressure_classification_count(self) -> usize {
        self.pressure_classification_count
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ResourceSafePointObservationOrdinal(u64);

impl ResourceSafePointObservationOrdinal {
    pub(crate) const ZERO: Self = Self(0);

    pub(crate) const fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ResourceSafePointObservationDenialClass {
    RequestUnavailable,
    QueueUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceSafePointObservationDenial {
    request_id: ResourceRequestId,
    class: ResourceSafePointObservationDenialClass,
    counters: ResourceSafePointObservationCounters,
}

impl ResourceSafePointObservationDenial {
    pub(crate) const fn request_unavailable(
        request_id: ResourceRequestId,
        counters: ResourceSafePointObservationCounters,
    ) -> Self {
        Self {
            request_id,
            class: ResourceSafePointObservationDenialClass::RequestUnavailable,
            counters,
        }
    }

    pub(crate) const fn queue_unavailable(
        request_id: ResourceRequestId,
        counters: ResourceSafePointObservationCounters,
    ) -> Self {
        Self {
            request_id,
            class: ResourceSafePointObservationDenialClass::QueueUnavailable,
            counters,
        }
    }

    pub const fn request_id(&self) -> ResourceRequestId {
        self.request_id
    }

    pub const fn class(&self) -> ResourceSafePointObservationDenialClass {
        self.class
    }

    pub const fn counters(&self) -> ResourceSafePointObservationCounters {
        self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceSafePointObservationReport {
    ordinal: ResourceSafePointObservationOrdinal,
    request: ResourceRequestHandle,
    status: ResourceInFlightStatus,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
    pressure: ResourceQueuePressureObservation,
    timeout_wake_id: Option<TemporalWakeId>,
    counters: ResourceSafePointObservationCounters,
}

pub(crate) struct ResourceSafePointObservationEvidence {
    pub request: ResourceRequestHandle,
    pub status: ResourceInFlightStatus,
    pub lifecycle_ordinal: ResourceLifecycleOrdinal,
    pub pressure: ResourceQueuePressureObservation,
    pub timeout_wake_id: Option<TemporalWakeId>,
}

impl ResourceSafePointObservationReport {
    pub(crate) const fn new(
        ordinal: ResourceSafePointObservationOrdinal,
        evidence: ResourceSafePointObservationEvidence,
        counters: ResourceSafePointObservationCounters,
    ) -> Self {
        Self {
            ordinal,
            request: evidence.request,
            status: evidence.status,
            lifecycle_ordinal: evidence.lifecycle_ordinal,
            pressure: evidence.pressure,
            timeout_wake_id: evidence.timeout_wake_id,
            counters,
        }
    }

    pub const fn ordinal(&self) -> ResourceSafePointObservationOrdinal {
        self.ordinal
    }

    pub const fn request(&self) -> ResourceRequestHandle {
        self.request
    }

    pub const fn status(&self) -> ResourceInFlightStatus {
        self.status
    }

    pub const fn lifecycle_ordinal(&self) -> ResourceLifecycleOrdinal {
        self.lifecycle_ordinal
    }

    pub const fn pressure(&self) -> ResourceQueuePressureObservation {
        self.pressure
    }

    pub const fn timeout_wake_id(&self) -> Option<TemporalWakeId> {
        self.timeout_wake_id
    }

    pub const fn counters(&self) -> ResourceSafePointObservationCounters {
        self.counters
    }
}
