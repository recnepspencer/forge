use worth_ui::facade::{
    app::WorthUi,
    diagnostics::CapabilityDiagnosticCode,
    registry::{
        MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture,
        MosaicRegionKindDescriptor, MosaicRegionPersistence, MosaicRegionRole,
        MosaicScrollOwnership, MosaicSizingBehavior, SurfacePlacementClass,
    },
};

#[path = "mosaic_region_registry/mosaic_region_registry_assertions.rs"]
mod mosaic_region_registry_assertions;
#[path = "mosaic_region_registry/mosaic_region_registry_fixtures.rs"]
mod mosaic_region_registry_fixtures;
#[path = "mosaic_region_registry/determinism_cases.rs"]
mod region_determinism_cases;
#[path = "mosaic_region_registry/rejection_cases.rs"]
mod region_rejection_cases;
#[path = "mosaic_region_registry/snapshot_metadata_cases.rs"]
mod region_snapshot_metadata_cases;

#[path = "mosaic_placement_registry/mosaic_placement_registry_assertions.rs"]
mod mosaic_placement_registry_assertions;
#[path = "mosaic_placement_registry/mosaic_placement_registry_fixtures.rs"]
mod mosaic_placement_registry_fixtures;
#[path = "mosaic_placement_registry/determinism_cases.rs"]
mod placement_determinism_cases;
#[path = "mosaic_placement_registry/rejection_cases.rs"]
mod placement_rejection_cases;
#[path = "mosaic_placement_registry/snapshot_metadata_cases.rs"]
mod placement_snapshot_metadata_cases;

#[path = "mosaic_registry/sizing_assertions.rs"]
mod sizing_assertions;
#[path = "mosaic_registry/sizing_determinism_cases.rs"]
mod sizing_determinism_cases;
#[path = "mosaic_registry/sizing_fixtures.rs"]
mod sizing_fixtures;
#[path = "mosaic_registry/sizing_rejection_cases.rs"]
mod sizing_rejection_cases;

#[path = "mosaic_registry/state_assertions.rs"]
mod state_assertions;
#[path = "mosaic_registry/state_determinism_cases.rs"]
mod state_determinism_cases;
#[path = "mosaic_registry/state_fixtures.rs"]
mod state_fixtures;
#[path = "mosaic_registry/state_rejection_cases.rs"]
mod state_rejection_cases;

use mosaic_region_registry_assertions::{
    assert_diagnostic_codes, assert_diagnostic_codes_and_identities,
    assert_exact_diagnostic_topology, assert_registered_mosaic_region_ids, DiagnosticTopology,
};
use mosaic_region_registry_fixtures::{
    complete_mosaic_region_descriptor, mosaic_region_descriptor,
    mosaic_region_descriptor_with_role, mosaic_region_id,
};
