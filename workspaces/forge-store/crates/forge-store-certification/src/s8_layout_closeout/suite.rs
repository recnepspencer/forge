use super::S8LayoutCloseoutDenial;
use crate::courtroom::layout::runtime_matrix::require_complete_layout_runtime_matrix;
use forge_store_physical_certification::layout_harness::runtime::LayoutRuntimeCoverageMatrix;
use forge_store_physical_certification::layout_harness::scenario_inventory::{
    verify_canonical_s8_layout_scenario_inventory, S8LayoutScenarioInventoryReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutCloseoutSuiteCertificate {
    inventory: S8LayoutScenarioInventoryReceipt,
}

pub fn certify_s8_layout_closeout_suite(
    runtime_matrix: &LayoutRuntimeCoverageMatrix,
) -> Result<S8LayoutCloseoutSuiteCertificate, S8LayoutCloseoutDenial> {
    require_complete_layout_runtime_matrix(runtime_matrix)
        .map_err(S8LayoutCloseoutDenial::RuntimeMatrixIncomplete)?;
    let inventory = verify_canonical_s8_layout_scenario_inventory()
        .map_err(|_| S8LayoutCloseoutDenial::CanonicalScenarioInventoryMismatch)?;
    Ok(S8LayoutCloseoutSuiteCertificate { inventory })
}

impl S8LayoutCloseoutSuiteCertificate {
    pub const fn inventory(&self) -> S8LayoutScenarioInventoryReceipt {
        self.inventory
    }
}
