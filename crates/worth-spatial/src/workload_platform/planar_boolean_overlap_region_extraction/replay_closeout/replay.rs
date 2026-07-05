use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::checkpoint::{
    ComparePlanarBooleanOverlapRegionCheckpointParity,
    PlanarBooleanOverlapRegionCheckpointParityReceipt,
};
use super::evidence::PlanarBooleanOverlapRegionEvidenceReceipt;
use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionLedgerReceipt;
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionReplayParityCounters {
    compared_rows: usize,
    rejected_replay_mismatches: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionReplayParityDenialKind {
    OverlapEvidenceMismatch,
    RequestIdentityMismatch,
    ReadinessHandoffMismatch,
    ReadinessConsumerMismatch,
    ReadinessBindingMismatch,
    DecisionLogMismatch,
    OverlapLedgerMismatch,
    IdentityMapMismatch,
    PersistentNameMismatch,
    SubshapeSignatureMismatch,
    CheckpointAuthorityMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionReplayParityDenial {
    kind: PlanarBooleanOverlapRegionReplayParityDenialKind,
    original_identity: String,
    replayed_identity: String,
    counters: PlanarBooleanOverlapRegionReplayParityCounters,
}

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanOverlapRegionReplayParityInput<'a> {
    original_ledger_receipt: &'a PlanarBooleanOverlapRegionLedgerReceipt,
    replayed_ledger_receipt: &'a PlanarBooleanOverlapRegionLedgerReceipt,
    original_evidence_receipt: &'a PlanarBooleanOverlapRegionEvidenceReceipt,
    replayed_evidence_receipt: &'a PlanarBooleanOverlapRegionEvidenceReceipt,
    replay_receipts: &'a ReplayReceiptSet,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionReplayParityRowKind {
    OverlapEvidenceReceipt,
    RequestIdentity,
    ReadinessHandoff,
    ReadinessConsumer,
    ReadinessBinding,
    OverlapDecisionLog,
    OverlapLedgerReceipt,
    OverlapIdentityMap,
    PersistentNamePropagationMap,
    SubshapeSignatureMap,
    RetainedReplayCheckpoint,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionReplayParityRow {
    kind: PlanarBooleanOverlapRegionReplayParityRowKind,
    original_identity: String,
    replayed_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionReplayParityReceipt {
    replay_identity: String,
    checkpoint_receipt: PlanarBooleanOverlapRegionCheckpointParityReceipt,
    rows: Vec<PlanarBooleanOverlapRegionReplayParityRow>,
    counters: PlanarBooleanOverlapRegionReplayParityCounters,
}

pub struct ComparePlanarBooleanOverlapRegionReplayParity;

impl PlanarBooleanOverlapRegionReplayParityCounters {
    pub fn compared_rows(self) -> usize { self.compared_rows }
    pub fn rejected_replay_mismatches(self) -> usize { self.rejected_replay_mismatches }
    fn compared_row(&mut self) { self.compared_rows += 1; }
    pub(crate) fn rejected_replay_mismatch(&mut self) { self.rejected_replay_mismatches += 1; }
}

impl PlanarBooleanOverlapRegionReplayParityDenial {
    pub fn new(
        kind: PlanarBooleanOverlapRegionReplayParityDenialKind,
        original_identity: impl Into<String>,
        replayed_identity: impl Into<String>,
        counters: PlanarBooleanOverlapRegionReplayParityCounters,
    ) -> Self {
        Self { kind, original_identity: original_identity.into(), replayed_identity: replayed_identity.into(), counters }
    }

    pub fn kind(&self) -> PlanarBooleanOverlapRegionReplayParityDenialKind { self.kind }
    pub fn counters(&self) -> PlanarBooleanOverlapRegionReplayParityCounters { self.counters }
}

impl<'a> PlanarBooleanOverlapRegionReplayParityInput<'a> {
    pub fn admit_from_ledger_and_evidence(
        original_ledger_receipt: &'a PlanarBooleanOverlapRegionLedgerReceipt,
        replayed_ledger_receipt: &'a PlanarBooleanOverlapRegionLedgerReceipt,
        original_evidence_receipt: &'a PlanarBooleanOverlapRegionEvidenceReceipt,
        replayed_evidence_receipt: &'a PlanarBooleanOverlapRegionEvidenceReceipt,
        replay_receipts: &'a ReplayReceiptSet,
    ) -> Result<Self, PlanarBooleanOverlapRegionReplayParityDenial> {
        if original_evidence_receipt.overlap_ledger_receipt_identity() != original_ledger_receipt.receipt_identity() {
            return Err(PlanarBooleanOverlapRegionReplayParityDenial::new(
                PlanarBooleanOverlapRegionReplayParityDenialKind::OverlapEvidenceMismatch,
                original_ledger_receipt.receipt_identity(),
                original_evidence_receipt.overlap_ledger_receipt_identity(),
                Default::default(),
            ));
        }
        if replayed_evidence_receipt.overlap_ledger_receipt_identity() != replayed_ledger_receipt.receipt_identity() {
            return Err(PlanarBooleanOverlapRegionReplayParityDenial::new(
                PlanarBooleanOverlapRegionReplayParityDenialKind::OverlapEvidenceMismatch,
                replayed_ledger_receipt.receipt_identity(),
                replayed_evidence_receipt.overlap_ledger_receipt_identity(),
                Default::default(),
            ));
        }
        Ok(Self { original_ledger_receipt, replayed_ledger_receipt, original_evidence_receipt, replayed_evidence_receipt, replay_receipts })
    }
}

impl ComparePlanarBooleanOverlapRegionReplayParity {
    pub fn compare(
        input: PlanarBooleanOverlapRegionReplayParityInput<'_>,
    ) -> Result<PlanarBooleanOverlapRegionReplayParityReceipt, PlanarBooleanOverlapRegionReplayParityDenial> {
        let mut counters = PlanarBooleanOverlapRegionReplayParityCounters::default();
        let mut rows = Vec::new();

        compare_row(PlanarBooleanOverlapRegionReplayParityRowKind::OverlapEvidenceReceipt, input.original_evidence_receipt.receipt_identity(), input.replayed_evidence_receipt.receipt_identity(), PlanarBooleanOverlapRegionReplayParityDenialKind::OverlapEvidenceMismatch, &mut counters, &mut rows)?;
        compare_row(PlanarBooleanOverlapRegionReplayParityRowKind::RequestIdentity, input.original_evidence_receipt.request_identity(), input.replayed_evidence_receipt.request_identity(), PlanarBooleanOverlapRegionReplayParityDenialKind::RequestIdentityMismatch, &mut counters, &mut rows)?;
        compare_row(PlanarBooleanOverlapRegionReplayParityRowKind::ReadinessHandoff, input.original_evidence_receipt.readiness_handoff_identity(), input.replayed_evidence_receipt.readiness_handoff_identity(), PlanarBooleanOverlapRegionReplayParityDenialKind::ReadinessHandoffMismatch, &mut counters, &mut rows)?;
        compare_row(PlanarBooleanOverlapRegionReplayParityRowKind::ReadinessConsumer, input.original_evidence_receipt.readiness_consumer_identity(), input.replayed_evidence_receipt.readiness_consumer_identity(), PlanarBooleanOverlapRegionReplayParityDenialKind::ReadinessConsumerMismatch, &mut counters, &mut rows)?;
        compare_row(PlanarBooleanOverlapRegionReplayParityRowKind::ReadinessBinding, input.original_evidence_receipt.readiness_binding_identity(), input.replayed_evidence_receipt.readiness_binding_identity(), PlanarBooleanOverlapRegionReplayParityDenialKind::ReadinessBindingMismatch, &mut counters, &mut rows)?;
        compare_row(PlanarBooleanOverlapRegionReplayParityRowKind::OverlapDecisionLog, input.original_evidence_receipt.overlap_decision_log_identity(), input.replayed_evidence_receipt.overlap_decision_log_identity(), PlanarBooleanOverlapRegionReplayParityDenialKind::DecisionLogMismatch, &mut counters, &mut rows)?;
        compare_row(PlanarBooleanOverlapRegionReplayParityRowKind::OverlapLedgerReceipt, input.original_ledger_receipt.receipt_identity(), input.replayed_ledger_receipt.receipt_identity(), PlanarBooleanOverlapRegionReplayParityDenialKind::OverlapLedgerMismatch, &mut counters, &mut rows)?;
        compare_row(PlanarBooleanOverlapRegionReplayParityRowKind::OverlapIdentityMap, input.original_evidence_receipt.overlap_region_identity_map_identity(), input.replayed_evidence_receipt.overlap_region_identity_map_identity(), PlanarBooleanOverlapRegionReplayParityDenialKind::IdentityMapMismatch, &mut counters, &mut rows)?;
        compare_row(PlanarBooleanOverlapRegionReplayParityRowKind::PersistentNamePropagationMap, input.original_evidence_receipt.persistent_name_propagation_map_identity(), input.replayed_evidence_receipt.persistent_name_propagation_map_identity(), PlanarBooleanOverlapRegionReplayParityDenialKind::PersistentNameMismatch, &mut counters, &mut rows)?;
        compare_row(PlanarBooleanOverlapRegionReplayParityRowKind::SubshapeSignatureMap, input.original_evidence_receipt.subshape_signature_map_identity(), input.replayed_evidence_receipt.subshape_signature_map_identity(), PlanarBooleanOverlapRegionReplayParityDenialKind::SubshapeSignatureMismatch, &mut counters, &mut rows)?;

        let checkpoint_receipt = ComparePlanarBooleanOverlapRegionCheckpointParity::compare(
            input.original_evidence_receipt,
            input.replayed_evidence_receipt,
            input.replay_receipts,
            &mut counters,
        )?;
        rows.push(PlanarBooleanOverlapRegionReplayParityRow {
            kind: PlanarBooleanOverlapRegionReplayParityRowKind::RetainedReplayCheckpoint,
            original_identity: checkpoint_receipt.checkpoint_identity().to_string(),
            replayed_identity: checkpoint_receipt.checkpoint_identity().to_string(),
        });

        Ok(PlanarBooleanOverlapRegionReplayParityReceipt {
            replay_identity: truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "planar-boolean-overlap-region-replay-parity".to_string(),
                    format!("evidence:{}", input.original_evidence_receipt.receipt_identity()),
                    format!("request:{}", input.original_evidence_receipt.request_identity()),
                    format!("binding:{}", input.original_evidence_receipt.readiness_binding_identity()),
                    format!("ledger:{}", input.original_ledger_receipt.receipt_identity()),
                    format!("checkpoint:{}", checkpoint_receipt.checkpoint_identity()),
                ],
            ),
            checkpoint_receipt,
            rows,
            counters,
        })
    }
}

fn compare_row(
    kind: PlanarBooleanOverlapRegionReplayParityRowKind,
    original: &str,
    replayed: &str,
    denial_kind: PlanarBooleanOverlapRegionReplayParityDenialKind,
    counters: &mut PlanarBooleanOverlapRegionReplayParityCounters,
    rows: &mut Vec<PlanarBooleanOverlapRegionReplayParityRow>,
) -> Result<(), PlanarBooleanOverlapRegionReplayParityDenial> {
    if original != replayed {
        counters.rejected_replay_mismatch();
        return Err(PlanarBooleanOverlapRegionReplayParityDenial::new(
            denial_kind,
            original,
            replayed,
            *counters,
        ));
    }
    counters.compared_row();
    rows.push(PlanarBooleanOverlapRegionReplayParityRow {
        kind,
        original_identity: original.to_string(),
        replayed_identity: replayed.to_string(),
    });
    Ok(())
}

impl PlanarBooleanOverlapRegionReplayParityRow {
    pub fn kind(&self) -> PlanarBooleanOverlapRegionReplayParityRowKind { self.kind }
}

impl PlanarBooleanOverlapRegionReplayParityReceipt {
    pub fn replay_identity(&self) -> &str { &self.replay_identity }
    pub fn checkpoint_receipt(&self) -> &PlanarBooleanOverlapRegionCheckpointParityReceipt { &self.checkpoint_receipt }
    pub fn rows(&self) -> &[PlanarBooleanOverlapRegionReplayParityRow] { &self.rows }
    pub fn counters(&self) -> PlanarBooleanOverlapRegionReplayParityCounters { self.counters }
}
