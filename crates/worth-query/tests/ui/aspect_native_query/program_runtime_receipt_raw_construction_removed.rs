use worth_query::facade::runtime::{WorthQueryInstalledOperation, WorthQueryRunReceipt};

fn main() {
    let operation = WorthQueryInstalledOperation {
        program_id: "program.raw".to_string(),
        operation_id: "operation.raw".to_string(),
    };
    let _receipt = WorthQueryRunReceipt {
        run_id: "run.raw".to_string(),
        operation,
        outputs: Vec::new(),
        write_receipts: Vec::new(),
        patch_batches: Vec::new(),
    };
}
