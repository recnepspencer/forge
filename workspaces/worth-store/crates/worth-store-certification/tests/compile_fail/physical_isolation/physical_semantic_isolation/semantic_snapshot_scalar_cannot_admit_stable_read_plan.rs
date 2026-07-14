use worth_relational::facade::snapshots::SnapshotId;
use worth_store_physical_isolation::{PhysicalEpoch, StableReadPlan};

fn main() {
    let semantic_snapshot = SnapshotId(7);
    let _plan = StableReadPlan::new(PhysicalEpoch(semantic_snapshot.0));
}
