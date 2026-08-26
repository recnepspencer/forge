//! Open-to-closed structural progression over untrusted typed records.

use crate::package::{
    WorthQueryPortablePackageManifest, WorthQueryPortablePackageRecord,
    WorthQueryPortablePackageRecordFamily, WORTH_QUERY_PORTABLE_PACKAGE_MANIFEST_VERSION,
    WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES,
};

use super::{
    WorthQueryPortablePackageReconstructionCandidate,
    WorthQueryPortablePackageReconstructionDenial as Denial,
    WorthQueryPortablePackageReconstructionLimits, WorthQueryPortablePackageReconstructionWork,
};

pub struct WorthQueryPortablePackageReconstruction {
    manifest: WorthQueryPortablePackageManifest,
    records: Vec<WorthQueryPortablePackageRecord>,
    limits: WorthQueryPortablePackageReconstructionLimits,
    work: WorthQueryPortablePackageReconstructionWork,
}

impl WorthQueryPortablePackageReconstruction {
    pub fn begin(
        manifest: WorthQueryPortablePackageManifest,
        limits: WorthQueryPortablePackageReconstructionLimits,
    ) -> Result<Self, Denial> {
        let limits = limits.narrowed();
        validate_manifest(&manifest, limits)?;
        let capacity =
            usize::try_from(manifest.record_count()).map_err(|_| Denial::RecordBudgetExceeded {
                declared: manifest.record_count(),
                maximum: limits.narrowed().maximum_records(),
            })?;
        Ok(Self {
            manifest,
            records: Vec::with_capacity(capacity),
            limits,
            work: WorthQueryPortablePackageReconstructionWork::default(),
        })
    }

    pub fn push_record(
        mut self,
        canonical_index: u32,
        record: WorthQueryPortablePackageRecord,
    ) -> Result<Self, Denial> {
        let expected_index =
            u32::try_from(self.records.len()).expect("manifest record budget fits within u32");
        if canonical_index != expected_index {
            return Err(Denial::RecordIndexMismatch {
                expected: expected_index,
                observed: canonical_index,
            });
        }
        if expected_index >= self.manifest.record_count() {
            return Err(Denial::RecordCountExceeded {
                declared: self.manifest.record_count(),
            });
        }
        let expected_family = expected_family(&self.manifest, expected_index)
            .expect("validated family counts cover every declared record");
        let observed_family = record.family();
        if observed_family != expected_family {
            return Err(Denial::RecordFamilyMismatch {
                canonical_index,
                expected: expected_family,
                observed: observed_family,
            });
        }
        self.work = self.work.observe_record(&record, self.limits)?;
        self.records.push(record);
        Ok(self)
    }

    pub fn close(self) -> Result<WorthQueryPortablePackageReconstructionCandidate, Denial> {
        let observed =
            u32::try_from(self.records.len()).expect("manifest record budget fits within u32");
        if observed != self.manifest.record_count() {
            return Err(Denial::RecordCountIncomplete {
                declared: self.manifest.record_count(),
                observed,
            });
        }
        Ok(WorthQueryPortablePackageReconstructionCandidate::new(
            self.manifest,
            self.records,
            self.limits,
            self.work,
        ))
    }
}

fn validate_manifest(
    manifest: &WorthQueryPortablePackageManifest,
    limits: WorthQueryPortablePackageReconstructionLimits,
) -> Result<(), Denial> {
    if manifest.version() != WORTH_QUERY_PORTABLE_PACKAGE_MANIFEST_VERSION {
        return Err(Denial::UnsupportedManifestVersion {
            observed: manifest.version(),
            supported: WORTH_QUERY_PORTABLE_PACKAGE_MANIFEST_VERSION,
        });
    }
    if manifest.record_count() > limits.maximum_records() {
        return Err(Denial::RecordBudgetExceeded {
            declared: manifest.record_count(),
            maximum: limits.maximum_records(),
        });
    }
    let maximum_logical_bytes = limits
        .maximum_logical_bytes()
        .min(WORTH_QUERY_PORTABLE_PACKAGE_MAX_LOGICAL_EXPORT_BYTES);
    if manifest.logical_export_bytes() > maximum_logical_bytes {
        return Err(Denial::DeclaredLogicalByteCeilingExceeded {
            declared: manifest.logical_export_bytes(),
            maximum: maximum_logical_bytes,
        });
    }
    if manifest.canonical_source_bytes() > limits.maximum_canonical_work_bytes() {
        return Err(Denial::DeclaredCanonicalWorkBudgetExceeded {
            declared: manifest.canonical_source_bytes(),
            maximum: limits.maximum_canonical_work_bytes(),
        });
    }
    if manifest.canonical_source_bytes() > manifest.logical_export_bytes() {
        return Err(Denial::CanonicalSourceExceedsLogicalExport {
            canonical_source_bytes: manifest.canonical_source_bytes(),
            logical_export_bytes: manifest.logical_export_bytes(),
        });
    }
    let declared_family_total = manifest
        .family_counts()
        .iter()
        .try_fold(0_u32, |total, count| total.checked_add(*count))
        .ok_or(Denial::FamilyCountOverflow)?;
    if declared_family_total != manifest.record_count() {
        return Err(Denial::FamilyCountMismatch {
            declared_family_total,
            declared_record_count: manifest.record_count(),
        });
    }
    Ok(())
}

fn expected_family(
    manifest: &WorthQueryPortablePackageManifest,
    canonical_index: u32,
) -> Option<WorthQueryPortablePackageRecordFamily> {
    let mut family_start = 0_u32;
    for family in WorthQueryPortablePackageRecordFamily::ALL {
        let family_end = family_start.checked_add(manifest.family_count(family))?;
        if canonical_index < family_end {
            return Some(family);
        }
        family_start = family_end;
    }
    None
}
