use worth_ui_certification::topology::audit_milestone_3103_phase4_watched_replacement;

use crate::workspace_source_inventory;

#[test]
fn milestone_3103_phase4_proves_watched_replacement_preservation_and_recovery() {
    audit_milestone_3103_phase4_watched_replacement(workspace_source_inventory())
        .expect("Milestone 3.10.3 Phase 4 watched-replacement contract should remain closed");
}
