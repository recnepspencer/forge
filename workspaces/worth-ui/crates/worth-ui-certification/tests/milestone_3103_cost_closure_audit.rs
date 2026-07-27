use worth_ui_certification::topology::audit_milestone_3103_phase5_cost_closure;

use crate::workspace_source_inventory;

#[test]
fn milestone_3103_phase5_closes_cost_platform_and_successor_handoffs() {
    audit_milestone_3103_phase5_cost_closure(workspace_source_inventory())
        .expect("Milestone 3.10.3 Phase 5 cost and successor handoff should remain closed");
}
