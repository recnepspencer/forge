use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::{
    CurrentPhysicalRecordPlacement, DurableInlineRecordPlacement, DurablePhysicalRootManifest,
};

use super::super::{
    planning::inline_plan_failure::manifest_lookup_failure, AdmittedPhysicalRecordFormat,
    AdmittedRecordAccessPolicy, AdmittedRecordPlacementPolicy, RecordAppendError,
};

pub(in crate::physical_runtime::record_serving) fn last_inline_placement(
    media: &QualifiedFilesystemMedia,
    frame_load: &(dyn super::super::residency::frame_ports::FrameLoadPort + Send + Sync),
    format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
    root: &DurablePhysicalRootManifest,
    placement: AdmittedRecordPlacementPolicy,
) -> Result<Option<DurableInlineRecordPlacement>, RecordAppendError> {
    let Some(record) = root.last_inline_record() else {
        return Ok(None);
    };
    let mut counters =
        super::super::access::manifest_routing::ManifestDiscoveryCounterSnapshot::default();
    let reader = super::super::access::manifest_routing::ManifestReader::with_loader(
        media, frame_load, format, access, root,
    );
    let found = reader
        .locate(record, &mut counters)
        .map_err(manifest_lookup_failure)?;
    match found {
        Some(CurrentPhysicalRecordPlacement::Inline(value))
            if root.last_inline_segment() == Some(value.segment_cell()) =>
        {
            Ok((value.segment_page_capacity() == placement.segment_pages().get()).then_some(value))
        }
        _ => Err(RecordAppendError::Denied(
            super::super::RecordAppendDenial::PublishedLayoutDamaged,
        )),
    }
}
