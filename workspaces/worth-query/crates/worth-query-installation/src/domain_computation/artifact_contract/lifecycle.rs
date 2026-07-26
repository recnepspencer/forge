#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryArtifactLifecycleContract {
    Transient,
    ArenaScoped,
    Retained,
    ReconstructibleDerived,
    ExternallyDurable,
    ReconstructibleAsAuthoritative,
}

impl WorthQueryArtifactLifecycleContract {
    pub const fn arena_scoped() -> Self {
        Self::ArenaScoped
    }

    pub const fn is_reusable(self) -> bool {
        matches!(
            self,
            Self::Retained
                | Self::ReconstructibleDerived
                | Self::ExternallyDurable
                | Self::ReconstructibleAsAuthoritative
        )
    }
}
