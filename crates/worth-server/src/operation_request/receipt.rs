use crate::{
    request_context::DiagnosticRichnessProfile, WorthServerSurfaceFamily, WorthServerTransportClass,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationRequestReceipt {
    surface_family: WorthServerSurfaceFamily,
    transport_class: WorthServerTransportClass,
    diagnostics_profile: DiagnosticRichnessProfile,
    request_context_digest: String,
    source_contract_digest: Option<String>,
    canonical_digest: String,
}

impl WorthServerOperationRequestReceipt {
    pub(crate) fn new(
        surface_family: WorthServerSurfaceFamily,
        transport_class: WorthServerTransportClass,
        diagnostics_profile: DiagnosticRichnessProfile,
        request_context_digest: String,
        source_contract_digest: Option<String>,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-operation-request-receipt-v1|surface={:?}|transport={:?}|diagnostics={:?}|request_context={request_context_digest}|source_contract={}",
            surface_family,
            transport_class,
            diagnostics_profile,
            source_contract_digest.as_deref().unwrap_or("none"),
        );
        Self {
            surface_family,
            transport_class,
            diagnostics_profile,
            request_context_digest,
            source_contract_digest,
            canonical_digest,
        }
    }

    pub fn diagnostics_profile(&self) -> DiagnosticRichnessProfile {
        self.diagnostics_profile
    }

    pub fn source_contract_digest(&self) -> Option<&str> {
        self.source_contract_digest.as_deref()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
