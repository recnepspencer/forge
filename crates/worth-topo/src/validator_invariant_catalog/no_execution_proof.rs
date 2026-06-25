#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyLegalityCatalogNoExecutionProof {
    selected_obligation_digests: Vec<String>,
    enforcement_receipt_digests: Vec<String>,
    proof_digest: String,
}

impl WorthTopologyLegalityCatalogNoExecutionProof {
    pub(in crate::validator_invariant_catalog) fn phase_two_catalog_only() -> Self {
        Self {
            selected_obligation_digests: Vec::new(),
            enforcement_receipt_digests: Vec::new(),
            proof_digest: "worth-topo-legality-catalog-no-execution-proof-v1:selected=0:receipts=0"
                .to_string(),
        }
    }

    pub fn selected_obligation_digests(&self) -> &[String] {
        &self.selected_obligation_digests
    }

    pub fn enforcement_receipt_digests(&self) -> &[String] {
        &self.enforcement_receipt_digests
    }

    pub fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_digests.len()
    }

    pub fn enforcement_receipt_count(&self) -> usize {
        self.enforcement_receipt_digests.len()
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }

    pub const fn claims_selected_obligations(&self) -> bool {
        false
    }

    pub const fn claims_enforcement_receipts(&self) -> bool {
        false
    }
}
