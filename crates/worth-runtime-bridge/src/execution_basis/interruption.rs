use worth_signal::facade::{
    ClockAdvanceRequest, ClockDomain, ClockTick, ResourceCancellationReason,
    ResourceRejectionReason,
};

use crate::facade::RuntimeBridge;
use crate::source::with_async_request_signal_runtime;

use super::BridgeBoundExecutionBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeManagedExecutionCancellationReason {
    HostRequested,
    RuntimePolicy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeManagedExecutionCancellation {
    reason: BridgeManagedExecutionCancellationReason,
    cancellation_ordinal: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeManagedExecutionRejectionReason {
    HostFailure,
    SemanticFailure,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeManagedExecutionRejection {
    reason: BridgeManagedExecutionRejectionReason,
    rejection_ordinal: u64,
}

impl BridgeManagedExecutionRejection {
    pub const fn reason(&self) -> BridgeManagedExecutionRejectionReason {
        self.reason
    }

    pub const fn rejection_ordinal(&self) -> u64 {
        self.rejection_ordinal
    }
}

impl BridgeManagedExecutionCancellation {
    pub const fn reason(&self) -> BridgeManagedExecutionCancellationReason {
        self.reason
    }

    pub const fn cancellation_ordinal(&self) -> u64 {
        self.cancellation_ordinal
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeManagedExecutionInterruptionFailureKind {
    SignalRuntimeThreadAffinityViolation,
    SignalCancellationFailed,
    SignalCancellationDenied,
    SignalRejectionFailed,
    SignalRejectionDenied,
    SignalRequestMismatch,
    SignalClockAdvanceFailed,
    SignalRequestMissing,
    SignalTimeoutWakeMissing,
    SignalTimeoutWakeNotReady,
    SignalTimeoutDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeManagedExecutionInterruptionFailure {
    kind: BridgeManagedExecutionInterruptionFailureKind,
    detail: String,
}

impl BridgeManagedExecutionInterruptionFailure {
    pub const fn kind(&self) -> BridgeManagedExecutionInterruptionFailureKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeManagedExecutionClockAdvance {
    tick_millis: u64,
}

impl BridgeManagedExecutionClockAdvance {
    pub const fn tick_millis(&self) -> u64 {
        self.tick_millis
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeManagedExecutionTimeout {
    timeout_ordinal: u64,
    timeout_wake_identity: u64,
}

impl BridgeManagedExecutionTimeout {
    pub const fn timeout_ordinal(&self) -> u64 {
        self.timeout_ordinal
    }

    pub const fn timeout_wake_identity(&self) -> u64 {
        self.timeout_wake_identity
    }
}

impl BridgeBoundExecutionBasis {
    pub fn request_cancellation(
        &self,
        reason: BridgeManagedExecutionCancellationReason,
    ) -> Result<BridgeManagedExecutionCancellation, BridgeManagedExecutionInterruptionFailure> {
        let report = with_async_request_signal_runtime(self.bridge_runtime_key, |runtime| {
            runtime.cancel_resource_request(self.request.request_handle(), lower_reason(reason))
        })
        .map_err(thread_affinity_failure)?
        .map_err(|error| {
            failure(
                BridgeManagedExecutionInterruptionFailureKind::SignalCancellationFailed,
                error.to_string(),
            )
        })?;
        let cancelled = report.cancelled_request().ok_or_else(|| {
            failure(
                BridgeManagedExecutionInterruptionFailureKind::SignalCancellationDenied,
                "Signal denied cancellation of the exact managed execution request",
            )
        })?;
        if cancelled.handle() != self.request.request_handle() {
            return Err(failure(
                BridgeManagedExecutionInterruptionFailureKind::SignalRequestMismatch,
                "Signal cancellation evidence belongs to another managed execution request",
            ));
        }
        Ok(BridgeManagedExecutionCancellation {
            reason,
            cancellation_ordinal: cancelled.cancellation_ordinal().get(),
        })
    }

    pub fn admit_ready_timeout(
        &self,
    ) -> Result<BridgeManagedExecutionTimeout, BridgeManagedExecutionInterruptionFailure> {
        with_async_request_signal_runtime(self.bridge_runtime_key, |runtime| {
            let wake = runtime
                .in_flight_resource_request(self.request.request_handle())
                .ok_or_else(|| {
                    failure(
                        BridgeManagedExecutionInterruptionFailureKind::SignalRequestMissing,
                        "managed execution request is no longer retained by Signal",
                    )
                })?
                .timeout_wake_id()
                .ok_or_else(|| {
                    failure(
                        BridgeManagedExecutionInterruptionFailureKind::SignalTimeoutWakeMissing,
                        "managed execution request has no admitted timeout wake",
                    )
                })?;
            let ready = runtime.promote_temporal_wake_ready(wake).map_err(|error| {
                failure(
                    BridgeManagedExecutionInterruptionFailureKind::SignalTimeoutWakeNotReady,
                    format!("{error:?}"),
                )
            })?;
            let report = runtime
                .admit_resource_timeout(self.request.request_handle(), ready)
                .map_err(|error| {
                    failure(
                        BridgeManagedExecutionInterruptionFailureKind::SignalTimeoutDenied,
                        error.to_string(),
                    )
                })?;
            let timed_out = report.timed_out_request().ok_or_else(|| {
                failure(
                    BridgeManagedExecutionInterruptionFailureKind::SignalTimeoutDenied,
                    "Signal denied timeout of the exact managed execution request",
                )
            })?;
            if timed_out.handle() != self.request.request_handle() {
                return Err(failure(
                    BridgeManagedExecutionInterruptionFailureKind::SignalRequestMismatch,
                    "Signal timeout evidence belongs to another managed execution request",
                ));
            }
            Ok(BridgeManagedExecutionTimeout {
                timeout_ordinal: timed_out.timeout_ordinal().get(),
                timeout_wake_identity: wake.get(),
            })
        })
        .map_err(thread_affinity_failure)?
    }

    pub fn reject_execution(
        &self,
        reason: BridgeManagedExecutionRejectionReason,
    ) -> Result<BridgeManagedExecutionRejection, BridgeManagedExecutionInterruptionFailure> {
        let report = with_async_request_signal_runtime(self.bridge_runtime_key, |runtime| {
            runtime.reject_resource_request(
                self.request.request_handle(),
                lower_rejection_reason(reason),
            )
        })
        .map_err(thread_affinity_failure)?
        .map_err(|error| {
            failure(
                BridgeManagedExecutionInterruptionFailureKind::SignalRejectionFailed,
                error.to_string(),
            )
        })?;
        let rejected = report.rejected_request().ok_or_else(|| {
            failure(
                BridgeManagedExecutionInterruptionFailureKind::SignalRejectionDenied,
                "Signal denied rejection of the exact managed execution request",
            )
        })?;
        if rejected.clone().handle() != self.request.request_handle() {
            return Err(failure(
                BridgeManagedExecutionInterruptionFailureKind::SignalRequestMismatch,
                "Signal rejection evidence belongs to another managed execution request",
            ));
        }
        Ok(BridgeManagedExecutionRejection {
            reason,
            rejection_ordinal: rejected.rejection_ordinal().get(),
        })
    }
}

impl RuntimeBridge {
    pub fn advance_managed_execution_clock(
        &self,
        tick_millis: u64,
    ) -> Result<BridgeManagedExecutionClockAdvance, BridgeManagedExecutionInterruptionFailure> {
        with_async_request_signal_runtime(self.signal_runtime_key, |runtime| {
            runtime.advance_clock(ClockAdvanceRequest::new(
                ClockDomain::MonotonicExecution,
                ClockTick::new(tick_millis),
            ))
        })
        .map_err(thread_affinity_failure)?
        .map_err(|error| {
            failure(
                BridgeManagedExecutionInterruptionFailureKind::SignalClockAdvanceFailed,
                error.to_string(),
            )
        })?;
        Ok(BridgeManagedExecutionClockAdvance { tick_millis })
    }
}

const fn lower_reason(
    reason: BridgeManagedExecutionCancellationReason,
) -> ResourceCancellationReason {
    match reason {
        BridgeManagedExecutionCancellationReason::HostRequested => {
            ResourceCancellationReason::HostRequested
        }
        BridgeManagedExecutionCancellationReason::RuntimePolicy => {
            ResourceCancellationReason::RuntimePolicy
        }
    }
}

const fn lower_rejection_reason(
    reason: BridgeManagedExecutionRejectionReason,
) -> ResourceRejectionReason {
    match reason {
        BridgeManagedExecutionRejectionReason::HostFailure => ResourceRejectionReason::HostFailure,
        BridgeManagedExecutionRejectionReason::SemanticFailure => {
            ResourceRejectionReason::SemanticFailure
        }
    }
}

fn thread_affinity_failure(
    error: crate::source::SignalRuntimeThreadAffinityError,
) -> BridgeManagedExecutionInterruptionFailure {
    failure(
        BridgeManagedExecutionInterruptionFailureKind::SignalRuntimeThreadAffinityViolation,
        format!(
            "bridge Signal runtime {} belongs to thread {:?}, not {:?}",
            error.runtime_key(),
            error.owner(),
            error.current()
        ),
    )
}

fn failure(
    kind: BridgeManagedExecutionInterruptionFailureKind,
    detail: impl Into<String>,
) -> BridgeManagedExecutionInterruptionFailure {
    BridgeManagedExecutionInterruptionFailure {
        kind,
        detail: detail.into(),
    }
}
