use crate::native::presentation::{UiNativePresentationFault, UiNativePresentationRecoveryClass};
use crate::native::{UiNativeEffectPosture, UiNativePresentationEffectPhase};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeProtocolClosePoint {
    PreparedUpload,
    Prepared,
    SurfaceAcquired,
    Encoded,
    Submitted,
    PresentHandoff,
    Readback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeProtocolReadback {
    Complete,
    PendingThenComplete,
    Indeterminate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeProtocolSurfaceTransition {
    ZeroSized,
    Minimized,
    Resize,
    Dpi,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeProtocolNextAction {
    Complete,
    RetryAfterTimeout,
    WaitForVisibility,
    RejectValidation,
    Reconstruct(UiNativePresentationRecoveryClass),
    Closed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeProtocolPredecessor {
    Retained,
    Replaced,
    Released,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeProtocolCloseDisposition {
    Open,
    ClosedAt(UiNativeProtocolClosePoint),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiNativeProtocolResourceCensus {
    pub queued_readiness: usize,
    pub prepared_uploads: usize,
    pub surfaces: usize,
    pub devices: usize,
    pub queues: usize,
    pub pending_presentations: usize,
    pub readbacks: usize,
    pub reconstruction_requirements: usize,
}

impl UiNativeProtocolResourceCensus {
    pub const fn is_zero(self) -> bool {
        self.queued_readiness == 0
            && self.prepared_uploads == 0
            && self.surfaces == 0
            && self.devices == 0
            && self.queues == 0
            && self.pending_presentations == 0
            && self.readbacks == 0
            && self.reconstruction_requirements == 0
    }

    pub(in crate::native::lifecycle) fn max(self, other: Self) -> Self {
        Self {
            queued_readiness: self.queued_readiness.max(other.queued_readiness),
            prepared_uploads: self.prepared_uploads.max(other.prepared_uploads),
            surfaces: self.surfaces.max(other.surfaces),
            devices: self.devices.max(other.devices),
            queues: self.queues.max(other.queues),
            pending_presentations: self.pending_presentations.max(other.pending_presentations),
            readbacks: self.readbacks.max(other.readbacks),
            reconstruction_requirements: self
                .reconstruction_requirements
                .max(other.reconstruction_requirements),
        }
    }

    pub(in crate::native::lifecycle) fn from_registry(
        census: crate::native::UiNativeResourceCensus,
        reconstruction_requirements: usize,
    ) -> Self {
        Self {
            queued_readiness: census.event_wake_registrations,
            prepared_uploads: census.atlas_staging_buffers,
            surfaces: census.surfaces,
            devices: census.devices,
            queues: census.queues,
            pending_presentations: census.pending_presentations,
            readbacks: census.readback_buffers,
            reconstruction_requirements,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeLifecycleProtocolSchedule {
    queued_readiness: bool,
    close_at: Option<UiNativeProtocolClosePoint>,
    acquisition_fault: Option<UiNativePresentationFault>,
    readback: UiNativeProtocolReadback,
    recover: bool,
    resume_after_recovery: bool,
    recovery_bindings: usize,
    surface_transition: Option<UiNativeProtocolSurfaceTransition>,
}

impl UiNativeLifecycleProtocolSchedule {
    pub const fn ordinary() -> Self {
        Self {
            queued_readiness: false,
            close_at: None,
            acquisition_fault: None,
            readback: UiNativeProtocolReadback::Complete,
            recover: false,
            resume_after_recovery: false,
            recovery_bindings: 1,
            surface_transition: None,
        }
    }

    pub const fn with_queued_readiness(mut self) -> Self {
        self.queued_readiness = true;
        self
    }

    pub const fn close_at(mut self, close_at: UiNativeProtocolClosePoint) -> Self {
        self.close_at = Some(close_at);
        self
    }

    pub const fn fault(mut self, fault: UiNativePresentationFault) -> Self {
        self.acquisition_fault = Some(fault);
        self
    }

    pub const fn readback(mut self, readback: UiNativeProtocolReadback) -> Self {
        self.readback = readback;
        self
    }

    pub const fn recover_and_resume(mut self) -> Self {
        self.recover = true;
        self.resume_after_recovery = true;
        self
    }

    pub const fn with_recovery_bindings(mut self, bindings: usize) -> Self {
        self.recovery_bindings = bindings;
        self
    }

    pub const fn surface_transition(
        mut self,
        transition: UiNativeProtocolSurfaceTransition,
    ) -> Self {
        self.surface_transition = Some(transition);
        self
    }

    pub const fn queued_readiness(self) -> bool {
        self.queued_readiness
    }

    pub const fn close_point(self) -> Option<UiNativeProtocolClosePoint> {
        self.close_at
    }

    pub const fn acquisition_fault(self) -> Option<UiNativePresentationFault> {
        self.acquisition_fault
    }

    pub const fn readback_posture(self) -> UiNativeProtocolReadback {
        self.readback
    }

    pub const fn recovers(self) -> bool {
        self.recover
    }

    pub const fn resumes_after_recovery(self) -> bool {
        self.resume_after_recovery
    }

    pub const fn recovery_bindings(self) -> usize {
        self.recovery_bindings
    }

    pub const fn scheduled_surface_transition(self) -> Option<UiNativeProtocolSurfaceTransition> {
        self.surface_transition
    }
}

#[must_use = "lifecycle reports carry the terminal posture and resource census"]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativeLifecycleProtocolReport {
    pub(in crate::native::lifecycle) effect_posture: UiNativeEffectPosture,
    pub(in crate::native::lifecycle) completed_stages: Box<[UiNativePresentationEffectPhase]>,
    pub(in crate::native::lifecycle) next_action: UiNativeProtocolNextAction,
    pub(in crate::native::lifecycle) predecessor: UiNativeProtocolPredecessor,
    pub(in crate::native::lifecycle) close: UiNativeProtocolCloseDisposition,
    pub(in crate::native::lifecycle) recovery_binding: Option<u64>,
    pub(in crate::native::lifecycle) reconstructed_bindings: usize,
    pub(in crate::native::lifecycle) device_generation: u64,
    pub(in crate::native::lifecycle) surface_generation: u64,
    pub(in crate::native::lifecycle) pending_readback_observed: bool,
    pub(in crate::native::lifecycle) peak: UiNativeProtocolResourceCensus,
    pub(in crate::native::lifecycle) terminal: UiNativeProtocolResourceCensus,
}

impl UiNativeLifecycleProtocolReport {
    pub const fn effect_posture(&self) -> UiNativeEffectPosture {
        self.effect_posture
    }

    pub fn completed_stages(&self) -> &[UiNativePresentationEffectPhase] {
        &self.completed_stages
    }

    pub const fn next_action(&self) -> UiNativeProtocolNextAction {
        self.next_action
    }

    pub const fn predecessor(&self) -> UiNativeProtocolPredecessor {
        self.predecessor
    }

    pub const fn close_disposition(&self) -> UiNativeProtocolCloseDisposition {
        self.close
    }

    pub const fn recovery_binding(&self) -> Option<u64> {
        self.recovery_binding
    }

    pub const fn reconstructed_bindings(&self) -> usize {
        self.reconstructed_bindings
    }

    pub const fn device_generation(&self) -> u64 {
        self.device_generation
    }

    pub const fn surface_generation(&self) -> u64 {
        self.surface_generation
    }

    pub const fn pending_readback_observed(&self) -> bool {
        self.pending_readback_observed
    }

    pub const fn peak_census(&self) -> UiNativeProtocolResourceCensus {
        self.peak
    }

    pub const fn terminal_census(&self) -> UiNativeProtocolResourceCensus {
        self.terminal
    }
}
