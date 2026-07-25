use super::{ReadPlacement, RecordReadSession};
use crate::physical_runtime::{
    RecordReadObservation, RecordStreamFailure, RecordStreamFailureKind,
};

impl RecordReadSession {
    pub fn read_next(&mut self, target: &mut [u8]) -> Result<usize, RecordStreamFailure> {
        let runtime = self.runtime.upgrade().ok_or_else(|| {
            RecordStreamFailure::during_read(
                RecordStreamFailureKind::ServingRequiresInspection,
                self.observation.payload_bytes(),
            )
        })?;
        if runtime.health.require(self.health_permit).is_err() {
            return Err(RecordStreamFailure::during_read(
                RecordStreamFailureKind::ServingRequiresInspection,
                self.observation.payload_bytes(),
            ));
        }
        if target.is_empty() {
            return Ok(0);
        }
        let count = match &mut self.placement {
            ReadPlacement::Inline {
                frame,
                payload,
                offset,
            } => {
                let count = target.len().min(payload.len().saturating_sub(*offset));
                let start = payload.start + *offset;
                frame.copy_range_into(start..start + count, &mut target[..count]);
                *offset += count;
                count
            }
            ReadPlacement::Extent(state) => match state.read_next(target, &mut self.observation) {
                Ok(count) => count,
                Err(failure) => {
                    runtime.health.observe_stream_failure(failure.kind());
                    return Err(failure);
                }
            },
        };
        self.observation.observe_copy(count);
        self.observation.payload_bytes =
            self.observation.payload_bytes.saturating_add(count as u64);
        Ok(count)
    }

    pub const fn observation(&self) -> RecordReadObservation {
        self.observation
    }
}
