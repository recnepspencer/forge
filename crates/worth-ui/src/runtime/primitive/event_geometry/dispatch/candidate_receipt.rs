use super::super::super::{
    WorthUiPrimitiveActivationPosture, WorthUiPrimitiveResolvedCursorPosture,
};
use super::super::receipt::WorthUiPrimitiveEventContainment;
use super::region_receipt::{WorthUiPrimitiveEventRegionOrder, WorthUiPrimitiveEventRegionReceipt};

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiPrimitiveEventDispatchCandidateReceipt {
    NoHit(WorthUiPrimitiveNoHitCandidateReceipt),
    DisabledHit(WorthUiPrimitiveDisabledHitCandidateReceipt),
    EnabledPrimaryHit(WorthUiPrimitiveEnabledHitCandidateReceipt),
    CursorTarget(WorthUiPrimitiveCursorTargetCandidateReceipt),
    BubbledAncestor(WorthUiPrimitiveBubbledCandidateReceipt),
    CapturedTarget(WorthUiPrimitiveCapturedCandidateReceipt),
    PassThrough(WorthUiPrimitivePassThroughCandidateReceipt),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveNoHitCandidateReceipt {
    region: WorthUiPrimitiveEventCandidateRegion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveDisabledHitCandidateReceipt {
    region: WorthUiPrimitiveEventCandidateRegion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveEnabledHitCandidateReceipt {
    region: WorthUiPrimitiveEventCandidateRegion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveCursorTargetCandidateReceipt {
    region: WorthUiPrimitiveEventCandidateRegion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveBubbledCandidateReceipt {
    region: WorthUiPrimitiveEventCandidateRegion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveCapturedCandidateReceipt {
    region: WorthUiPrimitiveEventCandidateRegion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitivePassThroughCandidateReceipt {
    region: WorthUiPrimitiveEventCandidateRegion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveEventCandidateRegion {
    surface_id: String,
    parent_surface_id: Option<String>,
    order: WorthUiPrimitiveEventRegionOrder,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    containment: WorthUiPrimitiveEventContainment,
    activation_posture: WorthUiPrimitiveActivationPosture,
}

impl WorthUiPrimitiveEventDispatchCandidateReceipt {
    pub(super) fn no_hit(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
        Self::NoHit(WorthUiPrimitiveNoHitCandidateReceipt::from_region(region))
    }

    pub(super) fn disabled_hit(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
        Self::DisabledHit(WorthUiPrimitiveDisabledHitCandidateReceipt::from_region(
            region,
        ))
    }

    pub(super) fn enabled_primary_hit(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
        Self::EnabledPrimaryHit(WorthUiPrimitiveEnabledHitCandidateReceipt::from_region(
            region,
        ))
    }

    pub(super) fn cursor_target(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
        Self::CursorTarget(WorthUiPrimitiveCursorTargetCandidateReceipt::from_region(
            region,
        ))
    }

    pub(super) fn bubbled_ancestor(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
        Self::BubbledAncestor(WorthUiPrimitiveBubbledCandidateReceipt::from_region(region))
    }

    pub(super) fn captured_target(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
        Self::CapturedTarget(WorthUiPrimitiveCapturedCandidateReceipt::from_region(
            region,
        ))
    }

    pub(super) fn pass_through(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
        Self::PassThrough(WorthUiPrimitivePassThroughCandidateReceipt::from_region(
            region,
        ))
    }

    pub fn surface_id(&self) -> &str {
        self.region().surface_id()
    }

    pub fn parent_surface_id(&self) -> Option<&str> {
        self.region().parent_surface_id()
    }

    pub fn order(&self) -> WorthUiPrimitiveEventRegionOrder {
        self.region().order()
    }

    pub fn hit(&self) -> bool {
        !matches!(self, Self::NoHit(_) | Self::PassThrough(_))
    }

    pub fn selected(&self) -> bool {
        matches!(
            self,
            Self::DisabledHit(_)
                | Self::EnabledPrimaryHit(_)
                | Self::CapturedTarget(_)
                | Self::CursorTarget(_)
        )
    }

    pub fn emitted(&self) -> bool {
        matches!(
            self,
            Self::EnabledPrimaryHit(_) | Self::BubbledAncestor(_) | Self::CapturedTarget(_)
        )
    }

    pub fn activation_posture(&self) -> WorthUiPrimitiveActivationPosture {
        self.region().activation_posture()
    }

    pub fn cursor(&self) -> WorthUiPrimitiveResolvedCursorPosture {
        self.region().cursor()
    }

    pub fn containment(&self) -> WorthUiPrimitiveEventContainment {
        self.region().containment()
    }

    fn region(&self) -> &WorthUiPrimitiveEventCandidateRegion {
        match self {
            Self::NoHit(candidate) => candidate.region(),
            Self::DisabledHit(candidate) => candidate.region(),
            Self::EnabledPrimaryHit(candidate) => candidate.region(),
            Self::CursorTarget(candidate) => candidate.region(),
            Self::BubbledAncestor(candidate) => candidate.region(),
            Self::CapturedTarget(candidate) => candidate.region(),
            Self::PassThrough(candidate) => candidate.region(),
        }
    }
}

macro_rules! candidate_receipt {
    ($name:ident) => {
        impl $name {
            fn from_region(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
                Self {
                    region: WorthUiPrimitiveEventCandidateRegion::from_region(region),
                }
            }

            fn region(&self) -> &WorthUiPrimitiveEventCandidateRegion {
                &self.region
            }
        }
    };
}

candidate_receipt!(WorthUiPrimitiveNoHitCandidateReceipt);
candidate_receipt!(WorthUiPrimitiveDisabledHitCandidateReceipt);
candidate_receipt!(WorthUiPrimitiveEnabledHitCandidateReceipt);
candidate_receipt!(WorthUiPrimitiveCursorTargetCandidateReceipt);
candidate_receipt!(WorthUiPrimitiveBubbledCandidateReceipt);
candidate_receipt!(WorthUiPrimitiveCapturedCandidateReceipt);
candidate_receipt!(WorthUiPrimitivePassThroughCandidateReceipt);

impl WorthUiPrimitiveEventCandidateRegion {
    fn from_region(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
        Self {
            surface_id: region.surface_id().to_owned(),
            parent_surface_id: region.parent_surface_id().map(str::to_owned),
            order: region.order(),
            cursor: region.cursor(),
            containment: region.containment(),
            activation_posture: region.activation_posture(),
        }
    }

    pub fn surface_id(&self) -> &str {
        &self.surface_id
    }

    pub fn parent_surface_id(&self) -> Option<&str> {
        self.parent_surface_id.as_deref()
    }

    pub fn order(&self) -> WorthUiPrimitiveEventRegionOrder {
        self.order
    }

    pub fn cursor(&self) -> WorthUiPrimitiveResolvedCursorPosture {
        self.cursor
    }

    pub fn containment(&self) -> WorthUiPrimitiveEventContainment {
        self.containment
    }

    pub fn activation_posture(&self) -> WorthUiPrimitiveActivationPosture {
        self.activation_posture
    }
}
