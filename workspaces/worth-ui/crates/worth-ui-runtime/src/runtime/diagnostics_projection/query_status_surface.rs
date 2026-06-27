use crate::runtime::{
    WorthUiDiagnosticSource, WorthUiExecutionPlanInspection, WorthUiQueryBindingIdentity,
    WorthUiQueryInspectionLinks, WorthUiReloadCheckedStopPosture, WorthUiRuntimeDiagnostic,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryStatusSurface {
    rows: Vec<WorthUiQueryStatusRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiQueryStatusRow {
    CheckedStop {
        posture: WorthUiReloadCheckedStopPosture,
        evidence_digest: u64,
    },
    InspectionLinks {
        binding_identity: WorthUiQueryBindingIdentity,
        inspection_digest: String,
        projection_consumption_digest: String,
        async_result_state_digest: String,
        recovery_digest: String,
        preservation_receipt: Option<String>,
    },
}

impl WorthUiQueryStatusSurface {
    pub(crate) fn from_sources(
        diagnostics: &[WorthUiRuntimeDiagnostic],
        inspection: Option<&WorthUiExecutionPlanInspection>,
    ) -> Self {
        let mut rows = Vec::new();
        rows.extend(diagnostics.iter().filter_map(row_from_diagnostic));
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

    pub fn rows(&self) -> &[WorthUiQueryStatusRow] {
        &self.rows
    }
}

fn row_from_diagnostic(row: &WorthUiRuntimeDiagnostic) -> Option<WorthUiQueryStatusRow> {
    match row.source() {
        WorthUiDiagnosticSource::QueryStop {
            checked_stop_posture,
            evidence_digest,
        } => Some(WorthUiQueryStatusRow::CheckedStop {
            posture: checked_stop_posture,
            evidence_digest,
        }),
        _ => None,
    }
}

fn row_from_query_links(links: &WorthUiQueryInspectionLinks) -> WorthUiQueryStatusRow {
    WorthUiQueryStatusRow::InspectionLinks {
        binding_identity: links.binding_identity().clone(),
        inspection_digest: links.inspection_digest().to_owned(),
        projection_consumption_digest: links.projection_consumption_digest().to_owned(),
        async_result_state_digest: links.async_result_state_digest().to_owned(),
        recovery_digest: links.recovery_digest().to_owned(),
        preservation_receipt: links.preservation_receipt().map(ToOwned::to_owned),
    }
}
