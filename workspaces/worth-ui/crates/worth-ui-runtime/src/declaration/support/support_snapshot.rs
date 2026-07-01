use worth_ui_inspection::{
    UiInspectionMilestoneExpectation, UiInspectionScope, UiInspectionScopeSupportRow,
    UiInspectionSupportReason,
};

use super::{
    support_row_schema::DECLARATION_SUPPORT_ROW_SCHEMA, UiDeclarationSupportMilestoneExpectation,
    UiDeclarationSupportRow, UiDeclarationSupportRowSchemaKind, UiDeclarationUnsupportedPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDeclarationSupportSnapshot {
    rows: [UiDeclarationSupportRow; 5],
}

impl UiDeclarationSupportSnapshot {
    pub(crate) const fn new(rows: [UiDeclarationSupportRow; 5]) -> Self {
        Self { rows }
    }

    pub fn rows(&self) -> &[UiDeclarationSupportRow] {
        &self.rows
    }

    pub fn row(&self, kind: UiDeclarationSupportRowSchemaKind) -> Option<&UiDeclarationSupportRow> {
        self.rows.iter().find(|row| row.schema_kind() == kind)
    }

    pub fn inspection_rows(&self, scope: UiInspectionScope) -> Box<[UiInspectionScopeSupportRow]> {
        self.rows
            .iter()
            .filter_map(|row| inspection_row_for_support_row(row, scope))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

fn inspection_row_for_support_row(
    row: &UiDeclarationSupportRow,
    scope: UiInspectionScope,
) -> Option<UiInspectionScopeSupportRow> {
    let schema = DECLARATION_SUPPORT_ROW_SCHEMA
        .iter()
        .find(|schema| schema.kind == row.schema_kind())
        .expect("support snapshot rows must come from declared schema");
    if schema.inspection_scope != scope {
        return None;
    }

    Some(match row.unsupported_posture() {
        Some(UiDeclarationUnsupportedPosture::ArchitecturallyOwnedButNotYetAdmitted {
            expected_in,
        }) => UiInspectionScopeSupportRow::unsupported(
            row.schema_kind().as_support_subsystem(),
            scope,
            UiInspectionSupportReason::BelongsArchitecturallyNotYetAdmitted,
            Some(map_milestone_expectation(expected_in)),
        ),
        None => {
            UiInspectionScopeSupportRow::supported(row.schema_kind().as_support_subsystem(), scope)
        }
    })
}

const fn map_milestone_expectation(
    expected_in: UiDeclarationSupportMilestoneExpectation,
) -> UiInspectionMilestoneExpectation {
    match expected_in {
        UiDeclarationSupportMilestoneExpectation::Milestone32 => {
            UiInspectionMilestoneExpectation::Milestone32
        }
    }
}
