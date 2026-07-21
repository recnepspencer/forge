impl super::UiAllocationInvalidationAuthority {
    #[cfg(not(test))]
    pub(super) fn context_for_scope(
        &self,
        scope: &crate::evidence::UiAllocationNeighborhoodScope,
    ) -> Option<&super::UiCommittedAllocationInvalidationContext> {
        self.catalog
            .row(scope)
            .map(|row| row.committed_invalidation_context())
    }

    #[cfg(test)]
    pub(super) fn context_for_scope(
        &self,
        scope: &crate::evidence::UiAllocationNeighborhoodScope,
    ) -> Option<&super::UiCommittedAllocationInvalidationContext> {
        self.catalog
            .row(scope)
            .map(|row| row.committed_invalidation_context())
            .or(self.fixture_contexts.get(scope))
    }

    #[cfg(not(test))]
    pub(super) fn has_invalidation_contexts(&self) -> bool {
        !self.catalog.is_empty()
    }

    #[cfg(test)]
    pub(super) fn has_invalidation_contexts(&self) -> bool {
        !self.catalog.is_empty() || !self.fixture_contexts.is_empty()
    }

    #[cfg(test)]
    pub(super) fn install_fixture_context(
        &mut self,
        context: super::UiCommittedAllocationInvalidationContext,
    ) {
        let scope = crate::evidence::UiAllocationNeighborhoodScope::from_neighborhood(
            &context.neighborhood,
        );
        self.fixture_contexts.insert(scope, context);
    }
}
