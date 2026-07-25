use worth_store_physical_format::{DurableInlineRecordPlacement, RecordArtifactFile};

use super::super::failure_classification::{manifest_failure, read_failure};
use super::super::PhysicalRecordReader;
use crate::physical_runtime::record_serving::{
    residency::record_frame_reader::RecordFrameReader, RecordReadDenial, RecordReadObservation,
    StalePhysicalRecordPlacement,
};

pub(super) struct InlinePageLocation {
    artifact: RecordArtifactFile,
    offset: u64,
    page_bytes: u32,
    segment_bytes: u64,
}

pub(super) fn locate_inline_page(
    reader: &PhysicalRecordReader,
    placement: DurableInlineRecordPlacement,
    observation: &mut RecordReadObservation,
    artifacts: &RecordFrameReader<'_>,
) -> Result<InlinePageLocation, RecordReadDenial> {
    let mut discovery =
        super::super::super::manifest_routing::ManifestDiscoveryCounterSnapshot::default();
    let page_entry = super::super::super::segment_membership::SegmentMembershipReader::serving(
        reader.frame_ports.clone(),
        reader.source.clone(),
        reader.format,
        reader.access,
        reader.current_root.clone(),
    )
    .locate(placement.segment(), placement.page(), &mut discovery);
    observation.observe_manifest(discovery);
    let page_entry =
        page_entry
            .map_err(manifest_failure)?
            .ok_or(RecordReadDenial::StalePlacement(
                StalePhysicalRecordPlacement::SegmentMembership,
            ))?;
    require_current_membership(&page_entry, placement, observation)?;

    let page_bytes = reader.format.declaration().page_size().bytes();
    let artifact = RecordArtifactFile::Segment {
        segment: placement.segment().get(),
        generation: page_entry.data_generation(),
    };
    InlinePageLocation {
        artifact,
        offset: u64::from(page_entry.frame_index()) * u64::from(page_bytes),
        page_bytes,
        segment_bytes: u64::from(page_entry.data_page_count()) * u64::from(page_bytes),
    }
    .require_complete_segment(artifacts, observation)
}

impl InlinePageLocation {
    pub(super) fn load(
        self,
        artifacts: &RecordFrameReader<'_>,
        observation: &mut RecordReadObservation,
    ) -> Result<
        crate::physical_runtime::record_serving::residency::frame_loading::LoadedPhysicalFrame,
        RecordReadDenial,
    > {
        let page = artifacts
            .load_exact(self.artifact, self.offset, self.page_bytes)
            .map_err(|failure| {
                observation.observe_physical_work(failure.work_trace());
                read_failure(failure)
            })?;
        observation.observe_physical_work(page.work_trace());
        observation.observe_transfer(page.len());
        Ok(page)
    }

    fn require_complete_segment(
        self,
        artifacts: &RecordFrameReader<'_>,
        observation: &mut RecordReadObservation,
    ) -> Result<Self, RecordReadDenial> {
        let segment_length = artifacts.file_length(self.artifact).map_err(|failure| {
            observation.observe_physical_work(failure.work_trace());
            read_failure(failure)
        })?;
        observation.observe_physical_work(segment_length.work_trace());
        if segment_length.bytes() != self.segment_bytes {
            segment_length.reject_structural_damage();
            return Err(RecordReadDenial::ArtifactDamaged);
        }
        Ok(self)
    }
}

fn require_current_membership(
    page_entry: &worth_store_physical_format::RecordSegmentPageManifestEntry,
    placement: DurableInlineRecordPlacement,
    observation: &mut RecordReadObservation,
) -> Result<(), RecordReadDenial> {
    if !observation.check_generation(page_entry.data_segment_cell() == placement.segment_cell()) {
        return Err(RecordReadDenial::StalePlacement(
            StalePhysicalRecordPlacement::SegmentGeneration,
        ));
    }
    if !observation.check_generation(page_entry.page_cell() == placement.page_cell()) {
        return Err(RecordReadDenial::StalePlacement(
            StalePhysicalRecordPlacement::PageGeneration,
        ));
    }
    Ok(())
}
