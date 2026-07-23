use std::collections::BTreeSet;

use super::evidence::WorthUiQueryBindingEvidence;
use crate::runtime::replacement::query_binding::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingUiRequirements,
};
use crate::source::{WorthUiBoundViewBindingReference, WorthUiRuntimeDependencyHook};
use crate::{capability::ViewBindingId, source::WorthUiRuntimeDependencyHookKind};

#[derive(Default)]
pub(super) struct WorthUiQueryBindingEvidenceAccumulator {
    definition: Option<worth_ui_query_binding::WorthUiQueryViewDefinition>,
    denial_presentation: Option<crate::capability::QueryDenialPresentation>,
    runtime_surfaces: BTreeSet<WorthUiRuntimeDependencyHookKind>,
    inspection_available: bool,
    projection_consumption_available: bool,
}

impl WorthUiQueryBindingEvidenceAccumulator {
    pub(super) fn record_bound_view_binding(
        &mut self,
        view_binding: &WorthUiBoundViewBindingReference,
    ) {
        let query = view_binding.query_semantics();
        self.record_definition(query.definition());
        self.denial_presentation = Some(*query.denial_presentation());
        self.inspection_available = true;
        self.projection_consumption_available = true;
    }

    pub(super) fn record_runtime_hook(&mut self, hook: &WorthUiRuntimeDependencyHook) {
        self.record_definition(hook.definition());
        self.denial_presentation = Some(*hook.denial_presentation());
        self.runtime_surfaces.insert(hook.kind());
    }

    pub(super) fn finish(self, view_binding_id: &str) -> Option<WorthUiQueryBindingEvidence> {
        let definition = self.definition?;
        let identity = WorthUiQueryBindingIdentity::new(
            &ViewBindingId::new(view_binding_id).expect("indexed view binding id remains valid"),
            &definition,
        );
        let ui_requirements =
            WorthUiQueryBindingUiRequirements::new(super::WorthUiQueryBindingUiRequirementsInput {
                lifecycle: definition.lifecycle(),
                async_result_presentation: self
                    .runtime_surfaces
                    .contains(&WorthUiRuntimeDependencyHookKind::AsyncResultState),
                recovery_presentation: self
                    .runtime_surfaces
                    .contains(&WorthUiRuntimeDependencyHookKind::SignalContinuation),
                inspection_relevance: self.inspection_available,
                projection_consumption: self.projection_consumption_available,
                denial_presentation: self
                    .denial_presentation
                    .unwrap_or_else(crate::capability::QueryDenialPresentation::structured_status),
            });
        Some(WorthUiQueryBindingEvidence::new(identity, ui_requirements))
    }

    fn record_definition(
        &mut self,
        definition: &worth_ui_query_binding::WorthUiQueryViewDefinition,
    ) {
        match &self.definition {
            Some(retained) => assert_eq!(
                retained, definition,
                "one admitted UI binding cannot carry competing Query definitions"
            ),
            None => self.definition = Some(definition.clone()),
        }
    }
}
