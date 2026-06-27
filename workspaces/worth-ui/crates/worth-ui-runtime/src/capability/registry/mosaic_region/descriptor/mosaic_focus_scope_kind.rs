/// Focus scope created by a mosaic region kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicFocusScopeKind {
    ActiveSurfaceScope,
    RegionScope,
    ModalTrapScope,
    ToolbarScope,
    StatusScope,
    NoFocusScope,
    MissingForDiagnostics,
}

impl MosaicFocusScopeKind {
    pub fn active_surface_scope() -> Self {
        Self::ActiveSurfaceScope
    }

    pub fn region_scope() -> Self {
        Self::RegionScope
    }

    pub fn modal_trap_scope() -> Self {
        Self::ModalTrapScope
    }

    pub fn toolbar_scope() -> Self {
        Self::ToolbarScope
    }

    pub fn status_scope() -> Self {
        Self::StatusScope
    }

    pub fn no_focus_scope() -> Self {
        Self::NoFocusScope
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::ActiveSurfaceScope => "active_surface_scope",
            Self::RegionScope => "region_scope",
            Self::ModalTrapScope => "modal_trap_scope",
            Self::ToolbarScope => "toolbar_scope",
            Self::StatusScope => "status_scope",
            Self::NoFocusScope => "no_focus_scope",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
