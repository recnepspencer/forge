use forge_store_wal::layout_access::WalLayoutAccess;

fn main() {
    let access = WalLayoutAccess::s8();
    access.lower_baseline_lsm_compaction(todo!(), todo!(), todo!());
}
