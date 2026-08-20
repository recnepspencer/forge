use super::super::{
    UiNativeEffectPosture, UiNativeGraphicsObservation, UiNativePresentationObservation,
    UiNativeResourceCensus, UiNativeRetainedFrameObservation,
};
use super::UiNativeEventLoopCleanup;

mod client_derived_state;
mod client_resources;
mod client_shutdown;
mod readiness_grant;
mod stop_report;
pub use client_derived_state::{
    UiNativeClientDerivedStateLossClass, UiNativeClientDerivedStateReconstructionObservation,
};
pub use client_resources::UiNativeClientResourceObservation;
pub use client_shutdown::mounted_identity::UiNativeClientAuthoredMountedInstanceObservation;
pub use client_shutdown::{
    UiNativeClientConditionalOutcome, UiNativeClientPresentationMechanicIdentityObservation,
    UiNativeClientPresentationSemanticChange,
    UiNativeClientPresentationSemanticFrontierObservation,
    UiNativeClientPresentationSemanticSubscriberObservation,
    UiNativeClientPresentationTransitionKind, UiNativeClientPresentationTransitionObservation,
    UiNativeClientShutdownObservation, UiNativeClientTextPresentationWorkObservation,
};

pub trait UiNativeEventLoopClient {
    fn native_surface_ready(
        &mut self,
        grant: UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()>;
    fn redraw_ready(
        &mut self,
        grant: UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()>;
    fn physical_work_progressed(
        &mut self,
        _grant: UiNativePhysicalProgressGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        Ok(UiNativeEventLoopDirective::Continue)
    }
    fn external_close_requested(&mut self) -> Result<UiNativeEventLoopDirective, ()> {
        Ok(UiNativeEventLoopDirective::Close)
    }
    fn presentation_attribution(&self) -> Option<UiNativeClientPresentationAttribution>;
    fn close(self) -> UiNativeEventLoopClientClose;
}

#[must_use]
pub struct UiNativePhysicalProgressGrant {
    class: UiNativePhysicalProgressClass,
    presentation: Option<super::UiNativePhysicalPresentationCorrelation>,
    duplicate_presentation_observed: bool,
}

impl UiNativePhysicalProgressGrant {
    pub(super) const fn issued(
        class: UiNativePhysicalProgressClass,
        presentation: Option<super::UiNativePhysicalPresentationCorrelation>,
        duplicate_presentation_observed: bool,
    ) -> Self {
        Self {
            class,
            presentation,
            duplicate_presentation_observed,
        }
    }

    pub const fn class(&self) -> UiNativePhysicalProgressClass {
        self.class
    }

    pub const fn presentation(&self) -> Option<super::UiNativePhysicalPresentationCorrelation> {
        self.presentation
    }

    pub const fn duplicate_presentation_observed(&self) -> bool {
        self.duplicate_presentation_observed
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativePhysicalProgressClass {
    TextAtlas,
    Presentation,
    PresentationRecovery,
}

pub trait UiNativeEventLoopClientCleanup {
    fn retry(self: Box<Self>) -> UiNativeEventLoopClientClose;
}

pub enum UiNativeEventLoopClientClose {
    Complete,
    CompleteWithObservation(UiNativeClientShutdownObservation),
    Incomplete(Box<dyn UiNativeEventLoopClientCleanup>),
}

impl UiNativeEventLoopClientClose {
    pub(super) fn into_parts(
        self,
    ) -> (
        Option<Box<dyn UiNativeEventLoopClientCleanup>>,
        Option<UiNativeClientShutdownObservation>,
    ) {
        match self {
            Self::Complete => (None, None),
            Self::CompleteWithObservation(observation) => (None, Some(observation)),
            Self::Incomplete(cleanup) => (Some(cleanup), None),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiNativeClientPresentationAttribution {
    frame: u64,
    surface: u64,
    binding: u64,
    mounted_instance: u64,
    node_receipt: u64,
    presentation_attempt: u64,
    authored_provenance_digest: u64,
    authored_semantic_identity_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeEventLoopDirective {
    Continue,
    ExternalObservationReady,
    WaitUntil(std::time::Instant),
    Close,
}

#[must_use]
pub struct UiNativeReadinessGrant {
    generation: u64,
    scale_factor_milli: u32,
    client_physical_size: [u32; 2],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeEventLoopRunDenial {
    EventLoopCreation,
    WindowCreation,
    GraphicsPreparation,
    ApplicationDriver,
    EventLoopRun,
    IncompleteCleanup,
}

#[derive(Debug)]
pub struct UiNativeEventLoopStopReport {
    pub(super) cause: UiNativeEventLoopRunDenial,
    pub(super) effect_posture: UiNativeEffectPosture,
    pub(super) peak_census: UiNativeResourceCensus,
    pub(super) terminal_census: UiNativeResourceCensus,
    pub(super) client_cleanup_complete: bool,
    pub(super) cleanup: Option<UiNativeEventLoopCleanup>,
    pub(super) peak_text_pins: Box<[crate::native::text_atlas::UiNativeTextPinObservation]>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativeEventLoopRunReport {
    pub(super) presentation: UiNativePresentationObservation,
    pub(super) graphics: UiNativeGraphicsObservation,
    pub(super) event_loop_thread: Box<str>,
    pub(super) event_loop_thread_matches_launch: bool,
    pub(super) client_attribution: UiNativeClientPresentationAttribution,
    pub(super) readiness_signals: u64,
    pub(super) redraw_turns: u64,
    pub(super) idle_wait_turns: u64,
    pub(super) coalesced_wakes: u64,
    pub(super) peak_census: UiNativeResourceCensus,
    pub(super) terminal_census: UiNativeResourceCensus,
    pub(super) port_crossings: u8,
    pub(super) retained_frames: Box<[UiNativeRetainedFrameObservation]>,
    pub(super) peak_text_pins: Box<[crate::native::text_atlas::UiNativeTextPinObservation]>,
    pub(super) text_pin_frame_counts: Box<[u32]>,
    pub(super) text_pin_frame_observations:
        Box<[Box<[crate::native::text_atlas::UiNativeTextPinObservation]>]>,
    pub(super) text_atlas_model_frame_digests: Box<[[u8; 32]]>,
    pub(super) observation_history_complete: bool,
    pub(super) text_atlas_transactions: u64,
    pub(super) derived_state_reconstruction:
        Option<crate::UiNativeDerivedStateReconstructionObservation>,
    pub(super) text_atlas_plan_observations:
        Box<[crate::native::text_atlas::UiNativeTextAtlasPlanObservation]>,
    pub(super) physical_signal_transition_observations:
        Box<[crate::native::physical_work_signal::UiNativePhysicalSignalTransitionObservation]>,
    pub(super) physical_signal_transition_trace_complete: bool,
    pub(super) physical_signal_lifecycle: crate::native::UiNativePhysicalSignalLifecycleObservation,
    pub(super) client_shutdown: Option<UiNativeClientShutdownObservation>,
}

impl UiNativeEventLoopRunReport {
    pub fn presentation(&self) -> &UiNativePresentationObservation {
        &self.presentation
    }

    pub const fn terminal_census(&self) -> UiNativeResourceCensus {
        self.terminal_census
    }

    pub fn graphics(&self) -> &UiNativeGraphicsObservation {
        &self.graphics
    }

    pub fn event_loop_thread(&self) -> &str {
        &self.event_loop_thread
    }

    pub const fn event_loop_thread_matches_launch(&self) -> bool {
        self.event_loop_thread_matches_launch
    }

    pub const fn client_attribution(&self) -> UiNativeClientPresentationAttribution {
        self.client_attribution
    }

    pub const fn readiness_signals(&self) -> u64 {
        self.readiness_signals
    }

    pub const fn redraw_turns(&self) -> u64 {
        self.redraw_turns
    }

    pub const fn idle_wait_turns(&self) -> u64 {
        self.idle_wait_turns
    }

    pub const fn coalesced_wakes(&self) -> u64 {
        self.coalesced_wakes
    }

    pub const fn peak_census(&self) -> UiNativeResourceCensus {
        self.peak_census
    }

    pub const fn port_crossings(&self) -> u8 {
        self.port_crossings
    }

    pub const fn client_shutdown(&self) -> Option<&UiNativeClientShutdownObservation> {
        self.client_shutdown.as_ref()
    }

    pub fn retained_frames(&self) -> &[UiNativeRetainedFrameObservation] {
        &self.retained_frames
    }

    #[doc(hidden)]
    pub fn peak_text_pins(&self) -> &[crate::native::text_atlas::UiNativeTextPinObservation] {
        &self.peak_text_pins
    }

    pub fn peak_text_layout_count(&self) -> usize {
        self.peak_text_pins
            .iter()
            .map(|pin| pin.layout_digest())
            .collect::<std::collections::BTreeSet<_>>()
            .len()
    }

    pub fn text_pin_frame_counts(&self) -> &[u32] {
        &self.text_pin_frame_counts
    }

    #[doc(hidden)]
    pub fn text_pin_frame_observations(
        &self,
    ) -> &[Box<[crate::native::text_atlas::UiNativeTextPinObservation]>] {
        &self.text_pin_frame_observations
    }

    pub fn text_atlas_model_frame_digests(&self) -> &[[u8; 32]] {
        &self.text_atlas_model_frame_digests
    }

    pub const fn observation_history_complete(&self) -> bool {
        self.observation_history_complete
    }

    pub const fn text_atlas_transactions(&self) -> u64 {
        self.text_atlas_transactions
    }

    pub const fn derived_state_reconstruction(
        &self,
    ) -> Option<crate::UiNativeDerivedStateReconstructionObservation> {
        self.derived_state_reconstruction
    }

    pub fn text_atlas_plan_observations(
        &self,
    ) -> &[crate::native::text_atlas::UiNativeTextAtlasPlanObservation] {
        &self.text_atlas_plan_observations
    }

    pub fn physical_signal_transition_observations(
        &self,
    ) -> &[crate::native::physical_work_signal::UiNativePhysicalSignalTransitionObservation] {
        &self.physical_signal_transition_observations
    }

    pub const fn physical_signal_transition_trace_complete(&self) -> bool {
        self.physical_signal_transition_trace_complete
    }

    pub const fn physical_signal_lifecycle(
        &self,
    ) -> crate::native::UiNativePhysicalSignalLifecycleObservation {
        self.physical_signal_lifecycle
    }
}

impl UiNativeClientPresentationAttribution {
    pub const fn reported(mechanical: [u64; 6], authored: [u64; 2]) -> Self {
        let [frame, surface, binding, mounted_instance, node_receipt, presentation_attempt] =
            mechanical;
        let [authored_provenance_digest, authored_semantic_identity_digest] = authored;
        Self {
            frame,
            surface,
            binding,
            mounted_instance,
            node_receipt,
            presentation_attempt,
            authored_provenance_digest,
            authored_semantic_identity_digest,
        }
    }

    pub(super) const fn matches(self, observation: &UiNativePresentationObservation) -> bool {
        self.frame == observation.presented_frame()
            && self.surface == observation.semantic_surface()
            && self.binding == observation.binding_generation()
            && self.mounted_instance == observation.mounted_instance()
            && self.node_receipt == observation.node_receipt()
            && self.presentation_attempt == observation.presentation_attempt()
    }

    pub const fn frame(self) -> u64 {
        self.frame
    }

    pub const fn surface(self) -> u64 {
        self.surface
    }

    pub const fn binding(self) -> u64 {
        self.binding
    }

    pub const fn mounted_instance(self) -> u64 {
        self.mounted_instance
    }

    pub const fn node_receipt(self) -> u64 {
        self.node_receipt
    }

    pub const fn presentation_attempt(self) -> u64 {
        self.presentation_attempt
    }

    pub const fn authored_provenance_digest(self) -> u64 {
        self.authored_provenance_digest
    }

    pub const fn authored_semantic_identity_digest(self) -> u64 {
        self.authored_semantic_identity_digest
    }
}
