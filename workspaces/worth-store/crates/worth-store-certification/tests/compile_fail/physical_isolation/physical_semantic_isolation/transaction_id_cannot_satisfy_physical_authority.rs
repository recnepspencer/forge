use worth_relational::facade::transactions::TransactionId;
use worth_store_physical_isolation::PhysicalReadStabilityAuthority;

fn require_physical_authority(_: PhysicalReadStabilityAuthority) {}

fn main() {
    require_physical_authority(TransactionId(7));
}
