use super::WorthGraphReadAccessCoveredSource;
use crate::graph_read_access_inventory::inventory_lane::{
    WorthGraphReadAccessCostPosture, WorthGraphReadAccessOwner,
};

pub(crate) const TOPOLOGY_READ_TEST_SUPPORT: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::certification_only(
        "crates/worth-topo/src/certification/projection_closeout/tests/topology_reads",
        WorthGraphReadAccessOwner::WorthTopo,
        "topology read certification fixtures",
        WorthGraphReadAccessCostPosture::FabricatedReceiptOrSupportRow,
    );

pub(crate) const SPATIAL_LOOP_RECONSTRUCTION_TEST_SUPPORT: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::certification_only(
        "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction/test_support",
        WorthGraphReadAccessOwner::WorthSpatial,
        "planar boolean continuation index test support",
        WorthGraphReadAccessCostPosture::FrontierOrVisitedSet,
    );

pub(crate) const KERNEL_BINDING_TEST_SUPPORT: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::certification_only(
        "crates/worth-kernel/src/binding/tests",
        WorthGraphReadAccessOwner::WorthKernel,
        "binding replacement neighborhood test support",
        WorthGraphReadAccessCostPosture::FabricatedReceiptOrSupportRow,
    );
