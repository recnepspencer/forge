#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct UiCommandRouteCandidate {
    command: crate::capability::CommandId,
    shortcut: Option<crate::capability::UiCommandShortcutSequence>,
    route: crate::capability::UiCommandRouteDeclaration,
}

impl UiCommandRouteCandidate {
    pub(super) fn new(
        command: crate::capability::CommandId,
        shortcut: Option<crate::capability::UiCommandShortcutSequence>,
        route: crate::capability::UiCommandRouteDeclaration,
    ) -> Self {
        Self {
            command,
            shortcut,
            route,
        }
    }

    pub(super) fn command(&self) -> &crate::capability::CommandId {
        &self.command
    }

    pub(super) const fn shortcut(&self) -> Option<crate::capability::UiCommandShortcutSequence> {
        self.shortcut
    }

    pub(super) const fn route(&self) -> crate::capability::UiCommandRouteDeclaration {
        self.route
    }

    pub(super) fn rank(&self) -> (u8, i16, u32) {
        let route = self.route;
        (
            route.scope().precedence(),
            route.priority().value(),
            route.context().bits().count_ones(),
        )
    }
}
