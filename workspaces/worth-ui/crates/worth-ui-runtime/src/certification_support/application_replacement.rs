use crate::facade::entry::WorthUiPreparedApplicationReplacement;
use crate::facade::graph::{UiGraphNodeIdentity, UiGraphTouchDenial, UiGraphTouchDescriptor};

/// Certification-only observation of candidate-owned allocation authority.
pub trait WorthUiApplicationReplacementCertificationExt {
    fn candidate_allocation_touch_for_node(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Result<UiGraphTouchDescriptor, UiGraphTouchDenial>;
}

impl WorthUiApplicationReplacementCertificationExt for WorthUiPreparedApplicationReplacement {
    fn candidate_allocation_touch_for_node(
        &self,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Result<UiGraphTouchDescriptor, UiGraphTouchDenial> {
        self.try_candidate_allocation_touch_for_node(graph_node_identity)
    }
}
