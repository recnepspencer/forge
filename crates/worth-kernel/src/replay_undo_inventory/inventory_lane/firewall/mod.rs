mod current_manifests;
mod report;
mod violation;

use crate::replay_undo_inventory::inventory_lane::closeout::current_replay_undo_inventory_report;

pub use report::ReplayUndoSourceFirewallReport;
pub use violation::ReplayUndoSourceFirewallViolation;

pub fn current_replay_undo_source_firewall_report() -> ReplayUndoSourceFirewallReport {
    let closeout = current_replay_undo_inventory_report().expect("replay/undo closeout");
    ReplayUndoSourceFirewallReport::new(closeout)
}
