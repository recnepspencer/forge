use std::ops::Range;

use worth_store_physical_format::{
    decode_inline_record, inspect_inline_page, DurableInlineRecordPlacement, InlinePageDenial,
};

use super::super::PhysicalRecordReader;
use crate::physical_runtime::record_serving::{
    residency::frame_loading::LoadedPhysicalFrame, PhysicalRecordId, RecordReadDenial,
    RecordReadObservation, StalePhysicalRecordPlacement,
};

pub(super) struct ProjectedInlineRecord {
    pub(super) frame: LoadedPhysicalFrame,
    pub(super) payload: Range<usize>,
}

pub(super) struct InlineRecordProjection<'projection> {
    pub(super) reader: &'projection PhysicalRecordReader,
    pub(super) record: PhysicalRecordId,
    pub(super) placement: DurableInlineRecordPlacement,
    pub(super) page: LoadedPhysicalFrame,
    pub(super) observation: &'projection mut RecordReadObservation,
}

pub(super) fn project_inline_record(
    projection: InlineRecordProjection<'_>,
) -> Result<ProjectedInlineRecord, RecordReadDenial> {
    let InlineRecordProjection {
        reader,
        record,
        placement,
        page,
        observation,
    } = projection;
    let geometry = match inspect_inline_page(reader.format.declaration(), &page) {
        Ok(geometry) => geometry,
        Err(_) => {
            page.reject_projection_failure();
            return Err(RecordReadDenial::ArtifactDamaged);
        }
    };
    if !observation.check_generation(geometry.page_cell() == placement.page_cell()) {
        page.reject_projection_failure();
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
    let (payload, format) = match decoded {
        Ok(decoded) => decoded,
        Err(denial) => {
            page.reject_projection_failure();
            return Err(classify_projection_denial(denial));
        }
    };
    if format != reader.format.declaration()
        || payload.range().len() as u64 != placement.payload_bytes()
    {
        page.reject_projection_failure();
        return Err(RecordReadDenial::FormatMismatch);
    }
    Ok(ProjectedInlineRecord {
        frame: page,
        payload: payload.range(),
    })
}

fn classify_projection_denial(denial: InlinePageDenial) -> RecordReadDenial {
    if denial == InlinePageDenial::SlotGenerationMismatch {
        RecordReadDenial::StalePlacement(StalePhysicalRecordPlacement::SlotGeneration)
    } else {
        RecordReadDenial::ArtifactDamaged
    }
}

fn observe_slot_generation(
    observation: &mut RecordReadObservation,
    decoded: &Result<
        (
            worth_store_physical_format::InlineRecordRange,
            worth_store_physical_format::PhysicalRecordFormatDeclaration,
        ),
        InlinePageDenial,
    >,
) {
    match decoded {
        Ok(_) => {
            observation.check_generation(true);
        }
        Err(InlinePageDenial::SlotGenerationMismatch) => {
            observation.check_generation(false);
        }
        Err(_) => {}
    }
}
