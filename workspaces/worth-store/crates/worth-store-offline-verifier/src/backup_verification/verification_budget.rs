use worth_store_physical_format::BackupBundleManifestReadLimits;

use crate::OfflineInspectionBudget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackupVerificationBudget {
    inspection: OfflineInspectionBudget,
    manifest: BackupBundleManifestReadLimits,
}

impl BackupVerificationBudget {
    pub const fn new(
        inspection: OfflineInspectionBudget,
        manifest: BackupBundleManifestReadLimits,
    ) -> Option<Self> {
        if manifest.read_buffer_bytes() > inspection.max_buffer_bytes()
            || manifest.maximum_encoded_bytes() > inspection.max_total_read_bytes()
            || manifest.maximum_owned_allocation_bytes()
                > inspection.maximum_owned_allocation_bytes()
        {
            None
        } else {
            Some(Self {
                inspection,
                manifest,
            })
        }
    }

    pub const fn from_inspection(inspection: OfflineInspectionBudget) -> Self {
        let maximum_encoded_bytes = if inspection.max_total_read_bytes() < 64 * 1024 * 1024 {
            inspection.max_total_read_bytes()
        } else {
            64 * 1024 * 1024
        };
        let canonical_manifest = BackupBundleManifestReadLimits::canonical();
        let encoded_as_buffer = if maximum_encoded_bytes > usize::MAX as u64 {
            usize::MAX
        } else {
            maximum_encoded_bytes as usize
        };
        let read_buffer_bytes = if inspection.max_buffer_bytes() < encoded_as_buffer {
            inspection.max_buffer_bytes()
        } else {
            encoded_as_buffer
        };
        let read_buffer_bytes = if read_buffer_bytes < canonical_manifest.read_buffer_bytes() {
            read_buffer_bytes
        } else {
            canonical_manifest.read_buffer_bytes()
        };
        let maximum_artifacts = if inspection.acquisition().max_files() < 262_144 {
            inspection.acquisition().max_files()
        } else {
            262_144
        };
        let maximum_owned_allocation_bytes =
            if inspection.maximum_owned_allocation_bytes() < 128 * 1024 * 1024 {
                inspection.maximum_owned_allocation_bytes()
            } else {
                128 * 1024 * 1024
            };
        Self {
            inspection,
            manifest: BackupBundleManifestReadLimits::new(
                maximum_encoded_bytes,
                maximum_artifacts,
                read_buffer_bytes,
                maximum_owned_allocation_bytes,
            )
            .expect("positive inspection budget produces positive manifest limits"),
        }
    }

    pub const fn inspection(self) -> OfflineInspectionBudget {
        self.inspection
    }

    pub const fn manifest(self) -> BackupBundleManifestReadLimits {
        self.manifest
    }
}

impl From<OfflineInspectionBudget> for BackupVerificationBudget {
    fn from(inspection: OfflineInspectionBudget) -> Self {
        Self::from_inspection(inspection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_manifest_limits_clamp_buffer_and_cardinality_without_panicking() {
        let acquisition = crate::OfflineMediaAcquisitionBudget::bounded(7, 2, 1024, 4)
            .expect("acquisition budget");
        let inspection = OfflineInspectionBudget::bounded(2, 3)
            .expect("inspection budget")
            .with_maximum_owned_allocation_bytes(11)
            .expect("owned allocation budget")
            .with_acquisition_budget(acquisition);
        let budget = BackupVerificationBudget::from_inspection(inspection);
        assert_eq!(budget.manifest().maximum_encoded_bytes(), 3);
        assert_eq!(budget.manifest().read_buffer_bytes(), 2);
        assert_eq!(budget.manifest().maximum_artifacts(), 7);
        assert_eq!(budget.manifest().maximum_owned_allocation_bytes(), 11);
    }

    #[test]
    fn manifest_memory_is_independent_from_media_read_budget() {
        let inspection = OfflineInspectionBudget::bounded(4, 1)
            .expect("inspection budget")
            .with_maximum_owned_allocation_bytes(64)
            .expect("owned allocation budget");
        let budget = BackupVerificationBudget::from_inspection(inspection);

        assert_eq!(budget.manifest().maximum_encoded_bytes(), 1);
        assert_eq!(budget.manifest().maximum_owned_allocation_bytes(), 64);
    }
}
