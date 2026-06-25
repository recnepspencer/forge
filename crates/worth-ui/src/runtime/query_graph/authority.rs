use crate::capability::{ComponentId, SurfaceId};
use crate::runtime::{
    WorthUiInteractionKind, WorthUiInteractionOperabilityBasis, WorthUiInteractionReadiness,
    WorthUiInteractionTarget, WorthUiLiveViewConditionalProjectionGraphPosture,
    WorthUiLiveViewControlProjectionGraphPosture, WorthUiLiveViewEffectIntentGraphPosture,
    WorthUiLiveViewReadinessPosture, WorthUiLiveViewStateBindingGraphPosture,
    WorthUiPrimitiveContentGraphPosture, WorthUiPrimitiveEventGraphDispatchPosture,
    WorthUiPrimitiveFocusPosture, WorthUiQueryGraphExecutionReceipt,
    WorthUiQueryGraphOperatingWorld, WorthUiQueryGraphTouchDescriptor, WorthUiRuntimeFactId,
    WorthUiRuntimeGraphAuthority, WorthUiUserIntentOperationFamily, WorthUiUserIntentTargetPosture,
};

use super::{
    WorthUiCompositionContextGraphPlan, WorthUiCompositionGraphAccessPlan,
    WorthUiCompositionParticipationGraphPlan, WorthUiCompositionTopologyGraphPlan,
    WorthUiLiveViewConditionalProjectionGraphPlan, WorthUiLiveViewControlProjectionGraphPlan,
    WorthUiLiveViewExpressionProjectionGraphPlan, WorthUiLiveViewInteractionIntentGraphPlan,
    WorthUiLiveViewPayloadProjectionGraphPlan, WorthUiLiveViewReadinessProjectionGraphPlan,
    WorthUiLiveViewStateBindingGraphPlan,
    WorthUiMountedInteractionGraphPlan, WorthUiPrimitiveConstructionGraphPlan,
    WorthUiPrimitiveContentAnatomyGraphPlan, WorthUiPrimitiveEventDispatchGraphPlan,
    WorthUiQueryGraphOperationPlan, WorthUiUserIntentTargetBindingGraphPlan,
};

impl WorthUiRuntimeGraphAuthority {
    pub fn plan_primitive_construction_graph_operation(
        &self,
        surface_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> WorthUiPrimitiveConstructionGraphPlan {
        let touch =
            WorthUiQueryGraphTouchDescriptor::primitive_construction(surface_id, dependency_facts);
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::primitive_construction(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_composition_topology_graph_operation(
        &self,
        root_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> WorthUiCompositionTopologyGraphPlan {
        let touch =
            WorthUiQueryGraphTouchDescriptor::composition_topology(root_id, dependency_facts);
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::composition_topology(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_composition_graph_access_operation(
        &self,
        root_id: &str,
        access_kind: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> WorthUiCompositionGraphAccessPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::composition_graph_access(
            root_id,
            access_kind,
            dependency_facts,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::composition_graph_access(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_composition_participation_graph_operation(
        &self,
        root_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> WorthUiCompositionParticipationGraphPlan {
        let touch =
            WorthUiQueryGraphTouchDescriptor::composition_participation(root_id, dependency_facts);
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::composition_participation(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_composition_context_graph_operation(
        &self,
        root_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> WorthUiCompositionContextGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::composition_context_propagation(
            root_id,
            dependency_facts,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::composition_context_propagation(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn plan_mounted_interaction_graph_operation(
        &self,
        surface_id: &SurfaceId,
        interaction_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        basis: WorthUiInteractionOperabilityBasis,
        readiness: WorthUiInteractionReadiness,
        kind: WorthUiInteractionKind,
        target: &WorthUiInteractionTarget,
        focus: WorthUiPrimitiveFocusPosture,
    ) -> WorthUiMountedInteractionGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::mounted_interaction_activation(
            surface_id,
            interaction_id,
            dependency_facts,
            basis,
            readiness,
            kind,
            target,
            focus,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::mounted_interaction_activation(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_primitive_event_dispatch_graph_operation(
        &self,
        surface_id: impl Into<String>,
        interaction_id: impl Into<String>,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        posture: WorthUiPrimitiveEventGraphDispatchPosture,
    ) -> WorthUiPrimitiveEventDispatchGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::primitive_event_dispatch(
            surface_id,
            interaction_id,
            dependency_facts,
            posture,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::primitive_event_dispatch(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_primitive_content_anatomy_graph_operation(
        &self,
        surface_id: impl Into<String>,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        posture: WorthUiPrimitiveContentGraphPosture,
    ) -> WorthUiPrimitiveContentAnatomyGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::primitive_content_anatomy(
            surface_id,
            dependency_facts,
            posture,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::primitive_content_anatomy(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_user_intent_target_binding_graph_operation(
        &self,
        slot_name: &str,
        surface_id: &SurfaceId,
        component_id: &ComponentId,
        operation_family: WorthUiUserIntentOperationFamily,
        posture: WorthUiUserIntentTargetPosture,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> WorthUiUserIntentTargetBindingGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::user_intent_target_binding(
            slot_name,
            surface_id,
            component_id,
            operation_family,
            posture,
            dependency_facts,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::user_intent_target_binding(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_live_view_state_binding_graph_operation(
        &self,
        live_view_id: &str,
        target_binding_digest: u64,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        posture: WorthUiLiveViewStateBindingGraphPosture,
    ) -> WorthUiLiveViewStateBindingGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::live_view_state_binding(
            live_view_id,
            target_binding_digest,
            dependency_facts,
            posture,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::live_view_state_binding(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_live_view_control_projection_graph_operation(
        &self,
        live_view_id: &str,
        control_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        posture: WorthUiLiveViewControlProjectionGraphPosture,
    ) -> WorthUiLiveViewControlProjectionGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::live_view_control_projection(
            live_view_id,
            control_id,
            dependency_facts,
            posture,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::live_view_control_projection(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_live_view_conditional_projection_graph_operation(
        &self,
        live_view_id: &str,
        control_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        posture: WorthUiLiveViewConditionalProjectionGraphPosture,
    ) -> WorthUiLiveViewConditionalProjectionGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::live_view_conditional_projection(
            live_view_id,
            control_id,
            dependency_facts,
            posture,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::live_view_conditional_projection(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_live_view_readiness_projection_graph_operation(
        &self,
        live_view_id: &str,
        readiness_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> WorthUiLiveViewReadinessProjectionGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::live_view_readiness_projection(
            live_view_id,
            readiness_id,
            dependency_facts,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::live_view_readiness_projection(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_live_view_expression_projection_graph_operation(
        &self,
        live_view_id: &str,
        expression_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> WorthUiLiveViewExpressionProjectionGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::live_view_expression_projection(
            live_view_id,
            expression_id,
            dependency_facts,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::live_view_expression_projection(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_live_view_interaction_intent_graph_operation(
        &self,
        live_view_id: &str,
        interaction_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        readiness: WorthUiLiveViewReadinessPosture,
        effect_posture: WorthUiLiveViewEffectIntentGraphPosture,
    ) -> WorthUiLiveViewInteractionIntentGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::live_view_interaction_intent(
            live_view_id,
            interaction_id,
            dependency_facts,
            readiness,
            effect_posture,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::live_view_interaction_intent(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }

    pub fn plan_live_view_payload_projection_graph_operation(
        &self,
        live_view_id: &str,
        payload_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> WorthUiLiveViewPayloadProjectionGraphPlan {
        let touch = WorthUiQueryGraphTouchDescriptor::live_view_payload_projection(
            live_view_id,
            payload_id,
            dependency_facts,
        );
        WorthUiQueryGraphOperationPlan::new(
            WorthUiQueryGraphExecutionReceipt::live_view_payload_projection(
                touch,
                WorthUiQueryGraphOperatingWorld::runtime_preview(),
            ),
        )
    }
}
