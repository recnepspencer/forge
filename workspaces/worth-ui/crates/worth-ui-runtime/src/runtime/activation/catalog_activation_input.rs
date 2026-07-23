pub(crate) struct UiAllocationCatalogDeltaActivationInput<'application> {
    pub(crate) admitted_delta: crate::graph::UiAdmittedAllocationCatalogDelta,
    pub(crate) active_graph: crate::graph::UiGraphSnapshot,
    pub(crate) graph_changed_nodes: std::collections::BTreeSet<crate::graph::UiGraphNodeIdentity>,
    pub(crate) boundary: crate::runtime::WorthUiFrameBoundary,
    pub(crate) lane_parity_report: Option<crate::runtime::WorthUiLaneParityReport>,
    pub(crate) candidate_query_binding: worth_ui_query_binding::WorthUiRuntimeQueryBinding,
    pub(crate) successor_planning_authority:
        std::rc::Rc<crate::runtime::WorthUiRetainedAllocationPlanningEvidenceRegistry>,
    pub(crate) application_publication: super::WorthUiPreparedApplicationPublication<'application>,
}
