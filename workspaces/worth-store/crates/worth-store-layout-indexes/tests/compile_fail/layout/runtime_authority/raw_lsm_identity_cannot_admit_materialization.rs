use worth_store_layout_indexes::access_planning;
use worth_store_wal::{BlobWalRecordIdentity, BlobWalRecordKind};

fn main() {
    let copied_identity = BlobWalRecordIdentity::new(41, BlobWalRecordKind::LsmValue).unwrap();

    let _ =
        access_planning().admit_lsm_replacement_materialization(todo!(), todo!(), copied_identity);
}
