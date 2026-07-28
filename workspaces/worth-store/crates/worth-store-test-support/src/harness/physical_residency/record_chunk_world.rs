use worth_store::physical_runtime::{
    PhysicalRecordChunkView, RecordAppendBatch, RecordAppendDenial, RecordAppendError,
    RecordByteLimit, RecordReadError, RecordReadLimits, RecordStreamFailure,
    ServingPhysicalRuntime,
};

use super::PhysicalResidencyStoreWorld;

#[derive(Debug)]
pub enum PhysicalResidencyRecordWorldFailure {
    EmptyPayload,
    PayloadTooLarge,
    Batch(RecordAppendDenial),
    Append(RecordAppendError),
    Read(Box<RecordReadError>),
    Stream(RecordStreamFailure),
    MissingChunk,
}

impl PhysicalResidencyStoreWorld {
    pub fn with_record_chunk<R>(
        &self,
        payload: &[u8],
        run: impl FnOnce(&ServingPhysicalRuntime, PhysicalRecordChunkView<'_>) -> R,
    ) -> Result<R, PhysicalResidencyRecordWorldFailure> {
        let width = u32::try_from(payload.len())
            .map_err(|_| PhysicalResidencyRecordWorldFailure::PayloadTooLarge)?;
        let limit =
            RecordByteLimit::new(width).ok_or(PhysicalResidencyRecordWorldFailure::EmptyPayload)?;
        let batch = RecordAppendBatch::try_from_iter([payload])
            .map_err(PhysicalResidencyRecordWorldFailure::Batch)?;
        let published = self
            .serving()
            .record_submission()
            .append_batch(batch, self.placement)
            .map_err(PhysicalResidencyRecordWorldFailure::Append)?;
        let mut session = self
            .serving()
            .records()
            .open(
                published
                    .record_id(0)
                    .expect("one admitted record produces one identity"),
                RecordReadLimits::new(limit),
            )
            .map_err(|error| PhysicalResidencyRecordWorldFailure::Read(Box::new(error)))?;
        let chunk = session
            .next_chunk()
            .map_err(PhysicalResidencyRecordWorldFailure::Stream)?
            .ok_or(PhysicalResidencyRecordWorldFailure::MissingChunk)?;
        Ok(run(self.serving(), chunk))
    }
}
