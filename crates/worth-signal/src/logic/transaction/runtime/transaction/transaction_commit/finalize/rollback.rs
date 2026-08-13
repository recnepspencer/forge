use crate::diagnostics::{ExecutionFailureContext, ExecutionFailurePhase};

use super::super::super::transaction_types::{SignalTransaction, TransactionOutcome};

impl<'a, D, I, E, Ctx, T> SignalTransaction<'a, D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub(super) fn restore_baseline_if_requested(
        &mut self,
        restore_baseline: bool,
        outcome: TransactionOutcome,
    ) -> TransactionOutcome {
        if !restore_baseline {
            return outcome;
        }
        match self.apply_rollback_packets() {
            Ok(()) => outcome,
            Err(error) => {
                self.record_rollback_poison(&error);
                TransactionOutcome::Poisoned
            }
        }
    }

    fn record_rollback_poison(&mut self, error: &crate::data::error::SignalError) {
        self.telemetry.transaction.transaction_poison_count += 1;
        self.scratch.semantic_delta.failure_summary = Some(
            ExecutionFailureContext::from_error(ExecutionFailurePhase::Rollback, error, None)
                .summarize(
                    self.scratch.semantic_delta.rollback.as_ref(),
                    self.graph.diagnostics_profile(),
                ),
        );
        self.scratch.semantic_delta.replay_events.push(
            crate::logic::transaction::runtime::transaction::TransactionReplayEntry {
                kind: crate::diagnostics::replay::ReplayEventKind::FailureRecorded,
                detail: error.to_string(),
                execution_record_id: None,
                semantic_segment_id: None,
            },
        );
    }

    fn apply_rollback_packets(&mut self) -> Result<(), crate::data::error::SignalError> {
        let packets = self.rollback_packets.drain_ordered();
        self.telemetry.transaction.rollback_packet_breadth += packets.len() as u64;
        for packet in packets {
            self.apply_rollback_packet(packet)?;
        }
        Ok(())
    }

    fn apply_rollback_packet(
        &mut self,
        packet: crate::logic::transaction::runtime::transaction::TransactionRollbackPacket<T>,
    ) -> Result<(), crate::data::error::SignalError> {
        use crate::logic::transaction::runtime::transaction::TransactionRollbackPacket as Packet;
        match packet {
            Packet::Config(delta) => {
                self.telemetry.transaction.rollback_packet_config_count += 1;
                *self.config = delta.baseline;
            }
            Packet::DiagnosticsRequired(delta) => {
                self.telemetry.transaction.rollback_packet_diagnostics_count += 1;
                *self.graph.diagnostics_state_mut() = delta.baseline;
            }
            Packet::GraphPatches(delta) => {
                self.telemetry.transaction.rollback_packet_graph_patch_count += 1;
                delta.patches.rollback_from_packet(self.graph)?;
            }
            Packet::CreatedNodes(delta) => {
                self.telemetry
                    .transaction
                    .rollback_packet_created_node_count += 1;
                self.graph.rollback_created_nodes(&delta.created_nodes);
            }
            Packet::GraphCauseAuthority(delta) => {
                self.graph.cause_sets = delta.baseline;
                self.graph.cause_readmission_required = delta.readmission_required;
            }
            Packet::SubscriberRepair(delta) => {
                self.telemetry
                    .transaction
                    .rollback_packet_subscriber_repair_count += 1;
                self.graph
                    .reconcile_subscriber_membership_for_sources(&delta.sources)?;
            }
            Packet::Resource(delta) => {
                self.telemetry.transaction.rollback_packet_resource_count += 1;
                *self.resource = delta.baseline;
            }
            Packet::Temporal(delta) => {
                self.telemetry.transaction.rollback_packet_temporal_count += 1;
                *self.temporal = delta.baseline;
            }
        }
        Ok(())
    }
}
