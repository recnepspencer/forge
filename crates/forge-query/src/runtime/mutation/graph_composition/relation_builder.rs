use serde::Serialize;

use super::symbols::ForgeQueryGraphEntitySymbol;
use crate::runtime::mutation::ForgeQueryAspectMutationBuilder;

#[derive(Clone, Debug, Default)]
pub struct ForgeQueryGraphRelationMutationBuilder {
    inner: ForgeQueryAspectMutationBuilder,
}

impl ForgeQueryGraphRelationMutationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn aspect<T: Serialize>(mut self, path: impl Into<String>, value: T) -> Self {
        self.inner = self.inner.aspect(path, value);
        self
    }

    pub fn existing_entity_identity(
        mut self,
        path: impl Into<String>,
        entity_identity: impl Into<String>,
    ) -> Self {
        self.inner = self.inner.aspect(path, entity_identity.into());
        self
    }

    pub fn symbolic_entity_identity(
        mut self,
        path: impl Into<String>,
        symbol: &ForgeQueryGraphEntitySymbol,
    ) -> Self {
        self.inner = self
            .inner
            .symbolic_entity_identity(path, symbol.reference());
        self
    }

    pub(crate) fn into_inner(self) -> ForgeQueryAspectMutationBuilder {
        self.inner
    }
}
