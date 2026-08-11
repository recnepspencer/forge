use std::sync::Arc;

#[derive(Debug)]
pub(in crate::domain_computation) struct WorthQueryCandidateOccurrenceBinding(Arc<str>);

impl WorthQueryCandidateOccurrenceBinding {
    pub(in crate::domain_computation) fn owner_derived(identity: Arc<str>) -> Self {
        Self(identity)
    }

    pub(in crate::domain_computation) fn identity(&self) -> &str {
        &self.0
    }
}
