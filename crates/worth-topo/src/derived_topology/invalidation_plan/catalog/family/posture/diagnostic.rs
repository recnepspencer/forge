use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DerivedTopologyDiagnosticPosture {
    ProductFamilyWitnessRequired,
    ExecutionReceiptWitnessRequired,
}

impl DerivedTopologyDiagnosticPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProductFamilyWitnessRequired => "product_family_witness_required",
            Self::ExecutionReceiptWitnessRequired => "execution_receipt_witness_required",
        }
    }
}
