use worth_store_physical_isolation::{
    PhysicalReadStabilityAuthority, PhysicalSnapshotCorrelation,
};

fn require_physical_authority(_: PhysicalReadStabilityAuthority) {}

fn main() {
    let correlation: PhysicalSnapshotCorrelation = todo!();
    require_physical_authority(correlation);
}
