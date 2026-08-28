impl super::UiScrollRuntimeState {
    pub(crate) fn inspect_for_certification(
        &self,
    ) -> (usize, usize, u64, u64, u64, u64, u64, u64, u64, u64) {
        (
            self.owners.len(),
            self.ownership_catalog.len(),
            self.revision,
            self.counters.admitted_requests(),
            self.counters.rejected_requests(),
            self.counters.owners_visited(),
            self.counters.owners_changed(),
            self.ownership_resolutions,
            self.ownership_graph_nodes_visited,
            self.ownership_plan_nodes_visited,
        )
    }

    pub(crate) fn inspect_owner_geometry_for_certification(
        &self,
    ) -> Box<
        [(
            crate::runtime::scroll::UiScrollOwnerIdentity,
            crate::runtime::scroll::UiScrollOffset,
            crate::runtime::scroll::UiScrollBounds,
        )],
    > {
        self.owners
            .iter()
            .map(|(owner, record)| (*owner, record.offset, record.bounds))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub(crate) fn inspect_ownership_incarnations_for_certification(&self) -> Box<[u64]> {
        self.ownership_catalog
            .values()
            .map(|record| record.incarnation.as_u64())
            .collect()
    }

    pub(crate) fn inspect_ownership_mounted_instances_for_certification(&self) -> Box<[u64]> {
        self.ownership_catalog
            .keys()
            .map(|identity| identity.diagnostic_value())
            .collect()
    }
}
