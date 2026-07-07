use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    WorthWorkloadOrdinaryConsumerCutover, WorthWorkloadOrdinaryConsumerCutoverRow,
    WorthWorkloadOrdinaryConsumerSelectedPlanWitness,
};
use crate::workload_composition::BatchAdmissionExecutionReceipt;

impl WorthWorkloadOrdinaryConsumerCutover {
    pub(super) fn new(
        batch_execution_receipt: BatchAdmissionExecutionReceipt,
        mut rows: Vec<WorthWorkloadOrdinaryConsumerCutoverRow>,
    ) -> Self {
        rows.sort_by(|left, right| left.surface_name.cmp(&right.surface_name));
        let _cutover_digest = cutover_digest(&batch_execution_receipt, &rows);
        Self {
            batch_execution_receipt,
            rows,
        }
    }

    pub fn batch_execution_receipt(&self) -> &BatchAdmissionExecutionReceipt {
        &self.batch_execution_receipt
    }

    pub fn rows(&self) -> &[WorthWorkloadOrdinaryConsumerCutoverRow] {
        &self.rows
    }

    pub(crate) fn replay_undo_boundary_proof_digests(&self) -> Vec<String> {
        sorted_unique_selected_plan_values(self, |witness| {
            witness.replay_undo_boundary_proof_digest()
        })
    }

    pub(crate) fn transaction_packet_identities(&self) -> Vec<String> {
        sorted_unique_selected_plan_values(self, |witness| witness.transaction_packet_identity())
    }

    pub(crate) fn replay_scope_identities(&self) -> Vec<String> {
        sorted_unique_selected_plan_values(self, |witness| witness.replay_scope_identity())
    }

    pub(crate) fn undo_scope_identities(&self) -> Vec<String> {
        sorted_unique_selected_plan_values(self, |witness| witness.undo_scope_identity())
    }

    pub(crate) fn replay_undo_selected_plan_witness_count(&self) -> usize {
        self.rows
            .iter()
            .filter_map(WorthWorkloadOrdinaryConsumerCutoverRow::selected_plan_witness)
            .filter(|witness| witness.replay_undo_boundary_proof_digest().is_some())
            .count()
    }
}

fn cutover_digest(
    batch_execution_receipt: &BatchAdmissionExecutionReceipt,
    rows: &[WorthWorkloadOrdinaryConsumerCutoverRow],
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}:{}:{}",
                    row.surface_name(),
                    row.owner(),
                    row.blocker(),
                    row.removal_trigger(),
                    row.posture().as_str()
                )
            })
            .chain(rows.iter().filter_map(|row| {
                row.selected_plan_witness()
                    .map(|witness| selected_plan_witness_digest_row(row.surface_name(), witness))
            }))
            .chain(std::iter::once(format!(
                "batch-execution:{}",
                batch_execution_receipt.execution_receipt_digest()
            )))
            .chain(std::iter::once(
                "worth-kernel:ordinary-consumer-cutover:v1".to_string(),
            ))
            .collect::<Vec<_>>(),
    )
}

fn selected_plan_witness_digest_row(
    surface_name: &str,
    witness: &WorthWorkloadOrdinaryConsumerSelectedPlanWitness,
) -> String {
    format!(
        "selected-plan-witness:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        surface_name,
        witness.route_kind().as_str(),
        witness.route_lineage_digest(),
        witness.route_authority_digest(),
        witness
            .replay_undo_boundary_proof_digest()
            .unwrap_or("not-applicable"),
        witness
            .transaction_packet_identity()
            .unwrap_or("not-applicable"),
        witness.replay_scope_identity().unwrap_or("not-applicable"),
        witness.undo_scope_identity().unwrap_or("not-applicable"),
        witness.batch_execution_receipt_digest()
    )
}

fn sorted_unique_selected_plan_values(
    cutover: &WorthWorkloadOrdinaryConsumerCutover,
    select: impl Fn(&WorthWorkloadOrdinaryConsumerSelectedPlanWitness) -> Option<&str>,
) -> Vec<String> {
    let mut values = cutover
        .rows()
        .iter()
        .filter_map(WorthWorkloadOrdinaryConsumerCutoverRow::selected_plan_witness)
        .filter_map(select)
        .map(str::to_string)
        .collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}
