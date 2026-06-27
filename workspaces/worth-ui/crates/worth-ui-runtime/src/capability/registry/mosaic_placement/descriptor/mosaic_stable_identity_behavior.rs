/// Identity behavior used when placement state is replayed or reconciled.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicStableIdentityBehavior {
    PreserveSurfaceIdentity,
    AssignPlacementIdentity,
    ReplaceOnReload,
    MissingForDiagnostics,
}

impl MosaicStableIdentityBehavior {
    pub fn preserve_surface_identity() -> Self {
        Self::PreserveSurfaceIdentity
    }

    pub fn assign_placement_identity() -> Self {
        Self::AssignPlacementIdentity
    }

    pub fn replace_on_reload() -> Self {
        Self::ReplaceOnReload
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::PreserveSurfaceIdentity => "preserve_surface_identity",
            Self::AssignPlacementIdentity => "assign_placement_identity",
            Self::ReplaceOnReload => "replace_on_reload",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
