#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PhysicalWorkCourtroomFinding {
    MissingCausalRecord,
    CausalEvidenceOverflow,
    ForeignStoreIdentity,
    ForeignRuntimeIdentity,
    ForeignLifecycleGeneration,
    DuplicateOperationIdentity,
    DuplicateSignalAttemptIdentity,
    DuplicateBackendOperationIdentity,
    InvalidRetryCausalChain,
    MixedBackendProfile,
    ShutdownResidual,
    ShutdownOvercount,
    DrainEvidenceOverflow,
    DrainResidual,
    MissingArtifactManifest,
    DuplicateArtifactPath,
    OracleRejected,
    MissingMutantLocalization,
    MutantSurvived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalWorkCourtroomVerdict {
    Accepted,
    Rejected(Box<[PhysicalWorkCourtroomFinding]>),
}

impl PhysicalWorkCourtroomVerdict {
    pub const fn accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }

    pub fn findings(&self) -> &[PhysicalWorkCourtroomFinding] {
        match self {
            Self::Accepted => &[],
            Self::Rejected(findings) => findings,
        }
    }

    pub(super) fn from_findings(
        findings: impl IntoIterator<Item = PhysicalWorkCourtroomFinding>,
    ) -> Self {
        let mut findings = findings.into_iter().collect::<Vec<_>>();
        findings.sort_unstable();
        findings.dedup();
        if findings.is_empty() {
            Self::Accepted
        } else {
            Self::Rejected(findings.into_boxed_slice())
        }
    }
}
