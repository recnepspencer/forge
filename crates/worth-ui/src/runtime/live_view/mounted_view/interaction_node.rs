use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiAppearanceStatePosture, WorthUiCompositionNodeContextReceipt,
    WorthUiLiveViewCompositionChildBindingReceipt, WorthUiLiveViewInteractionIntentReceipt,
    WorthUiLiveViewProjectionRenderInteractionPosture, WorthUiPrimitiveColor,
    WorthUiPrimitiveEventCursor, WorthUiPrimitiveEventGeometryReceipt,
    WorthUiPrimitiveFocusPosture, WorthUiPrimitiveOperabilityPosture, WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedInteractionNodeReceipt {
    composition_child_binding: WorthUiLiveViewCompositionChildBindingReceipt,
    interaction: WorthUiLiveViewInteractionIntentReceipt,
    posture: WorthUiLiveViewProjectionRenderInteractionPosture,
    node_context: Option<WorthUiCompositionNodeContextReceipt>,
    contextual_event_posture: WorthUiMountedContextualEventPostureReceipt,
    style: WorthUiMountedInteractionStyleReceipt,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedContextualEventPostureReceipt {
    hover_enabled: bool,
    press_enabled: bool,
    focus: WorthUiPrimitiveFocusPosture,
    cursor: WorthUiPrimitiveEventCursor,
    activation_enabled: bool,
    context_digest: u64,
    event_geometry_digest: u64,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedInteractionStyleReceipt {
    padding_top_points: f32,
    padding_right_points: f32,
    padding_bottom_points: f32,
    padding_left_points: f32,
    border_width_points: f32,
    radius_points: f32,
    background_color: WorthUiPrimitiveColor,
    text_color: WorthUiPrimitiveColor,
    border_color: WorthUiPrimitiveColor,
    cursor: WorthUiPrimitiveEventCursor,
    style_digest: u64,
}

impl WorthUiMountedInteractionNodeReceipt {
    pub(super) fn from_parts_with_context(
        composition_child_binding: WorthUiLiveViewCompositionChildBindingReceipt,
        interaction: WorthUiLiveViewInteractionIntentReceipt,
        posture: WorthUiLiveViewProjectionRenderInteractionPosture,
        node_context: Option<WorthUiCompositionNodeContextReceipt>,
    ) -> Self {
        let context_suppressed = node_context
            .as_ref()
            .is_some_and(WorthUiCompositionNodeContextReceipt::suppresses_interaction);
        let contextual_event_posture = WorthUiMountedContextualEventPostureReceipt::from_parts(
            &interaction,
            posture,
            node_context.as_ref(),
        );
        let style = WorthUiMountedInteractionStyleReceipt::from_interaction(
            &interaction,
            posture,
            context_suppressed,
        );
        let consumed_facts = vec![
            WorthUiRuntimeFactId::live_view_interaction_intent(format!(
                "{}:{}",
                interaction.live_view_id(),
                interaction.interaction_id()
            )),
            WorthUiRuntimeFactId::primitive_flow_layout(subject_identity(&interaction)),
            WorthUiRuntimeFactId::primitive_appearance_state(subject_identity(&interaction)),
            WorthUiRuntimeFactId::primitive_event_geometry(subject_identity(&interaction)),
        ]
        .into_iter()
        .chain(
            node_context
                .as_ref()
                .into_iter()
                .flat_map(|context| context.consumed_facts().iter().cloned()),
        )
        .chain(composition_child_binding.consumed_facts().iter().cloned())
        .collect::<Vec<_>>();
        let receipt_digest = digest_parts([
            composition_child_binding.binding_digest().to_string(),
            interaction.interaction_intent_digest().to_string(),
            context_suppressed.to_string(),
            node_context
                .as_ref()
                .map(|context| context.receipt_digest())
                .unwrap_or_default()
                .to_string(),
            contextual_event_posture.receipt_digest().to_string(),
            style.style_digest().to_string(),
        ]);
        Self {
            composition_child_binding,
            interaction,
            posture,
            node_context,
            contextual_event_posture,
            style,
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn interaction(&self) -> &WorthUiLiveViewInteractionIntentReceipt {
        &self.interaction
    }

    pub fn composition_child_binding(&self) -> &WorthUiLiveViewCompositionChildBindingReceipt {
        &self.composition_child_binding
    }

    pub fn posture(&self) -> WorthUiLiveViewProjectionRenderInteractionPosture {
        self.posture
    }

    pub fn is_enabled(&self) -> bool {
        self.posture == WorthUiLiveViewProjectionRenderInteractionPosture::Enabled
            && !self.is_context_suppressed()
    }

    pub fn is_context_suppressed(&self) -> bool {
        self.node_context
            .as_ref()
            .is_some_and(WorthUiCompositionNodeContextReceipt::suppresses_interaction)
    }

    pub fn node_context(&self) -> Option<&WorthUiCompositionNodeContextReceipt> {
        self.node_context.as_ref()
    }

    pub fn contextual_event_posture(&self) -> &WorthUiMountedContextualEventPostureReceipt {
        &self.contextual_event_posture
    }

    pub fn style(&self) -> &WorthUiMountedInteractionStyleReceipt {
        &self.style
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiMountedContextualEventPostureReceipt {
    fn from_parts(
        interaction: &WorthUiLiveViewInteractionIntentReceipt,
        posture: WorthUiLiveViewProjectionRenderInteractionPosture,
        node_context: Option<&WorthUiCompositionNodeContextReceipt>,
    ) -> Self {
        let context_suppressed =
            node_context.is_some_and(|context| context.suppresses_interaction());
        let activation_enabled = posture
            == WorthUiLiveViewProjectionRenderInteractionPosture::Enabled
            && !context_suppressed;
        let hover_enabled = activation_enabled;
        let press_enabled = activation_enabled;
        let focus = if activation_enabled {
            WorthUiPrimitiveFocusPosture::Focusable
        } else {
            WorthUiPrimitiveFocusPosture::None
        };
        let cursor = if activation_enabled {
            interaction.event_geometry().cursor()
        } else {
            WorthUiPrimitiveEventCursor::Default
        };
        let context_digest = node_context
            .map(WorthUiCompositionNodeContextReceipt::receipt_digest)
            .unwrap_or_default();
        let event_geometry_digest = interaction.event_geometry().receipt_digest();
        let receipt_digest = digest_parts([
            "mounted_contextual_event_posture".to_owned(),
            interaction.interaction_intent_digest().to_string(),
            format!("{posture:?}"),
            context_suppressed.to_string(),
            context_digest.to_string(),
            event_geometry_digest.to_string(),
            format!("{cursor:?}"),
            format!("{focus:?}"),
        ]);
        Self {
            hover_enabled,
            press_enabled,
            focus,
            cursor,
            activation_enabled,
            context_digest,
            event_geometry_digest,
            receipt_digest,
        }
    }

    pub fn hover_enabled(&self) -> bool {
        self.hover_enabled
    }

    pub fn press_enabled(&self) -> bool {
        self.press_enabled
    }

    pub fn focus(&self) -> WorthUiPrimitiveFocusPosture {
        self.focus
    }

    pub fn cursor(&self) -> WorthUiPrimitiveEventCursor {
        self.cursor
    }

    pub fn activation_enabled(&self) -> bool {
        self.activation_enabled
    }

    pub fn context_digest(&self) -> u64 {
        self.context_digest
    }

    pub fn event_geometry_digest(&self) -> u64 {
        self.event_geometry_digest
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiMountedInteractionStyleReceipt {
    fn from_interaction(
        interaction: &WorthUiLiveViewInteractionIntentReceipt,
        posture: WorthUiLiveViewProjectionRenderInteractionPosture,
        context_suppressed: bool,
    ) -> Self {
        let padding = interaction.flow_layout().padding_edges();
        let appearance = interaction
            .appearance()
            .resolve_active(appearance_posture(posture, context_suppressed));
        let cursor = cursor_for_posture(interaction.event_geometry(), posture, context_suppressed);
        let style_digest = digest_parts([
            interaction.flow_layout().receipt_digest().to_string(),
            appearance.receipt_digest().to_string(),
            format!("{cursor:?}"),
        ]);
        Self {
            padding_top_points: padding.top(),
            padding_right_points: padding.right(),
            padding_bottom_points: padding.bottom(),
            padding_left_points: padding.left(),
            border_width_points: appearance.border_width_points(),
            radius_points: appearance.radius_points(),
            background_color: appearance.background_color(),
            text_color: appearance.text_color(),
            border_color: appearance.border_color(),
            cursor,
            style_digest,
        }
    }

    pub fn padding_top_points(&self) -> f32 {
        self.padding_top_points
    }

    pub fn padding_right_points(&self) -> f32 {
        self.padding_right_points
    }

    pub fn padding_bottom_points(&self) -> f32 {
        self.padding_bottom_points
    }

    pub fn padding_left_points(&self) -> f32 {
        self.padding_left_points
    }

    pub fn border_width_points(&self) -> f32 {
        self.border_width_points
    }

    pub fn radius_points(&self) -> f32 {
        self.radius_points
    }

    pub fn background_color(&self) -> WorthUiPrimitiveColor {
        self.background_color
    }

    pub fn text_color(&self) -> WorthUiPrimitiveColor {
        self.text_color
    }

    pub fn border_color(&self) -> WorthUiPrimitiveColor {
        self.border_color
    }

    pub fn cursor(&self) -> WorthUiPrimitiveEventCursor {
        self.cursor
    }

    pub fn style_digest(&self) -> u64 {
        self.style_digest
    }
}

fn appearance_posture(
    posture: WorthUiLiveViewProjectionRenderInteractionPosture,
    context_suppressed: bool,
) -> WorthUiAppearanceStatePosture {
    match (posture, context_suppressed) {
        (_, true) => WorthUiAppearanceStatePosture::disabled_posture(
            WorthUiPrimitiveOperabilityPosture::Disabled,
        ),
        (WorthUiLiveViewProjectionRenderInteractionPosture::Enabled, false) => {
            WorthUiAppearanceStatePosture::enabled_rest()
        }
        (WorthUiLiveViewProjectionRenderInteractionPosture::ReadinessDenied, false) => {
            WorthUiAppearanceStatePosture::disabled_posture(
                WorthUiPrimitiveOperabilityPosture::Disabled,
            )
        }
    }
}

fn cursor_for_posture(
    event_geometry: &WorthUiPrimitiveEventGeometryReceipt,
    posture: WorthUiLiveViewProjectionRenderInteractionPosture,
    context_suppressed: bool,
) -> WorthUiPrimitiveEventCursor {
    match (posture, context_suppressed) {
        (_, true) => WorthUiPrimitiveEventCursor::Default,
        (WorthUiLiveViewProjectionRenderInteractionPosture::Enabled, false) => {
            event_geometry.cursor()
        }
        (WorthUiLiveViewProjectionRenderInteractionPosture::ReadinessDenied, false) => {
            WorthUiPrimitiveEventCursor::Default
        }
    }
}

fn subject_identity(interaction: &WorthUiLiveViewInteractionIntentReceipt) -> String {
    format!(
        "{}:{}",
        interaction.live_view_id(),
        interaction.interaction_id()
    )
}
