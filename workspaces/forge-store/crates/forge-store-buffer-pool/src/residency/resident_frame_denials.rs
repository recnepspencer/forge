use crate::ProtectedFrameDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResidentFrameDenial {
    kind: ResidentFrameDenialKind,
    protected_frame_denial: Option<ProtectedFrameDenial>,
}

impl ResidentFrameDenial {
    pub(crate) const fn new(kind: ResidentFrameDenialKind) -> Self {
        Self {
            kind,
            protected_frame_denial: None,
        }
    }

    pub(crate) const fn protected_frame(
        kind: ResidentFrameDenialKind,
        protected_frame_denial: ProtectedFrameDenial,
    ) -> Self {
        Self {
            kind,
            protected_frame_denial: Some(protected_frame_denial),
        }
    }

    pub const fn from_shortcut_attempt(attempt: ResidentFrameShortcutAttempt) -> Self {
        Self {
            kind: attempt.denial_kind(),
            protected_frame_denial: None,
        }
    }

    pub const fn kind(self) -> ResidentFrameDenialKind {
        self.kind
    }

    pub const fn is_stale_resident_generation(self) -> bool {
        matches!(self.kind, ResidentFrameDenialKind::StaleResidentGeneration)
    }

    pub const fn protected_frame_denial(self) -> Option<ProtectedFrameDenial> {
        self.protected_frame_denial
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResidentFrameDenialKind {
    TableCapacityIsZero,
    FrameSizeIsZero,
    FrameSizeOverflow,
    HeaderOwnerMismatch,
    PhysicalReferenceKindRejected,
    PhysicalHeaderKindRejected,
    ResidentMemoryBudgetExceeded,
    ResidentFrameTableFull,
    ResidentFrameSlotOutOfRange,
    ResidentFrameSlotNotResident,
    StaleResidentGeneration,
    GenerationSeparationNotObserved,
    ResidentGenerationOverflow,
    ResidentPayloadWitnessMismatch,
    ResidentPayloadLengthMismatch,
    ResidentBytesNotAdmitted,
    ResidentFramePinned,
    PinnedPageBudgetExceeded,
    DirtyPageBudgetExceeded,
    DirtyMutationBehindLiveRecordView,
    ResidentFrameNotDirty,
    DirtyPublicationBehindActiveLease,
    DirtyPublicationPlanStale,
    DirtyFrameUnpublished,
    EvictionPressureIsZero,
    NoResidentEvictionCandidates,
    AllEvictionCandidatesProtected,
    EvictionPlanStale,
    PageLeaseStale,
    PageLeaseNotPinned,
    BackendPrivateResidueRejected,
    OsPageCacheStateRejected,
    SemanticObjectPresenceRejected,
    PhysicalGenerationAsResidentProofRejected,
    ResidentGenerationAsPhysicalProofRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResidentFrameShortcutAttempt {
    BackendPrivateResidue,
    OsPageCacheState,
    SemanticObjectPresence,
    PhysicalGenerationAsResidentProof,
    ResidentGenerationAsPhysicalProof,
}

impl ResidentFrameShortcutAttempt {
    pub const fn physical_substrate_forbidden_attempts() -> &'static [Self] {
        &[
            Self::BackendPrivateResidue,
            Self::OsPageCacheState,
            Self::SemanticObjectPresence,
            Self::PhysicalGenerationAsResidentProof,
            Self::ResidentGenerationAsPhysicalProof,
        ]
    }

    pub const fn denial_kind(self) -> ResidentFrameDenialKind {
        match self {
            Self::BackendPrivateResidue => ResidentFrameDenialKind::BackendPrivateResidueRejected,
            Self::OsPageCacheState => ResidentFrameDenialKind::OsPageCacheStateRejected,
            Self::SemanticObjectPresence => ResidentFrameDenialKind::SemanticObjectPresenceRejected,
            Self::PhysicalGenerationAsResidentProof => {
                ResidentFrameDenialKind::PhysicalGenerationAsResidentProofRejected
            }
            Self::ResidentGenerationAsPhysicalProof => {
                ResidentFrameDenialKind::ResidentGenerationAsPhysicalProofRejected
            }
        }
    }
}
