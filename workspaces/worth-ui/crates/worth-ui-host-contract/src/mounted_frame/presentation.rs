#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiMountedEffectFamily {
    RecordedProjection,
    NativePaint,
    Accessibility,
    Focus,
    CanvasSpatial,
    Realtime,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedCompletedEffects {
    families: Box<[UiMountedEffectFamily]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedSurfacePresentationCompletion {
    mode: crate::UiHostSurfacePresentationMode,
    effects: UiMountedCompletedEffects,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostSurfacePresentationDenial {
    AdapterDeclined,
    CancelledBeforeEffects,
    UnsupportedPresentationMode(crate::UiHostSurfacePresentationMode),
    UnsupportedEffect(UiMountedEffectFamily),
    Protocol(crate::UiHostProtocolDenial),
    ProtocolChanged,
    CapabilityGenerationChanged,
    CapabilityProfileChanged,
    SurfaceBindingChanged,
    MalformedProjection,
    DeadlineExpired,
    CapacityExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPresentationDeadline {
    tick: u64,
}

pub struct UiMountedFrameConsumptionView<'frame> {
    authority: Rc<()>,
    host_session_identity: u64,
    protocol: crate::UiHostProtocolAgreement,
    capability_generation: crate::WorthUiHostCapabilityObservationGeneration,
    capability_profile_digest: u64,
    attempt: crate::UiMountedPresentationAttemptIdentity,
    deadline: UiPresentationDeadline,
    requirement: crate::UiMountedSurfaceBindingRequirement,
    projection: &'frame crate::UiMountedProjectionView,
}

#[doc(hidden)]
pub struct UiMountedFrameConsumptionInput<'frame> {
    pub host_session_identity: u64,
    pub protocol: crate::UiHostProtocolAgreement,
    pub capability_generation: crate::WorthUiHostCapabilityObservationGeneration,
    pub capability_profile_digest: u64,
    pub attempt: crate::UiMountedPresentationAttemptIdentity,
    pub deadline: UiPresentationDeadline,
    pub requirement: crate::UiMountedSurfaceBindingRequirement,
    pub projection: &'frame crate::UiMountedProjectionView,
}

pub struct UiMountedPresentationLease {
    seal: Rc<()>,
    active: Weak<RefCell<Option<Weak<()>>>>,
}

#[derive(Clone, Default)]
pub struct UiMountedPresentationLeaseGate {
    active: Rc<RefCell<Option<Weak<()>>>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPresentationLeaseDenial {
    AlreadyBound,
    Unsupported,
}

pub struct UiHostPresentationCompletionToken {
    identity: u64,
    authority: Rc<()>,
}

impl UiMountedPresentationLease {
    #[doc(hidden)]
    pub fn open<'frame>(
        &self,
        input: UiMountedFrameConsumptionInput<'frame>,
    ) -> UiMountedFrameConsumptionView<'frame> {
        UiMountedFrameConsumptionView {
            authority: Rc::clone(&self.seal),
            host_session_identity: input.host_session_identity,
            protocol: input.protocol,
            capability_generation: input.capability_generation,
            capability_profile_digest: input.capability_profile_digest,
            attempt: input.attempt,
            deadline: input.deadline,
            requirement: input.requirement,
            projection: input.projection,
        }
    }
}

impl Drop for UiMountedPresentationLease {
    fn drop(&mut self) {
        let Some(active) = self.active.upgrade() else {
            return;
        };
        let mut active = active.borrow_mut();
        let matches = active
            .as_ref()
            .and_then(Weak::upgrade)
            .is_some_and(|current| Rc::ptr_eq(&current, &self.seal));
        if matches {
            *active = None;
        }
    }
}

impl UiMountedPresentationLeaseGate {
    pub fn claim(&self) -> Result<UiMountedPresentationLease, UiMountedPresentationLeaseDenial> {
        let mut active = self.active.borrow_mut();
        if active.as_ref().is_some_and(|seal| seal.upgrade().is_some()) {
            return Err(UiMountedPresentationLeaseDenial::AlreadyBound);
        }
        let seal = Rc::new(());
        *active = Some(Rc::downgrade(&seal));
        Ok(UiMountedPresentationLease {
            seal,
            active: Rc::downgrade(&self.active),
        })
    }

    pub fn admits(&self, view: &UiMountedFrameConsumptionView<'_>) -> bool {
        self.active
            .borrow()
            .as_ref()
            .and_then(Weak::upgrade)
            .is_some_and(|active| Rc::ptr_eq(&active, &view.authority))
    }

    pub fn admits_token(&self, token: &UiHostPresentationCompletionToken) -> bool {
        self.active
            .borrow()
            .as_ref()
            .and_then(Weak::upgrade)
            .is_some_and(|active| Rc::ptr_eq(&active, &token.authority))
    }
}

impl UiMountedFrameConsumptionView<'_> {
    pub fn host_session_identity(&self) -> u64 {
        self.host_session_identity
    }

    pub fn protocol(&self) -> crate::UiHostProtocolAgreement {
        self.protocol
    }

    pub fn capability_generation(&self) -> crate::WorthUiHostCapabilityObservationGeneration {
        self.capability_generation
    }

    pub fn capability_profile_digest(&self) -> u64 {
        self.capability_profile_digest
    }

    pub fn attempt(&self) -> crate::UiMountedPresentationAttemptIdentity {
        self.attempt
    }

    pub fn deadline(&self) -> UiPresentationDeadline {
        self.deadline
    }

    pub fn requirement(&self) -> crate::UiMountedSurfaceBindingRequirement {
        self.requirement
    }

    pub fn projection(&self) -> &crate::UiMountedProjectionView {
        self.projection
    }

    pub fn issue_completion_token(&self) -> UiHostPresentationCompletionToken {
        static NEXT_TOKEN: AtomicU64 = AtomicU64::new(1);
        let identity = NEXT_TOKEN
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .expect("presentation completion token identity exhausted");
        UiHostPresentationCompletionToken {
            identity,
            authority: Rc::clone(&self.authority),
        }
    }
}

impl UiHostPresentationCompletionToken {
    pub fn diagnostic_value(&self) -> u64 {
        self.identity
    }
}

impl std::fmt::Debug for UiHostPresentationCompletionToken {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiHostPresentationCompletionToken")
            .field("identity", &self.identity)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub enum UiHostSurfacePresentationOutcome {
    RejectedBeforeEffects(UiHostSurfacePresentationDenial),
    Presented(UiMountedSurfacePresentationCompletion),
    InFlight(crate::UiHostPresentationCompletionToken),
    PresentationIndeterminate,
}

#[derive(Debug)]
pub enum UiHostSurfaceInFlightCompletion {
    Pending(UiHostPresentationCompletionToken),
    RejectedBeforeEffects(UiHostSurfacePresentationDenial),
    Presented(UiMountedSurfacePresentationCompletion),
    PresentationIndeterminate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostSurfaceCancellationOutcome {
    CancelledBeforeEffects,
    EffectsMayHaveBegun,
}

impl UiMountedCompletedEffects {
    pub fn new(mut families: Vec<UiMountedEffectFamily>) -> Self {
        families.sort();
        families.dedup();
        Self {
            families: families.into_boxed_slice(),
        }
    }

    pub fn families(&self) -> &[UiMountedEffectFamily] {
        &self.families
    }
}

impl UiMountedSurfacePresentationCompletion {
    pub fn new(
        mode: crate::UiHostSurfacePresentationMode,
        effects: UiMountedCompletedEffects,
    ) -> Self {
        Self { mode, effects }
    }

    pub fn mode(&self) -> crate::UiHostSurfacePresentationMode {
        self.mode
    }

    pub fn effects(&self) -> &UiMountedCompletedEffects {
        &self.effects
    }

    pub fn into_effects(self) -> UiMountedCompletedEffects {
        self.effects
    }
}

impl UiPresentationDeadline {
    pub const fn at_tick(tick: u64) -> Self {
        Self { tick }
    }

    pub const fn tick(self) -> u64 {
        self.tick
    }

    pub const fn expired_at(self, now: u64) -> bool {
        now >= self.tick
    }
}
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU64, Ordering};
