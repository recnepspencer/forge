use worth_store_recovery_physics::RecoverySourceApplicationRole;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourcePrecedenceAction {
    CandidateDiscovered {
        discovery_order: u64,
        role: RecoverySourceApplicationRole,
    },
    CandidateAdmitted {
        discovery_order: u64,
    },
    CandidateAdvisoryOnly {
        discovery_order: u64,
    },
    CandidateRejected {
        discovery_order: u64,
    },
    ContradictionPreserved,
    SourceSelected,
    SourceQuarantined,
    SourceDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SourcePrecedenceActionKind {
    CandidateDiscovered,
    CandidateAdmitted,
    CandidateAdvisoryOnly,
    CandidateRejected,
    ContradictionPreserved,
    SourceSelected,
    SourceQuarantined,
    SourceDenied,
}

impl SourcePrecedenceAction {
    pub const fn kind(self) -> SourcePrecedenceActionKind {
        match self {
            Self::CandidateDiscovered { .. } => SourcePrecedenceActionKind::CandidateDiscovered,
            Self::CandidateAdmitted { .. } => SourcePrecedenceActionKind::CandidateAdmitted,
            Self::CandidateAdvisoryOnly { .. } => SourcePrecedenceActionKind::CandidateAdvisoryOnly,
            Self::CandidateRejected { .. } => SourcePrecedenceActionKind::CandidateRejected,
            Self::ContradictionPreserved => SourcePrecedenceActionKind::ContradictionPreserved,
            Self::SourceSelected => SourcePrecedenceActionKind::SourceSelected,
            Self::SourceQuarantined => SourcePrecedenceActionKind::SourceQuarantined,
            Self::SourceDenied => SourcePrecedenceActionKind::SourceDenied,
        }
    }
}

impl SourcePrecedenceActionKind {
    pub const fn all() -> [Self; 8] {
        [
            Self::CandidateDiscovered,
            Self::CandidateAdmitted,
            Self::CandidateAdvisoryOnly,
            Self::CandidateRejected,
            Self::ContradictionPreserved,
            Self::SourceSelected,
            Self::SourceQuarantined,
            Self::SourceDenied,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourcePrecedenceDenial {
    DerivedSourceCannotBeAuthority,
    QuarantinedSourceCannotBeSelected,
    CandidateNotAdmitted,
}
