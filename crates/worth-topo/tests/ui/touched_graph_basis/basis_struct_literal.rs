use topology::facade::{
    TopologyGraphLifecyclePosture, TopologyTouchedGraphBasis, TopologyTouchedOperatingWorld,
};

fn main() {
    let _basis = TopologyTouchedGraphBasis {
        entities: Vec::new(),
        relations: Vec::new(),
        relation_kinds: Vec::new(),
        aspects: Vec::new(),
        topology_scopes: Vec::new(),
        lifecycle_posture: TopologyGraphLifecyclePosture::ExistingRelationRetarget,
        operating_world: TopologyTouchedOperatingWorld::mainline(),
        counters: unsafe { std::mem::zeroed() },
        digest: String::new(),
    };
}
