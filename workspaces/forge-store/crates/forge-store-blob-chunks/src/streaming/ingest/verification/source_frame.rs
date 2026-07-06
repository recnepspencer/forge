use crate::BlobStreamingIngestDenial;

pub(crate) fn reject_whole_object_frame(
    frame_len: u64,
    declared_total_bytes: u64,
) -> Result<(), BlobStreamingIngestDenial> {
    if frame_len >= declared_total_bytes {
        Err(BlobStreamingIngestDenial::WholeObjectMaterializationRejected { bytes: frame_len })
    } else {
        Ok(())
    }
}

pub(crate) fn reject_if_offset_exceeds_declared(
    start_offset: u64,
    declared_total_bytes: u64,
) -> Result<(), BlobStreamingIngestDenial> {
    if start_offset > declared_total_bytes {
        Err(BlobStreamingIngestDenial::WholeObjectMaterializationRejected {
            bytes: start_offset,
        })
    } else {
        Ok(())
    }
}