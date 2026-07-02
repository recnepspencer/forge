use forge_store_aspect_native::StoreAspectBoundaryFact;
use forge_store_physical_certification::PhysicalScenarioSchedule;

use crate::NativeStoreAspectFixture;

pub fn s5_boundary_fact(label: &str, segment: u64) -> StoreAspectBoundaryFact {
    NativeStoreAspectFixture::segment_header(label, segment)
        .boundary_fact()
        .clone()
}

pub fn s5_boundary_yieldpoint() -> PhysicalScenarioSchedule {
    PhysicalScenarioSchedule::named_boundary_yieldpoint("root-publication-before-observe")
}
