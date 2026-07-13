use worth_query::facade::runtime::WorthQueryWriteReceipt;

fn main() {}

fn removed_write_receipt_collection_aliases(receipt: &WorthQueryWriteReceipt) {
    let _ = receipt.declared_collection();
    let _ = receipt.target_collection();
}
