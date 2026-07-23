use crate::runtime::{
    WorthUiExecutionPlanInspection, WorthUiQueryBindingIdentity, WorthUiQueryInspectionLinks,
    WorthUiQuerySettledFactLink, WorthUiRuntimeDiagnostic,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiBindingObservationSurface {
    rows: Vec<WorthUiBindingObservationRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiBindingObservationRow {
    binding_identity: WorthUiQueryBindingIdentity,
    settled_fact_link: WorthUiQuerySettledFactLink,
    preservation_receipt: Option<crate::runtime::WorthUiQueryBindingPreservationReceipt>,
}

impl WorthUiBindingObservationSurface {
    pub(crate) fn from_sources(
        _diagnostics: &[WorthUiRuntimeDiagnostic],
        inspection: Option<&WorthUiExecutionPlanInspection>,
    ) -> Self {
        let mut rows = Vec::new();
        if let Some(inspection) = inspection {
            rows.extend(
                inspection
                    .nodes()
                    .iter()
                    .filter_map(|node| node.query_inspection_links().map(row_from_query_links)),
            );
        }
        Self { rows }
    }

    pub fn rows(&self) -> &[WorthUiBindingObservationRow] {
        &self.rows
    }
}

fn row_from_query_links(links: &WorthUiQueryInspectionLinks) -> WorthUiBindingObservationRow {
    WorthUiBindingObservationRow {
        binding_identity: links.binding_identity().clone(),
        settled_fact_link: links.settled_fact_link().clone(),
        preservation_receipt: links.preservation_receipt(),
    }
}

impl WorthUiBindingObservationRow {
    pub fn binding_identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.binding_identity
    }

    pub fn settled_fact_link(&self) -> &WorthUiQuerySettledFactLink {
        &self.settled_fact_link
    }

    pub fn preservation_receipt(
        &self,
    ) -> Option<crate::runtime::WorthUiQueryBindingPreservationReceipt> {
        self.preservation_receipt
    }
}
