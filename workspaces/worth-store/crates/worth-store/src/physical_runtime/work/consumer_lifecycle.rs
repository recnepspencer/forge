use worth_signal::facade::{ResourceCancellationReport, ResourceRequestHandle};
use worth_signal::facade::{
    ResourceRetryAdmissionReport, ResourceRetryScheduleReport, ResourceSupersessionRecord,
    ResourceTimeoutReport, ScheduledResourceRetry,
};

use super::{PhysicalSignalAspectBindingDigest, PhysicalWorkIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkConsumerHandle {
    identity: PhysicalWorkIdentity,
    signal_request: ResourceRequestHandle,
    route: PhysicalSignalAspectBindingDigest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalEffectObligation {
    NotDispatched,
    SettlementContinues,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkCancellationJoin {
    signal: ResourceCancellationReport,
    obligation: PhysicalEffectObligation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkCancellationFailure {
    DerivedStateUnavailable,
}

pub struct PhysicalWorkRetrySchedule {
    identity: PhysicalWorkIdentity,
    route: PhysicalSignalAspectBindingDigest,
    scheduled: ScheduledResourceRetry,
}

pub enum PhysicalWorkRetryScheduleOutcome {
    Scheduled(PhysicalWorkRetrySchedule),
    Denied(ResourceRetryScheduleReport),
}

pub struct PhysicalWorkRetryAdmission {
    identity: PhysicalWorkIdentity,
    route: PhysicalSignalAspectBindingDigest,
    signal: ResourceRetryAdmissionReport,
    ready: super::ReadyPhysicalWork,
    command: super::PhysicalRetryCommand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkRetryFailure {
    EffectNotProvenSafe,
    DerivedStateUnavailable,
    RetryWakeNotReady,
    SignalDenied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkTimeoutJoin {
    signal: ResourceTimeoutReport,
    obligation: PhysicalEffectObligation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkSupersessionJoin {
    signal: ResourceSupersessionRecord,
    previous_obligation: PhysicalEffectObligation,
}

impl PhysicalWorkConsumerHandle {
    pub(in crate::physical_runtime) const fn new(
        identity: PhysicalWorkIdentity,
        signal_request: ResourceRequestHandle,
        route: PhysicalSignalAspectBindingDigest,
    ) -> Self {
        Self {
            identity,
            signal_request,
            route,
        }
    }

    pub const fn identity(self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub const fn signal_request(self) -> ResourceRequestHandle {
        self.signal_request
    }

    pub(in crate::physical_runtime) const fn route(self) -> PhysicalSignalAspectBindingDigest {
        self.route
    }
}

impl PhysicalWorkCancellationJoin {
    pub(in crate::physical_runtime) fn new(
        signal: ResourceCancellationReport,
        obligation: PhysicalEffectObligation,
    ) -> Self {
        Self { signal, obligation }
    }

    pub const fn signal(&self) -> &ResourceCancellationReport {
        &self.signal
    }

    pub const fn obligation(&self) -> PhysicalEffectObligation {
        self.obligation
    }
}

impl PhysicalWorkRetrySchedule {
    pub(in crate::physical_runtime) const fn new(
        identity: PhysicalWorkIdentity,
        route: PhysicalSignalAspectBindingDigest,
        scheduled: ScheduledResourceRetry,
    ) -> Self {
        Self {
            identity,
            route,
            scheduled,
        }
    }

    pub const fn identity(&self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub const fn scheduled(&self) -> &ScheduledResourceRetry {
        &self.scheduled
    }

    pub(in crate::physical_runtime) const fn route(&self) -> PhysicalSignalAspectBindingDigest {
        self.route
    }
}

impl PhysicalWorkRetryAdmission {
    pub(in crate::physical_runtime) const fn new(
        identity: PhysicalWorkIdentity,
        route: PhysicalSignalAspectBindingDigest,
        signal: ResourceRetryAdmissionReport,
        ready: super::ReadyPhysicalWork,
        command: super::PhysicalRetryCommand,
    ) -> Self {
        Self {
            identity,
            route,
            signal,
            ready,
            command,
        }
    }

    pub const fn identity(&self) -> PhysicalWorkIdentity {
        self.identity
    }

    pub const fn signal(&self) -> &ResourceRetryAdmissionReport {
        &self.signal
    }

    pub fn consumer_handle(&self) -> Option<PhysicalWorkConsumerHandle> {
        self.signal.admitted_retry().map(|retry| {
            PhysicalWorkConsumerHandle::new(
                self.identity,
                retry.admitted_request().handle(),
                self.route,
            )
        })
    }

    pub fn into_parts(
        self,
    ) -> (
        super::ReadyPhysicalWork,
        super::PhysicalRetryCommand,
        ResourceRetryAdmissionReport,
    ) {
        (self.ready, self.command, self.signal)
    }
}

impl PhysicalWorkTimeoutJoin {
    pub(in crate::physical_runtime) fn new(
        signal: ResourceTimeoutReport,
        obligation: PhysicalEffectObligation,
    ) -> Self {
        Self { signal, obligation }
    }

    pub const fn signal(&self) -> &ResourceTimeoutReport {
        &self.signal
    }

    pub const fn obligation(&self) -> PhysicalEffectObligation {
        self.obligation
    }
}

impl PhysicalWorkSupersessionJoin {
    pub(in crate::physical_runtime) const fn before_dispatch(
        signal: ResourceSupersessionRecord,
    ) -> Self {
        Self {
            signal,
            previous_obligation: PhysicalEffectObligation::NotDispatched,
        }
    }

    pub const fn signal(&self) -> &ResourceSupersessionRecord {
        &self.signal
    }

    pub const fn previous_obligation(&self) -> PhysicalEffectObligation {
        self.previous_obligation
    }
}
