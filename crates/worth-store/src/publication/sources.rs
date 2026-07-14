use crate::{
    backend::records::{SnapshotBasisRecord, SnapshotImageRecord},
    failure::{StoreError, StoreErrorKind},
};

use super::LocalAdmittedPublicationSource;

pub(crate) fn admit_local_snapshot_basis_source(
    record: SnapshotBasisRecord,
) -> Result<LocalAdmittedPublicationSource<SnapshotBasisRecord>, StoreError> {
    if record.snapshot_family_version != crate::snapshot::SNAPSHOT_FAMILY_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} uses unsupported family version {}",
                record.snapshot_id.0, record.snapshot_family_version
            ),
        ));
    }
    if record.snapshot_basis_version != crate::snapshot::SNAPSHOT_BASIS_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} uses unsupported basis version {}",
                record.snapshot_id.0, record.snapshot_basis_version
            ),
        ));
    }
    if record.snapshot_image_format_version != crate::snapshot::SNAPSHOT_IMAGE_FORMAT_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} uses unsupported image format version {}",
                record.snapshot_id.0, record.snapshot_image_format_version
            ),
        ));
    }
    Ok(LocalAdmittedPublicationSource::new(record))
}

pub(crate) fn admit_local_snapshot_image_source(
    record: SnapshotImageRecord,
) -> Result<LocalAdmittedPublicationSource<SnapshotImageRecord>, StoreError> {
    if record.image.snapshot_family_version() != crate::snapshot::SNAPSHOT_FAMILY_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} image uses unsupported family version {}",
                record.snapshot_id.0,
                record.image.snapshot_family_version()
            ),
        ));
    }
    if record.image.snapshot_basis_version() != crate::snapshot::SNAPSHOT_BASIS_VERSION {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} image uses unsupported basis version {}",
                record.snapshot_id.0,
                record.image.snapshot_basis_version()
            ),
        ));
    }
    if record.image.snapshot_image_format_version()
        != crate::snapshot::SNAPSHOT_IMAGE_FORMAT_VERSION
    {
        return Err(StoreError::new(
            StoreErrorKind::SnapshotFamilyVersionUnsupported,
            format!(
                "snapshot {} image uses unsupported image format version {}",
                record.snapshot_id.0,
                record.image.snapshot_image_format_version()
            ),
        ));
    }
    Ok(LocalAdmittedPublicationSource::new(record))
}

pub(crate) fn admit_local_wal_record<'a>(
    record: &'a crate::wal::WalRecord,
) -> Result<LocalAdmittedPublicationSource<&'a crate::wal::WalRecord>, StoreError> {
    record.validate_integrity()?;
    Ok(LocalAdmittedPublicationSource::new(record))
}
