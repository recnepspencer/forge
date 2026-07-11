use forge_store_layout_indexes::layout_rebuild::{
    S8IndexMaintenanceMode, S8IndexPublicationProtocol, S8LiveExactMaintenanceWitness,
};

fn main() {
    let _ = S8LiveExactMaintenanceWitness {
        family: panic!("private fields prevent raw witness construction"),
        exact_coverage: panic!("private fields prevent raw witness construction"),
        maintenance_mode: S8IndexMaintenanceMode::SynchronousExact,
        publication_protocol: S8IndexPublicationProtocol::StableRootSwap,
    };
}
