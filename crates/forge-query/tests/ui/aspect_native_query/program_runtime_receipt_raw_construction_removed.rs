use forge_query::facade::{ForgeQueryInstalledOperation, ForgeQueryRunReceipt};

fn main() {
    let operation = ForgeQueryInstalledOperation {
        program_id: "program.raw".to_string(),
        operation_id: "operation.raw".to_string(),
    };
    let _receipt = ForgeQueryRunReceipt {
        run_id: "run.raw".to_string(),
        operation,
        outputs: Vec::new(),
        write_receipts: Vec::new(),
        patch_batches: Vec::new(),
    };
}
