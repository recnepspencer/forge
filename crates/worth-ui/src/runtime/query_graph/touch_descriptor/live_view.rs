use forge_query::facade::runtime::{ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily};

use crate::runtime::{
    WorthUiLiveViewConditionalProjectionGraphPosture, WorthUiLiveViewControlProjectionGraphPosture,
    WorthUiLiveViewEffectIntentGraphPosture, WorthUiLiveViewReadinessPosture,
    WorthUiLiveViewStateBindingGraphPosture, WorthUiRuntimeFactId,
};

use super::super::operation_declaration::{
    live_view_conditional_projection_touch_operations,
    live_view_control_projection_touch_operations, live_view_interaction_intent_touch_operations,
    live_view_expression_projection_touch_operations, live_view_payload_projection_touch_operations,
    live_view_readiness_projection_touch_operations, live_view_state_binding_touch_operations,
};
use super::{fact_paths, WorthUiQueryGraphTouchDescriptor};

impl WorthUiQueryGraphTouchDescriptor {
    pub fn live_view_state_binding(
        live_view_id: &str,
        target_binding_digest: u64,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        posture: WorthUiLiveViewStateBindingGraphPosture,
    ) -> Self {
        let mut touched_paths = fact_paths(dependency_facts);
        touched_paths.push(format!("target_binding_digest.{target_binding_digest}"));
        touched_paths.sort();
        touched_paths.dedup();
        let descriptor = forge_query::facade::runtime::ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::LIVE_VIEW_STATE_BINDING_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            live_view_state_binding_touch_operations(posture),
            touched_paths,
        )
        .expect("Worth live view descriptors use validated non-empty constants");
        Self {
            surface_id: live_view_id.to_owned(),
            interaction_id: format!("worth.ui.live-view-state-binding.{live_view_id}"),
            descriptor,
        }
    }

    pub fn live_view_control_projection(
        live_view_id: &str,
        control_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        posture: WorthUiLiveViewControlProjectionGraphPosture,
    ) -> Self {
        let touched_paths = sorted_fact_paths(dependency_facts);
        let descriptor = forge_query::facade::runtime::ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::LIVE_VIEW_CONTROL_PROJECTION_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            live_view_control_projection_touch_operations(posture),
            touched_paths,
        )
        .expect("Worth live view control descriptors use validated non-empty constants");
        Self {
            surface_id: live_view_id.to_owned(),
            interaction_id: format!("worth.ui.live-view-control.{live_view_id}.{control_id}"),
            descriptor,
        }
    }

    pub fn live_view_conditional_projection(
        live_view_id: &str,
        control_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        posture: WorthUiLiveViewConditionalProjectionGraphPosture,
    ) -> Self {
        let touched_paths = sorted_fact_paths(dependency_facts);
        let descriptor = forge_query::facade::runtime::ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::LIVE_VIEW_CONDITIONAL_PROJECTION_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            live_view_conditional_projection_touch_operations(posture),
            touched_paths,
        )
        .expect("Worth live view conditional descriptors use validated non-empty constants");
        Self {
            surface_id: live_view_id.to_owned(),
            interaction_id: format!("worth.ui.live-view-conditional.{live_view_id}.{control_id}"),
            descriptor,
        }
    }

    pub fn live_view_readiness_projection(
        live_view_id: &str,
        readiness_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> Self {
        let descriptor = live_view_projection_descriptor(
            Self::LIVE_VIEW_READINESS_PROJECTION_COLLECTION,
            live_view_readiness_projection_touch_operations(),
            dependency_facts,
        );
        Self {
            surface_id: live_view_id.to_owned(),
            interaction_id: format!("worth.ui.live-view-readiness.{live_view_id}.{readiness_id}"),
            descriptor,
        }
    }

    pub fn live_view_expression_projection(
        live_view_id: &str,
        expression_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> Self {
        let descriptor = live_view_projection_descriptor(
            Self::LIVE_VIEW_EXPRESSION_PROJECTION_COLLECTION,
            live_view_expression_projection_touch_operations(),
            dependency_facts,
        );
        Self {
            surface_id: live_view_id.to_owned(),
            interaction_id: format!("worth.ui.live-view-expression.{live_view_id}.{expression_id}"),
            descriptor,
        }
    }

    pub fn live_view_interaction_intent(
        live_view_id: &str,
        interaction_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        readiness: WorthUiLiveViewReadinessPosture,
        effect_posture: WorthUiLiveViewEffectIntentGraphPosture,
    ) -> Self {
        let descriptor = live_view_projection_descriptor(
            Self::LIVE_VIEW_INTERACTION_INTENT_COLLECTION,
            live_view_interaction_intent_touch_operations(readiness, effect_posture),
            dependency_facts,
        );
        Self {
            surface_id: live_view_id.to_owned(),
            interaction_id: format!(
                "worth.ui.live-view-interaction.{live_view_id}.{interaction_id}"
            ),
            descriptor,
        }
    }

    pub fn live_view_payload_projection(
        live_view_id: &str,
        payload_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> Self {
        let descriptor = live_view_projection_descriptor(
            Self::LIVE_VIEW_PAYLOAD_PROJECTION_COLLECTION,
            live_view_payload_projection_touch_operations(),
            dependency_facts,
        );
        Self {
            surface_id: live_view_id.to_owned(),
            interaction_id: format!("worth.ui.live-view-payload.{live_view_id}.{payload_id}"),
            descriptor,
        }
    }

    pub fn live_view_state_binding_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::LIVE_VIEW_STATE_BINDING_COLLECTION)
            .expect("Worth live view state binding selector uses validated non-empty constants")
    }

    pub fn live_view_control_projection_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::LIVE_VIEW_CONTROL_PROJECTION_COLLECTION)
            .expect("Worth live view control selector uses validated non-empty constants")
    }

    pub fn live_view_conditional_projection_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::LIVE_VIEW_CONDITIONAL_PROJECTION_COLLECTION)
            .expect("Worth live view conditional selector uses validated non-empty constants")
    }

    pub fn live_view_readiness_projection_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::LIVE_VIEW_READINESS_PROJECTION_COLLECTION)
            .expect("Worth live view readiness selector uses validated constants")
    }

    pub fn live_view_expression_projection_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::LIVE_VIEW_EXPRESSION_PROJECTION_COLLECTION)
            .expect("Worth live view expression selector uses validated constants")
    }

    pub fn live_view_interaction_intent_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::LIVE_VIEW_INTERACTION_INTENT_COLLECTION)
            .expect("Worth live view interaction selector uses validated constants")
    }

    pub fn live_view_payload_projection_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::LIVE_VIEW_PAYLOAD_PROJECTION_COLLECTION)
            .expect("Worth live view payload selector uses validated constants")
    }
}

fn live_view_projection_descriptor(
    collection: &str,
    operations: Vec<String>,
    dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
) -> forge_query::facade::runtime::ForgeQueryGraphTouchDescriptor {
    forge_query::facade::runtime::ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        collection,
        ForgeQueryMutationFamily::Update,
        None,
        operations,
        sorted_fact_paths(dependency_facts),
    )
    .expect("Worth live view projection descriptors use validated constants")
}

fn sorted_fact_paths(
    dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
) -> Vec<String> {
    let mut touched_paths = fact_paths(dependency_facts);
    touched_paths.sort();
    touched_paths.dedup();
    touched_paths
}
