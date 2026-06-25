use crate::runtime::{WorthUiPrimitiveProofReceipt, WorthUiRuntimeFactId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveDrawPlanGraphBasis {
    surface_id: String,
    produced_fact: WorthUiRuntimeFactId,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    source_parse_count: usize,
    artifact_scan_count: usize,
}

impl WorthUiPrimitiveDrawPlanGraphBasis {
    pub(crate) fn from_primitive_receipt(receipt: &WorthUiPrimitiveProofReceipt) -> Self {
        let surface_id = receipt.surface_id().to_owned();
        Self {
            produced_fact: WorthUiRuntimeFactId::primitive_draw_plan(&surface_id),
            consumed_facts: vec![
                WorthUiRuntimeFactId::primitive_flow_layout(&surface_id),
                WorthUiRuntimeFactId::primitive_content(&surface_id),
            ],
            surface_id,
            source_parse_count: 0,
            artifact_scan_count: 0,
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn produced_fact(&self) -> &WorthUiRuntimeFactId {
        &self.produced_fact
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn source_parse_count(&self) -> usize {
        self.source_parse_count
    }

    pub fn artifact_scan_count(&self) -> usize {
        self.artifact_scan_count
    }
}
