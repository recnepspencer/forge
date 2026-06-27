use super::denial_kind::{
    ReplayUndoForbiddenConsumerSurfaceEnforcement, ReplayUndoForbiddenConsumerSurfaceKind,
    ReplayUndoForbiddenConsumerSurfaceRow,
};
use super::source_firewall::{
    current_replay_undo_forbidden_surface_firewall_report,
    ReplayUndoForbiddenConsumerSurfaceFirewallReport,
};
use crate::replay_undo_consumer_cutover::error::{
    missing_forbidden_surface_denial, ReplayUndoConsumerCutoverError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoForbiddenConsumerSurfaceDenialLedger {
    rows: Vec<ReplayUndoForbiddenConsumerSurfaceRow>,
    source_firewall: ReplayUndoForbiddenConsumerSurfaceFirewallReport,
}

const REQUIRED_PHASE_ELEVEN_DENIALS: &[ReplayUndoForbiddenConsumerSurfaceKind] = &[
    ReplayUndoForbiddenConsumerSurfaceKind::OldReplayHelper,
    ReplayUndoForbiddenConsumerSurfaceKind::BroadTopologyRediscovery,
    ReplayUndoForbiddenConsumerSurfaceKind::BroadEvidenceRediscovery,
    ReplayUndoForbiddenConsumerSurfaceKind::RawReceiptAdmission,
    ReplayUndoForbiddenConsumerSurfaceKind::LocalRollbackShortcut,
];

pub fn current_replay_undo_forbidden_surface_denial_ledger(
) -> ReplayUndoForbiddenConsumerSurfaceDenialLedger {
    ReplayUndoForbiddenConsumerSurfaceDenialLedger::new(
        vec![
            ReplayUndoForbiddenConsumerSurfaceRow::new(
                ReplayUndoForbiddenConsumerSurfaceKind::OldReplayHelper,
                "CompletedBooleanLoopReconstructionHandoff::complete_boolean_chain_integration_handoff",
                ReplayUndoForbiddenConsumerSurfaceEnforcement::SourceFirewall,
                "ordinary chain closeout requires replay/undo transaction packet",
            ),
            ReplayUndoForbiddenConsumerSurfaceRow::new(
                ReplayUndoForbiddenConsumerSurfaceKind::BroadTopologyRediscovery,
                "tests/fixtures/replay_undo_semantic_graph/topology_replay_scope_not_from_spatial_lookup_receipt.rs",
                ReplayUndoForbiddenConsumerSurfaceEnforcement::CompileFail,
                "topology replay scope must enter through admitted topology replay products",
            ),
            ReplayUndoForbiddenConsumerSurfaceRow::new(
                ReplayUndoForbiddenConsumerSurfaceKind::BroadEvidenceRediscovery,
                "tests/fixtures/replay_undo_semantic_graph/lookup_consumed_workload_handoff_not_hand_filled.rs",
                ReplayUndoForbiddenConsumerSurfaceEnforcement::CompileFail,
                "spatial replay scope must consume lookup handoff proof, not rebuilt evidence rows",
            ),
            ReplayUndoForbiddenConsumerSurfaceRow::new(
                ReplayUndoForbiddenConsumerSurfaceKind::RawReceiptAdmission,
                "tests/fixtures/replay_undo_semantic_graph/spatial_replay_scope_not_from_raw_stage_index_identity.rs",
                ReplayUndoForbiddenConsumerSurfaceEnforcement::CompileFail,
                "stage identity must come from WorkloadEvidenceStageIndexProduct",
            ),
            ReplayUndoForbiddenConsumerSurfaceRow::new(
                ReplayUndoForbiddenConsumerSurfaceKind::LocalRollbackShortcut,
                "legacy packetless loop closeout denial",
                ReplayUndoForbiddenConsumerSurfaceEnforcement::SourceFirewall,
                "rollback consumers must carry the replay/undo transaction boundary packet",
            ),
        ],
        current_replay_undo_forbidden_surface_firewall_report(),
    )
}

impl ReplayUndoForbiddenConsumerSurfaceDenialLedger {
    pub(crate) fn new(
        rows: Vec<ReplayUndoForbiddenConsumerSurfaceRow>,
        source_firewall: ReplayUndoForbiddenConsumerSurfaceFirewallReport,
    ) -> Self {
        Self {
            rows,
            source_firewall,
        }
    }

    pub fn require_phase_eleven_denials(&self) -> Result<(), ReplayUndoConsumerCutoverError> {
        for required_kind in REQUIRED_PHASE_ELEVEN_DENIALS {
            if !self.rows.iter().any(|row| {
                row.kind() == *required_kind
                    && !row.surface().is_empty()
                    && !row.removal_trigger().is_empty()
            }) {
                return Err(missing_forbidden_surface_denial(*required_kind));
            }
        }
        self.source_firewall.require_clean()?;
        Ok(())
    }

    pub fn rows(&self) -> &[ReplayUndoForbiddenConsumerSurfaceRow] {
        &self.rows
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub const fn source_firewall(&self) -> &ReplayUndoForbiddenConsumerSurfaceFirewallReport {
        &self.source_firewall
    }
}
