use std::collections::{BTreeMap, BTreeSet, VecDeque};

use worth_ui_host_contract::{
    UiHostObservationCanonicalCore, UiHostObservationCoalescingIdentity,
    UiHostObservationIntegrity, UiHostObservationSequence, UiHostObservationSequenceRange,
    UiMountedFrameIdentity, UiSurfaceBindingGeneration,
};

use super::{
    UiHostObservationCapacity, UiHostObservationDisposition, UiHostObservationFrameRelation,
    UiQuarantinedHostObservationBatch, UiValidatedHostObservationReport,
};

const RECENT_BATCH_FINGERPRINT_LIMIT: usize = 16;
const TERMINAL_FRAME_IDENTITY_LIMIT: usize = 64;

#[derive(Clone)]
pub(super) struct UiHostObservationPartition {
    pub(super) last_sequence: Option<UiHostObservationSequence>,
    pub(super) reports: VecDeque<UiRetainedHostObservationReport>,
    pub(super) byte_count: usize,
    pub(super) recent_batches: VecDeque<UiHostObservationBatchFingerprint>,
}

#[derive(Clone)]
pub(super) struct UiRetainedHostObservationReport {
    pub(super) report: UiValidatedHostObservationReport,
    pub(super) relation: UiHostObservationFrameRelation,
    pub(super) coalescing_identity: Option<UiHostObservationCoalescingIdentity>,
    pub(super) encoded_len: usize,
}

#[derive(Clone, Copy)]
pub(super) struct UiHostObservationBatchFingerprint {
    pub(super) sequences: UiHostObservationSequenceRange,
    pub(super) integrity: UiHostObservationIntegrity,
}

pub struct UiHostObservationReportValidation {
    ingress: super::WorthUiHostObservationIngress,
    pub(super) capacity: UiHostObservationCapacity,
    pub(super) partitions: BTreeMap<UiSurfaceBindingGeneration, UiHostObservationPartition>,
    pub(super) global_reports: usize,
    pub(super) global_bytes: usize,
    pub(super) quarantine: VecDeque<UiQuarantinedHostObservationBatch>,
    pub(super) quarantine_fingerprints: VecDeque<UiHostObservationBatchFingerprint>,
    rejected_frames: BTreeSet<UiMountedFrameIdentity>,
    rejected_order: VecDeque<UiMountedFrameIdentity>,
    never_presented_frames: BTreeSet<UiMountedFrameIdentity>,
    never_presented_order: VecDeque<UiMountedFrameIdentity>,
    indeterminate_frames: BTreeSet<(UiMountedFrameIdentity, UiSurfaceBindingGeneration)>,
    indeterminate_order: VecDeque<(UiMountedFrameIdentity, UiSurfaceBindingGeneration)>,
    pub(super) shutdown: bool,
}

impl Default for UiHostObservationReportValidation {
    fn default() -> Self {
        Self::new(UiHostObservationCapacity::default())
    }
}

impl UiHostObservationReportValidation {
    pub fn new(capacity: UiHostObservationCapacity) -> Self {
        Self {
            ingress: super::WorthUiHostObservationIngress::new(),
            capacity,
            partitions: BTreeMap::new(),
            global_reports: 0,
            global_bytes: 0,
            quarantine: VecDeque::new(),
            quarantine_fingerprints: VecDeque::new(),
            rejected_frames: BTreeSet::new(),
            rejected_order: VecDeque::new(),
            never_presented_frames: BTreeSet::new(),
            never_presented_order: VecDeque::new(),
            indeterminate_frames: BTreeSet::new(),
            indeterminate_order: VecDeque::new(),
            shutdown: false,
        }
    }

    pub(crate) fn record_rejected_frame(&mut self, frame: UiMountedFrameIdentity) {
        remember_terminal(frame, &mut self.rejected_frames, &mut self.rejected_order);
    }

    pub(crate) fn record_never_presented_frame(&mut self, frame: UiMountedFrameIdentity) {
        remember_terminal(
            frame,
            &mut self.never_presented_frames,
            &mut self.never_presented_order,
        );
    }

    pub(super) fn is_rejected(&self, frame: UiMountedFrameIdentity) -> bool {
        self.rejected_frames.contains(&frame)
    }

    pub(super) fn is_never_presented(&self, frame: UiMountedFrameIdentity) -> bool {
        self.never_presented_frames.contains(&frame)
    }

    pub(crate) fn record_indeterminate_frame(
        &mut self,
        frame: UiMountedFrameIdentity,
        bindings: &[UiSurfaceBindingGeneration],
    ) {
        for binding in bindings {
            let basis = (frame, *binding);
            if self.indeterminate_frames.insert(basis) {
                self.indeterminate_order.push_back(basis);
            }
        }
        while self.indeterminate_order.len() > TERMINAL_FRAME_IDENTITY_LIMIT {
            let forgotten = self
                .indeterminate_order
                .pop_front()
                .expect("over-limit indeterminate frame queue is non-empty");
            self.indeterminate_frames.remove(&forgotten);
        }
    }

    pub(super) fn is_indeterminate(
        &self,
        frame: UiMountedFrameIdentity,
        binding: UiSurfaceBindingGeneration,
    ) -> bool {
        self.indeterminate_frames.contains(&(frame, binding))
    }

    pub(crate) fn shutdown(&mut self) {
        self.ingress.shutdown();
        self.shutdown = true;
        self.partitions.clear();
        self.quarantine.clear();
        self.quarantine_fingerprints.clear();
        self.indeterminate_frames.clear();
        self.indeterminate_order.clear();
        self.global_reports = 0;
        self.global_bytes = 0;
    }

    pub fn retained_report_count(&self) -> usize {
        self.global_reports
    }

    pub fn retained_byte_count(&self) -> usize {
        self.global_bytes
    }

    pub fn quarantined_batch_count(&self) -> usize {
        self.quarantine.len()
    }

    pub(crate) fn ingress(&self) -> super::WorthUiHostObservationIngress {
        self.ingress.clone()
    }

    pub(crate) fn drain_ingress(&self) -> Vec<worth_ui_host_contract::UiHostObservationBatch> {
        self.ingress.drain()
    }
}

impl UiHostObservationPartition {
    pub(super) fn empty() -> Self {
        Self {
            last_sequence: None,
            reports: VecDeque::new(),
            byte_count: 0,
            recent_batches: VecDeque::new(),
        }
    }

    pub(super) fn remember_batch(&mut self, fingerprint: UiHostObservationBatchFingerprint) {
        self.recent_batches.push_back(fingerprint);
        if self.recent_batches.len() > RECENT_BATCH_FINGERPRINT_LIMIT {
            self.recent_batches.pop_front();
        }
    }

    pub(super) fn duplicate(
        &self,
        core: UiHostObservationCanonicalCore,
        integrity: UiHostObservationIntegrity,
    ) -> bool {
        self.recent_batches.iter().any(|candidate| {
            candidate.sequences == core.sequences() && candidate.integrity == integrity
        })
    }
}

impl UiRetainedHostObservationReport {
    pub(super) fn replaced_range(&self) -> UiHostObservationSequenceRange {
        match self.report.disposition() {
            UiHostObservationDisposition::Retained => UiHostObservationSequenceRange::new(
                self.report.report().sequence(),
                self.report.report().sequence(),
            ),
            UiHostObservationDisposition::Coalesced { replaced } => replaced,
        }
    }
}

fn remember_terminal(
    frame: UiMountedFrameIdentity,
    set: &mut BTreeSet<UiMountedFrameIdentity>,
    order: &mut VecDeque<UiMountedFrameIdentity>,
) {
    if set.insert(frame) {
        order.push_back(frame);
    }
    while order.len() > TERMINAL_FRAME_IDENTITY_LIMIT {
        let forgotten = order
            .pop_front()
            .expect("over-limit terminal frame queue is non-empty");
        set.remove(&forgotten);
    }
}
