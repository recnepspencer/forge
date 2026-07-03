use forge_store_io_scheduler::S6IoQosSecurityScopeHandoff;
use forge_store_security::StoreRawSecurityScopeDeclaration;

fn main() {
    let raw: StoreRawSecurityScopeDeclaration = todo!();
    let _ = S6IoQosSecurityScopeHandoff::from_s5_1_readiness(raw);
}
