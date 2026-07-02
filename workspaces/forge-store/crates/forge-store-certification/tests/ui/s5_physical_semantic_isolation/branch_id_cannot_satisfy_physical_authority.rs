use forge_relational::facade::history::BranchId;
use forge_store_physical_isolation::PhysicalReadStabilityAuthority;

fn require_physical_authority(_: PhysicalReadStabilityAuthority) {}

fn main() {
    require_physical_authority(BranchId("main".to_string()));
}
