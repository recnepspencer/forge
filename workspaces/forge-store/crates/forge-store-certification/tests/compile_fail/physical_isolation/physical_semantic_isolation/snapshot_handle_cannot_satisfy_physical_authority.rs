use forge_relational::facade::snapshots::SnapshotHandle;
use forge_store_physical_isolation::PhysicalReadStabilityAuthority;

fn require_physical_authority(_: PhysicalReadStabilityAuthority) {}

fn main() {
    require_physical_authority(SnapshotHandle::new(1, 2));
}
