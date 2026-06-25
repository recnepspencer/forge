mod changed_fact_evidence_row;
mod rebind_plan;

use crate::runtime::{
    WorthUiPrimitiveProofTargetBinding, WorthUiRuntimeHost,
    WorthUiValidationChangedFactMappingReceipt,
};

pub use changed_fact_evidence_row::WorthUiPrimitiveChangedFactEvidenceRow;
pub use rebind_plan::WorthUiPrimitiveProjectionRebindPlan;

use super::{WorthUiPrimitiveProofDenial, WorthUiPrimitiveProofReceipt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPrimitiveProjectionRebindStatus {
    Rebound,
    Unchanged,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveProjectionReceipt {
    primitive_receipt: WorthUiPrimitiveProofReceipt,
    rebind_status: WorthUiPrimitiveProjectionRebindStatus,
    rebind_plan: WorthUiPrimitiveProjectionRebindPlan,
    changed_rows: Vec<WorthUiPrimitiveChangedFactEvidenceRow>,
}

impl WorthUiRuntimeHost {
    pub fn resolve_primitive_projection_for_target(
        &self,
        target: &WorthUiPrimitiveProofTargetBinding,
        changed_fact_mapping: Option<&WorthUiValidationChangedFactMappingReceipt>,
    ) -> Result<WorthUiPrimitiveProjectionReceipt, WorthUiPrimitiveProofDenial> {
        let surface_id = target.surface_id();
        let primitive_receipt = self.resolve_primitive_proof_for_target(target)?;
        let changed_rows = changed_fact_mapping
            .map(|mapping| primitive_changed_rows(mapping, surface_id.as_str()))
            .unwrap_or_default();
        let rebind_plan = WorthUiPrimitiveProjectionRebindPlan::from_changed_rows(
            primitive_receipt.dependency_facts(),
            &changed_rows,
        );
        let rebind_status = if rebind_plan.has_rebuilt_facts() {
            WorthUiPrimitiveProjectionRebindStatus::Rebound
        } else {
            WorthUiPrimitiveProjectionRebindStatus::Unchanged
        };
        Ok(WorthUiPrimitiveProjectionReceipt::new(
            primitive_receipt,
            rebind_status,
            rebind_plan,
            changed_rows,
        ))
    }
}

impl WorthUiPrimitiveProjectionReceipt {
    pub(crate) fn new(
        primitive_receipt: WorthUiPrimitiveProofReceipt,
        rebind_status: WorthUiPrimitiveProjectionRebindStatus,
        rebind_plan: WorthUiPrimitiveProjectionRebindPlan,
        changed_rows: Vec<WorthUiPrimitiveChangedFactEvidenceRow>,
    ) -> Self {
        Self {
            primitive_receipt,
            rebind_status,
            rebind_plan,
            changed_rows,
        }
    }

    pub fn primitive_receipt(&self) -> &WorthUiPrimitiveProofReceipt {
        &self.primitive_receipt
    }

    pub fn rebind_status(&self) -> WorthUiPrimitiveProjectionRebindStatus {
        self.rebind_status
    }

    pub fn rebind_plan(&self) -> &WorthUiPrimitiveProjectionRebindPlan {
        &self.rebind_plan
    }

    pub fn changed_rows(&self) -> &[WorthUiPrimitiveChangedFactEvidenceRow] {
        &self.changed_rows
    }
}

fn primitive_changed_rows(
    mapping: &WorthUiValidationChangedFactMappingReceipt,
    surface_id: &str,
) -> Vec<WorthUiPrimitiveChangedFactEvidenceRow> {
    mapping
        .rows()
        .iter()
        .filter_map(|row| {
            WorthUiPrimitiveChangedFactEvidenceRow::from_changed_fact_row(row, surface_id)
        })
        .collect()
}
