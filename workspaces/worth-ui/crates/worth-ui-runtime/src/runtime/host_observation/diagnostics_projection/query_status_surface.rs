use crate::runtime::{
    WorthUiDiagnosticSource, WorthUiExecutionPlanInspection, WorthUiQueryBindingIdentity,
    WorthUiQueryBindingPosture, WorthUiQueryInspectionLinks, WorthUiReloadCheckedStopPosture,
    WorthUiRuntimeDiagnostic,
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
        posture: WorthUiQueryBindingPosture,
        preservation_receipt: Option<crate::runtime::WorthUiQueryBindingPreservationReceipt>,
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
        posture: links.posture().clone(),
        preservation_receipt: links.preservation_receipt(),
    }
}
