use crate::runtime::{
    WorthUiPrimitiveContentGraphPosture, WorthUiPrimitiveContentIconRenderPosture,
    WorthUiPrimitiveContentReceipt, WorthUiQueryGraphExecutionReceipt,
    WorthUiRuntimeGraphAuthority,
};

impl WorthUiPrimitiveContentReceipt {
    pub fn query_graph_execution_receipt(&self) -> WorthUiQueryGraphExecutionReceipt {
        let posture = content_graph_posture(self);
        WorthUiRuntimeGraphAuthority::new()
            .plan_primitive_content_anatomy_graph_operation(
                self.dependency_fact().identity(),
                [self.dependency_fact().clone()],
                posture,
            )
            .into_execution_receipt()
    }
}

fn content_graph_posture(
    receipt: &WorthUiPrimitiveContentReceipt,
) -> WorthUiPrimitiveContentGraphPosture {
    match receipt.participation() {
        crate::runtime::WorthUiPrimitiveContentParticipationPosture::Denied => {
            return WorthUiPrimitiveContentGraphPosture::Denied;
        }
        crate::runtime::WorthUiPrimitiveContentParticipationPosture::Unsupported => {
            return WorthUiPrimitiveContentGraphPosture::UnsupportedCapability;
        }
        _ => {}
    }
    let icon_posture = receipt
        .items()
        .iter()
        .filter_map(|item| item.as_icon().map(|icon| icon.render_posture()))
        .next();
    match icon_posture {
        Some(WorthUiPrimitiveContentIconRenderPosture::NativeVector) => {
            WorthUiPrimitiveContentGraphPosture::NativeVector
        }
        Some(WorthUiPrimitiveContentIconRenderPosture::SymbolFallback) => {
            WorthUiPrimitiveContentGraphPosture::FallbackEligible
        }
        None => WorthUiPrimitiveContentGraphPosture::Accepted,
    }
}
