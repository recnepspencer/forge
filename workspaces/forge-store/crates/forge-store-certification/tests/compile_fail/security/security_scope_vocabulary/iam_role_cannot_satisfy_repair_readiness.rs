use forge_store_operations::RepairBlastRadiusReadiness;
use forge_store_security::StoreIamRoleClaim;

fn require_repair_readiness(_: RepairBlastRadiusReadiness) {}

fn main() {
    require_repair_readiness(StoreIamRoleClaim::raw("arn:aws:iam::123:role/store"));
}
