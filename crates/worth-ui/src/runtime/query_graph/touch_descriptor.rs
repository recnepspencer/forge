mod composition_access;
mod live_view;

use forge_query::facade::runtime::{
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily,
};

use super::touch_posture::{
    WorthUiPrimitiveContentGraphPosture, WorthUiPrimitiveEventGraphDispatchPosture,
};
use crate::capability::{ComponentId, SurfaceId};
use crate::runtime::{
    WorthUiInteractionKind, WorthUiInteractionOperabilityBasis, WorthUiInteractionReadiness,
    WorthUiInteractionTarget, WorthUiPrimitiveFocusPosture, WorthUiRuntimeFactId,
    WorthUiUserIntentOperationFamily, WorthUiUserIntentTargetPosture,
};

use super::operation_declaration::{
    composition_context_touch_operations, composition_topology_touch_operations,
    mounted_interaction_touch_operations, primitive_construction_touch_operations,
    primitive_content_touch_operations, primitive_event_touch_operations,
    user_intent_target_touch_operations,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryGraphTouchDescriptor {
    surface_id: String,
    interaction_id: String,
    descriptor: ForgeQueryGraphTouchDescriptor,
}

impl WorthUiQueryGraphTouchDescriptor {
    const PRIMITIVE_CONSTRUCTION_COLLECTION: &'static str = "worth-ui.primitive-construction";
    const MOUNTED_INTERACTION_COLLECTION: &'static str = "worth-ui.mounted-interaction-activation";
    const EVENT_DISPATCH_COLLECTION: &'static str = "worth-ui.primitive-event-dispatch";
    const CONTENT_ANATOMY_COLLECTION: &'static str = "worth-ui.primitive-content-anatomy";
    const USER_INTENT_TARGET_COLLECTION: &'static str = "worth-ui.user-intent-target-binding";
    const LIVE_VIEW_STATE_BINDING_COLLECTION: &'static str = "worth-ui.live-view-state-binding";
    const LIVE_VIEW_CONTROL_PROJECTION_COLLECTION: &'static str =
        "worth-ui.live-view-control-projection";
    const LIVE_VIEW_CONDITIONAL_PROJECTION_COLLECTION: &'static str =
        "worth-ui.live-view-conditional-projection";
    const LIVE_VIEW_EXPRESSION_PROJECTION_COLLECTION: &'static str =
        "worth-ui.live-view-expression-projection";
    const LIVE_VIEW_READINESS_PROJECTION_COLLECTION: &'static str =
        "worth-ui.live-view-readiness-projection";
    const LIVE_VIEW_INTERACTION_INTENT_COLLECTION: &'static str =
        "worth-ui.live-view-interaction-intent";
    const LIVE_VIEW_PAYLOAD_PROJECTION_COLLECTION: &'static str =
        "worth-ui.live-view-payload-projection";
    const COMPOSITION_TOPOLOGY_COLLECTION: &'static str = "worth-ui.composition-topology";
    const COMPOSITION_GRAPH_ACCESS_COLLECTION: &'static str = "worth-ui.composition-graph-access";
    const COMPOSITION_CONTEXT_COLLECTION: &'static str = "worth-ui.composition-context";
    const COMPOSITION_PARTICIPATION_COLLECTION: &'static str = "worth-ui.composition-participation";

    pub fn primitive_construction(
        surface_id: impl Into<String>,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> Self {
        let surface_id = surface_id.into();
        let touched_paths = dependency_facts
            .into_iter()
            .map(|fact| format!("{}.{}", fact.family().token(), fact.identity()))
            .collect::<Vec<_>>();
        let descriptor = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::PRIMITIVE_CONSTRUCTION_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            primitive_construction_touch_operations(),
            touched_paths,
        )
        .expect("Worth primitive construction descriptors use validated non-empty constants");
        Self {
            interaction_id: "worth.ui.primitive-construction".to_owned(),
            surface_id,
            descriptor,
        }
    }

    pub fn composition_topology(
        root_id: impl Into<String>,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> Self {
        let root_id = root_id.into();
        let mut touched_paths = fact_paths(dependency_facts);
        touched_paths.push(format!("composition_root.{root_id}"));
        touched_paths.sort();
        touched_paths.dedup();
        let descriptor = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::COMPOSITION_TOPOLOGY_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            composition_topology_touch_operations(),
            touched_paths,
        )
        .expect("Worth composition topology descriptors use validated non-empty constants");
        Self {
            interaction_id: format!("worth.ui.composition-topology.{root_id}"),
            surface_id: root_id,
            descriptor,
        }
    }

    pub fn composition_context_propagation(
        root_id: impl Into<String>,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> Self {
        let root_id = root_id.into();
        let mut touched_paths = fact_paths(dependency_facts);
        touched_paths.push(format!("composition_context_root.{root_id}"));
        touched_paths.sort();
        touched_paths.dedup();
        let descriptor = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::COMPOSITION_CONTEXT_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            composition_context_touch_operations(),
            touched_paths,
        )
        .expect("Worth composition context descriptors use validated non-empty constants");
        Self {
            interaction_id: format!("worth.ui.composition-context.{root_id}"),
            surface_id: root_id,
            descriptor,
        }
    }

    pub fn composition_participation(
        root_id: impl Into<String>,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> Self {
        let root_id = root_id.into();
        let mut touched_paths = fact_paths(dependency_facts);
        touched_paths.push(format!("composition_participation_root.{root_id}"));
        touched_paths.sort();
        touched_paths.dedup();
        let descriptor = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::COMPOSITION_PARTICIPATION_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            super::operation_declaration::composition_participation_touch_operations(),
            touched_paths,
        )
        .expect("Worth composition participation descriptors use validated non-empty constants");
        Self {
            interaction_id: format!("worth.ui.composition-participation.{root_id}"),
            surface_id: root_id,
            descriptor,
        }
    }

    pub fn mounted_interaction_activation(
        surface_id: &SurfaceId,
        interaction_id: &str,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        basis: WorthUiInteractionOperabilityBasis,
        readiness: WorthUiInteractionReadiness,
        kind: WorthUiInteractionKind,
        target: &WorthUiInteractionTarget,
        focus: WorthUiPrimitiveFocusPosture,
    ) -> Self {
        let touched_paths = dependency_facts
            .into_iter()
            .map(|fact| format!("{}.{}", fact.family().token(), fact.identity()))
            .collect::<Vec<_>>();
        let descriptor = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::MOUNTED_INTERACTION_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            mounted_interaction_touch_operations(basis, readiness, kind, target, focus),
            touched_paths,
        )
        .expect("Worth mounted interaction descriptors use validated non-empty constants");
        Self {
            surface_id: surface_id.as_str().to_owned(),
            interaction_id: interaction_id.to_owned(),
            descriptor,
        }
    }

    pub fn primitive_event_dispatch(
        surface_id: impl Into<String>,
        interaction_id: impl Into<String>,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        posture: WorthUiPrimitiveEventGraphDispatchPosture,
    ) -> Self {
        let surface_id = surface_id.into();
        let interaction_id = interaction_id.into();
        let touched_paths = dependency_facts
            .into_iter()
            .map(|fact| format!("{}.{}", fact.family().token(), fact.identity()))
            .collect::<Vec<_>>();
        let descriptor = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::EVENT_DISPATCH_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            primitive_event_touch_operations(posture),
            touched_paths,
        )
        .expect("Worth primitive event descriptors use validated non-empty constants");
        Self {
            surface_id,
            interaction_id,
            descriptor,
        }
    }

    pub fn primitive_content_anatomy(
        surface_id: impl Into<String>,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
        posture: WorthUiPrimitiveContentGraphPosture,
    ) -> Self {
        let surface_id = surface_id.into();
        let touched_paths = dependency_facts
            .into_iter()
            .map(|fact| format!("{}.{}", fact.family().token(), fact.identity()))
            .collect::<Vec<_>>();
        let descriptor = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::CONTENT_ANATOMY_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            primitive_content_touch_operations(posture),
            touched_paths,
        )
        .expect("Worth primitive content descriptors use validated non-empty constants");
        Self {
            interaction_id: "worth.ui.primitive-content-anatomy".to_owned(),
            surface_id,
            descriptor,
        }
    }

    pub fn user_intent_target_binding(
        slot_name: &str,
        surface_id: &SurfaceId,
        component_id: &ComponentId,
        operation_family: WorthUiUserIntentOperationFamily,
        posture: WorthUiUserIntentTargetPosture,
        dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>,
    ) -> Self {
        let mut touched_paths = dependency_facts
            .into_iter()
            .map(|fact| format!("{}.{}", fact.family().token(), fact.identity()))
            .collect::<Vec<_>>();
        touched_paths.push(format!("slot.{slot_name}"));
        touched_paths.sort();
        touched_paths.dedup();
        let descriptor = ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
            Self::USER_INTENT_TARGET_COLLECTION,
            ForgeQueryMutationFamily::Update,
            None,
            user_intent_target_touch_operations(operation_family, posture),
            touched_paths,
        )
        .expect("Worth user intent target descriptors use validated non-empty constants");
        Self {
            surface_id: surface_id.as_str().to_owned(),
            interaction_id: format!(
                "worth.ui.target.{:?}.{}",
                operation_family,
                component_id.as_str()
            ),
            descriptor,
        }
    }

    pub fn mounted_interaction_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::MOUNTED_INTERACTION_COLLECTION)
            .expect("Worth mounted interaction selector uses validated non-empty constants")
    }

    pub fn primitive_construction_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::PRIMITIVE_CONSTRUCTION_COLLECTION)
            .expect("Worth primitive construction selector uses validated non-empty constants")
    }

    pub fn composition_topology_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::COMPOSITION_TOPOLOGY_COLLECTION)
            .expect("Worth composition selector uses validated non-empty constants")
    }

    pub fn composition_graph_access_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::COMPOSITION_GRAPH_ACCESS_COLLECTION)
            .expect("Worth composition access selector uses validated non-empty constants")
    }

    pub fn composition_context_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::COMPOSITION_CONTEXT_COLLECTION)
            .expect("Worth composition context selector uses validated non-empty constants")
    }

    pub fn composition_participation_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::COMPOSITION_PARTICIPATION_COLLECTION)
            .expect("Worth composition participation selector uses validated constants")
    }

    pub(in crate::runtime::query_graph) fn operation_selector(
        operation_id: impl AsRef<str>,
    ) -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::declared_aspect_operation(operation_id.as_ref())
            .expect("Worth query graph operation selectors use validated non-empty constants")
    }

    pub fn primitive_event_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::EVENT_DISPATCH_COLLECTION)
            .expect("Worth primitive event selector uses validated non-empty constants")
    }

    pub fn primitive_content_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::CONTENT_ANATOMY_COLLECTION)
            .expect("Worth primitive content selector uses validated non-empty constants")
    }

    pub fn user_intent_target_collection_selector() -> ForgeQueryGraphTouchSelector {
        ForgeQueryGraphTouchSelector::collection(Self::USER_INTENT_TARGET_COLLECTION)
            .expect("Worth user intent target selector uses validated non-empty constants")
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn interaction_id(&self) -> &str {
        &self.interaction_id
    }

    pub fn descriptor(&self) -> &ForgeQueryGraphTouchDescriptor {
        &self.descriptor
    }

    pub fn descriptor_digest(&self) -> &str {
        self.descriptor.descriptor_digest()
    }
}

fn fact_paths(dependency_facts: impl IntoIterator<Item = WorthUiRuntimeFactId>) -> Vec<String> {
    dependency_facts
        .into_iter()
        .map(|fact| format!("{}.{}", fact.family().token(), fact.identity()))
        .collect()
}
