use worth_query_installation::facade::{
    WorthQueryPortablePackageManifest, WorthQueryPortablePackageRecordFamily,
    WORTH_QUERY_PORTABLE_PACKAGE_MANIFEST_VERSION,
};

use crate::binary_output::BinaryOutput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::limits::WorthQueryPackageArchiveLimits;
use crate::protocol::{MAGIC, WORTH_QUERY_PACKAGE_ARCHIVE_PROTOCOL_VERSION};

use super::protocol::{MANIFEST_FRAME_BYTES, MANIFEST_PAYLOAD_BYTES};

pub fn encode_manifest_frame(
    manifest: &WorthQueryPortablePackageManifest,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<Vec<u8>, Denial> {
    let limits = limits.narrowed();
    validate(manifest, limits)?;
    let mut output = BinaryOutput::with_capacity(MANIFEST_FRAME_BYTES as usize);
    output.raw_bytes(&MAGIC);
    output.u16(WORTH_QUERY_PACKAGE_ARCHIVE_PROTOCOL_VERSION);
    output.u32(MANIFEST_PAYLOAD_BYTES);
    output.u16(manifest.version().get());
    output.raw_bytes(manifest.package_identity().bytes());
    output.u32(manifest.record_count());
    output.u64(manifest.canonical_source_bytes());
    output.u64(manifest.logical_export_bytes());
    output.u16(WorthQueryPortablePackageRecordFamily::ALL.len() as u16);
    for family in WorthQueryPortablePackageRecordFamily::ALL {
        output.u32(manifest.family_count(family));
    }
    Ok(output.into_bytes())
}

fn validate(
    manifest: &WorthQueryPortablePackageManifest,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<(), Denial> {
    if MANIFEST_FRAME_BYTES > limits.maximum_manifest_frame_bytes() {
        return Err(Denial::new(Kind::ManifestFrameByteBudgetExceeded));
    }
    if manifest.version() != WORTH_QUERY_PORTABLE_PACKAGE_MANIFEST_VERSION {
        return Err(Denial::new(Kind::UnsupportedManifestVersion));
    }
    if manifest.record_count() > limits.maximum_records() {
        return Err(Denial::new(Kind::RecordBudgetExceeded));
    }
    if manifest.logical_export_bytes() > limits.maximum_logical_bytes() {
        return Err(Denial::new(Kind::LogicalByteBudgetExceeded));
    }
    if manifest.canonical_source_bytes() > limits.maximum_canonical_work_bytes() {
        return Err(Denial::new(Kind::CanonicalWorkBudgetExceeded));
    }
    let family_total = WorthQueryPortablePackageRecordFamily::ALL
        .iter()
        .try_fold(0_u32, |total, family| {
            total.checked_add(manifest.family_count(*family))
        })
        .ok_or_else(|| Denial::new(Kind::InvalidFamilyCount))?;
    if family_total != manifest.record_count() {
        return Err(Denial::new(Kind::InvalidFamilyCount));
    }
    Ok(())
}
