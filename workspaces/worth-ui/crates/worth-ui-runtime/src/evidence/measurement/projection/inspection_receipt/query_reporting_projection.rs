use super::super::UiProjectionFactReceipt;

/// Query terminal projections exposed only for inspection/reporting.
impl UiProjectionFactReceipt {
    pub fn query_basis_digest_for_diagnostics(&self) -> &str {
        self.query_authority()
            .authority()
            .contract()
            .basis_digest()
            .unwrap_or_default()
    }

    pub fn projection_contract_digest_for_diagnostics(&self) -> &str {
        self.query_authority()
            .authority()
            .contract()
            .contract_digest()
    }

    pub fn projection_consumption_declaration_digest_for_diagnostics(&self) -> &str {
        self.query_authority()
            .authority()
            .receipt()
            .declaration_digest()
    }

    pub fn projection_consumption_receipt_digest_for_diagnostics(&self) -> &str {
        self.query_authority()
            .authority()
            .receipt()
            .receipt_digest()
    }

    pub fn projection_fact_set_digest_for_diagnostics(&self) -> &str {
        self.query_authority()
            .authority()
            .receipt()
            .fact_set_digest()
    }

    pub fn projection_source_identity_for_diagnostics(&self) -> &str {
        self.query_authority()
            .authority()
            .source_identity()
            .as_str()
    }
}
