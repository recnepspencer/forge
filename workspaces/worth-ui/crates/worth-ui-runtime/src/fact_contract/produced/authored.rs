#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAuthoredFactSelector {
    Module(Box<str>),
    Node(Box<str>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAuthoredFactKind {
    Created,
    Retired,
    Moved,
    KindChanged,
    SemanticsChanged,
    SurfacePlacementChanged,
    SurfaceCommandSlotsChanged,
}

#[derive(Debug, Eq, PartialEq)]
pub struct UiAuthoredChangedFact {
    selector: UiAuthoredFactSelector,
    kind: UiAuthoredFactKind,
}

impl UiAuthoredChangedFact {
    pub(crate) fn new(selector: UiAuthoredFactSelector, kind: UiAuthoredFactKind) -> Self {
        Self { selector, kind }
    }

    pub fn selector(&self) -> &UiAuthoredFactSelector {
        &self.selector
    }

    pub const fn kind(&self) -> UiAuthoredFactKind {
        self.kind
    }
}

impl UiAuthoredFactSelector {
    pub(crate) fn module(module: impl Into<Box<str>>) -> Self {
        Self::Module(module.into())
    }

    pub(crate) fn node(identity: impl Into<Box<str>>) -> Self {
        Self::Node(identity.into())
    }
}
