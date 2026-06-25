use crate::runtime::reconciliation::WorthUiDurableStateReconciliationPlanner;
use crate::runtime::state_inventory::WorthUiDurableStateInventoryBuilder;
use crate::runtime::{
    WorthUiDurableStateInventory, WorthUiDurableStateReconciliationDenial,
    WorthUiDurableStateReconciliationPlan, WorthUiNodeReplacementPlan, WorthUiRuntimeHost,
};

impl WorthUiRuntimeHost {
    pub fn durable_state_inventory(&self) -> WorthUiDurableStateInventoryBuilder {
        WorthUiDurableStateInventoryBuilder::new()
    }

    pub fn reconcile_durable_state(
        &self,
        node_plan: &WorthUiNodeReplacementPlan,
        inventory: &WorthUiDurableStateInventory,
    ) -> Result<WorthUiDurableStateReconciliationPlan, WorthUiDurableStateReconciliationDenial>
    {
        WorthUiDurableStateReconciliationPlanner::reconcile(node_plan, inventory)
    }
}
