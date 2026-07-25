use worth_store_physical_format::{
    decode_inline_record, inspect_inline_page, DurableInlineRecordPlacement, RecordArtifactFile,
};

use super::failure_classification::{manifest_failure, read_failure};
use super::{PhysicalRecordReader, ReadPlacement, RecordReadSession};
use crate::physical_runtime::record_serving::{
    residency::record_frame_reader::RecordFrameReader, PhysicalRecordId, RecordReadDenial,
    RecordReadObservation, StalePhysicalRecordPlacement,
};

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
        let artifacts = RecordFrameReader::serving(self.frame_ports.clone(), self.source.clone());
        let mut discovery =
            super::super::manifest_routing::ManifestDiscoveryCounterSnapshot::default();
        let page_entry = super::super::segment_membership::SegmentMembershipReader::serving(
            self.frame_ports.clone(),
            self.source.clone(),
            self.format,
            self.access,
            self.current_root.clone(),
        )
        .locate(placement.segment(), placement.page(), &mut discovery);
        observation.observe_manifest(discovery);
        let page_entry =
            page_entry
                .map_err(manifest_failure)?
                .ok_or(RecordReadDenial::StalePlacement(
                    StalePhysicalRecordPlacement::SegmentMembership,
                ))?;
        if !observation.check_generation(page_entry.data_segment_cell() == placement.segment_cell())
        {
            return Err(RecordReadDenial::StalePlacement(
                StalePhysicalRecordPlacement::SegmentGeneration,
            ));
        }
        if !observation.check_generation(page_entry.page_cell() == placement.page_cell()) {
            return Err(RecordReadDenial::StalePlacement(
                StalePhysicalRecordPlacement::PageGeneration,
            ));
        }
        let page_bytes = self.format.declaration().page_size().bytes();
        let segment_artifact = RecordArtifactFile::Segment {
            segment: placement.segment().get(),
            generation: page_entry.data_generation(),
        };
        let segment_length = artifacts.file_length(segment_artifact).map_err(|failure| {
            observation.observe_physical_work(failure.work_trace());
            read_failure(failure)
        })?;
        observation.observe_physical_work(segment_length.work_trace());
        if segment_length.bytes() != u64::from(page_entry.data_page_count()) * u64::from(page_bytes)
        {
            return Err(RecordReadDenial::ArtifactDamaged);
        }
        let page = artifacts
            .load_exact(
                segment_artifact,
                u64::from(page_entry.frame_index()) * u64::from(page_bytes),
                page_bytes,
            )
            .map_err(|failure| {
                observation.observe_physical_work(failure.work_trace());
                read_failure(failure)
            })?;
        observation.observe_physical_work(page.work_trace());
        observation.observe_transfer(page.len());
        let geometry = inspect_inline_page(self.format.declaration(), &page)
            .map_err(|_| RecordReadDenial::ArtifactDamaged)?;
        if !observation.check_generation(geometry.page_cell() == placement.page_cell()) {
            return Err(RecordReadDenial::StalePlacement(
                StalePhysicalRecordPlacement::PageIdentity,
            ));
        }
        let decoded = decode_inline_record(
            &page,
            record.persisted(),
            placement.page_cell(),
            placement.slot_cell(),
        );
        observe_slot_generation(observation, &decoded);
        let (payload, format) = decoded.map_err(|denial| {
            if denial == worth_store_physical_format::InlinePageDenial::SlotGenerationMismatch {
                RecordReadDenial::StalePlacement(StalePhysicalRecordPlacement::SlotGeneration)
            } else {
                RecordReadDenial::ArtifactDamaged
            }
        })?;
        if format != self.format.declaration()
            || payload.range().len() as u64 != placement.payload_bytes()
        {
            return Err(RecordReadDenial::FormatMismatch);
        }
        observation.touched_segments = 1;
        observation.touched_pages = 1;
        Ok(RecordReadSession {
            placement: ReadPlacement::Inline {
                frame: page,
                payload: payload.range(),
                offset: 0,
            },
            observation: *observation,
            runtime: self.runtime.clone(),
            health_permit,
            _lifecycle: self.lifecycle.read_session(),
            _allocation: allocation,
        })
    }
}

fn observe_slot_generation(
    observation: &mut RecordReadObservation,
    decoded: &Result<
        (
            worth_store_physical_format::InlineRecordRange,
            worth_store_physical_format::PhysicalRecordFormatDeclaration,
        ),
        worth_store_physical_format::InlinePageDenial,
    >,
) {
    match decoded {
        Ok(_) => {
            observation.check_generation(true);
        }
        Err(worth_store_physical_format::InlinePageDenial::SlotGenerationMismatch) => {
            observation.check_generation(false);
        }
        Err(_) => {}
    }
}
