use worth_kernel::replay_undo_consumer_cutover::{
    ReplayUndoForbiddenConsumerSurfaceDenialLedger,
    ReplayUndoForbiddenConsumerSurfaceFirewallReport, ReplayUndoForbiddenConsumerSurfaceFirewallRow,
    ReplayUndoForbiddenConsumerSurfaceKind,
};

fn main() {
    let row = ReplayUndoForbiddenConsumerSurfaceFirewallRow::new(
        ReplayUndoForbiddenConsumerSurfaceKind::OldReplayHelper,
        "forged-source.rs",
        ".complete_boolean_chain_integration_handoff(",
        0,
        0,
    );
    let report = ReplayUndoForbiddenConsumerSurfaceFirewallReport::new(vec![row]);
    let _ledger = ReplayUndoForbiddenConsumerSurfaceDenialLedger::new(Vec::new(), report);
}
