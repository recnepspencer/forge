use crate::domain_computation::authorization::WorthQueryAuthorizationDecisionFact;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryApplicationAuthorizationWorkEvidence {
    requirement_count: usize,
    paths_evaluated: usize,
    adjacency_lists_read: usize,
    adjacency_edges_inspected: usize,
    relation_records_inspected: usize,
    entity_records_inspected: usize,
    predicate_fields_inspected: usize,
    maximum_frontier_width: usize,
    reconstructive_graph_scans: usize,
    reconstructive_relation_records_scanned: usize,
    signal_dependency_count: usize,
}

impl WorthQueryApplicationAuthorizationWorkEvidence {
    pub(super) fn from_dependencies(dependencies: &[WorthQueryAuthorizationDecisionFact]) -> Self {
        dependencies.iter().fold(
            Self {
                requirement_count: dependencies.len(),
                ..Self::default()
            },
            |mut total, dependency| {
                let relational = dependency.relational.counters();
                total.paths_evaluated = total
                    .paths_evaluated
                    .saturating_add(relational.paths_evaluated);
                total.adjacency_lists_read = total
                    .adjacency_lists_read
                    .saturating_add(relational.adjacency_lists_read);
                total.adjacency_edges_inspected = total
                    .adjacency_edges_inspected
                    .saturating_add(relational.adjacency_edges_inspected);
                total.relation_records_inspected = total
                    .relation_records_inspected
                    .saturating_add(relational.relation_records_inspected);
                total.entity_records_inspected = total
                    .entity_records_inspected
                    .saturating_add(relational.entity_records_inspected);
                total.predicate_fields_inspected = total
                    .predicate_fields_inspected
                    .saturating_add(relational.predicate_fields_inspected);
                total.maximum_frontier_width = total
                    .maximum_frontier_width
                    .max(relational.maximum_frontier_width);
                total.reconstructive_graph_scans = total
                    .reconstructive_graph_scans
                    .saturating_add(relational.reconstructive_graph_scans);
                total.reconstructive_relation_records_scanned = total
                    .reconstructive_relation_records_scanned
                    .saturating_add(relational.reconstructive_relation_records_scanned);
                let bridge = dependency.bridge.counters();
                let signal_dependencies = bridge
                    .entities_depended_on
                    .saturating_add(bridge.relations_depended_on)
                    .saturating_add(bridge.adjacency_lists_depended_on)
                    .saturating_add(bridge.fields_depended_on);
                total.signal_dependency_count = total
                    .signal_dependency_count
                    .saturating_add(signal_dependencies);
                total
            },
        )
    }

    pub const fn requirement_count(self) -> usize {
        self.requirement_count
    }

    pub(super) fn combine(self, other: Self) -> Self {
        Self {
            requirement_count: self
                .requirement_count
                .saturating_add(other.requirement_count),
            paths_evaluated: self.paths_evaluated.saturating_add(other.paths_evaluated),
            adjacency_lists_read: self
                .adjacency_lists_read
                .saturating_add(other.adjacency_lists_read),
            adjacency_edges_inspected: self
                .adjacency_edges_inspected
                .saturating_add(other.adjacency_edges_inspected),
            relation_records_inspected: self
                .relation_records_inspected
                .saturating_add(other.relation_records_inspected),
            entity_records_inspected: self
                .entity_records_inspected
                .saturating_add(other.entity_records_inspected),
            predicate_fields_inspected: self
                .predicate_fields_inspected
                .saturating_add(other.predicate_fields_inspected),
            maximum_frontier_width: self
                .maximum_frontier_width
                .max(other.maximum_frontier_width),
            reconstructive_graph_scans: self
                .reconstructive_graph_scans
                .saturating_add(other.reconstructive_graph_scans),
            reconstructive_relation_records_scanned: self
                .reconstructive_relation_records_scanned
                .saturating_add(other.reconstructive_relation_records_scanned),
            signal_dependency_count: self
                .signal_dependency_count
                .saturating_add(other.signal_dependency_count),
        }
    }

    pub const fn paths_evaluated(self) -> usize {
        self.paths_evaluated
    }

    pub const fn adjacency_lists_read(self) -> usize {
        self.adjacency_lists_read
    }

    pub const fn adjacency_edges_inspected(self) -> usize {
        self.adjacency_edges_inspected
    }

    pub const fn relation_records_inspected(self) -> usize {
        self.relation_records_inspected
    }

    pub const fn entity_records_inspected(self) -> usize {
        self.entity_records_inspected
    }

    pub const fn predicate_fields_inspected(self) -> usize {
        self.predicate_fields_inspected
    }

    pub const fn maximum_frontier_width(self) -> usize {
        self.maximum_frontier_width
    }

    pub const fn reconstructive_graph_scans(self) -> usize {
        self.reconstructive_graph_scans
    }

    pub const fn reconstructive_relation_records_scanned(self) -> usize {
        self.reconstructive_relation_records_scanned
    }

    pub const fn signal_dependency_count(self) -> usize {
        self.signal_dependency_count
    }

    pub const fn observation_work_units(self) -> usize {
        self.paths_evaluated
            .saturating_add(self.adjacency_lists_read)
            .saturating_add(self.adjacency_edges_inspected)
            .saturating_add(self.relation_records_inspected)
            .saturating_add(self.entity_records_inspected)
            .saturating_add(self.predicate_fields_inspected)
            .saturating_add(self.reconstructive_graph_scans)
            .saturating_add(self.reconstructive_relation_records_scanned)
    }
}
