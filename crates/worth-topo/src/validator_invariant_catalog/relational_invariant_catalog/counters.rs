#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyRelationalInvariantCatalogCounters {
    invariant_family_count: usize,
    selected_invariant_family_count: usize,
    query_graph_obligation_registration_count: usize,
    old_pack_source_registration_count: usize,
    old_pack_ordinary_path_count: usize,
    source_firewall_violation_count: usize,
    execution_receipt_count: usize,
    counters_digest: String,
}

impl WorthTopologyRelationalInvariantCatalogCounters {
    pub(in crate::validator_invariant_catalog) fn from_parts(
        invariant_family_count: usize,
        selected_invariant_family_count: usize,
        query_graph_obligation_registration_count: usize,
        old_pack_source_registration_count: usize,
        old_pack_ordinary_path_count: usize,
        source_firewall_violation_count: usize,
    ) -> Self {
        let execution_receipt_count = 0;
        let counters_digest = [
            "worth-topo-relational-invariant-catalog-counters-v1",
            &invariant_family_count.to_string(),
            &selected_invariant_family_count.to_string(),
            &query_graph_obligation_registration_count.to_string(),
            &old_pack_source_registration_count.to_string(),
            &old_pack_ordinary_path_count.to_string(),
            &source_firewall_violation_count.to_string(),
            &execution_receipt_count.to_string(),
        ]
        .join("|");
        Self {
            invariant_family_count,
            selected_invariant_family_count,
            query_graph_obligation_registration_count,
            old_pack_source_registration_count,
            old_pack_ordinary_path_count,
            source_firewall_violation_count,
            execution_receipt_count,
            counters_digest,
        }
    }

    pub const fn invariant_family_count(&self) -> usize {
        self.invariant_family_count
    }

    pub const fn selected_invariant_family_count(&self) -> usize {
        self.selected_invariant_family_count
    }

    pub const fn query_graph_obligation_registration_count(&self) -> usize {
        self.query_graph_obligation_registration_count
    }

    pub const fn old_pack_source_registration_count(&self) -> usize {
        self.old_pack_source_registration_count
    }

    pub const fn old_pack_ordinary_path_count(&self) -> usize {
        self.old_pack_ordinary_path_count
    }

    pub const fn source_firewall_violation_count(&self) -> usize {
        self.source_firewall_violation_count
    }

    pub const fn execution_receipt_count(&self) -> usize {
        self.execution_receipt_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
