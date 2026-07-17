use crate::capability::ViewBindingId;

use super::{QueryDenialPresentation, ViewBindingFamily, VisibleStateBindingDeclaration};

/// UI presentation attached to one semantic Query binding definition.
/// Query capability, result, basis, live, and projection posture are derived
/// by `worth-ui-query-binding` and cannot be assembled here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ViewBindingDescriptor {
    id: ViewBindingId,
    family: ViewBindingFamily,
    definition: worth_ui_query_binding::WorthUiQueryViewDefinition,
    visible_state_bindings: Vec<VisibleStateBindingDeclaration>,
    denial_presentation: QueryDenialPresentation,
}

impl ViewBindingDescriptor {
    pub(crate) fn from_definition(
        id: ViewBindingId,
        family: ViewBindingFamily,
        definition: worth_ui_query_binding::WorthUiQueryViewDefinition,
    ) -> Self {
        Self {
            id,
            family,
            definition,
            visible_state_bindings: Vec::new(),
            denial_presentation: QueryDenialPresentation::StructuredStatus,
        }
    }

    pub fn with_visible_state_binding(
        mut self,
        visible_state_binding: VisibleStateBindingDeclaration,
    ) -> Self {
        self.visible_state_bindings.push(visible_state_binding);
        self
    }

    pub fn with_denial_presentation(
        mut self,
        denial_presentation: QueryDenialPresentation,
    ) -> Self {
        self.denial_presentation = denial_presentation;
        self
    }

    pub fn id(&self) -> &ViewBindingId {
        &self.id
    }

    pub fn family(&self) -> &ViewBindingFamily {
        &self.family
    }

    pub fn definition(&self) -> &worth_ui_query_binding::WorthUiQueryViewDefinition {
        &self.definition
    }

    pub fn visible_state_bindings(&self) -> &[VisibleStateBindingDeclaration] {
        &self.visible_state_bindings
    }

    pub fn denial_presentation(&self) -> &QueryDenialPresentation {
        &self.denial_presentation
    }
}
