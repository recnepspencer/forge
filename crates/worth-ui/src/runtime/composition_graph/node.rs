use super::{WorthUiCompositionNodeId, WorthUiCompositionRootId};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiCompositionRootKind {
    Surface,
    PageContentSlot,
    ComponentInstance,
    PortalEntry,
    CollectionItem,
    DiagnosticPanel,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiCompositionNodeKind {
    Container,
    Surface,
    FlowContainer,
    Content,
    Text,
    Icon,
    Control,
    Interaction,
    DiagnosticPanel,
    PortalHost,
    MosaicRegion,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiCompositionParentRef {
    Root(WorthUiCompositionRootId),
    Node(WorthUiCompositionNodeId),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiCompositionParticipation {
    Present,
    AbsentRetainsState,
    Inert,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiCompositionChildSizing {
    Auto,
    Hug,
    Fill(u32),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiCompositionPolicyKind {
    LocalLayout,
    InteractionContainment,
    DiagnosticPlacement,
    ViewportBoundary,
}

impl WorthUiCompositionRootKind {
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "surface" => Some(Self::Surface),
            "page_content_slot" => Some(Self::PageContentSlot),
            "component_instance" => Some(Self::ComponentInstance),
            "portal_entry" => Some(Self::PortalEntry),
            "collection_item" => Some(Self::CollectionItem),
            "diagnostic_panel" => Some(Self::DiagnosticPanel),
            _ => None,
        }
    }

    pub const fn token(self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::PageContentSlot => "page_content_slot",
            Self::ComponentInstance => "component_instance",
            Self::PortalEntry => "portal_entry",
            Self::CollectionItem => "collection_item",
            Self::DiagnosticPanel => "diagnostic_panel",
        }
    }
}

impl WorthUiCompositionNodeKind {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Container => "container",
            Self::Surface => "surface",
            Self::FlowContainer => "flow_container",
            Self::Content => "content",
            Self::Text => "text",
            Self::Icon => "icon",
            Self::Control => "control",
            Self::Interaction => "interaction",
            Self::DiagnosticPanel => "diagnostic_panel",
            Self::PortalHost => "portal_host",
            Self::MosaicRegion => "mosaic_region",
        }
    }

    pub const fn can_parent_children(self) -> bool {
        matches!(
            self,
            Self::Container | Self::Surface | Self::FlowContainer | Self::DiagnosticPanel
        )
    }
}

impl WorthUiCompositionParticipation {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::AbsentRetainsState => "absent_retains_state",
            Self::Inert => "inert",
        }
    }
}

impl WorthUiCompositionChildSizing {
    pub fn token(self) -> String {
        match self {
            Self::Auto => "auto".to_owned(),
            Self::Hug => "hug".to_owned(),
            Self::Fill(weight) => format!("fill({weight})"),
        }
    }
}

impl WorthUiCompositionPolicyKind {
    pub const fn token(self) -> &'static str {
        match self {
            Self::LocalLayout => "local_layout",
            Self::InteractionContainment => "interaction_containment",
            Self::DiagnosticPlacement => "diagnostic_placement",
            Self::ViewportBoundary => "viewport_boundary",
        }
    }

    pub const fn supports_node_kind(self, node_kind: WorthUiCompositionNodeKind) -> bool {
        match self {
            Self::LocalLayout => matches!(
                node_kind,
                WorthUiCompositionNodeKind::Container
                    | WorthUiCompositionNodeKind::Surface
                    | WorthUiCompositionNodeKind::FlowContainer
                    | WorthUiCompositionNodeKind::Content
                    | WorthUiCompositionNodeKind::Text
                    | WorthUiCompositionNodeKind::Icon
                    | WorthUiCompositionNodeKind::Control
                    | WorthUiCompositionNodeKind::PortalHost
                    | WorthUiCompositionNodeKind::MosaicRegion
            ),
            Self::InteractionContainment => {
                matches!(node_kind, WorthUiCompositionNodeKind::Interaction)
            }
            Self::DiagnosticPlacement => {
                matches!(node_kind, WorthUiCompositionNodeKind::DiagnosticPanel)
            }
            Self::ViewportBoundary => matches!(
                node_kind,
                WorthUiCompositionNodeKind::Container
                    | WorthUiCompositionNodeKind::Surface
                    | WorthUiCompositionNodeKind::FlowContainer
                    | WorthUiCompositionNodeKind::MosaicRegion
            ),
        }
    }
}

impl WorthUiCompositionParentRef {
    pub fn identity(&self) -> &str {
        match self {
            Self::Root(root_id) => root_id.as_str(),
            Self::Node(node_id) => node_id.as_str(),
        }
    }

    pub const fn kind_token(&self) -> &'static str {
        match self {
            Self::Root(_) => "root",
            Self::Node(_) => "node",
        }
    }
}
