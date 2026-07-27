use worth_ui_certification::topology::audit_milestone_3103_phase2_product_contract;

use crate::workspace_source_inventory;

#[test]
fn milestone_3103_phase2_freezes_the_live_product_contract_without_workflow_authority() {
    audit_milestone_3103_phase2_product_contract(workspace_source_inventory())
        .expect("Milestone 3.10.3 Phase 2 live product contract should remain closed");
}
