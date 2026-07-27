use super::WorthUiActiveApplicationSession;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiMountedAllocationProjectionInspectionDenial {
    NonFiniteGeometry,
    NegativeExtent,
}

/// SUPPORT AUTHORITY for inspecting committed allocation geometry independently
/// of mounted-frame translation.
pub trait WorthUiMountedAllocationInspectionCertificationExt {
    fn inspect_mounted_allocation_projection(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> Result<
        Option<worth_ui_host_contract::UiMountedAllocationProjection>,
        WorthUiMountedAllocationProjectionInspectionDenial,
    >;
}

impl WorthUiMountedAllocationInspectionCertificationExt for WorthUiActiveApplicationSession {
    fn inspect_mounted_allocation_projection(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> Result<
        Option<worth_ui_host_contract::UiMountedAllocationProjection>,
        WorthUiMountedAllocationProjectionInspectionDenial,
    > {
        self.application
            .mounted_allocation_projection_for(graph_node)
            .map_err(|denial| match denial {
                crate::runtime::UiMountedAllocationProjectionDenial::NonFiniteGeometry => {
                    WorthUiMountedAllocationProjectionInspectionDenial::NonFiniteGeometry
                }
                crate::runtime::UiMountedAllocationProjectionDenial::NegativeExtent => {
                    WorthUiMountedAllocationProjectionInspectionDenial::NegativeExtent
                }
            })
    }
}
