use worth_store_layout_indexes::{
    IndexMaintenanceMode, LiveExactMaintenanceWitness,
};

fn main() {
    let _ = LiveExactMaintenanceWitness {
        family: panic!("private fields prevent raw witness construction"),
        exact_coverage: panic!("private fields prevent raw witness construction"),
        maintenance_mode: IndexMaintenanceMode::SynchronousExact,
        publication_authority: panic!("private fields prevent raw witness construction"),
    };
}
