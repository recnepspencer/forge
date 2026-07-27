use worth_ui_certification::topology::audit_milestone_3102_pulse_seed;

use crate::workspace_source_inventory;

#[test]
fn milestone_3102_phase1_freezes_the_source_to_pixel_courtroom() {
    audit_milestone_3102_pulse_seed(workspace_source_inventory())
        .expect("Milestone 3.10.2 Phase 1 source-to-pixel courtroom should remain closed");
}
