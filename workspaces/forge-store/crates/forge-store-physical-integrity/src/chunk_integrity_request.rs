use crate::{
    ChunkIntegrityCounters, ChunkIntegrityDenial, ChunkIntegrityDenialKind,
    ChunkIntegrityStreamingWindowDenial, ScopedPhysicalValidatorInput,
};
use forge_store_buffer_pool::{AdmittedBackgroundEnvelope, BackgroundWorkClass};
use forge_store_physical_format::PhysicalScopeFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkIntegrityStreamingWindow {
    object_bytes: u64,
    window_bytes: u64,
}

impl ChunkIntegrityStreamingWindow {
    pub fn from_admitted_streaming_envelope(
        envelope: AdmittedBackgroundEnvelope,
    ) -> Result<Self, ChunkIntegrityStreamingWindowDenial> {
        if envelope.work_class() != BackgroundWorkClass::LargeRecordStreaming {
            return Err(
                ChunkIntegrityStreamingWindowDenial::WrongBackgroundEnvelopeClass {
                    actual: envelope.work_class(),
                },
            );
        }
        Self::bounded(
            envelope.streaming_object_bytes(),
            envelope.streaming_window_bytes(),
        )
    }

    pub(crate) fn bounded(
        object_bytes: u64,
        window_bytes: u64,
    ) -> Result<Self, ChunkIntegrityStreamingWindowDenial> {
        if window_bytes == 0 {
            return Err(ChunkIntegrityStreamingWindowDenial::EmptyWindow);
        }
        if window_bytes >= object_bytes {
            return Err(ChunkIntegrityStreamingWindowDenial::WholeObjectWindow);
        }
        Ok(Self {
            object_bytes,
            window_bytes,
        })
    }

    pub const fn object_bytes(self) -> u64 {
        self.object_bytes
    }

    pub const fn window_bytes(self) -> u64 {
        self.window_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkIntegrityInspectionRequest<'lease> {
    input: ScopedPhysicalValidatorInput<'lease>,
    streaming_window: ChunkIntegrityStreamingWindow,
}

impl<'lease> ChunkIntegrityInspectionRequest<'lease> {
    pub fn from_admitted_chunk_window(
        input: ScopedPhysicalValidatorInput<'lease>,
        streaming_window: ChunkIntegrityStreamingWindow,
    ) -> Result<Self, ChunkIntegrityDenial> {
        reject_non_chunk_family(&input)?;
        Ok(Self {
            input,
            streaming_window,
        })
    }

    pub(crate) const fn input(&self) -> &ScopedPhysicalValidatorInput<'lease> {
        &self.input
    }

    pub(crate) const fn streaming_window(&self) -> ChunkIntegrityStreamingWindow {
        self.streaming_window
    }
}

fn reject_non_chunk_family(
    input: &ScopedPhysicalValidatorInput<'_>,
) -> Result<(), ChunkIntegrityDenial> {
    if input.family() == PhysicalScopeFamily::ChunkLike {
        return Ok(());
    }
    Err(ChunkIntegrityDenial::new(
        ChunkIntegrityDenialKind::WrongPhysicalFamily,
        ChunkIntegrityCounters::start(1, 1, 0),
    )
    .with_basis(input.admission().basis().clone()))
}
