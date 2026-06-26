#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SkippedLogicalDecodeCounter {
    skipped_count: u32,
}

impl SkippedLogicalDecodeCounter {
    pub const fn none() -> Self {
        Self { skipped_count: 0 }
    }

    pub const fn one_skipped_decode() -> Self {
        Self { skipped_count: 1 }
    }

    pub const fn skipped_count(self) -> u32 {
        self.skipped_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SemanticDecoderInvocationCounter {
    invocation_count: u32,
}

impl SemanticDecoderInvocationCounter {
    pub const fn zero() -> Self {
        Self {
            invocation_count: 0,
        }
    }

    pub const fn one_invocation() -> Self {
        Self {
            invocation_count: 1,
        }
    }

    pub const fn invocation_count(self) -> u32 {
        self.invocation_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreDecodeAdmissionCounters {
    checked_byte_count: u64,
    checksum_execution_count: u32,
    skipped_logical_decode: SkippedLogicalDecodeCounter,
    semantic_decoder_invocations: SemanticDecoderInvocationCounter,
}

impl PreDecodeAdmissionCounters {
    pub const fn admitted(checked_byte_count: u64) -> Self {
        Self {
            checked_byte_count,
            checksum_execution_count: 1,
            skipped_logical_decode: SkippedLogicalDecodeCounter::none(),
            semantic_decoder_invocations: SemanticDecoderInvocationCounter::zero(),
        }
    }

    pub const fn denied_before_decode(checked_byte_count: u64) -> Self {
        Self {
            checked_byte_count,
            checksum_execution_count: 0,
            skipped_logical_decode: SkippedLogicalDecodeCounter::one_skipped_decode(),
            semantic_decoder_invocations: SemanticDecoderInvocationCounter::zero(),
        }
    }

    pub const fn denied_after_checksum(checked_byte_count: u64) -> Self {
        Self {
            checked_byte_count,
            checksum_execution_count: 1,
            skipped_logical_decode: SkippedLogicalDecodeCounter::one_skipped_decode(),
            semantic_decoder_invocations: SemanticDecoderInvocationCounter::zero(),
        }
    }

    pub const fn checked_byte_count(self) -> u64 {
        self.checked_byte_count
    }

    pub const fn checksum_execution_count(self) -> u32 {
        self.checksum_execution_count
    }

    pub const fn skipped_logical_decode(self) -> SkippedLogicalDecodeCounter {
        self.skipped_logical_decode
    }

    pub const fn semantic_decoder_invocations(self) -> SemanticDecoderInvocationCounter {
        self.semantic_decoder_invocations
    }
}
