use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::{DurableFreeSpaceManifestHeader, DurablePhysicalRootManifest};

use super::super::{
    AdmittedPhysicalRecordFormat, AdmittedRecordPlacementPolicy, RecordAllocationFrontier,
};

pub(in crate::physical_runtime::record_serving) struct PlacementPlanningContext<'plan> {
    pub(in crate::physical_runtime::record_serving) media: &'plan QualifiedFilesystemMedia,
    pub(in crate::physical_runtime::record_serving) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime::record_serving) access:
        super::super::AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime::record_serving) current_root:
        &'plan DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) current_free_space:
        &'plan DurableFreeSpaceManifestHeader,
    pub(in crate::physical_runtime::record_serving) frontier: &'plan mut RecordAllocationFrontier,
    pub(in crate::physical_runtime::record_serving) placement: AdmittedRecordPlacementPolicy,
    pub(in crate::physical_runtime::record_serving) frame_ports:
        super::super::residency::frame_ports::RecordFramePorts,
    pub(in crate::physical_runtime::record_serving) source:
        super::super::residency::frame_loading::CanonicalFrameReadSource,
}
