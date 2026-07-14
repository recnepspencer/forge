use worth_query::facade::runtime::{WorthQueryBatchWriteReceipt, WorthQueryBatchWriteReceiptInspection, WorthQueryIntentReceipt, WorthQueryWriteReceipt};

fn main() {
    fn write_receipt(receipt: &WorthQueryWriteReceipt) {
        let _ = receipt.affected_live_view_ids();
        let _ = receipt.affected_derived_view_ids();
    }

    fn batch_receipt(receipt: &WorthQueryBatchWriteReceipt) {
        let _ = receipt.affected_live_view_ids();
        let _ = receipt.affected_derived_view_ids();
    }

    fn intent_receipt(receipt: &WorthQueryIntentReceipt) {
        let _ = receipt.affected_live_view_ids();
        let _ = receipt.affected_derived_view_ids();
    }

    fn batch_inspection(inspection: &WorthQueryBatchWriteReceiptInspection) {
        let _ = inspection.affected_live_view_ids();
        let _ = inspection.affected_derived_view_ids();
    }
}
