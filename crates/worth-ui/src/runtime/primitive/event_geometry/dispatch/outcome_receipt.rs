use super::super::super::WorthUiPrimitiveResolvedCursorPosture;
use super::super::receipt::WorthUiPrimitiveEventContainment;
use super::region_receipt::WorthUiPrimitiveEventRegionReceipt;

#[derive(Clone, Debug, PartialEq)]
pub enum WorthUiPrimitiveEventDispatchOutcome {
    NoHit(WorthUiPrimitiveNoHitReceipt),
    EnabledHit(WorthUiPrimitiveEnabledHitReceipt),
    DisabledHit(WorthUiPrimitiveDisabledHitReceipt),
    Bubbled(WorthUiPrimitiveBubbledHitReceipt),
    Captured(WorthUiPrimitiveCapturedHitReceipt),
    Denied(WorthUiPrimitiveDispatchDeniedReceipt),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveNoHitReceipt {
    cursor: WorthUiPrimitiveResolvedCursorPosture,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveEnabledHitReceipt {
    surface_id: String,
    emitted_surface_id: String,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    containment: WorthUiPrimitiveEventContainment,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveDisabledHitReceipt {
    surface_id: String,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    containment: WorthUiPrimitiveEventContainment,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveBubbledHitReceipt {
    primary_surface_id: String,
    emitted_surface_ids: Vec<String>,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    containment: WorthUiPrimitiveEventContainment,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveCapturedHitReceipt {
    surface_id: String,
    emitted_surface_id: String,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
    containment: WorthUiPrimitiveEventContainment,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveDispatchDeniedReceipt {
    surface_id: String,
    reason: String,
    cursor: WorthUiPrimitiveResolvedCursorPosture,
}

impl WorthUiPrimitiveEventDispatchOutcome {
    pub(super) fn no_hit() -> Self {
        Self::NoHit(WorthUiPrimitiveNoHitReceipt {
            cursor: WorthUiPrimitiveResolvedCursorPosture::Default,
        })
    }

    pub(super) fn enabled(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
        Self::EnabledHit(WorthUiPrimitiveEnabledHitReceipt {
            surface_id: region.surface_id().to_owned(),
            emitted_surface_id: region.surface_id().to_owned(),
            cursor: region.cursor(),
            containment: region.containment(),
        })
    }

    pub(super) fn disabled(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
        Self::DisabledHit(WorthUiPrimitiveDisabledHitReceipt {
            surface_id: region.surface_id().to_owned(),
            cursor: region.cursor(),
            containment: region.containment(),
        })
    }

    pub(super) fn bubbled(
        region: &WorthUiPrimitiveEventRegionReceipt,
        emitted_surface_ids: Vec<String>,
    ) -> Self {
        Self::Bubbled(WorthUiPrimitiveBubbledHitReceipt {
            primary_surface_id: region.surface_id().to_owned(),
            emitted_surface_ids,
            cursor: region.cursor(),
            containment: region.containment(),
        })
    }

    pub(super) fn captured(region: &WorthUiPrimitiveEventRegionReceipt) -> Self {
        Self::Captured(WorthUiPrimitiveCapturedHitReceipt {
            surface_id: region.surface_id().to_owned(),
            emitted_surface_id: region.surface_id().to_owned(),
            cursor: region.cursor(),
            containment: region.containment(),
        })
    }

    pub(super) fn denied(
        region: &WorthUiPrimitiveEventRegionReceipt,
        reason: impl Into<String>,
    ) -> Self {
        Self::Denied(WorthUiPrimitiveDispatchDeniedReceipt {
            surface_id: region.surface_id().to_owned(),
            reason: reason.into(),
            cursor: region.cursor(),
        })
    }

    pub fn primary_surface_id(&self) -> Option<&str> {
        match self {
            Self::NoHit(_) => None,
            Self::EnabledHit(receipt) => Some(&receipt.surface_id),
            Self::DisabledHit(receipt) => Some(&receipt.surface_id),
            Self::Bubbled(receipt) => Some(&receipt.primary_surface_id),
            Self::Captured(receipt) => Some(&receipt.surface_id),
            Self::Denied(receipt) => Some(&receipt.surface_id),
        }
    }

    pub fn emitted_surface_ids(&self) -> &[String] {
        match self {
            Self::EnabledHit(receipt) => std::slice::from_ref(&receipt.emitted_surface_id),
            Self::Bubbled(receipt) => &receipt.emitted_surface_ids,
            Self::Captured(receipt) => std::slice::from_ref(&receipt.emitted_surface_id),
            Self::NoHit(_) | Self::DisabledHit(_) | Self::Denied(_) => &[],
        }
    }

    pub fn cursor(&self) -> WorthUiPrimitiveResolvedCursorPosture {
        match self {
            Self::NoHit(receipt) => receipt.cursor,
            Self::EnabledHit(receipt) => receipt.cursor,
            Self::DisabledHit(receipt) => receipt.cursor,
            Self::Bubbled(receipt) => receipt.cursor,
            Self::Captured(receipt) => receipt.cursor,
            Self::Denied(receipt) => receipt.cursor,
        }
    }

    pub fn containment(&self) -> Option<WorthUiPrimitiveEventContainment> {
        match self {
            Self::NoHit(_) => None,
            Self::EnabledHit(receipt) => Some(receipt.containment),
            Self::DisabledHit(receipt) => Some(receipt.containment),
            Self::Bubbled(receipt) => Some(receipt.containment),
            Self::Captured(receipt) => Some(receipt.containment),
            Self::Denied(_) => None,
        }
    }

    pub fn activation_surface_id(&self) -> Option<&str> {
        match self {
            Self::EnabledHit(receipt) => Some(&receipt.surface_id),
            Self::Bubbled(receipt) => Some(&receipt.primary_surface_id),
            Self::Captured(receipt) => Some(&receipt.surface_id),
            Self::NoHit(_) | Self::DisabledHit(_) | Self::Denied(_) => None,
        }
    }
}
