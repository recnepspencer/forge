#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitFragmentCoverageRow {
    row_identity: String,
    schedule_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    fragments_checked: usize,
    covered_domain_start_bits: u64,
    covered_domain_end_bits: u64,
}

impl PlanarBooleanSplitFragmentCoverageRow {
    pub(crate) fn new(
        row_identity: String,
        schedule_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        fragments_checked: usize,
        covered_domain_start_bits: u64,
        covered_domain_end_bits: u64,
    ) -> Self {
        Self {
            row_identity,
            schedule_identity,
            source_edge_identity,
            carrier_identity,
            fragments_checked,
            covered_domain_start_bits,
            covered_domain_end_bits,
        }
    }

    pub fn row_identity(&self) -> &str {
        &self.row_identity
    }
    pub fn schedule_identity(&self) -> &str {
        &self.schedule_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn fragments_checked(&self) -> usize {
        self.fragments_checked
    }
    pub fn covered_domain_start_bits(&self) -> u64 {
        self.covered_domain_start_bits
    }
    pub fn covered_domain_end_bits(&self) -> u64 {
        self.covered_domain_end_bits
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapChainCoverageRow {
    row_identity: String,
    chain_identity: String,
    interval_event_identity: String,
    source_interval_identity: String,
    source_edge_identity: String,
    carrier_identity: String,
    members_checked: usize,
}

impl PlanarBooleanOverlapChainCoverageRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        row_identity: String,
        chain_identity: String,
        interval_event_identity: String,
        source_interval_identity: String,
        source_edge_identity: String,
        carrier_identity: String,
        members_checked: usize,
    ) -> Self {
        Self {
            row_identity,
            chain_identity,
            interval_event_identity,
            source_interval_identity,
            source_edge_identity,
            carrier_identity,
            members_checked,
        }
    }

    pub fn row_identity(&self) -> &str {
        &self.row_identity
    }
    pub fn chain_identity(&self) -> &str {
        &self.chain_identity
    }
    pub fn interval_event_identity(&self) -> &str {
        &self.interval_event_identity
    }
    pub fn source_interval_identity(&self) -> &str {
        &self.source_interval_identity
    }
    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }
    pub fn carrier_identity(&self) -> &str {
        &self.carrier_identity
    }
    pub fn members_checked(&self) -> usize {
        self.members_checked
    }
}
