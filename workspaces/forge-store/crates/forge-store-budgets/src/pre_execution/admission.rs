use super::denial::PreExecutionBudgetDenial;
use super::receipt::PreExecutionBudgetAdmissionReceipt;
use super::request::PreExecutionBudgetRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreExecutionBudgetScope {
    Foreground,
    Maintenance,
    Verifier,
    Terminal,
    Streaming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreExecutionBudgetEnvelope {
    scope: PreExecutionBudgetScope,
    admitted_memory_bytes: u64,
    admitted_page_reads: u16,
    admitted_chunk_reads: u16,
    admitted_range_touches: u16,
    admitted_byte_reads: u64,
}

impl PreExecutionBudgetEnvelope {
    pub const fn new(
        scope: PreExecutionBudgetScope,
        admitted_memory_bytes: u64,
        admitted_page_reads: u16,
        admitted_chunk_reads: u16,
        admitted_range_touches: u16,
        admitted_byte_reads: u64,
    ) -> Self {
        Self {
            scope,
            admitted_memory_bytes,
            admitted_page_reads,
            admitted_chunk_reads,
            admitted_range_touches,
            admitted_byte_reads,
        }
    }

    pub const fn foreground_default() -> Self {
        Self::new(
            PreExecutionBudgetScope::Foreground,
            16_384,
            8,
            0,
            16,
            65_536,
        )
    }

    pub const fn maintenance_default() -> Self {
        Self::new(
            PreExecutionBudgetScope::Maintenance,
            32_768,
            16,
            4,
            32,
            131_072,
        )
    }

    pub const fn verifier_default() -> Self {
        Self::new(PreExecutionBudgetScope::Verifier, 24_576, 12, 2, 24, 98_304)
    }

    pub const fn terminal_default() -> Self {
        Self::new(PreExecutionBudgetScope::Terminal, 24_576, 12, 2, 24, 98_304)
    }

    pub const fn streaming_default() -> Self {
        Self::new(PreExecutionBudgetScope::Streaming, 32_768, 4, 8, 8, 262_144)
    }

    pub const fn scope(self) -> PreExecutionBudgetScope {
        self.scope
    }

    pub const fn admitted_memory_bytes(self) -> u64 {
        self.admitted_memory_bytes
    }

    pub const fn admitted_page_reads(self) -> u16 {
        self.admitted_page_reads
    }

    pub const fn admitted_chunk_reads(self) -> u16 {
        self.admitted_chunk_reads
    }

    pub const fn admitted_range_touches(self) -> u16 {
        self.admitted_range_touches
    }

    pub const fn admitted_byte_reads(self) -> u64 {
        self.admitted_byte_reads
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PreExecutionBudgetAdmissionCase {
    Admitted(PreExecutionBudgetAdmissionReceipt),
    Denied(PreExecutionBudgetDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PreExecutionBudgetAdmissionCaseId {
    Admitted,
    ScopeMismatch,
    MemoryBytesExceeded,
    PageReadsExceeded,
    ChunkReadsExceeded,
    RangeTouchesExceeded,
    ByteReadsExceeded,
}

impl PreExecutionBudgetAdmissionCaseId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::ScopeMismatch => "scope_mismatch",
            Self::MemoryBytesExceeded => "memory_bytes_exceeded",
            Self::PageReadsExceeded => "page_reads_exceeded",
            Self::ChunkReadsExceeded => "chunk_reads_exceeded",
            Self::RangeTouchesExceeded => "range_touches_exceeded",
            Self::ByteReadsExceeded => "byte_reads_exceeded",
        }
    }
}

pub fn pre_execution_budget_admission_cases(
) -> impl Iterator<Item = PreExecutionBudgetAdmissionCaseId> {
    [
        PreExecutionBudgetAdmissionCaseId::Admitted,
        PreExecutionBudgetAdmissionCaseId::ScopeMismatch,
        PreExecutionBudgetAdmissionCaseId::MemoryBytesExceeded,
        PreExecutionBudgetAdmissionCaseId::PageReadsExceeded,
        PreExecutionBudgetAdmissionCaseId::ChunkReadsExceeded,
        PreExecutionBudgetAdmissionCaseId::RangeTouchesExceeded,
        PreExecutionBudgetAdmissionCaseId::ByteReadsExceeded,
    ]
    .into_iter()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreExecutionBudgetAdmissionOutcome {
    case: PreExecutionBudgetAdmissionCase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreExecutionBudgetAdmissionView<'a> {
    Admitted(&'a PreExecutionBudgetAdmissionReceipt),
    Denied(&'a PreExecutionBudgetDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreExecutionBudgetAdmissionObservation {
    case_id: PreExecutionBudgetAdmissionCaseId,
}

impl PreExecutionBudgetAdmissionObservation {
    pub const fn case_id(self) -> PreExecutionBudgetAdmissionCaseId {
        self.case_id
    }
}

impl PreExecutionBudgetAdmissionOutcome {
    fn admitted(receipt: PreExecutionBudgetAdmissionReceipt) -> Self {
        Self {
            case: PreExecutionBudgetAdmissionCase::Admitted(receipt),
        }
    }

    fn denied(denial: PreExecutionBudgetDenial) -> Self {
        Self {
            case: PreExecutionBudgetAdmissionCase::Denied(denial),
        }
    }

    pub const fn view(&self) -> PreExecutionBudgetAdmissionView<'_> {
        match &self.case {
            PreExecutionBudgetAdmissionCase::Admitted(receipt) => {
                PreExecutionBudgetAdmissionView::Admitted(receipt)
            }
            PreExecutionBudgetAdmissionCase::Denied(denial) => {
                PreExecutionBudgetAdmissionView::Denied(denial)
            }
        }
    }

    pub const fn case_id(&self) -> PreExecutionBudgetAdmissionCaseId {
        match self.case {
            PreExecutionBudgetAdmissionCase::Admitted(_) => {
                PreExecutionBudgetAdmissionCaseId::Admitted
            }
            PreExecutionBudgetAdmissionCase::Denied(PreExecutionBudgetDenial::ScopeMismatch {
                ..
            }) => PreExecutionBudgetAdmissionCaseId::ScopeMismatch,
            PreExecutionBudgetAdmissionCase::Denied(
                PreExecutionBudgetDenial::MemoryBytesExceeded { .. },
            ) => PreExecutionBudgetAdmissionCaseId::MemoryBytesExceeded,
            PreExecutionBudgetAdmissionCase::Denied(
                PreExecutionBudgetDenial::PageReadsExceeded { .. },
            ) => PreExecutionBudgetAdmissionCaseId::PageReadsExceeded,
            PreExecutionBudgetAdmissionCase::Denied(
                PreExecutionBudgetDenial::ChunkReadsExceeded { .. },
            ) => PreExecutionBudgetAdmissionCaseId::ChunkReadsExceeded,
            PreExecutionBudgetAdmissionCase::Denied(
                PreExecutionBudgetDenial::RangeTouchesExceeded { .. },
            ) => PreExecutionBudgetAdmissionCaseId::RangeTouchesExceeded,
            PreExecutionBudgetAdmissionCase::Denied(
                PreExecutionBudgetDenial::ByteReadsExceeded { .. },
            ) => PreExecutionBudgetAdmissionCaseId::ByteReadsExceeded,
        }
    }

    pub const fn owner_case_observation(&self) -> PreExecutionBudgetAdmissionObservation {
        PreExecutionBudgetAdmissionObservation {
            case_id: self.case_id(),
        }
    }

    pub fn into_result(
        self,
    ) -> Result<PreExecutionBudgetAdmissionReceipt, PreExecutionBudgetDenial> {
        match self.case {
            PreExecutionBudgetAdmissionCase::Admitted(receipt) => Ok(receipt),
            PreExecutionBudgetAdmissionCase::Denied(denial) => Err(denial),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreExecutionBudgetAdmission;

impl PreExecutionBudgetAdmission {
    pub fn admit(
        &self,
        request: PreExecutionBudgetRequest,
        envelope: PreExecutionBudgetEnvelope,
    ) -> PreExecutionBudgetAdmissionOutcome {
        if request.scope() != envelope.scope() {
            return PreExecutionBudgetAdmissionOutcome::denied(
                PreExecutionBudgetDenial::ScopeMismatch {
                    requested: request.scope(),
                    admitted: envelope.scope(),
                },
            );
        }

        if request.estimated_memory_bytes() > envelope.admitted_memory_bytes() {
            return PreExecutionBudgetAdmissionOutcome::denied(
                PreExecutionBudgetDenial::MemoryBytesExceeded {
                    estimated: request.estimated_memory_bytes(),
                    admitted: envelope.admitted_memory_bytes(),
                },
            );
        }
        if request.estimated_page_reads() > envelope.admitted_page_reads() {
            return PreExecutionBudgetAdmissionOutcome::denied(
                PreExecutionBudgetDenial::PageReadsExceeded {
                    estimated: request.estimated_page_reads(),
                    admitted: envelope.admitted_page_reads(),
                },
            );
        }
        if request.estimated_chunk_reads() > envelope.admitted_chunk_reads() {
            return PreExecutionBudgetAdmissionOutcome::denied(
                PreExecutionBudgetDenial::ChunkReadsExceeded {
                    estimated: request.estimated_chunk_reads(),
                    admitted: envelope.admitted_chunk_reads(),
                },
            );
        }
        if request.estimated_range_touches() > envelope.admitted_range_touches() {
            return PreExecutionBudgetAdmissionOutcome::denied(
                PreExecutionBudgetDenial::RangeTouchesExceeded {
                    estimated: request.estimated_range_touches(),
                    admitted: envelope.admitted_range_touches(),
                },
            );
        }
        if request.estimated_byte_reads() > envelope.admitted_byte_reads() {
            return PreExecutionBudgetAdmissionOutcome::denied(
                PreExecutionBudgetDenial::ByteReadsExceeded {
                    estimated: request.estimated_byte_reads(),
                    admitted: envelope.admitted_byte_reads(),
                },
            );
        }

        PreExecutionBudgetAdmissionOutcome::admitted(PreExecutionBudgetAdmissionReceipt::new(
            request,
            request.scope(),
            envelope,
        ))
    }
}

pub const fn pre_execution_budget_admission() -> PreExecutionBudgetAdmission {
    PreExecutionBudgetAdmission
}
