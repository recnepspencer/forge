use crate::{AllocationMapIntegrityReport, ManifestIntegrityCounters};
use forge_store_physical_format::PhysicalRootManifest;

pub(crate) fn allocation_map_report(
    root: &PhysicalRootManifest,
    counters: ManifestIntegrityCounters,
) -> AllocationMapIntegrityReport {
    AllocationMapIntegrityReport::new(
        root.allocation_classes().len() as u32,
        root.free_space().len() as u32,
        counters,
    )
}
