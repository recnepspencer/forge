use forge_query::facade::{
    ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection, ForgeQueryIntentReceipt,
    ForgeQueryWriteReceipt,
};

fn main() {
    fn write_receipt(receipt: &ForgeQueryWriteReceipt) {
        let _ = receipt.affected_live_view_ids();
        let _ = receipt.affected_derived_view_ids();
    }

    fn batch_receipt(receipt: &ForgeQueryBatchWriteReceipt) {
        let _ = receipt.affected_live_view_ids();
        let _ = receipt.affected_derived_view_ids();
    }

    fn intent_receipt(receipt: &ForgeQueryIntentReceipt) {
        let _ = receipt.affected_live_view_ids();
        let _ = receipt.affected_derived_view_ids();
    }

    fn batch_inspection(inspection: &ForgeQueryBatchWriteReceiptInspection) {
        let _ = inspection.affected_live_view_ids();
        let _ = inspection.affected_derived_view_ids();
    }
}
