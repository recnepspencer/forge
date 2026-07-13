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
pub enum PreExecutionBudgetAdmissionOutcome {
    Admitted(PreExecutionBudgetAdmissionReceipt),
    Denied(PreExecutionBudgetDenial),
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
            return PreExecutionBudgetAdmissionOutcome::Denied(
                PreExecutionBudgetDenial::ScopeMismatch {
                    requested: request.scope(),
                    admitted: envelope.scope(),
                },
            );
        }

        if request.estimated_memory_bytes() > envelope.admitted_memory_bytes() {
            return PreExecutionBudgetAdmissionOutcome::Denied(
                PreExecutionBudgetDenial::MemoryBytesExceeded {
                    estimated: request.estimated_memory_bytes(),
                    admitted: envelope.admitted_memory_bytes(),
                },
            );
        }
        if request.estimated_page_reads() > envelope.admitted_page_reads() {
            return PreExecutionBudgetAdmissionOutcome::Denied(
                PreExecutionBudgetDenial::PageReadsExceeded {
                    estimated: request.estimated_page_reads(),
                    admitted: envelope.admitted_page_reads(),
                },
            );
        }
        if request.estimated_chunk_reads() > envelope.admitted_chunk_reads() {
            return PreExecutionBudgetAdmissionOutcome::Denied(
                PreExecutionBudgetDenial::ChunkReadsExceeded {
                    estimated: request.estimated_chunk_reads(),
                    admitted: envelope.admitted_chunk_reads(),
                },
            );
        }
        if request.estimated_range_touches() > envelope.admitted_range_touches() {
            return PreExecutionBudgetAdmissionOutcome::Denied(
                PreExecutionBudgetDenial::RangeTouchesExceeded {
                    estimated: request.estimated_range_touches(),
                    admitted: envelope.admitted_range_touches(),
                },
            );
        }
        if request.estimated_byte_reads() > envelope.admitted_byte_reads() {
            return PreExecutionBudgetAdmissionOutcome::Denied(
                PreExecutionBudgetDenial::ByteReadsExceeded {
                    estimated: request.estimated_byte_reads(),
                    admitted: envelope.admitted_byte_reads(),
                },
            );
        }

        PreExecutionBudgetAdmissionOutcome::Admitted(PreExecutionBudgetAdmissionReceipt::new(
            request,
            request.scope(),
            envelope,
        ))
    }
}

pub const fn pre_execution_budget_admission() -> PreExecutionBudgetAdmission {
    PreExecutionBudgetAdmission
}
