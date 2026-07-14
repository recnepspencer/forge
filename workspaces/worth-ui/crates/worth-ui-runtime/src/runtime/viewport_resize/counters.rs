#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiViewportResizeCounters {
    admitted_observations: u16,
    selected_neighborhoods: u16,
    committed_receipts: u16,
    durable_mutations: u16,
    replay_hits: u16,
    authority_probes: u16,
    emitted_targets: u16,
    materialized_host_target_sets: u16,
}

impl UiViewportResizeCounters {
    pub(super) fn from_committed(
        evidence: crate::evidence::UiViewportResizeEvidence,
        replayed: bool,
    ) -> Self {
        Self {
            admitted_observations: evidence.admitted_observations(),
            selected_neighborhoods: evidence.selected_neighborhoods(),
            committed_receipts: if replayed {
                0
            } else {
                evidence.committed_receipts()
            },
            durable_mutations: evidence.durable_mutations(),
            replay_hits: u16::from(replayed),
            authority_probes: evidence.authority_probes(),
            emitted_targets: evidence.emitted_targets(),
            materialized_host_target_sets: evidence.materialized_host_target_sets(),
        }
    }
    pub fn admitted_observations(self) -> u16 {
        self.admitted_observations
    }
    pub fn selected_neighborhoods(self) -> u16 {
        self.selected_neighborhoods
    }
    pub fn committed_receipts(self) -> u16 {
        self.committed_receipts
    }
    pub fn durable_mutations(self) -> u16 {
        self.durable_mutations
    }
    pub fn replay_hits(self) -> u16 {
        self.replay_hits
    }
    pub fn authority_probes(self) -> u16 {
        self.authority_probes
    }
    pub fn emitted_targets(self) -> u16 {
        self.emitted_targets
    }
    pub fn materialized_host_target_sets(self) -> u16 {
        self.materialized_host_target_sets
    }
}
