use worth_ui_certification::topology::audit_milestone_3103_phase3_external_world;

use crate::workspace_source_inventory;

#[test]
fn milestone_3103_phase3_proves_the_real_external_world_and_native_boundary() {
    audit_milestone_3103_phase3_external_world(workspace_source_inventory())
        .expect("Milestone 3.10.3 Phase 3 external-world contract should remain closed");
}
