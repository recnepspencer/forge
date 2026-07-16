#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImportPublicationAction {
    RawDeclarationObserved,
    CurrentScopeReadmitted,
    RecoveredArtifactAdmitted,
    LayoutMaterializationAdmitted,
    PublicationPending,
    PublicationDurable,
    CrashBeforePublication,
    PublicationDenied,
}

impl ImportPublicationAction {
    pub const fn all() -> [Self; 8] {
        [
            Self::RawDeclarationObserved,
            Self::CurrentScopeReadmitted,
            Self::RecoveredArtifactAdmitted,
            Self::LayoutMaterializationAdmitted,
            Self::PublicationPending,
            Self::PublicationDurable,
            Self::CrashBeforePublication,
            Self::PublicationDenied,
        ]
    }
}
