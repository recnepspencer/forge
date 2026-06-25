use super::{
    WorthUiMountedDiagnosticPanelNodeReceipt, WorthUiMountedIconNodeReceipt,
    WorthUiMountedMosaicRegionNodeReceipt, WorthUiMountedPortalHostNodeReceipt,
    WorthUiMountedTextNodeReceipt,
};

macro_rules! placeholder_node_getters {
    ($type_name:ident) => {
        impl $type_name {
            pub fn node_id(&self) -> &str {
                &self.node_id
            }

            pub fn semantic_slice(&self) -> &'static str {
                self.semantic_slice
            }

            pub fn receipt_digest(&self) -> u64 {
                self.receipt_digest
            }
        }
    };
}

placeholder_node_getters!(WorthUiMountedTextNodeReceipt);
placeholder_node_getters!(WorthUiMountedIconNodeReceipt);
placeholder_node_getters!(WorthUiMountedDiagnosticPanelNodeReceipt);
placeholder_node_getters!(WorthUiMountedPortalHostNodeReceipt);
placeholder_node_getters!(WorthUiMountedMosaicRegionNodeReceipt);
