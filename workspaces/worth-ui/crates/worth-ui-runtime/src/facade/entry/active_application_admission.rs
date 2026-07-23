use super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    /// Borrow admission authority from the generation currently executing.
    pub fn admission(&self) -> crate::admission::UiAdmissionBoundary<'_> {
        self.app.admission()
    }

    pub fn admit_query_measurement_eligibility_for_touch_from_settled_fact(
        &self,
        touch: &crate::obligations::touch::UiGraphTouchDescriptor,
        view_binding_id: crate::capability::ViewBindingId,
        fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
    ) -> Option<crate::admission::UiQueryMeasurementEligibility> {
        self.app
            .admit_query_measurement_eligibility_for_touch_from_settled_fact(
                touch,
                view_binding_id,
                fact,
            )
    }

    pub fn try_query_touch_for_node(
        &self,
        graph_node_identity: crate::graph::UiGraphNodeIdentity,
    ) -> Result<
        crate::obligations::touch::UiGraphTouchDescriptor,
        crate::obligations::touch::UiGraphTouchDenial,
    > {
        self.app.try_query_touch_for_node(graph_node_identity)
    }
}
