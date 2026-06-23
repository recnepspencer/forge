use crate::capability::{
    AppearanceTokenId, CommandId, CommandProjectionId, ComponentId, DensityTokenId, SurfaceId,
    ThemeTokenId, ViewBindingId,
};
use crate::runtime::{
    WorthUiActionPostureId, WorthUiAppearanceRecipeId, WorthUiContentSlotId,
    WorthUiDurableStateFamilyId, WorthUiInspectorSurfaceId, WorthUiOverlaySurfaceId,
    WorthUiPageInstanceId, WorthUiPageTemplateId, WorthUiShellSurfaceId, WorthUiToastSurfaceId,
    WorthUiVirtualizedDataFrameTarget,
};

use super::WorthUiRuntimeFactFamily;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiRuntimeFactId {
    family: WorthUiRuntimeFactFamily,
    identity: String,
}

impl WorthUiRuntimeFactId {
    pub fn active_artifact() -> Self {
        Self::new(WorthUiRuntimeFactFamily::ActiveArtifact, "active")
    }

    pub fn execution_plan() -> Self {
        Self::new(WorthUiRuntimeFactFamily::ExecutionPlan, "active")
    }

    pub fn theme_token(token_id: &ThemeTokenId) -> Self {
        Self::new(WorthUiRuntimeFactFamily::ThemeToken, token_id.as_str())
    }

    pub fn command(command_id: &CommandId) -> Self {
        Self::new(WorthUiRuntimeFactFamily::Command, command_id.as_str())
    }

    pub fn command_projection(projection_id: &CommandProjectionId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::CommandProjection,
            projection_id.as_str(),
        )
    }

    pub fn dropdown_selection_state(projection_id: &CommandProjectionId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::DropdownSelectionState,
            projection_id.as_str(),
        )
    }

    pub fn component_interaction_state(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::ComponentInteractionState,
            identity,
        )
    }

    pub fn command_projection_interaction_policy(projection_id: &CommandProjectionId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::InteractionPolicy,
            projection_id.as_str(),
        )
    }

    pub fn query_binding(view_binding_id: &ViewBindingId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryBinding,
            view_binding_id.as_str(),
        )
    }

    pub fn query_result_posture(view_binding_id: &ViewBindingId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryResultPosture,
            view_binding_id.as_str(),
        )
    }

    pub fn query_projection_fact(receipt_identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryProjectionFact,
            receipt_identity,
        )
    }

    pub fn query_live_view(view_binding_id: &ViewBindingId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::LiveViewBinding,
            view_binding_id.as_str(),
        )
    }

    pub fn live_view_binding(view_binding_id: &ViewBindingId) -> Self {
        Self::query_live_view(view_binding_id)
    }

    pub fn query_computed_view(view_binding_id: &ViewBindingId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryComputedView,
            view_binding_id.as_str(),
        )
    }

    pub fn query_state_snapshot(snapshot_identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryStateSnapshot,
            snapshot_identity,
        )
    }

    pub fn query_effect_posture(effect_identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryEffectPosture,
            effect_identity,
        )
    }

    pub fn query_recovery_posture(recovery_identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryRecoveryPosture,
            recovery_identity,
        )
    }

    pub fn query_inspection_target(inspection_identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::QueryInspectionTarget,
            inspection_identity,
        )
    }

    pub fn layout_padding(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::LayoutPadding, identity)
    }

    pub fn layout_gap(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::LayoutGap, identity)
    }

    pub fn component(component_id: &ComponentId) -> Self {
        Self::new(WorthUiRuntimeFactFamily::Component, component_id.as_str())
    }

    pub fn shell_surface(surface_id: &WorthUiShellSurfaceId) -> Self {
        Self::new(WorthUiRuntimeFactFamily::ShellSurface, surface_id.as_str())
    }

    pub fn page_template(page_template_id: &WorthUiPageTemplateId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::PageTemplate,
            page_template_id.as_str(),
        )
    }

    pub fn page_instance(page_instance_id: &WorthUiPageInstanceId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::PageInstance,
            page_instance_id.as_str(),
        )
    }

    pub fn page_instance_template_binding(
        page_instance_id: &WorthUiPageInstanceId,
        page_template_id: &WorthUiPageTemplateId,
    ) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::PageInstanceTemplateBinding,
            joined_identity([page_instance_id.as_str(), page_template_id.as_str()]),
        )
    }

    pub fn page_content_slot(
        page_template_id: &WorthUiPageTemplateId,
        content_slot_id: &WorthUiContentSlotId,
    ) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::PageContentSlot,
            joined_identity([page_template_id.as_str(), content_slot_id.as_str()]),
        )
    }

    pub fn surface_mount(surface_id: &SurfaceId) -> Self {
        Self::new(WorthUiRuntimeFactFamily::SurfaceMount, surface_id.as_str())
    }

    pub fn surface_mount_raw(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::SurfaceMount, identity)
    }

    pub fn layout_topology(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::LayoutTopology, identity)
    }

    pub fn content_mount(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::ContentMount, identity)
    }

    pub fn shell_slot_assignment(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::ShellSlotAssignment, identity)
    }

    pub fn authored_mount_component_selection(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::AuthoredMountComponentSelection,
            identity,
        )
    }

    pub fn authored_surface_props(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::AuthoredSurfaceProps, identity)
    }

    pub fn primitive_content(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::PrimitiveContent, identity)
    }

    pub fn primitive_container(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::PrimitiveContainer, identity)
    }

    pub fn primitive_measurement(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::PrimitiveMeasurement, identity)
    }

    pub fn primitive_appearance(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::PrimitiveAppearance, identity)
    }

    pub fn primitive_appearance_state(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::PrimitiveAppearanceState, identity)
    }

    pub fn primitive_interaction(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::PrimitiveInteraction, identity)
    }

    pub fn primitive_motion(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::PrimitiveMotion, identity)
    }

    pub fn primitive_flow_layout(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::PrimitiveFlowLayout, identity)
    }

    pub fn primitive_event_geometry(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::PrimitiveEventGeometry, identity)
    }

    pub fn authored_query_binding_shape(identity: impl Into<String>) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::AuthoredQueryBindingShape,
            identity,
        )
    }

    pub fn appearance(identity: impl Into<String>) -> Self {
        Self::new(WorthUiRuntimeFactFamily::Appearance, identity)
    }

    pub fn appearance_token(appearance_token_id: &AppearanceTokenId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::Appearance,
            appearance_token_id.as_str(),
        )
    }

    pub fn appearance_recipe(recipe_id: &WorthUiAppearanceRecipeId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::AppearanceRecipe,
            recipe_id.as_str(),
        )
    }

    pub fn density_token(density_token_id: &DensityTokenId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::DensityToken,
            density_token_id.as_str(),
        )
    }

    pub fn action_posture(action_posture_id: &WorthUiActionPostureId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::ActionPosture,
            action_posture_id.as_str(),
        )
    }

    pub fn virtualized_data_frame(target: WorthUiVirtualizedDataFrameTarget) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::VirtualizedDataFrame,
            target.digest_basis(),
        )
    }

    pub fn durable_state_family(family_id: &WorthUiDurableStateFamilyId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::DurableStateFamily,
            durable_state_identity(family_id),
        )
    }

    pub fn overlay_surface(surface_id: &WorthUiOverlaySurfaceId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::OverlaySurface,
            surface_id.as_str(),
        )
    }

    pub fn toast_surface(surface_id: &WorthUiToastSurfaceId) -> Self {
        Self::new(WorthUiRuntimeFactFamily::ToastSurface, surface_id.as_str())
    }

    pub fn inspector_surface(surface_id: &WorthUiInspectorSurfaceId) -> Self {
        Self::new(
            WorthUiRuntimeFactFamily::InspectorSurface,
            surface_id.as_str(),
        )
    }

    pub fn family(&self) -> WorthUiRuntimeFactFamily {
        self.family
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    fn new(family: WorthUiRuntimeFactFamily, identity: impl Into<String>) -> Self {
        Self {
            family,
            identity: identity.into(),
        }
    }
}

fn joined_identity<const N: usize>(parts: [&str; N]) -> String {
    parts
        .into_iter()
        .map(|part| format!("{}:{part}", part.len()))
        .collect::<Vec<_>>()
        .join("|")
}

fn durable_state_identity(family_id: &WorthUiDurableStateFamilyId) -> String {
    match family_id {
        WorthUiDurableStateFamilyId::FocusChain => "platform:focus_chain".to_string(),
        WorthUiDurableStateFamilyId::ScrollAnchor => "platform:scroll_anchor".to_string(),
        WorthUiDurableStateFamilyId::SelectionRange => "platform:selection_range".to_string(),
        WorthUiDurableStateFamilyId::TextEditBuffer => "platform:text_edit_buffer".to_string(),
        WorthUiDurableStateFamilyId::SplitterPosition => "platform:splitter_position".to_string(),
        WorthUiDurableStateFamilyId::TabState => "platform:tab_state".to_string(),
        WorthUiDurableStateFamilyId::PanelVisibility => "platform:panel_visibility".to_string(),
        WorthUiDurableStateFamilyId::Custom(id) => format!("custom:{}:{id}", id.len()),
    }
}
