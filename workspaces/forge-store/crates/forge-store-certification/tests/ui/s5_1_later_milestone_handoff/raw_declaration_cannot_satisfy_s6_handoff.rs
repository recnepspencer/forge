use forge_store_io_scheduler::SchedulerSecurityScopeEvidence;
use forge_store_security::StoreRawSecurityScopeDeclaration;

fn main() {
    let raw: StoreRawSecurityScopeDeclaration = todo!();
    let _ = SchedulerSecurityScopeEvidence::from_s5_1_readiness(raw);
}
