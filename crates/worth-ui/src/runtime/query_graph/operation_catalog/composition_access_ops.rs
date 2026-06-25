use forge_query::facade::runtime::ForgeQueryGraphObligationSupportStatus;

use crate::runtime::WorthUiQueryGraphObligationSemantic;

use super::super::operation_declaration::WorthUiQueryGraphOperationDeclaration;

pub(in crate::runtime::query_graph) fn composition_graph_access_operation(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> WorthUiQueryGraphOperationDeclaration {
    WorthUiQueryGraphOperationDeclaration::new(
        semantic,
        ForgeQueryGraphObligationSupportStatus::Supported,
    )
}

pub(in crate::runtime::query_graph) fn composition_graph_access_operation_catalog(
    semantic: WorthUiQueryGraphObligationSemantic,
) -> Vec<WorthUiQueryGraphOperationDeclaration> {
    vec![composition_graph_access_operation(semantic)]
}
