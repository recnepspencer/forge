use worth_ui_certification::topology::audit_milestone_3103_phase1;

use crate::workspace_source_inventory;

#[test]
fn milestone_3103_phase1_freezes_executable_world_evidence_before_implementation() {
    audit_milestone_3103_phase1(workspace_source_inventory())
        .expect("Milestone 3.10.3 Phase 1 evidence and courtroom should remain closed");
}
