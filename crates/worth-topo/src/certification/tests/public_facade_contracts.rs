mod compile_fail_contracts {
    include!("../public_facade_contracts/compile_fail_contracts.rs");
}

mod public_api {
    use crate as topology;

    include!("../public_facade_contracts/contracts/public_api.rs");
}

mod topology_workload_seeds_lib {
    use crate::facade::{
        TopologySeed, TopologySeedCleanFailClass, TopologySeedCleanFailReasonCode,
        TopologySeedCleanFailStage, TopologySeedCounters, TopologySeedKind,
        TopologySeedTopologyPosture,
    };

    include!("../public_facade_contracts/contracts/topology_workload_seeds/invalid_topology_clean_fail.rs");
    include!("../public_facade_contracts/contracts/topology_workload_seeds/real_topology_truth.rs");
}
