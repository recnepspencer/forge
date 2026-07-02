use worth_ui_inspection::UiInspectionScope;

use crate::declaration::UiDeclaredPostureLaneKind;

use super::UiDeclarationSupportRowSchemaKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiDeclarationSupportRowSchema {
    pub(crate) kind: UiDeclarationSupportRowSchemaKind,
    pub(crate) lane: UiDeclaredPostureLaneKind,
    pub(crate) inspection_scope: UiInspectionScope,
}

pub(crate) const DECLARATION_SUPPORT_ROW_SCHEMA: [UiDeclarationSupportRowSchema; 5] = [
    UiDeclarationSupportRowSchema {
        kind: UiDeclarationSupportRowSchemaKind::QueryBinding,
        lane: UiDeclaredPostureLaneKind::QueryBinding,
        inspection_scope: UiInspectionScope::Rebind,
    },
    UiDeclarationSupportRowSchema {
        kind: UiDeclarationSupportRowSchemaKind::ServiceUsage,
        lane: UiDeclaredPostureLaneKind::ServiceUsage,
        inspection_scope: UiInspectionScope::Mounting,
    },
    UiDeclarationSupportRowSchema {
        kind: UiDeclarationSupportRowSchemaKind::TouchMeaning,
        lane: UiDeclaredPostureLaneKind::TouchMeaning,
        inspection_scope: UiInspectionScope::Mounting,
    },
    UiDeclarationSupportRowSchema {
        kind: UiDeclarationSupportRowSchemaKind::MeasurementPolicy,
        lane: UiDeclaredPostureLaneKind::MeasurementPolicy,
        inspection_scope: UiInspectionScope::Measurement,
    },
    UiDeclarationSupportRowSchema {
        kind: UiDeclarationSupportRowSchemaKind::HostCapability,
        lane: UiDeclaredPostureLaneKind::HostCapability,
        inspection_scope: UiInspectionScope::Mounting,
    },
];
