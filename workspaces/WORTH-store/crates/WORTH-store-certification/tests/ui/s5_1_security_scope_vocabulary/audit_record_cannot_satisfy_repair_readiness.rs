use worth_store_operations::RepairBlastRadiusReadiness;
use worth_store_security::StoreRepairAuditRecordClaim;

fn require_repair_readiness(_: RepairBlastRadiusReadiness) {}

fn main() {
    require_repair_readiness(StoreRepairAuditRecordClaim::raw("audit-record-123"));
}
