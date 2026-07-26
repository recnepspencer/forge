use worth_store_physical_format::DurableInlineRecordPlacement;

use super::{PhysicalRecordReader, ReadPlacement, RecordReadSession};
use crate::physical_runtime::record_serving::{
    residency::record_frame_reader::RecordFrameReader, PhysicalRecordId, RecordReadDenial,
    RecordReadObservation,
};

mod page_location;
mod record_projection;

impl PhysicalRecordReader {
    pub(super) fn open_inline(
        &self,
        record: PhysicalRecordId,
        placement: DurableInlineRecordPlacement,
        observation: &mut RecordReadObservation,
        allocation: worth_store_buffer_pool::OperationAllocationGrant,
    ) -> Result<RecordReadSession, RecordReadDenial> {
        let runtime = self
            .runtime
            .upgrade()
            .ok_or(RecordReadDenial::ServingRequiresInspection)?;
        let health_permit = runtime
            .health
            .permit()
            .map_err(|_| RecordReadDenial::ServingRequiresInspection)?;
        let artifacts = RecordFrameReader::serving(self.residency.clone());
        let location = page_location::locate_inline_page(
            self,
            placement,
            observation,
            &allocation,
            &artifacts,
        )?;
        let page = location.load(&allocation, &artifacts, observation)?;
        let projected =
            record_projection::project_inline_record(record_projection::InlineRecordProjection {
                reader: self,
                record,
                placement,
                page,
                observation,
            })?;
        observation.touched_segments = 1;
        observation.touched_pages = 1;
        Ok(RecordReadSession {
            placement: ReadPlacement::Inline {
                frame: projected.frame,
                payload: projected.payload,
                offset: 0,
            },
            identity: self.read_identity(record),
            observation: *observation,
            runtime: self.runtime.clone(),
            health_permit,
            _lifecycle: self.lifecycle.read_session(),
            _allocation: allocation,
        })
    }
}
