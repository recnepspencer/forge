use super::WorthGraphReadAccessCoveredSource;
use crate::graph_read_access_inventory::inventory_lane::{
    WorthGraphReadAccessCostPosture, WorthGraphReadAccessOwner,
};

pub(crate) const TOPOLOGY_READ_DOMAIN: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::declaration_candidate(
        "crates/worth-topo/src/projection/read_views/domain",
        WorthGraphReadAccessOwner::WorthTopo,
        "TopologyReadLedger::read_views",
        WorthGraphReadAccessCostPosture::PerResultNeighborLookup,
    );

pub(crate) const TOPOLOGY_READ_EXECUTION: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::declaration_candidate(
        "crates/worth-topo/src/projection/runtime_boundary/read_execution",
        WorthGraphReadAccessOwner::WorthTopo,
        "execute_shared_neighborhood_read and execute_local_rewire_read",
        WorthGraphReadAccessCostPosture::BoundedTouchedRegion,
    );

pub(crate) const TOPOLOGY_READ_PROOF_SUPPORT: WorthGraphReadAccessCoveredSource =
    WorthGraphReadAccessCoveredSource::certification_only(
        "crates/worth-topo/src/projection/read_views/domain/read_proof",
        WorthGraphReadAccessOwner::WorthTopo,
        "TopologyReadGraphAccessProof and TopologyNoNPlusOneContract",
        WorthGraphReadAccessCostPosture::FabricatedReceiptOrSupportRow,
    );
