use worth_kernel::replay_undo_consumer_cutover::{
    ReplayUndoForbiddenConsumerSurfaceDenialLedger,
    ReplayUndoForbiddenConsumerSurfaceFirewallReport, ReplayUndoForbiddenConsumerSurfaceFirewallRow,
    ReplayUndoForbiddenConsumerSurfaceKind,
};

fn main() {
    let row = ReplayUndoForbiddenConsumerSurfaceFirewallRow {
        kind: ReplayUndoForbiddenConsumerSurfaceKind::OldReplayHelper,
        scanned_source: "forged-source.rs",
        forbidden_pattern: ".complete_boolean_chain_integration_handoff(",
        ordinary_occurrence_count: 0,
        allowed_non_authority_occurrence_count: 0,
    };
    let report = ReplayUndoForbiddenConsumerSurfaceFirewallReport { rows: vec![row] };
    let _ledger = ReplayUndoForbiddenConsumerSurfaceDenialLedger {
        rows: Vec::new(),
        source_firewall: report,
    };
}
