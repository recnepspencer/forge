#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiGraphScrollPlanningAuthority {
    neighborhood_identity: crate::evidence::UiAllocationNeighborhoodIdentity,
    host_sources: Box<[crate::evidence::UiHostMeasurementAuthorityWitness]>,
    query_sources: Box<[crate::evidence::measurement::basis::UiQueryAllocationTargetMapping]>,
    counters: crate::evidence::UiScrollOwnerSourceAdmissionCounters,
}

impl UiGraphScrollPlanningAuthority {
    pub(super) fn seal(
        neighborhood_identity: crate::evidence::UiAllocationNeighborhoodIdentity,
        mut host_sources: Vec<crate::evidence::UiHostMeasurementAuthorityWitness>,
        mut query_sources: Vec<crate::evidence::measurement::basis::UiQueryAllocationTargetMapping>,
        counters: crate::evidence::UiScrollOwnerSourceAdmissionCounters,
    ) -> Self {
        host_sources.sort_unstable();
        host_sources.dedup();
        query_sources.sort_by_key(|source| source.identity_digest());
        query_sources.dedup();
        Self {
            neighborhood_identity,
            host_sources: host_sources.into_boxed_slice(),
            query_sources: query_sources.into_boxed_slice(),
            counters,
        }
    }

    pub(crate) fn neighborhood_identity(
        &self,
    ) -> &crate::evidence::UiAllocationNeighborhoodIdentity {
        &self.neighborhood_identity
    }
    pub(crate) fn host_sources(&self) -> &[crate::evidence::UiHostMeasurementAuthorityWitness] {
        &self.host_sources
    }
    pub(crate) fn query_sources(
        &self,
    ) -> &[crate::evidence::measurement::basis::UiQueryAllocationTargetMapping] {
        &self.query_sources
    }
    pub(crate) fn counters(&self) -> crate::evidence::UiScrollOwnerSourceAdmissionCounters {
        self.counters
    }
}
