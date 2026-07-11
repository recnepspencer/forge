use forge_store_wal::layout_access::baseline_lsm_counter_observation::BaselineLsmCompactionPublicationReceipt;

fn extract_authority(receipt: BaselineLsmCompactionPublicationReceipt) {
    let BaselineLsmCompactionPublicationReceipt { key, .. } = receipt;
    let _ = key;
}

fn main() {}
