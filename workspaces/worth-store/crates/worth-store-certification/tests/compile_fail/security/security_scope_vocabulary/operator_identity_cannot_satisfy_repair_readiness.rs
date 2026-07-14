use worth_store_operations::RepairBlastRadiusReadiness;
use worth_store_security::StoreOperatorIdentityClaim;

fn require_repair_readiness(_: RepairBlastRadiusReadiness) {}

fn main() {
    require_repair_readiness(StoreOperatorIdentityClaim::raw("operator-123"));
}
