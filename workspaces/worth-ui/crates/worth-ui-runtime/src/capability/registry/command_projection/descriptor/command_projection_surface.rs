/// Domain-agnostic places where command-spine entries may appear.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommandProjectionSurface {
    kind: CommandProjectionSurfaceKind,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum CommandProjectionSurfaceKind {
    MenuBar,
    CommandPalette,
    Toolbar,
    ContextMenu,
    RegionHeaderAction,
    StatusAction,
    TabAction,
    AuxiliaryAction,
    UnsupportedForDiagnostics(String),
}

impl CommandProjectionSurface {
    pub fn menu_bar() -> Self {
        Self::known(CommandProjectionSurfaceKind::MenuBar)
    }

    pub fn command_palette() -> Self {
        Self::known(CommandProjectionSurfaceKind::CommandPalette)
    }

    pub fn toolbar() -> Self {
        Self::known(CommandProjectionSurfaceKind::Toolbar)
    }

    pub fn context_menu() -> Self {
        Self::known(CommandProjectionSurfaceKind::ContextMenu)
    }

    pub fn region_header_action() -> Self {
        Self::known(CommandProjectionSurfaceKind::RegionHeaderAction)
    }

    pub fn status_action() -> Self {
        Self::known(CommandProjectionSurfaceKind::StatusAction)
    }

    pub fn tab_action() -> Self {
        Self::known(CommandProjectionSurfaceKind::TabAction)
    }

    pub fn auxiliary_action() -> Self {
        Self::known(CommandProjectionSurfaceKind::AuxiliaryAction)
    }

    pub fn unsupported_for_diagnostics(surface: impl Into<String>) -> Self {
        Self::known(CommandProjectionSurfaceKind::UnsupportedForDiagnostics(
            surface.into(),
        ))
    }

    pub(crate) fn is_supported(&self) -> bool {
        !matches!(
            self.kind,
            CommandProjectionSurfaceKind::UnsupportedForDiagnostics(_)
        )
    }

    pub(crate) fn requires_mosaic_scope(&self) -> bool {
        matches!(
            self.kind,
            CommandProjectionSurfaceKind::RegionHeaderAction
                | CommandProjectionSurfaceKind::TabAction
        )
    }

    pub(crate) fn rejects_mosaic_scope(&self) -> bool {
        matches!(
            self.kind,
            CommandProjectionSurfaceKind::MenuBar
                | CommandProjectionSurfaceKind::CommandPalette
                | CommandProjectionSurfaceKind::Toolbar
                | CommandProjectionSurfaceKind::AuxiliaryAction
        )
    }

    pub fn digest_basis(&self) -> String {
        match &self.kind {
            CommandProjectionSurfaceKind::MenuBar => "menu_bar".to_owned(),
            CommandProjectionSurfaceKind::CommandPalette => "command_palette".to_owned(),
            CommandProjectionSurfaceKind::Toolbar => "toolbar".to_owned(),
            CommandProjectionSurfaceKind::ContextMenu => "context_menu".to_owned(),
            CommandProjectionSurfaceKind::RegionHeaderAction => "region_header_action".to_owned(),
            CommandProjectionSurfaceKind::StatusAction => "status_action".to_owned(),
            CommandProjectionSurfaceKind::TabAction => "tab_action".to_owned(),
            CommandProjectionSurfaceKind::AuxiliaryAction => "auxiliary_action".to_owned(),
            CommandProjectionSurfaceKind::UnsupportedForDiagnostics(surface) => {
                format!("unsupported:{surface}")
            }
        }
    }

    fn known(kind: CommandProjectionSurfaceKind) -> Self {
        Self { kind }
    }
}
