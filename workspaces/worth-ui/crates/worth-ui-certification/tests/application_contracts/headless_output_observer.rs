use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

use worth_ui::facade::host::{
    WorthUiHeadlessHost, WorthUiHostCapabilityReport, WorthUiHostContract,
    WorthUiHostOutputDisposition, WorthUiHostOutputEnvelope, WorthUiHostOutputPayload,
    WorthUiOperationalHostAdapter, WorthUiOrdinaryHostOutputTarget,
};
use worth_ui_host_contract::{
    UiHostObservationValue, UiMeasurementRequest, WorthUiMeasurementHostAdapter,
};

#[derive(Clone)]
pub(super) struct ObservingHeadlessHost {
    observation: Arc<HeadlessOutputObservation>,
}

pub(super) struct HeadlessOutputObservation {
    call_count: AtomicU64,
    host_session_identity: AtomicU64,
    active_artifact_digest: AtomicU64,
    active_plan_digest: AtomicU64,
    frame_epoch: AtomicU64,
    target: AtomicU64,
    touched_row_count: AtomicUsize,
    meaning_digest: AtomicU64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ObservedHeadlessOutput {
    pub(super) call_count: u64,
    pub(super) host_session_identity: u64,
    pub(super) active_artifact_digest: u64,
    pub(super) active_plan_digest: u64,
    pub(super) frame_epoch: u64,
    pub(super) target: WorthUiOrdinaryHostOutputTarget,
    pub(super) touched_row_count: usize,
    pub(super) meaning_digest: u64,
}

impl ObservingHeadlessHost {
    pub(super) fn new() -> (Self, Arc<HeadlessOutputObservation>) {
        let observation = Arc::new(HeadlessOutputObservation {
            call_count: AtomicU64::new(0),
            host_session_identity: AtomicU64::new(0),
            active_artifact_digest: AtomicU64::new(0),
            active_plan_digest: AtomicU64::new(0),
            frame_epoch: AtomicU64::new(0),
            target: AtomicU64::new(0),
            touched_row_count: AtomicUsize::new(0),
            meaning_digest: AtomicU64::new(0),
        });
        (
            Self {
                observation: Arc::clone(&observation),
            },
            observation,
        )
    }
}

impl HeadlessOutputObservation {
    pub(super) fn snapshot(&self) -> ObservedHeadlessOutput {
        let call_count = self.call_count.load(Ordering::Acquire);
        assert!(
            call_count > 0,
            "headless output must be observed before reading it"
        );
        ObservedHeadlessOutput {
            call_count,
            host_session_identity: self.host_session_identity.load(Ordering::Relaxed),
            active_artifact_digest: self.active_artifact_digest.load(Ordering::Relaxed),
            active_plan_digest: self.active_plan_digest.load(Ordering::Relaxed),
            frame_epoch: self.frame_epoch.load(Ordering::Relaxed),
            target: decode_target(self.target.load(Ordering::Relaxed)),
            touched_row_count: self.touched_row_count.load(Ordering::Relaxed),
            meaning_digest: self.meaning_digest.load(Ordering::Relaxed),
        }
    }
}

impl WorthUiMeasurementHostAdapter for ObservingHeadlessHost {
    fn observe_measurement(&self, request: &UiMeasurementRequest) -> UiHostObservationValue {
        WorthUiHeadlessHost.observe_measurement(request)
    }
}

impl WorthUiOperationalHostAdapter for ObservingHeadlessHost {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHeadlessHost.operational_host_contract()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHeadlessHost.operational_capability_report()
    }

    fn consume_output(&self, output: &WorthUiHostOutputEnvelope) -> WorthUiHostOutputDisposition {
        let disposition = WorthUiHeadlessHost.consume_output(output);
        let generation = output.generation();
        let ordinary = match output.payload() {
            WorthUiHostOutputPayload::Ordinary(ordinary) => ordinary,
            _ => panic!("headless observer received an unsupported output payload"),
        };
        self.observation
            .host_session_identity
            .store(generation.host_session_identity(), Ordering::Relaxed);
        self.observation
            .active_plan_digest
            .store(generation.active_plan_digest(), Ordering::Relaxed);
        self.observation
            .active_artifact_digest
            .store(generation.active_artifact_digest(), Ordering::Relaxed);
        self.observation
            .frame_epoch
            .store(generation.frame_epoch(), Ordering::Relaxed);
        self.observation
            .target
            .store(encode_target(ordinary.target()), Ordering::Relaxed);
        self.observation
            .touched_row_count
            .store(ordinary.touched_row_count(), Ordering::Relaxed);
        self.observation
            .meaning_digest
            .store(ordinary.meaning_digest(), Ordering::Relaxed);
        self.observation.call_count.fetch_add(1, Ordering::Release);
        disposition
    }
}

fn encode_target(target: WorthUiOrdinaryHostOutputTarget) -> u64 {
    match target {
        WorthUiOrdinaryHostOutputTarget::RootShell => 1,
        WorthUiOrdinaryHostOutputTarget::Component => 2,
        WorthUiOrdinaryHostOutputTarget::Command => 3,
        WorthUiOrdinaryHostOutputTarget::TokenSupport => 4,
        WorthUiOrdinaryHostOutputTarget::ChildRange => 5,
        WorthUiOrdinaryHostOutputTarget::StateSlot => 6,
    }
}

fn decode_target(target: u64) -> WorthUiOrdinaryHostOutputTarget {
    match target {
        1 => WorthUiOrdinaryHostOutputTarget::RootShell,
        2 => WorthUiOrdinaryHostOutputTarget::Component,
        3 => WorthUiOrdinaryHostOutputTarget::Command,
        4 => WorthUiOrdinaryHostOutputTarget::TokenSupport,
        5 => WorthUiOrdinaryHostOutputTarget::ChildRange,
        6 => WorthUiOrdinaryHostOutputTarget::StateSlot,
        _ => panic!("observed an unknown headless output target code: {target}"),
    }
}
