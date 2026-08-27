#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiFocusTraversalDirection {
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiFocusCause {
    Direct,
    KeyboardTraversal,
    PortalInitial,
    PortalRestoration,
    RebindPreserved,
    RebindFallback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiFocusRequest {
    Direct {
        scope: super::UiFocusScopeIdentity,
        participant: super::UiFocusParticipantIdentity,
        incarnation: worth_ui_host_contract::UiMountIncarnation,
        cause: UiFocusCause,
    },
    Traverse {
        scope: super::UiFocusScopeIdentity,
        direction: UiFocusTraversalDirection,
        wrap: bool,
    },
    First {
        scope: super::UiFocusScopeIdentity,
        cause: UiFocusCause,
    },
    Last {
        scope: super::UiFocusScopeIdentity,
        cause: UiFocusCause,
    },
    Restore(super::UiFocusRestorationToken),
    Clear {
        cause: UiFocusCause,
    },
}
