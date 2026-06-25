use crate::capability::ComponentId;
use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiLiveViewControlHostFrameReceipt, WorthUiLiveViewControlProjectionReceipt,
    WorthUiLiveViewInteractionIntentReceipt, WorthUiLiveViewParticipationReceipt,
    WorthUiLiveViewProjectionAdmissionReceipt, WorthUiRuntimeHost,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLiveViewProjectionRenderPlan {
    controls: Vec<WorthUiLiveViewProjectionRenderControl>,
    interactions: Vec<WorthUiLiveViewProjectionRenderInteraction>,
    consumers: Vec<WorthUiLiveViewProjectionConsumerRow>,
    render_plan_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLiveViewProjectionRenderControl {
    control: WorthUiLiveViewControlProjectionReceipt,
    participation: Option<WorthUiLiveViewParticipationReceipt>,
    host_frame: WorthUiLiveViewControlHostFrameReceipt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLiveViewProjectionRenderInteraction {
    interaction: WorthUiLiveViewInteractionIntentReceipt,
    posture: WorthUiLiveViewProjectionRenderInteractionPosture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewProjectionRenderInteractionPosture {
    Enabled,
    ReadinessDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewProjectionConsumerKind {
    FlowLayout,
    ContentAnatomy,
    Appearance,
    EventGeometry,
    Accessibility,
    Evidence,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewProjectionConsumerRow {
    control_id: String,
    kind: WorthUiLiveViewProjectionConsumerKind,
}

impl WorthUiRuntimeHost {
    pub fn plan_live_view_projection_render(
        &self,
        projection: &WorthUiLiveViewProjectionAdmissionReceipt,
    ) -> WorthUiLiveViewProjectionRenderPlan {
        WorthUiLiveViewProjectionRenderPlan::from_projection_admission(self, projection)
    }
}

impl WorthUiLiveViewProjectionRenderPlan {
    fn from_projection_admission(
        runtime: &WorthUiRuntimeHost,
        projection: &WorthUiLiveViewProjectionAdmissionReceipt,
    ) -> Self {
        let controls = projection
            .controls()
            .iter()
            .map(|control| {
                let participation = projection
                    .conditionals()
                    .iter()
                    .find(|conditional| conditional.control().control_id() == control.control_id())
                    .map(|conditional| conditional.participation().clone());
                WorthUiLiveViewProjectionRenderControl {
                    control: control.clone(),
                    participation: participation.clone(),
                    host_frame: runtime.resolve_live_view_control_host_frame_from_parts(
                        control.clone(),
                        participation,
                    ),
                }
            })
            .collect::<Vec<_>>();
        let consumers = controls
            .iter()
            .flat_map(|control| {
                WorthUiLiveViewProjectionConsumerKind::all()
                    .into_iter()
                    .map(|kind| WorthUiLiveViewProjectionConsumerRow {
                        control_id: control.control().control_id().to_owned(),
                        kind,
                    })
            })
            .collect::<Vec<_>>();
        let interactions = projection
            .interactions()
            .iter()
            .cloned()
            .map(|interaction| {
                let posture = if interaction.readiness().posture().is_enabled() {
                    WorthUiLiveViewProjectionRenderInteractionPosture::Enabled
                } else {
                    WorthUiLiveViewProjectionRenderInteractionPosture::ReadinessDenied
                };
                WorthUiLiveViewProjectionRenderInteraction {
                    interaction,
                    posture,
                }
            })
            .collect::<Vec<_>>();
        let render_plan_digest = digest_parts(
            controls
                .iter()
                .map(|control| control.control().control_projection_digest().to_string())
                .chain(
                    controls
                        .iter()
                        .map(|control| control.host_frame().frame_digest().to_string()),
                )
                .chain(
                    interactions
                        .iter()
                        .map(|row| row.interaction().interaction_intent_digest().to_string()),
                )
                .chain(consumers.iter().map(|consumer| {
                    format!("{}:{}", consumer.control_id(), consumer.kind().token())
                })),
        );
        Self {
            controls,
            interactions,
            consumers,
            render_plan_digest,
        }
    }

    pub fn controls(&self) -> &[WorthUiLiveViewProjectionRenderControl] {
        &self.controls
    }

    pub fn interactions(&self) -> &[WorthUiLiveViewProjectionRenderInteraction] {
        &self.interactions
    }

    pub fn consumers(&self) -> &[WorthUiLiveViewProjectionConsumerRow] {
        &self.consumers
    }

    pub fn render_plan_digest(&self) -> u64 {
        self.render_plan_digest
    }
}

impl WorthUiLiveViewProjectionRenderInteraction {
    pub fn interaction(&self) -> &WorthUiLiveViewInteractionIntentReceipt {
        &self.interaction
    }

    pub fn posture(&self) -> WorthUiLiveViewProjectionRenderInteractionPosture {
        self.posture
    }

    pub fn is_enabled(&self) -> bool {
        self.posture == WorthUiLiveViewProjectionRenderInteractionPosture::Enabled
    }
}

impl WorthUiLiveViewProjectionRenderControl {
    pub fn control(&self) -> &WorthUiLiveViewControlProjectionReceipt {
        &self.control
    }

    pub fn component_id(&self) -> &ComponentId {
        self.control.component_id()
    }

    pub fn participation(&self) -> Option<&WorthUiLiveViewParticipationReceipt> {
        self.participation.as_ref()
    }

    pub fn host_frame(&self) -> &WorthUiLiveViewControlHostFrameReceipt {
        &self.host_frame
    }
}

impl WorthUiLiveViewProjectionConsumerRow {
    pub fn control_id(&self) -> &str {
        &self.control_id
    }

    pub fn kind(&self) -> WorthUiLiveViewProjectionConsumerKind {
        self.kind
    }
}

impl WorthUiLiveViewProjectionConsumerKind {
    fn all() -> [Self; 6] {
        [
            Self::FlowLayout,
            Self::ContentAnatomy,
            Self::Appearance,
            Self::EventGeometry,
            Self::Accessibility,
            Self::Evidence,
        ]
    }

    pub fn token(self) -> &'static str {
        match self {
            Self::FlowLayout => "flow_layout",
            Self::ContentAnatomy => "content_anatomy",
            Self::Appearance => "appearance",
            Self::EventGeometry => "event_geometry",
            Self::Accessibility => "accessibility",
            Self::Evidence => "evidence",
        }
    }
}
