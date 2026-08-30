use worth_query_installation::facade::WorthQueryPortablePackageRecordSet;

use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::limits::WorthQueryPackageArchiveLimits;
use crate::manifest::encode_manifest_frame;
use crate::record::{encode_record_frame_after, RecordEncodingWork};

/// Encodes one validated package export as the canonical versioned archive stream.
pub fn encode_package_archive(
    records: &WorthQueryPortablePackageRecordSet,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<Vec<u8>, Denial> {
    let limits = limits.narrowed();
    let mut archive = encode_manifest_frame(records.manifest(), limits)?;
    require_archive_byte_budget(archive.len(), limits)?;
    let mut work = RecordEncodingWork::default();
    for view in records.views() {
        let remaining = remaining_archive_bytes(archive.len(), limits)?;
        let (frame, next_work) = encode_record_frame_after(view, limits, work, remaining)?;
        append_bounded(&mut archive, &frame, limits)?;
        work = next_work;
    }
    if work.record_frames() != records.manifest().record_count() {
        return Err(Denial::new(Kind::InvalidFamilyCount));
    }
    Ok(archive)
}

fn remaining_archive_bytes(
    observed: usize,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<u64, Denial> {
    let observed =
        u64::try_from(observed).map_err(|_| Denial::new(Kind::ArchiveByteBudgetExceeded))?;
    limits
        .maximum_archive_bytes()
        .checked_sub(observed)
        .ok_or_else(|| Denial::new(Kind::ArchiveByteBudgetExceeded))
}

fn append_bounded(
    archive: &mut Vec<u8>,
    frame: &[u8],
    limits: WorthQueryPackageArchiveLimits,
) -> Result<(), Denial> {
    let length = archive
        .len()
        .checked_add(frame.len())
        .ok_or_else(|| Denial::new(Kind::ArchiveByteBudgetExceeded))?;
    require_archive_byte_budget(length, limits)?;
    archive.extend_from_slice(frame);
    Ok(())
}

fn require_archive_byte_budget(
    observed: usize,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<(), Denial> {
    if u64::try_from(observed).unwrap_or(u64::MAX) > limits.maximum_archive_bytes() {
        return Err(Denial::new(Kind::ArchiveByteBudgetExceeded));
    }
    Ok(())
}
