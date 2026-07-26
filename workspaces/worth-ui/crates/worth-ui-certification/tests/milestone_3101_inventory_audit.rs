use worth_ui_certification::topology::audit_milestone_3101_inventory;

use crate::workspace_source_inventory;

#[test]
fn milestone_3101_phase1_inventories_are_exhaustive_and_adjudicated() {
    audit_milestone_3101_inventory(workspace_source_inventory())
        .expect("Milestone 3.10.1 Phase 1 inventory should remain closed");
}
