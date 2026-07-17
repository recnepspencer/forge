#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReplicationAdmissionAction {
    SourceAdmitted,
    SourcePeerIdentityDenied,
    SourceEpochRequiredDenied,
    SourceLineageIdentityDenied,
    SourceCurrentAuthorityDenied,
    SourceReplayIdentityDenied,
    FreshProgressObserved,
    ResumeProgressObserved,
    DuplicateObserved,
    ResumeCurrentAuthorityDenied,
    SourceEpochDivergenceDetected,
    LineageDivergenceDetected,
    ReplayOverlapDivergenceDetected,
    ResumeProgressGapDenied,
    FreshPublicationPending,
    ResumePublicationPending,
    FreshPublicationDurable,
    ResumePublicationDurable,
    PublicationCurrentAuthorityDenied,
    PublicationPeerProgressChangedDenied,
    PublicationPeerCapacityDenied,
    PublicationProgressStoreDenied,
}

impl ReplicationAdmissionAction {
    pub const fn all() -> [Self; 22] {
        [
            Self::SourceAdmitted,
            Self::SourcePeerIdentityDenied,
            Self::SourceEpochRequiredDenied,
            Self::SourceLineageIdentityDenied,
            Self::SourceCurrentAuthorityDenied,
            Self::SourceReplayIdentityDenied,
            Self::FreshProgressObserved,
            Self::ResumeProgressObserved,
            Self::DuplicateObserved,
            Self::ResumeCurrentAuthorityDenied,
            Self::SourceEpochDivergenceDetected,
            Self::LineageDivergenceDetected,
            Self::ReplayOverlapDivergenceDetected,
            Self::ResumeProgressGapDenied,
            Self::FreshPublicationPending,
            Self::ResumePublicationPending,
            Self::FreshPublicationDurable,
            Self::ResumePublicationDurable,
            Self::PublicationCurrentAuthorityDenied,
            Self::PublicationPeerProgressChangedDenied,
            Self::PublicationPeerCapacityDenied,
            Self::PublicationProgressStoreDenied,
        ]
    }
}
