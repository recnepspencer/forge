use super::super::closure_manifest::PlanarBooleanSplitReplayClosureManifest;
use super::super::parity_receipt::denial::{
    PlanarBooleanEdgeSplitReplayParityDenial, PlanarBooleanEdgeSplitReplayParityDenialKind as Kind,
};
use super::closeout::PlanarBooleanEdgeSplitCloseout;
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEdgeSplitReplayExecutionMode {
    RetainedReplay,
    CheckpointedReplay,
    ReversedSourceSenseVariant,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitReplayProductCounters {
    closeout_rows_read: usize,
    retained_replay_rows_read: usize,
    checkpoint_rows_read: usize,
    replay_rows_emitted: usize,
    closure_rows_emitted: usize,
    split_stage_reexecutions: usize,
    event_extraction_reexecutions: usize,
    candidate_index_reexecutions: usize,
}

pub struct PlanarBooleanEdgeSplitReplayProduct<'a> {
    product_identity: String,
    original: PlanarBooleanEdgeSplitCloseout<'a>,
    replayed: PlanarBooleanEdgeSplitCloseout<'a>,
    replay_receipts: &'a ReplayReceiptSet,
    execution_mode: PlanarBooleanEdgeSplitReplayExecutionMode,
    closure_manifest: PlanarBooleanSplitReplayClosureManifest,
    counters: PlanarBooleanEdgeSplitReplayProductCounters,
}

impl<'a> PlanarBooleanEdgeSplitReplayProduct<'a> {
    pub fn from_retained_closeouts(
        original: PlanarBooleanEdgeSplitCloseout<'a>,
        replayed: PlanarBooleanEdgeSplitCloseout<'a>,
        replay_receipts: &'a ReplayReceiptSet,
        execution_mode: PlanarBooleanEdgeSplitReplayExecutionMode,
    ) -> Result<Self, PlanarBooleanEdgeSplitReplayParityDenial> {
        validate_retained_replay_binding(&original, &replayed, replay_receipts)?;
        let closure_manifest =
            PlanarBooleanSplitReplayClosureManifest::compare_closeouts(&original, &replayed);
        let counters = PlanarBooleanEdgeSplitReplayProductCounters::from_manifest(
            replay_receipts,
            &closure_manifest,
            execution_mode,
        );
        let product_identity = format!(
            "edge-split-replay-product:{:?}:{}:{}:{}",
            execution_mode,
            replay_receipts.stage_identity().receipt_identity(),
            replay_receipts.replay_checkpoint_identity(),
            closure_manifest.manifest_identity()
        );
        Ok(Self {
            product_identity,
            original,
            replayed,
            replay_receipts,
            execution_mode,
            closure_manifest,
            counters,
        })
    }

    pub fn product_identity(&self) -> &str {
        &self.product_identity
    }
    pub fn original(&self) -> &PlanarBooleanEdgeSplitCloseout<'a> {
        &self.original
    }
    pub fn replayed(&self) -> &PlanarBooleanEdgeSplitCloseout<'a> {
        &self.replayed
    }
    pub fn replay_receipts(&self) -> &'a ReplayReceiptSet {
        self.replay_receipts
    }
    pub fn execution_mode(&self) -> PlanarBooleanEdgeSplitReplayExecutionMode {
        self.execution_mode
    }
    pub fn closure_manifest(&self) -> &PlanarBooleanSplitReplayClosureManifest {
        &self.closure_manifest
    }
    pub fn counters(&self) -> PlanarBooleanEdgeSplitReplayProductCounters {
        self.counters
    }
    pub fn certifies_query_owned_replay_product(&self) -> bool {
        !self.product_identity.is_empty()
            && self.closure_manifest.is_complete_and_matching()
            && self.counters.event_extraction_reexecutions == 0
            && self.counters.candidate_index_reexecutions == 0
            && self.counters.closure_rows_emitted == self.closure_manifest.rows().len()
            && self.replay_receipts.counters().replay_rows() > 0
    }
}

impl PlanarBooleanEdgeSplitReplayProductCounters {
    #[cfg(test)]
    pub(crate) fn new(
        closeout_rows_read: usize,
        retained_replay_rows_read: usize,
        replay_rows_emitted: usize,
        event_extraction_reexecutions: usize,
        candidate_index_reexecutions: usize,
    ) -> Self {
        Self {
            closeout_rows_read,
            retained_replay_rows_read,
            checkpoint_rows_read: 0,
            replay_rows_emitted,
            closure_rows_emitted: 20,
            split_stage_reexecutions: 1,
            event_extraction_reexecutions,
            candidate_index_reexecutions,
        }
    }

    fn from_manifest(
        replay_receipts: &ReplayReceiptSet,
        closure_manifest: &PlanarBooleanSplitReplayClosureManifest,
        execution_mode: PlanarBooleanEdgeSplitReplayExecutionMode,
    ) -> Self {
        Self {
            closeout_rows_read: 2,
            retained_replay_rows_read: replay_receipts.counters().retained_artifact_rows(),
            checkpoint_rows_read: usize::from(matches!(
                execution_mode,
                PlanarBooleanEdgeSplitReplayExecutionMode::CheckpointedReplay
            )),
            replay_rows_emitted: replay_receipts.counters().replay_rows(),
            closure_rows_emitted: closure_manifest.rows().len(),
            split_stage_reexecutions: usize::from(matches!(
                execution_mode,
                PlanarBooleanEdgeSplitReplayExecutionMode::RetainedReplay
                    | PlanarBooleanEdgeSplitReplayExecutionMode::ReversedSourceSenseVariant
            )),
            event_extraction_reexecutions: 0,
            candidate_index_reexecutions: 0,
        }
    }

    pub fn closeout_rows_read(self) -> usize {
        self.closeout_rows_read
    }
    pub fn retained_replay_rows_read(self) -> usize {
        self.retained_replay_rows_read
    }
    pub fn checkpoint_rows_read(self) -> usize {
        self.checkpoint_rows_read
    }
    pub fn replay_rows_emitted(self) -> usize {
        self.replay_rows_emitted
    }
    pub fn closure_rows_emitted(self) -> usize {
        self.closure_rows_emitted
    }
    pub fn split_stage_reexecutions(self) -> usize {
        self.split_stage_reexecutions
    }
    pub fn event_extraction_reexecutions(self) -> usize {
        self.event_extraction_reexecutions
    }
    pub fn candidate_index_reexecutions(self) -> usize {
        self.candidate_index_reexecutions
    }
}

fn validate_retained_replay_binding(
    original: &PlanarBooleanEdgeSplitCloseout<'_>,
    replayed: &PlanarBooleanEdgeSplitCloseout<'_>,
    replay_receipts: &ReplayReceiptSet,
) -> Result<(), PlanarBooleanEdgeSplitReplayParityDenial> {
    let Some(original_stage) = original.request().retained_replay_stage_identity() else {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::MissingSplitRequestRetainedReplay,
            original.request().split_request_identity(),
            "retained replay evidence row",
            "none",
            "edge-split replay product requires retained replay evidence in the split request",
        ));
    };
    if replayed.request().retained_replay_stage_identity() != Some(original_stage) {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::ReplaySplitRequestMismatch,
            "retained-replay-stage-identity",
            original_stage,
            replayed
                .request()
                .retained_replay_stage_identity()
                .unwrap_or("none"),
            "original and replayed closeouts must carry the same retained replay stage identity",
        ));
    }
    if replay_receipts.stage_identity().receipt_identity() != original_stage {
        return Err(PlanarBooleanEdgeSplitReplayParityDenial::new(
            Kind::ForeignRetainedReplayReceipt,
            "retained-replay-stage-identity",
            original_stage,
            replay_receipts.stage_identity().receipt_identity(),
            "retained replay receipt must be the receipt admitted into the split request",
        ));
    }
    Ok(())
}
