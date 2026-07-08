use super::denial::S8PreExecutionBudgetDenial;
use super::receipt::S8PreExecutionBudgetAdmissionReceipt;
use super::request::S8PreExecutionBudgetRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8PreExecutionBudgetScope {
    Foreground,
    Maintenance,
    Verifier,
    Terminal,
    Streaming,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PreExecutionBudgetEnvelope {
    scope: S8PreExecutionBudgetScope,
    admitted_memory_bytes: u64,
    admitted_page_reads: u16,
    admitted_chunk_reads: u16,
    admitted_range_touches: u16,
    admitted_byte_reads: u64,
}

impl S8PreExecutionBudgetEnvelope {
    pub const fn new(
        scope: S8PreExecutionBudgetScope,
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
            S8PreExecutionBudgetScope::Foreground,
            16_384,
            8,
            0,
            16,
            65_536,
        )
    }

    pub const fn maintenance_default() -> Self {
        Self::new(
            S8PreExecutionBudgetScope::Maintenance,
            32_768,
            16,
            4,
            32,
            131_072,
        )
    }

    pub const fn verifier_default() -> Self {
        Self::new(
            S8PreExecutionBudgetScope::Verifier,
            24_576,
            12,
            2,
            24,
            98_304,
        )
    }

    pub const fn terminal_default() -> Self {
        Self::new(
            S8PreExecutionBudgetScope::Terminal,
            24_576,
            12,
            2,
            24,
            98_304,
        )
    }

    pub const fn streaming_default() -> Self {
        Self::new(
            S8PreExecutionBudgetScope::Streaming,
            32_768,
            4,
            8,
            8,
            262_144,
        )
    }

    pub const fn scope(self) -> S8PreExecutionBudgetScope {
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
pub enum S8PreExecutionBudgetAdmissionOutcome {
    Admitted(S8PreExecutionBudgetAdmissionReceipt),
    Denied(S8PreExecutionBudgetDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PreExecutionBudgetAdmission;

impl S8PreExecutionBudgetAdmission {
    pub fn admit(
        &self,
        request: S8PreExecutionBudgetRequest,
        envelope: S8PreExecutionBudgetEnvelope,
    ) -> S8PreExecutionBudgetAdmissionOutcome {
        if request.scope() != envelope.scope() {
            return S8PreExecutionBudgetAdmissionOutcome::Denied(
                S8PreExecutionBudgetDenial::ScopeMismatch {
                    requested: request.scope(),
                    admitted: envelope.scope(),
                },
            );
        }

        if request.estimated_memory_bytes() > envelope.admitted_memory_bytes() {
            return S8PreExecutionBudgetAdmissionOutcome::Denied(
                S8PreExecutionBudgetDenial::MemoryBytesExceeded {
                    estimated: request.estimated_memory_bytes(),
                    admitted: envelope.admitted_memory_bytes(),
                },
            );
        }
        if request.estimated_page_reads() > envelope.admitted_page_reads() {
            return S8PreExecutionBudgetAdmissionOutcome::Denied(
                S8PreExecutionBudgetDenial::PageReadsExceeded {
                    estimated: request.estimated_page_reads(),
                    admitted: envelope.admitted_page_reads(),
                },
            );
        }
        if request.estimated_chunk_reads() > envelope.admitted_chunk_reads() {
            return S8PreExecutionBudgetAdmissionOutcome::Denied(
                S8PreExecutionBudgetDenial::ChunkReadsExceeded {
                    estimated: request.estimated_chunk_reads(),
                    admitted: envelope.admitted_chunk_reads(),
                },
            );
        }
        if request.estimated_range_touches() > envelope.admitted_range_touches() {
            return S8PreExecutionBudgetAdmissionOutcome::Denied(
                S8PreExecutionBudgetDenial::RangeTouchesExceeded {
                    estimated: request.estimated_range_touches(),
                    admitted: envelope.admitted_range_touches(),
                },
            );
        }
        if request.estimated_byte_reads() > envelope.admitted_byte_reads() {
            return S8PreExecutionBudgetAdmissionOutcome::Denied(
                S8PreExecutionBudgetDenial::ByteReadsExceeded {
                    estimated: request.estimated_byte_reads(),
                    admitted: envelope.admitted_byte_reads(),
                },
            );
        }

        S8PreExecutionBudgetAdmissionOutcome::Admitted(S8PreExecutionBudgetAdmissionReceipt::new(
            request.plan_binding(),
            request.scope(),
            envelope,
        ))
    }
}

pub const fn pre_execution_budget_admission() -> S8PreExecutionBudgetAdmission {
    S8PreExecutionBudgetAdmission
}
