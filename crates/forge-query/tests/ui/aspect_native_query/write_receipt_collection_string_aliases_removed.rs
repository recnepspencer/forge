use forge_query::facade::ForgeQueryWriteReceipt;

fn main() {}

fn removed_write_receipt_collection_aliases(receipt: &ForgeQueryWriteReceipt) {
    let _ = receipt.declared_collection();
    let _ = receipt.target_collection();
}
