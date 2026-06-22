use worth_spatial::facade::dirty_planar_clean_fail::{
    DirtyPlanarCleanFailCounters, DirtyPlanarCleanFailReceipt,
};

fn main() {
    let _ = DirtyPlanarCleanFailReceipt {
        clean_fail_digest: String::new(),
        workload_identity: String::new(),
        topology_clean_fail_identity: String::new(),
        clean_fail_boundary_identity: String::new(),
        dirty_case: worth_spatial::facade::dirty_planar_clean_fail::DirtyPlanarCleanFailCase::SelfIntersectingLoop,
        counters: DirtyPlanarCleanFailCounters {
            topology_clean_fail_receipts: 0,
            clean_fail_boundary_receipts: 0,
            recovery_receipts: 0,
            transform_posture_receipts: 0,
            diagnostic_receipts: 0,
            user_outcome_receipts: 0,
        },
    };
}
