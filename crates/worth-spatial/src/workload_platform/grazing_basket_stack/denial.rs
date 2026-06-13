use super::layer_scope::{BasketBoundaryScope, BasketLayerIndex};
use crate::workload_platform::user_response::WorthUserOutcomeCauseKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrazingBasketStackDenialKind {
    WrongTopologyPattern,
    WrongTopologyPosture,
    UnsupportedLayerProfile,
    MissingPlatformEvidence,
    LabelOnlyMotion,
    OpenBoundaryPerturbation,
    CrossLayerRetainedReplay,
    CrossLayerProjectionIdentity,
    SurfaceSupportSmuggling,
    CrossLayerParityLane,
    UnsupportedSurfaceFamily,
    StormExtractionSmuggling,
    FalseClosure,
    MissingLayerEvidence,
    MissingProjectionEvidence,
    MissingRetainedCheckpointEvidence,
    PredicateUncertain,
    WholeStackBroadening,
}

impl GrazingBasketStackDenialKind {
    pub fn cause_kind(self) -> WorthUserOutcomeCauseKind {
        match self {
            Self::LabelOnlyMotion => WorthUserOutcomeCauseKind::DeniedMovementOrRotation,
            Self::OpenBoundaryPerturbation => WorthUserOutcomeCauseKind::OverlapDenied,
            Self::CrossLayerRetainedReplay
            | Self::CrossLayerProjectionIdentity
            | Self::SurfaceSupportSmuggling
            | Self::CrossLayerParityLane
            | Self::StormExtractionSmuggling
            | Self::FalseClosure
            | Self::WholeStackBroadening => WorthUserOutcomeCauseKind::IntegrityMismatch,
            Self::WrongTopologyPattern
            | Self::WrongTopologyPosture
            | Self::UnsupportedLayerProfile
            | Self::UnsupportedSurfaceFamily => WorthUserOutcomeCauseKind::UnsupportedInput,
            Self::PredicateUncertain => WorthUserOutcomeCauseKind::PredicateUncertain,
            Self::MissingPlatformEvidence
            | Self::MissingLayerEvidence
            | Self::MissingProjectionEvidence
            | Self::MissingRetainedCheckpointEvidence => WorthUserOutcomeCauseKind::MissingEvidence,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrazingBasketStackDenial {
    kind: GrazingBasketStackDenialKind,
    source_layer: Option<BasketLayerIndex>,
    target_layer: Option<BasketLayerIndex>,
    boundary: Option<BasketBoundaryScope>,
    touched_layers: usize,
    evidence_digest: String,
    human_reason: String,
}

impl GrazingBasketStackDenial {
    pub(crate) fn new(
        kind: GrazingBasketStackDenialKind,
        source_layer: Option<BasketLayerIndex>,
        target_layer: Option<BasketLayerIndex>,
        boundary: Option<BasketBoundaryScope>,
        touched_layers: usize,
        evidence_digest: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source_layer,
            target_layer,
            boundary,
            touched_layers,
            evidence_digest: evidence_digest.into(),
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> GrazingBasketStackDenialKind {
        self.kind
    }

    pub fn source_layer(&self) -> Option<BasketLayerIndex> {
        self.source_layer
    }

    pub fn target_layer(&self) -> Option<BasketLayerIndex> {
        self.target_layer
    }

    pub fn boundary(&self) -> Option<&BasketBoundaryScope> {
        self.boundary.as_ref()
    }

    pub fn touched_layers(&self) -> usize {
        self.touched_layers
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
