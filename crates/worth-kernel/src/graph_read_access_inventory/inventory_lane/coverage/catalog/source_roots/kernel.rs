use super::WorthGraphReadAccessCoveredSource;
use crate::graph_read_access_inventory::inventory_lane::{
    WorthGraphReadAccessCostPosture, WorthGraphReadAccessOwner,
};

pub(super) const KERNEL_GRAPH_READ_ADOPTION: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::deletion_target(
        "crates/worth-kernel/src/query_adoption/graph_read_access",
        WorthGraphReadAccessOwner::WorthKernel,
        "deleted graph-read adoption scaffolding",
        WorthGraphReadAccessCostPosture::FabricatedReceiptOrSupportRow,
    );

pub(super) const KERNEL_WORKLOAD_COMPOSITION: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::declaration_candidate(
        "crates/worth-kernel/src/workload_composition",
        WorthGraphReadAccessOwner::WorthKernel,
        "boolean workload composition evidence consumers",
        WorthGraphReadAccessCostPosture::BoundedTouchedRegion,
    );

pub(super) const KERNEL_BINDING_ROOT: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::declaration_candidate(
        "crates/worth-kernel/src/binding",
        WorthGraphReadAccessOwner::WorthKernel,
        "binding topology replacement neighborhood consumers",
        WorthGraphReadAccessCostPosture::PerResultNeighborLookup,
    );
