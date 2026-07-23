impl super::UiAllocationInvalidationAuthority {
    pub(crate) fn certifies_selection(
        &self,
        selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
    ) -> bool {
        selection
            .ordered_neighborhoods()
            .iter()
            .all(|selected| self.graph_replan.certifies(&selected.generation_key()))
    }
}
