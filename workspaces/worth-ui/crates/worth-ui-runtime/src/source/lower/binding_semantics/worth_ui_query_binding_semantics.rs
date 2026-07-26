use crate::capability::FrozenViewBindingEntry;
use crate::source::{WorthUiBindingDiagnostic, WorthUiBoundQueryViewSemantics};

use super::worth_ui_binding_semantics_context::WorthUiBindingSemanticsContext;

pub(super) fn bind_query_view_semantics(
    _module_id: &worth_ui_dsl::WorthUiSourceModuleId,
    entry: &FrozenViewBindingEntry,
    _semantic_locus: &str,
    _provenance: &worth_ui_dsl::WorthUiArtifactInputProvenance,
    context: &mut WorthUiBindingSemanticsContext<'_>,
) -> Result<WorthUiBoundQueryViewSemantics, WorthUiBindingDiagnostic> {
    context.record_query_owned_semantic_check();
    Ok(WorthUiBoundQueryViewSemantics::new(
        entry.descriptor().definition().clone(),
        *entry.descriptor().denial_presentation(),
    ))
}
