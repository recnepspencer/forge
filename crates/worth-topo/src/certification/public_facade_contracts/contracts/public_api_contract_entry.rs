include!("public_api.rs");

mod public_api_topology_workload_seeds {
    use topology::facade::{
        TopologySeed, TopologySeedCleanFailClass, TopologySeedCleanFailReasonCode,
        TopologySeedCleanFailStage, TopologySeedCounters, TopologySeedEntityIdentities,
        TopologySeedKind, TopologySeedNeighborhoodReceipt, TopologySeedQueryReceipts,
        TopologySeedTopologyPosture, TopologySeedValidationReceipt,
    };

    include!("topology_workload_seeds/invalid_topology_clean_fail.rs");
    include!("topology_workload_seeds/real_topology_truth.rs");
}
