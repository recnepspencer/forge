#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiIdentitySeedingMetrics {
    node_count_seeded: usize,
    authored_seed_count: usize,
    structural_fallback_count: usize,
    durable_state_eligible_count: usize,
}

impl WorthUiIdentitySeedingMetrics {
    pub(crate) fn record_seed(&mut self, authored: bool, durable_state_eligible: bool) {
        self.node_count_seeded += 1;
        if authored {
            self.authored_seed_count += 1;
        } else {
            self.structural_fallback_count += 1;
        }
        if durable_state_eligible {
            self.durable_state_eligible_count += 1;
        }
    }

    #[cfg(test)]
    pub(crate) fn node_count_seeded(&self) -> usize {
        self.node_count_seeded
    }
    #[cfg(test)]
    pub(crate) fn authored_seed_count(&self) -> usize {
        self.authored_seed_count
    }
    #[cfg(test)]
    pub(crate) fn structural_fallback_count(&self) -> usize {
        self.structural_fallback_count
    }
    #[cfg(test)]
    pub(crate) fn durable_state_eligible_count(&self) -> usize {
        self.durable_state_eligible_count
    }
}
