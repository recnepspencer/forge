use worth_store_physical_format::{
    inspect_inline_page, DurableInlineRecordPlacement, InlinePageGeometry, RecordArtifactFile,
};

use super::super::{
    planning::inline_plan_failure::layout_failure, planning::inline_segment_plan::PlanningSegment,
    residency::serving_artifacts::ServingRecordArtifacts, AdmittedPhysicalRecordFormat,
    RecordAppendDenial, RecordAppendError,
};

pub(in crate::physical_runtime::record_serving) struct LoadedPublishedTailPage {
    pub(in crate::physical_runtime::record_serving) page: Vec<u8>,
    pub(in crate::physical_runtime::record_serving) geometry: InlinePageGeometry,
}

pub(in crate::physical_runtime::record_serving) fn load_published_tail_page(
    artifacts: &ServingRecordArtifacts<'_>,
    format: AdmittedPhysicalRecordFormat,
    last: DurableInlineRecordPlacement,
    segment: &PlanningSegment,
) -> Result<LoadedPublishedTailPage, RecordAppendError> {
    let page_entry = segment
        .last_published_page
        .ok_or(RecordAppendError::Denied(
            RecordAppendDenial::PublishedLayoutDamaged,
        ))?;
    if page_entry.page_cell() != last.page_cell() {
        return Err(RecordAppendError::Denied(
            RecordAppendDenial::PublishedLayoutDamaged,
        ));
    }
    let page_bytes = format.declaration().page_size().bytes();
    let source = RecordArtifactFile::Segment {
        segment: last.segment().get(),
        generation: page_entry.data_generation(),
    };
    let source_length = artifacts.file_length(source).map_err(layout_failure)?;
    if source_length.bytes() != u64::from(page_entry.data_page_count()) * u64::from(page_bytes) {
        source_length.reject_structural_damage();
        return Err(RecordAppendError::Denied(
            RecordAppendDenial::PublishedLayoutDamaged,
        ));
    }
    let resident = artifacts
        .load_exact(
            source,
            u64::from(page_entry.frame_index()) * u64::from(page_bytes),
            page_bytes,
        )
        .map_err(layout_failure)?;
    let mut page = Vec::new();
    page.try_reserve_exact(resident.len()).map_err(|_| {
        RecordAppendError::Denied(RecordAppendDenial::ResidencyUnavailable(
            worth_store_buffer_pool::PhysicalResidencyDenial::AllocationFailed,
        ))
    })?;
    page.extend_from_slice(&resident);
    let geometry = match inspect_inline_page(format.declaration(), &page) {
        Ok(geometry) => geometry,
        Err(_) => {
            resident.reject_projection_failure();
            return Err(RecordAppendError::Denied(
                RecordAppendDenial::PublishedLayoutDamaged,
            ));
        }
    };
    if geometry.page_cell() != last.page_cell() {
        resident.reject_projection_failure();
        return Err(RecordAppendError::Denied(
            RecordAppendDenial::PublishedLayoutDamaged,
        ));
    }
    Ok(LoadedPublishedTailPage { page, geometry })
}
