use super::super::{
    UiNativeEffectPosture, UiNativeGraphicsObservation, UiNativePresentationObservation,
    UiNativeResourceCensus, UiNativeRetainedFrameObservation,
};
use super::UiNativeEventLoopCleanup;

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
    fn presentation_attribution(&self) -> Option<UiNativeClientPresentationAttribution>;
    fn close(self) -> UiNativeEventLoopClientClose;
}

#[must_use]
pub struct UiNativePhysicalProgressGrant {
    _private: (),
}

impl UiNativePhysicalProgressGrant {
    pub(super) const fn issued() -> Self {
        Self { _private: () }
    }
}

pub trait UiNativeEventLoopClientCleanup {
    fn retry(self: Box<Self>) -> UiNativeEventLoopClientClose;
}

pub enum UiNativeEventLoopClientClose {
    Complete,
    Incomplete(Box<dyn UiNativeEventLoopClientCleanup>),
}

impl UiNativeEventLoopClientClose {
    pub(super) fn into_cleanup(self) -> Option<Box<dyn UiNativeEventLoopClientCleanup>> {
        match self {
            Self::Complete => None,
            Self::Incomplete(cleanup) => Some(cleanup),
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

    pub fn retained_frames(&self) -> &[UiNativeRetainedFrameObservation] {
        &self.retained_frames
    }

    #[doc(hidden)]
    pub fn peak_text_pins(&self) -> &[crate::native::text_atlas::UiNativeTextPinObservation] {
        &self.peak_text_pins
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

impl UiNativeEventLoopStopReport {
    pub const fn cause(&self) -> UiNativeEventLoopRunDenial {
        self.cause
    }

    pub const fn effect_posture(&self) -> UiNativeEffectPosture {
        self.effect_posture
    }

    pub const fn peak_census(&self) -> UiNativeResourceCensus {
        self.peak_census
    }

    pub const fn terminal_census(&self) -> UiNativeResourceCensus {
        self.terminal_census
    }

    pub const fn client_cleanup_complete(&self) -> bool {
        self.client_cleanup_complete
    }

    pub fn into_cleanup(self) -> Option<UiNativeEventLoopCleanup> {
        self.cleanup
    }

    #[doc(hidden)]
    pub fn peak_text_pins(&self) -> &[crate::native::text_atlas::UiNativeTextPinObservation] {
        &self.peak_text_pins
    }
}

impl UiNativeReadinessGrant {
    pub(super) const fn issued(
        generation: u64,
        scale_factor_milli: u32,
        client_physical_size: [u32; 2],
    ) -> Self {
        Self {
            generation,
            scale_factor_milli,
            client_physical_size,
        }
    }

    pub const fn generation(&self) -> u64 {
        self.generation
    }

    pub const fn scale_factor_milli(&self) -> u32 {
        self.scale_factor_milli
    }

    pub const fn client_physical_size(&self) -> [u32; 2] {
        self.client_physical_size
    }
}
