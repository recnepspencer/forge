use forge_store_buffer_pool::{
    AdmittedBackgroundEnvelope, AllocationScope, BackgroundEnvelopeCounterSnapshot,
    BackgroundWorkClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LargeRecordStreamingEnvelope {
    envelope: AdmittedBackgroundEnvelope,
}

impl LargeRecordStreamingEnvelope {
    pub fn from_admitted(
        envelope: AdmittedBackgroundEnvelope,
    ) -> Result<Self, LargeRecordStreamingEnvelopeDenial> {
        if envelope.work_class() != BackgroundWorkClass::LargeRecordStreaming {
            return Err(
                LargeRecordStreamingEnvelopeDenial::WrongBackgroundEnvelopeClass {
                    expected: BackgroundWorkClass::LargeRecordStreaming,
                    actual: envelope.work_class(),
                },
            );
        }
        Ok(Self { envelope })
    }

    pub const fn allocation_scope(self) -> AllocationScope {
        self.envelope.allocation_scope()
    }

    pub const fn object_bytes(self) -> u64 {
        self.envelope.streaming_object_bytes()
    }

    pub const fn window_bytes(self) -> u64 {
        self.envelope.streaming_window_bytes()
    }

    pub const fn counters(self) -> BackgroundEnvelopeCounterSnapshot {
        self.envelope.counters()
    }

    pub const fn proves_blob_lifecycle_completion(self) -> bool {
        false
    }

    pub const fn proves_blob_reachability(self) -> bool {
        false
    }

    pub const fn proves_blob_checksum_correctness(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LargeRecordStreamingEnvelopeDenial {
    WrongBackgroundEnvelopeClass {
        expected: BackgroundWorkClass,
        actual: BackgroundWorkClass,
    },
}
