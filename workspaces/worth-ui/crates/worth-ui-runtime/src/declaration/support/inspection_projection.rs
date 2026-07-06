use worth_ui_inspection::{
    UiInspectionScope, UiInspectionScopeSupportRow, UiInspectionSupportReport,
};

use crate::declaration::UiDeclarationArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiDeclarationInspectionSupportProjection {
    measurement: Box<[UiInspectionScopeSupportRow]>,
    mounting: Box<[UiInspectionScopeSupportRow]>,
    rebind: Box<[UiInspectionScopeSupportRow]>,
}

impl UiDeclarationInspectionSupportProjection {
    fn new(
        measurement: Box<[UiInspectionScopeSupportRow]>,
        mounting: Box<[UiInspectionScopeSupportRow]>,
        rebind: Box<[UiInspectionScopeSupportRow]>,
    ) -> Self {
        Self {
            measurement,
            mounting,
            rebind,
        }
    }

    pub(crate) fn support_report(
        &self,
        scope: UiInspectionScope,
    ) -> Option<UiInspectionSupportReport> {
        let rows = match scope {
            UiInspectionScope::Graph => return None,
            UiInspectionScope::Measurement => &self.measurement,
            UiInspectionScope::Planning => return None,
            UiInspectionScope::Mounting => &self.mounting,
            UiInspectionScope::Rebind => &self.rebind,
            _ => return None,
        };
        (!rows.is_empty()).then(|| UiInspectionSupportReport::from_scope_rows(scope, rows))
    }
}

pub(crate) fn derive_declaration_inspection_support_projection(
    artifacts: &[UiDeclarationArtifact],
) -> UiDeclarationInspectionSupportProjection {
    let measurement = inspection_rows_for_scope(artifacts, UiInspectionScope::Measurement);
    let mounting = inspection_rows_for_scope(artifacts, UiInspectionScope::Mounting);
    let rebind = inspection_rows_for_scope(artifacts, UiInspectionScope::Rebind);

    UiDeclarationInspectionSupportProjection::new(measurement, mounting, rebind)
}

fn inspection_rows_for_scope(
    artifacts: &[UiDeclarationArtifact],
    scope: UiInspectionScope,
) -> Box<[UiInspectionScopeSupportRow]> {
    artifacts
        .iter()
        .filter_map(|artifact| artifact.support_snapshot().ok())
        .flat_map(|snapshot| snapshot.inspection_rows(scope).into_vec())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}
