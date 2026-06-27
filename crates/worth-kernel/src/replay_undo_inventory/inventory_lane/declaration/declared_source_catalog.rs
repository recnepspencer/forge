use super::declared_source::ReplayUndoDeclaredSource;
use super::declared_source_identity::ReplayUndoDeclaredSourceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoDeclaredSourceCatalog {
    sources: Vec<ReplayUndoDeclaredSource>,
}

impl ReplayUndoDeclaredSourceCatalog {
    pub(crate) fn new(mut sources: Vec<ReplayUndoDeclaredSource>) -> Self {
        sources.sort_by_key(|source| source.identity());
        Self { sources }
    }

    pub fn sources(&self) -> &[ReplayUndoDeclaredSource] {
        &self.sources
    }

    pub fn require_source(
        &self,
        identity: ReplayUndoDeclaredSourceIdentity,
    ) -> Option<&ReplayUndoDeclaredSource> {
        self.sources
            .iter()
            .find(|source| source.identity() == identity)
    }
}
