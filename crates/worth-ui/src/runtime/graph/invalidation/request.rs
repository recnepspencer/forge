use crate::runtime::WorthUiRuntimeFactSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiGraphInvalidationRequest {
    authoritative_changed_facts: WorthUiRuntimeFactSet,
}

impl WorthUiGraphInvalidationRequest {
    pub fn from_authoritative_changed_facts(
        authoritative_changed_facts: WorthUiRuntimeFactSet,
    ) -> Self {
        Self {
            authoritative_changed_facts,
        }
    }

    pub fn authoritative_changed_facts(&self) -> &WorthUiRuntimeFactSet {
        &self.authoritative_changed_facts
    }

    pub(super) fn into_authoritative_changed_facts(self) -> WorthUiRuntimeFactSet {
        self.authoritative_changed_facts
    }
}
