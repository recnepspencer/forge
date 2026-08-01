use worth_query::facade::{foundation, runtime};

pub(super) struct WorthUiScalarProjectionSchema;

impl runtime::WorthQueryRuntimeSchemaAdapter for WorthUiScalarProjectionSchema {
    fn admit_live_view(
        &self,
        name: &str,
        request: &foundation::DeclarativeLiveQueryRequest,
        _schema_view: &runtime::QuerySchemaView,
    ) -> Result<
        runtime::LiveViewDeclarationAdmissionBoundaryReceipt,
        foundation::WorthQueryWorkspaceError,
    > {
        let receipt = self.build_live_view_declaration_admission_receipt(name, request);
        Ok(self.build_live_view_declaration_boundary_receipt(name, request, receipt))
    }
}
