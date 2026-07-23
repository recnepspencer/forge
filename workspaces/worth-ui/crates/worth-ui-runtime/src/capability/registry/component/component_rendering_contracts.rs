/// Accessibility support declared by a renderable component capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentAccessibilitySupport {
    Semantic,
    DecorativeOnly,
}

impl ComponentAccessibilitySupport {
    pub fn semantic() -> Self {
        Self::Semantic
    }

    pub fn decorative_only() -> Self {
        Self::DecorativeOnly
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Semantic => "semantic",
            Self::DecorativeOnly => "decorative_only",
        }
    }
}

/// Focus behavior declared by a renderable component capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentFocusSupport {
    NotFocusable,
    Focusable,
    FocusContainer,
}

impl ComponentFocusSupport {
    pub fn not_focusable() -> Self {
        Self::NotFocusable
    }

    pub fn focusable() -> Self {
        Self::Focusable
    }

    pub fn focus_container() -> Self {
        Self::FocusContainer
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::NotFocusable => "not_focusable",
            Self::Focusable => "focusable",
            Self::FocusContainer => "focus_container",
        }
    }
}

/// Execution lane hint consumed by later runtime lowering.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentExecutionLane {
    Passive,
    Interactive,
    Virtualized,
    CanvasSpatial,
    RealtimeOverlay,
}

impl ComponentExecutionLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passive => "passive",
            Self::Interactive => "interactive",
            Self::Virtualized => "virtualized",
            Self::CanvasSpatial => "canvas_spatial",
            Self::RealtimeOverlay => "realtime_overlay",
        }
    }
}

/// Bounded source-independent scale policy for one canvas component.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentCanvasSpatialContract {
    visible_primitive_limit: u32,
    overlay_row_limit: u16,
    tool_state_row_limit: u16,
}

impl ComponentCanvasSpatialContract {
    pub fn new(
        visible_primitive_limit: u32,
        overlay_row_limit: u16,
        tool_state_row_limit: u16,
    ) -> Option<Self> {
        (visible_primitive_limit > 0).then_some(Self {
            visible_primitive_limit,
            overlay_row_limit,
            tool_state_row_limit,
        })
    }

    pub fn visible_primitive_limit(self) -> u32 {
        self.visible_primitive_limit
    }

    pub fn overlay_row_limit(self) -> u16 {
        self.overlay_row_limit
    }

    pub fn tool_state_row_limit(self) -> u16 {
        self.tool_state_row_limit
    }

    pub(crate) fn digest_basis(self) -> u64 {
        u64::from(self.visible_primitive_limit)
            ^ u64::from(self.overlay_row_limit).rotate_left(21)
            ^ u64::from(self.tool_state_row_limit).rotate_left(43)
    }
}

/// Immutable priority carried by one realtime overlay declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentRealtimeOverlayPriority {
    HudOverlay,
    CriticalOverlay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentRealtimeOverlayContractDenialReason {
    ZeroOverlayRowLimit,
    ZeroDeclaredFrameCost,
    ZeroFrameBudget,
    FrameBudgetOverflow,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentRealtimeOverlayContractDenial {
    reason: ComponentRealtimeOverlayContractDenialReason,
}

/// Bounded, source-independent execution policy for one realtime overlay.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComponentRealtimeOverlayContract {
    overlay_row_limit: u16,
    declared_frame_cost_millis: u16,
    frame_budget_millis: u16,
    priority: ComponentRealtimeOverlayPriority,
}

impl ComponentRealtimeOverlayContract {
    pub fn new(
        overlay_row_limit: u16,
        declared_frame_cost_millis: u16,
        frame_budget_millis: u32,
        priority: ComponentRealtimeOverlayPriority,
    ) -> Result<Self, ComponentRealtimeOverlayContractDenial> {
        let reason = if overlay_row_limit == 0 {
            Some(ComponentRealtimeOverlayContractDenialReason::ZeroOverlayRowLimit)
        } else if declared_frame_cost_millis == 0 {
            Some(ComponentRealtimeOverlayContractDenialReason::ZeroDeclaredFrameCost)
        } else if frame_budget_millis == 0 {
            Some(ComponentRealtimeOverlayContractDenialReason::ZeroFrameBudget)
        } else if frame_budget_millis > u32::from(u16::MAX) {
            Some(ComponentRealtimeOverlayContractDenialReason::FrameBudgetOverflow)
        } else {
            None
        };
        if let Some(reason) = reason {
            return Err(ComponentRealtimeOverlayContractDenial { reason });
        }
        Ok(Self {
            overlay_row_limit,
            declared_frame_cost_millis,
            frame_budget_millis: frame_budget_millis as u16,
            priority,
        })
    }

    pub fn overlay_row_limit(self) -> u16 {
        self.overlay_row_limit
    }
    pub fn declared_frame_cost_millis(self) -> u16 {
        self.declared_frame_cost_millis
    }
    pub fn frame_budget_millis(self) -> u16 {
        self.frame_budget_millis
    }
    pub fn priority(self) -> ComponentRealtimeOverlayPriority {
        self.priority
    }

    pub(crate) fn digest_basis(self) -> u64 {
        u64::from(self.overlay_row_limit)
            ^ u64::from(self.declared_frame_cost_millis).rotate_left(17)
            ^ u64::from(self.frame_budget_millis).rotate_left(37)
            ^ self.priority.digest_tag().rotate_left(53)
    }
}

impl ComponentRealtimeOverlayContractDenial {
    pub fn reason(self) -> ComponentRealtimeOverlayContractDenialReason {
        self.reason
    }
}

impl ComponentRealtimeOverlayPriority {
    pub(crate) fn digest_tag(self) -> u64 {
        match self {
            Self::HudOverlay => 1,
            Self::CriticalOverlay => 2,
        }
    }
}
