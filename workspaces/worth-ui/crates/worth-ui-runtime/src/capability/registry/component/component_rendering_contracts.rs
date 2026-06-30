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
}

impl ComponentExecutionLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Passive => "passive",
            Self::Interactive => "interactive",
            Self::Virtualized => "virtualized",
        }
    }
}
