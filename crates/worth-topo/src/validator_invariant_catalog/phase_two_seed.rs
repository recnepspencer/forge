#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthTopologyLegalityCatalogPhaseThreeSeed {
    catalog_digest: String,
    query_registration_catalog_digest: String,
    validator_family_count: usize,
    invariant_family_count: usize,
    supported_family_count: usize,
    unsupported_family_count: usize,
    no_execution_proof_digest: String,
    selected_obligation_count: usize,
    enforcement_receipt_count: usize,
    seed_digest: String,
}

pub(super) struct WorthTopologyLegalityCatalogPhaseThreeSeedInput {
    pub catalog_digest: String,
    pub query_registration_catalog_digest: String,
    pub validator_family_count: usize,
    pub invariant_family_count: usize,
    pub supported_family_count: usize,
    pub unsupported_family_count: usize,
    pub no_execution_proof_digest: String,
}

impl WorthTopologyLegalityCatalogPhaseThreeSeed {
    pub(super) fn from_input(input: WorthTopologyLegalityCatalogPhaseThreeSeedInput) -> Self {
        let selected_obligation_count = 0;
        let enforcement_receipt_count = 0;
        let seed_digest = format!(
            "worth-topo-legality-catalog-phase-three:{}:{}:{}:{}:{}:{}:{}",
            input.catalog_digest,
            input.query_registration_catalog_digest,
            input.validator_family_count,
            input.invariant_family_count,
            input.supported_family_count,
            input.unsupported_family_count,
            input.no_execution_proof_digest
        );
        Self {
            catalog_digest: input.catalog_digest,
            query_registration_catalog_digest: input.query_registration_catalog_digest,
            validator_family_count: input.validator_family_count,
            invariant_family_count: input.invariant_family_count,
            supported_family_count: input.supported_family_count,
            unsupported_family_count: input.unsupported_family_count,
            no_execution_proof_digest: input.no_execution_proof_digest,
            selected_obligation_count,
            enforcement_receipt_count,
            seed_digest,
        }
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn query_registration_catalog_digest(&self) -> &str {
        &self.query_registration_catalog_digest
    }

    pub const fn validator_family_count(&self) -> usize {
        self.validator_family_count
    }

    pub const fn invariant_family_count(&self) -> usize {
        self.invariant_family_count
    }

    pub const fn supported_family_count(&self) -> usize {
        self.supported_family_count
    }

    pub const fn unsupported_family_count(&self) -> usize {
        self.unsupported_family_count
    }

    pub fn no_execution_proof_digest(&self) -> &str {
        &self.no_execution_proof_digest
    }

    pub const fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub const fn enforcement_receipt_count(&self) -> usize {
        self.enforcement_receipt_count
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }
}
