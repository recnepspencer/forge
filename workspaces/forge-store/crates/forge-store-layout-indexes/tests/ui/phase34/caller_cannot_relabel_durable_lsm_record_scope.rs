use forge_store_security::{StoreKeyScope, StoreTenantScope};
use forge_store_wal::layout_access::WalLayoutAccess;

fn main() {
    let access = WalLayoutAccess::s8();
    let _ = access.admit_baseline_lsm_record(
        todo!(),
        todo!(),
        StoreTenantScope::TenantPhysicalBoundary,
        StoreKeyScope::WalCheckpointEnvelope,
        *b"lsm-key1",
    );
}
