use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::checkpoint::ComparePlanarBooleanLoopCheckpointParity;
use super::counters::PlanarBooleanLoopReplayParityCounters;
use super::denial::{PlanarBooleanLoopReplayParityDenial, PlanarBooleanLoopReplayParityDenialKind};
use super::input::PlanarBooleanLoopReplayParityInput;
use super::product::PlanarBooleanLoopReplayParityReceipt;
use super::row::{PlanarBooleanLoopReplayParityRow, PlanarBooleanLoopReplayParityRowKind};

pub struct ComparePlanarBooleanLoopReplayParity;

impl ComparePlanarBooleanLoopReplayParity {
    pub fn compare(
        input: PlanarBooleanLoopReplayParityInput<'_>,
    ) -> Result<PlanarBooleanLoopReplayParityReceipt, PlanarBooleanLoopReplayParityDenial> {
        let mut counters = PlanarBooleanLoopReplayParityCounters::default();
        let mut rows = Vec::new();

        compare_row(
            PlanarBooleanLoopReplayParityRowKind::LoopEvidenceReceipt,
            input.original_evidence_receipt().receipt_identity(),
            input.replayed_evidence_receipt().receipt_identity(),
            PlanarBooleanLoopReplayParityDenialKind::LoopEvidenceMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_loop_evidence_receipts();
        compare_row(
            PlanarBooleanLoopReplayParityRowKind::ReconstructedLoopSet,
            input
                .original_evidence_receipt()
                .reconstructed_loop_set_identity(),
            input
                .replayed_evidence_receipt()
                .reconstructed_loop_set_identity(),
            PlanarBooleanLoopReplayParityDenialKind::ReconstructedLoopMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_reconstructed_loops();
        compare_row(
            PlanarBooleanLoopReplayParityRowKind::BornLoopSet,
            input.original_evidence_receipt().born_loop_set_identity(),
            input.replayed_evidence_receipt().born_loop_set_identity(),
            PlanarBooleanLoopReplayParityDenialKind::BornLoopMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_born_loops();
        compare_row(
            PlanarBooleanLoopReplayParityRowKind::IslandPartition,
            input
                .original_evidence_receipt()
                .island_partition_identity(),
            input
                .replayed_evidence_receipt()
                .island_partition_identity(),
            PlanarBooleanLoopReplayParityDenialKind::IslandPartitionMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_island_partitions();
        compare_row(
            PlanarBooleanLoopReplayParityRowKind::SplitAttribution,
            input
                .original_evidence_receipt()
                .split_attribution_identity(),
            input
                .replayed_evidence_receipt()
                .split_attribution_identity(),
            PlanarBooleanLoopReplayParityDenialKind::SplitAttributionMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_split_attributions();

        compare_row(
            PlanarBooleanLoopReplayParityRowKind::RoleOutcomeSet,
            input
                .original_evidence_receipt()
                .role_outcome_set_identity(),
            input
                .replayed_evidence_receipt()
                .role_outcome_set_identity(),
            PlanarBooleanLoopReplayParityDenialKind::RoleOutcomeMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_role_outcomes();

        compare_row(
            PlanarBooleanLoopReplayParityRowKind::DegenerateOutcomeSet,
            input
                .original_evidence_receipt()
                .degenerate_outcome_set_identity(),
            input
                .replayed_evidence_receipt()
                .degenerate_outcome_set_identity(),
            PlanarBooleanLoopReplayParityDenialKind::DegenerateOutcomeMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_degenerate_outcomes();

        compare_row(
            PlanarBooleanLoopReplayParityRowKind::DecisionLog,
            input.original_evidence_receipt().decision_log_identity(),
            input.replayed_evidence_receipt().decision_log_identity(),
            PlanarBooleanLoopReplayParityDenialKind::DecisionLogMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_decision_logs();

        compare_row(
            PlanarBooleanLoopReplayParityRowKind::LoopLedgerReceipt,
            input.original_ledger_receipt().receipt_identity(),
            input.replayed_ledger_receipt().receipt_identity(),
            PlanarBooleanLoopReplayParityDenialKind::LoopLedgerMismatch,
            &mut counters,
            &mut rows,
        )?;
        compare_row(
            PlanarBooleanLoopReplayParityRowKind::DownstreamConsumption,
            input
                .original_ledger_receipt()
                .downstream_consumption_identity(),
            input
                .replayed_ledger_receipt()
                .downstream_consumption_identity(),
            PlanarBooleanLoopReplayParityDenialKind::LoopLedgerMismatch,
            &mut counters,
            &mut rows,
        )?;
        counters.compared_loop_ledgers();
        counters.compared_loop_ledgers();

        let checkpoint_receipt = ComparePlanarBooleanLoopCheckpointParity::compare(
            input.original_evidence_receipt(),
            input.replayed_evidence_receipt(),
            input.replay_receipts(),
            &mut counters,
        )?;
        rows.push(PlanarBooleanLoopReplayParityRow::new(
            PlanarBooleanLoopReplayParityRowKind::RetainedReplayCheckpoint,
            checkpoint_receipt.checkpoint_identity(),
            checkpoint_receipt.checkpoint_identity(),
        ));

        let replay_identity = replay_identity(&input, &checkpoint_receipt);
        Ok(PlanarBooleanLoopReplayParityReceipt::new(
            replay_identity,
            checkpoint_receipt,
            rows,
            counters,
        ))
    }
}

fn replay_identity(
    input: &PlanarBooleanLoopReplayParityInput<'_>,
    checkpoint_receipt: &super::checkpoint::PlanarBooleanLoopCheckpointParityReceipt,
) -> String {
    let parts = vec![
        "planar-boolean-loop-replay-parity".to_string(),
        format!(
            "loop-evidence:{}",
            input.original_evidence_receipt().receipt_identity()
        ),
        format!(
            "reconstructed:{}",
            input
                .original_evidence_receipt()
                .reconstructed_loop_set_identity()
        ),
        format!(
            "born:{}",
            input.original_evidence_receipt().born_loop_set_identity()
        ),
        format!(
            "islands:{}",
            input
                .original_evidence_receipt()
                .island_partition_identity()
        ),
        format!(
            "split-attribution:{}",
            input
                .original_evidence_receipt()
                .split_attribution_identity()
        ),
        format!(
            "role-outcomes:{}",
            input
                .original_evidence_receipt()
                .role_outcome_set_identity()
        ),
        format!(
            "degenerate-outcomes:{}",
            input
                .original_evidence_receipt()
                .degenerate_outcome_set_identity()
        ),
        format!(
            "decision-log:{}",
            input.original_evidence_receipt().decision_log_identity()
        ),
        format!(
            "loop-ledger:{}",
            input.original_ledger_receipt().receipt_identity()
        ),
        format!(
            "downstream-consumption:{}",
            input
                .original_ledger_receipt()
                .downstream_consumption_identity()
        ),
        format!("checkpoint:{}", checkpoint_receipt.checkpoint_identity()),
        format!(
            "replay-evidence:{}",
            checkpoint_receipt.replay_evidence_identity()
        ),
    ];
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn compare_row(
    kind: PlanarBooleanLoopReplayParityRowKind,
    original: &str,
    replayed: &str,
    denial_kind: PlanarBooleanLoopReplayParityDenialKind,
    counters: &mut PlanarBooleanLoopReplayParityCounters,
    rows: &mut Vec<PlanarBooleanLoopReplayParityRow>,
) -> Result<(), PlanarBooleanLoopReplayParityDenial> {
    if original != replayed {
        counters.rejected_replay_mismatch();
        return Err(PlanarBooleanLoopReplayParityDenial::new(
            denial_kind,
            original,
            replayed,
            *counters,
        ));
    }
    rows.push(PlanarBooleanLoopReplayParityRow::new(
        kind, original, replayed,
    ));
    Ok(())
}
