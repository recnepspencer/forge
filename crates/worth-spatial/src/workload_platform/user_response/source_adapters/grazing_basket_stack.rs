use crate::workload_platform::grazing_basket_stack::{
    GrazingBasketStackDenial, GrazingBasketStackReceipt,
};
use crate::workload_platform::user_response::{
    source::WorthUserResponseSourceKind, WorthUserResponseSource,
};

impl WorthUserResponseSource {
    pub fn from_grazing_basket_stack(receipt: &GrazingBasketStackReceipt) -> Self {
        Self {
            kind: WorthUserResponseSourceKind::Admitted {
                message: format!(
                    "Grazing basket stack preserved {} layer-local open topology receipts.",
                    receipt.counters().total_layers()
                ),
                evidence_digest: receipt.stack_identity().to_string(),
                source_identity: receipt.stack_identity().to_string(),
            },
        }
    }

    pub fn from_grazing_basket_stack_denial(denial: &GrazingBasketStackDenial) -> Self {
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: denial.kind().cause_kind(),
                message: denial.human_reason().to_string(),
                evidence_digest: denial.evidence_digest().to_string(),
                source_identity: denial.evidence_digest().to_string(),
            },
        }
    }
}
