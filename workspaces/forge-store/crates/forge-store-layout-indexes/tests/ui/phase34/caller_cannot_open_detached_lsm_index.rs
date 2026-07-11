use forge_store_wal::layout_access::WalLayoutAccess;

fn main() {
    let _ = WalLayoutAccess::s8().open_baseline_lsm_index();
}
