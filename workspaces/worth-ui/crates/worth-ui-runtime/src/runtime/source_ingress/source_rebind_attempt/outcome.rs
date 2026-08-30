#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiSourceRebindAttemptBasis {
    revision: super::super::WorthUiSourcePackageRevision,
    ordering: super::super::WorthUiCandidateOrderingReceipt,
    counters: super::super::WorthUiSourceIngressCounters,
    capability_basis: crate::capability::CapabilitySnapshotDigest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiSourceCompilationDenialReceipt {
    basis: UiSourceRebindAttemptBasis,
    report: worth_ui_dsl::WorthUiDslCompileReport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiSourceRebindAttemptDenial {
    SourceIngress(super::super::WorthUiSourceIngressDenial),
    RuntimePreparation(crate::runtime::WorthUiSemanticHandoffPreparationDenial),
    Candidate(crate::runtime::WorthUiReplacementCandidateDenial),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiSourceRebindAttemptDenialReceipt {
    basis: UiSourceRebindAttemptBasis,
    denial: UiSourceRebindAttemptDenial,
}

#[derive(Debug, Eq, PartialEq)]
pub enum UiSourceRebindAttemptOutcome {
    Candidate(Box<super::super::WorthUiWatchedCandidateSubmission>),
    CompilationDenied(Box<UiSourceCompilationDenialReceipt>),
    Denied(Box<UiSourceRebindAttemptDenialReceipt>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiSourceRebindAttemptFailure {
    CompilationDenied(Box<UiSourceCompilationDenialReceipt>),
    Denied(Box<UiSourceRebindAttemptDenialReceipt>),
}

impl UiSourceRebindAttemptBasis {
    pub(super) fn seal(
        revision: super::super::WorthUiSourcePackageRevision,
        ordering: super::super::WorthUiCandidateOrderingReceipt,
        counters: super::super::WorthUiSourceIngressCounters,
        capability_basis: crate::capability::CapabilitySnapshotDigest,
    ) -> Self {
        Self {
            revision,
            ordering,
            counters,
            capability_basis,
        }
    }

    pub fn source_revision(&self) -> &super::super::WorthUiSourcePackageRevision {
        &self.revision
    }

    pub fn ordering_receipt(&self) -> &super::super::WorthUiCandidateOrderingReceipt {
        &self.ordering
    }

    pub const fn counters(&self) -> super::super::WorthUiSourceIngressCounters {
        self.counters
    }

    pub const fn capability_basis(&self) -> crate::capability::CapabilitySnapshotDigest {
        self.capability_basis
    }

    pub(super) fn into_source_parts(
        self,
    ) -> (
        super::super::WorthUiSourcePackageRevision,
        super::super::WorthUiCandidateOrderingReceipt,
        super::super::WorthUiSourceIngressCounters,
    ) {
        (self.revision, self.ordering, self.counters)
    }
}

impl UiSourceCompilationDenialReceipt {
    pub(super) fn new(
        basis: UiSourceRebindAttemptBasis,
        report: worth_ui_dsl::WorthUiDslCompileReport,
    ) -> Self {
        Self { basis, report }
    }

    pub const fn basis(&self) -> &UiSourceRebindAttemptBasis {
        &self.basis
    }

    pub fn source_revision(&self) -> &super::super::WorthUiSourcePackageRevision {
        self.basis.source_revision()
    }

    pub fn ordering_receipt(&self) -> &super::super::WorthUiCandidateOrderingReceipt {
        self.basis.ordering_receipt()
    }

    pub fn report(&self) -> &worth_ui_dsl::WorthUiDslCompileReport {
        &self.report
    }
}

impl UiSourceRebindAttemptDenialReceipt {
    pub(super) const fn new(
        basis: UiSourceRebindAttemptBasis,
        denial: UiSourceRebindAttemptDenial,
    ) -> Self {
        Self { basis, denial }
    }

    pub const fn basis(&self) -> &UiSourceRebindAttemptBasis {
        &self.basis
    }

    pub const fn denial(&self) -> &UiSourceRebindAttemptDenial {
        &self.denial
    }
}

impl UiSourceRebindAttemptOutcome {
    pub fn into_candidate_submission(
        self,
    ) -> Result<super::super::WorthUiWatchedCandidateSubmission, UiSourceRebindAttemptFailure> {
        match self {
            Self::Candidate(candidate) => Ok(*candidate),
            Self::CompilationDenied(receipt) => {
                Err(UiSourceRebindAttemptFailure::CompilationDenied(receipt))
            }
            Self::Denied(receipt) => Err(UiSourceRebindAttemptFailure::Denied(receipt)),
        }
    }
}

impl UiSourceRebindAttemptFailure {
    pub const fn basis(&self) -> &UiSourceRebindAttemptBasis {
        match self {
            Self::CompilationDenied(receipt) => receipt.basis(),
            Self::Denied(receipt) => receipt.basis(),
        }
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(super) fn into_migration_denial(
        self,
    ) -> super::super::WorthUiWatchedCandidateSubmissionDenial {
        match self {
            Self::CompilationDenied(receipt) => {
                super::super::WorthUiWatchedCandidateSubmissionDenial::DslCompilation(Box::new(
                    receipt.report,
                ))
            }
            Self::Denied(receipt) => match receipt.denial {
                UiSourceRebindAttemptDenial::SourceIngress(denial) => {
                    super::super::WorthUiWatchedCandidateSubmissionDenial::SourceIngress(denial)
                }
                UiSourceRebindAttemptDenial::RuntimePreparation(denial) => {
                    super::super::WorthUiWatchedCandidateSubmissionDenial::RuntimePreparation(
                        Box::new(denial),
                    )
                }
                UiSourceRebindAttemptDenial::Candidate(denial) => {
                    super::super::WorthUiWatchedCandidateSubmissionDenial::Candidate(Box::new(
                        denial,
                    ))
                }
            },
        }
    }
}
