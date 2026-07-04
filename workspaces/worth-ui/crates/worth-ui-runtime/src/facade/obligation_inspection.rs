use crate::admission::{UiAdmissionTarget, UiAdmissionWorld};
use crate::facade::{UiInspectionReceipt, WorthUiApp};
use crate::obligations::touch::{
    UiGraphTouchAspectPosture, UiGraphTouchAspects, UiGraphTouchDescriptor, UiGraphTouchTiming,
};
use worth_ui_inspection::UiInspectionQuery;

impl WorthUiApp {
    pub(crate) fn inspect_retained_obligation_query(
        &self,
        query: UiInspectionQuery,
    ) -> Option<UiInspectionReceipt> {
        match query.target() {
            worth_ui_inspection::UiInspectionTarget::ObligationEvidenceHandle { handle_digest } => {
                let selected = self
                    .retained_obligation_registry()
                    .retained_selection(*handle_digest)?;
                let receipt = selected.inspect(query);
                self.retained_obligation_registry()
                    .register(&selected, &receipt);
                Some(receipt)
            }
            worth_ui_inspection::UiInspectionTarget::ObligationTouch {
                graph_node_digest,
                touch_identity_digest,
            } => self
                .canonical_touch_for_node(*graph_node_digest, Some(*touch_identity_digest))
                .map(|touch| self.inspect_selected_obligations(touch, query)),
            worth_ui_inspection::UiInspectionTarget::ObligationGraphNode { graph_node_digest } => {
                self.canonical_touch_for_node(*graph_node_digest, None)
                    .map(|touch| self.inspect_selected_obligations(touch, query))
            }
            _ => None,
        }
    }

    fn inspect_selected_obligations(
        &self,
        touch: UiGraphTouchDescriptor,
        query: UiInspectionQuery,
    ) -> UiInspectionReceipt {
        let target = UiAdmissionTarget::graph_node(
            touch.target().graph_node_identity(),
            UiAdmissionWorld::from_graph_world_profile(touch.world().world_profile().clone()),
        );
        let selected = self.admission().select_obligations_for_target(&touch, target);
        let receipt = selected.inspect(query);
        self.retained_obligation_registry()
            .register(&selected, &receipt);
        receipt
    }

    fn canonical_touch_for_node(
        &self,
        graph_node_digest: u64,
        expected_touch_identity_digest: Option<u64>,
    ) -> Option<UiGraphTouchDescriptor> {
        let graph_node_identity = crate::graph::UiGraphNodeIdentity::new(graph_node_digest);
        let query_touch = self.query_touch_for_node(graph_node_identity);
        if let Some(expected_digest) = expected_touch_identity_digest {
            if query_touch
                .as_ref()
                .is_some_and(|touch| touch.identity_digest() == expected_digest)
            {
                return query_touch;
            }
        } else if query_touch.is_some() {
            return query_touch;
        }

        self.structural_touch_for_node(graph_node_identity)
            .filter(|touch| {
                expected_touch_identity_digest
                    .is_none_or(|expected_digest| touch.identity_digest() == expected_digest)
            })
    }

    fn query_touch_for_node(
        &self,
        graph_node_identity: crate::graph::UiGraphNodeIdentity,
    ) -> Option<UiGraphTouchDescriptor> {
        let control_node = self.graph().lookup().graph_node(graph_node_identity)?.value();
        let transition = self.graph().mounted_receipt_transition_for_node(
            graph_node_identity,
            control_node
                .participation_posture()
                .axis(crate::graph::UiGraphParticipationAxis::Mounted),
            crate::graph::UiGraphAxisParticipation::runtime_mutation(
                crate::graph::UiGraphParticipationStatus::Admitted,
            ),
        )?;

        self.graph()
            .touches()
            .query_fact_change_receipt()
            .ok()
            .and_then(|origin| {
                self.graph()
                    .touches()
                    .from_mounted_receipt_transition(
                        origin,
                        UiGraphTouchTiming::PostMutation,
                        transition,
                        UiGraphTouchAspects::new()
                            .query_binding(UiGraphTouchAspectPosture::Invalidated)
                            .participation(UiGraphTouchAspectPosture::Invalidated),
                    )
                    .ok()
            })
    }

    fn structural_touch_for_node(
        &self,
        graph_node_identity: crate::graph::UiGraphNodeIdentity,
    ) -> Option<UiGraphTouchDescriptor> {
        let artifact = self.declaration_artifact_for_graph_node(graph_node_identity)?;

        self.graph()
            .touches()
            .declaration_change_receipt(artifact)
            .ok()
            .and_then(|origin| {
                self.graph()
                    .touches()
                    .from_node(
                        origin,
                        UiGraphTouchTiming::PostMutation,
                        graph_node_identity,
                        UiGraphTouchAspects::new()
                            .structural(UiGraphTouchAspectPosture::Invalidated),
                    )
                    .ok()
            })
    }
}
