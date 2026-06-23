use super::WorthGraphReadAccessCoveredSource;
use crate::graph_read_access_inventory::inventory_lane::{
    WorthGraphReadAccessCostPosture, WorthGraphReadAccessOwner,
};

pub(super) const SPATIAL_EVIDENCE_LEDGER: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::declaration_candidate(
        "crates/worth-spatial/src/workload_platform/evidence_ledger",
        WorthGraphReadAccessOwner::WorthSpatial,
        "SpatialEvidenceLookupProduct",
        WorthGraphReadAccessCostPosture::BoundedTouchedRegion,
    );

pub(super) const SPATIAL_BOOLEAN_LOOP_RECONSTRUCTION: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::access_capability_gap(
        "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction",
        WorthGraphReadAccessOwner::WorthSpatial,
        "PlanarBooleanFragmentContinuationIndex",
        WorthGraphReadAccessCostPosture::FrontierOrVisitedSet,
    );

pub(super) const SPATIAL_BOOLEAN_EVENTS: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::access_capability_gap(
        "crates/worth-spatial/src/workload_platform/planar_boolean_events",
        WorthGraphReadAccessOwner::WorthSpatial,
        "planar boolean event neighborhood preparation",
        WorthGraphReadAccessCostPosture::BroadScan,
    );
