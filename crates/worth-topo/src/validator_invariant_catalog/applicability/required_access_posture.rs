#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorthTopologyRequiredAccessPosture {
    posture_digest: String,
    receipt_context_required: bool,
}

impl WorthTopologyRequiredAccessPosture {
    pub(in crate::validator_invariant_catalog) fn milestone_eight_receipt_backed(
        posture_digest: impl Into<String>,
    ) -> Self {
        Self {
            posture_digest: posture_digest.into(),
            receipt_context_required: true,
        }
    }

    pub fn posture_digest(&self) -> &str {
        &self.posture_digest
    }

    pub const fn receipt_context_required(&self) -> bool {
        self.receipt_context_required
    }
}
