use crate::runtime::host_observation::diagnostics::WorthUiDiagnosticRichnessTier;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiDiagnosticMaterialization {
    tier: WorthUiDiagnosticRichnessTier,
    rows_emitted: bool,
    phase_references_emitted: bool,
    query_links_emitted: bool,
    support_sections_emitted: bool,
}

impl WorthUiDiagnosticMaterialization {
    pub(crate) fn from_tier(tier: WorthUiDiagnosticRichnessTier) -> Self {
        Self {
            tier,
            rows_emitted: tier.emits_rows(),
            phase_references_emitted: tier.emits_phase_references(),
            query_links_emitted: tier.emits_query_links(),
            support_sections_emitted: tier.emits_support_sections(),
        }
    }

    pub fn tier(self) -> WorthUiDiagnosticRichnessTier {
        self.tier
    }

    pub fn rows_emitted(self) -> bool {
        self.rows_emitted
    }

    pub fn phase_references_emitted(self) -> bool {
        self.phase_references_emitted
    }

    pub fn query_links_emitted(self) -> bool {
        self.query_links_emitted
    }

    pub fn support_sections_emitted(self) -> bool {
        self.support_sections_emitted
    }
}
