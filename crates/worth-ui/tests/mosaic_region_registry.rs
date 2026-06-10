use worth_ui::facade::{
    CapabilityDiagnosticCode, MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind,
    MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionPersistence, MosaicRegionRole,
    MosaicScrollOwnership, MosaicSizingBehavior, SurfacePlacementClass, WorthUi,
};

#[path = "mosaic_region_registry/determinism_cases.rs"]
mod determinism_cases;
#[path = "mosaic_region_registry/mosaic_region_registry_assertions.rs"]
mod mosaic_region_registry_assertions;
#[path = "mosaic_region_registry/mosaic_region_registry_fixtures.rs"]
mod mosaic_region_registry_fixtures;
#[path = "mosaic_region_registry/rejection_cases.rs"]
mod rejection_cases;
#[path = "mosaic_region_registry/snapshot_metadata_cases.rs"]
mod snapshot_metadata_cases;

use mosaic_region_registry_assertions::{
    assert_diagnostic_codes, assert_diagnostic_codes_and_identities,
    assert_exact_diagnostic_topology, assert_registered_mosaic_region_ids, DiagnosticTopology,
};
use mosaic_region_registry_fixtures::{
    complete_mosaic_region_descriptor, mosaic_region_descriptor,
    mosaic_region_descriptor_with_role, mosaic_region_id,
};
