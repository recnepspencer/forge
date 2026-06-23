use super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};

pub fn reject_fabricated_graph_read_receipt_proof(
    _label: &str,
) -> Result<(), WorthGraphReadAccessInventoryError> {
    Err(error(
        WorthGraphReadAccessInventoryErrorKind::FabricatedReceiptProofDenied,
    ))
}

pub fn reject_local_support_row_graph_read_proof(
    _label: &str,
) -> Result<(), WorthGraphReadAccessInventoryError> {
    Err(error(
        WorthGraphReadAccessInventoryErrorKind::LocalSupportRowProofDenied,
    ))
}

const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}
