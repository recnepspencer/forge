use super::super::{
    WorthQueryAdmittedBooleanPredicateExpression, WorthQueryBooleanPredicateTopology,
    WorthQueryBooleanSelectivityAdmissionPosture, WorthQueryBooleanSelectivityShapeDigest,
    WorthQueryGraphReadAccessShape, WorthQueryPredicateAnchorPosture,
    WorthQueryTraversalPredicateOrderingPosture,
};
use super::{
    branch::WorthQueryBooleanSelectivityBranch, counters::WorthQueryBooleanSelectivityCounters,
    row::WorthQueryBooleanPredicateSelectivityRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBooleanSelectivityShape {
    digest: WorthQueryBooleanSelectivityShapeDigest,
    read_graph_digest: String,
    access_shape_digest: String,
    boolean_topology: WorthQueryBooleanPredicateTopology,
    anchor_posture: WorthQueryPredicateAnchorPosture,
    traversal_predicate_ordering_posture: WorthQueryTraversalPredicateOrderingPosture,
    admission_posture: WorthQueryBooleanSelectivityAdmissionPosture,
    branches: Vec<WorthQueryBooleanSelectivityBranch>,
    predicate_rows: Vec<WorthQueryBooleanPredicateSelectivityRow>,
    counters: WorthQueryBooleanSelectivityCounters,
}

impl WorthQueryBooleanSelectivityShape {
    pub fn digest(&self) -> &WorthQueryBooleanSelectivityShapeDigest {
        &self.digest
    }

    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn access_shape_digest(&self) -> &str {
        &self.access_shape_digest
    }

    pub fn boolean_topology(&self) -> &WorthQueryBooleanPredicateTopology {
        &self.boolean_topology
    }

    pub fn anchor_posture(&self) -> &WorthQueryPredicateAnchorPosture {
        &self.anchor_posture
    }

    pub fn traversal_predicate_ordering_posture(
        &self,
    ) -> &WorthQueryTraversalPredicateOrderingPosture {
        &self.traversal_predicate_ordering_posture
    }

    pub fn admission_posture(&self) -> &WorthQueryBooleanSelectivityAdmissionPosture {
        &self.admission_posture
    }

    pub fn predicate_rows(&self) -> &[WorthQueryBooleanPredicateSelectivityRow] {
        &self.predicate_rows
    }

    pub fn branches(&self) -> &[WorthQueryBooleanSelectivityBranch] {
        &self.branches
    }

    pub fn counters(&self) -> &WorthQueryBooleanSelectivityCounters {
        &self.counters
    }

    pub fn explain(&self) -> String {
        format!(
            "read_graph={} access_shape={} selectivity={} topology={} anchor={} predicate_ordering={} admission={} branches={} predicates={} pre_traversal_predicates={} broad_predicates={} risky_predicates={} executor_observations={}",
            self.read_graph_digest,
            self.access_shape_digest,
            self.digest.as_str(),
            self.boolean_topology.as_str(),
            self.anchor_posture.as_str(),
            self.traversal_predicate_ordering_posture.as_str(),
            self.admission_posture.as_str(),
            self.branches.len(),
            self.counters.predicate_rows_normalized(),
            self.counters.pre_traversal_eligible_count(),
            self.counters.broad_predicate_count(),
            self.counters.risky_predicate_count(),
            self.counters.executor_observations_consumed()
        )
    }

    pub(crate) fn new(
        access_shape: WorthQueryGraphReadAccessShape,
        boolean_topology: WorthQueryBooleanPredicateTopology,
        anchor_posture: WorthQueryPredicateAnchorPosture,
        traversal_predicate_ordering_posture: WorthQueryTraversalPredicateOrderingPosture,
        admission_posture: WorthQueryBooleanSelectivityAdmissionPosture,
        expression: WorthQueryAdmittedBooleanPredicateExpression,
        branches: Vec<WorthQueryBooleanSelectivityBranch>,
        predicate_rows: Vec<WorthQueryBooleanPredicateSelectivityRow>,
        deduplicated_predicate_count: usize,
    ) -> Self {
        let read_graph_digest = access_shape
            .operation_resolution()
            .read_graph_digest()
            .to_string();
        let access_shape_digest = access_shape.digest().as_str().to_string();
        let counters = WorthQueryBooleanSelectivityCounters::from_expression(
            &expression,
            &predicate_rows,
            deduplicated_predicate_count,
        );
        let mut parts = vec![
            format!("read_graph:{read_graph_digest}"),
            format!("access_shape:{access_shape_digest}"),
            format!("boolean_topology:{}", boolean_topology.as_str()),
            format!("anchor_posture:{}", anchor_posture.as_str()),
            format!(
                "predicate_ordering:{}",
                traversal_predicate_ordering_posture.as_str()
            ),
            format!("admission_posture:{}", admission_posture.as_str()),
            format!("predicate_count:{}", counters.predicate_rows_normalized()),
            format!("expression_nodes:{}", counters.expression_nodes_visited()),
            format!(
                "admitted_references_consulted:{}",
                counters.admitted_references_consulted()
            ),
            format!("branches_produced:{}", counters.branches_produced()),
            format!(
                "pre_traversal_count:{}",
                counters.pre_traversal_eligible_count()
            ),
            format!("broad_count:{}", counters.broad_predicate_count()),
            format!("risky_count:{}", counters.risky_predicate_count()),
        ];
        parts.extend(expression.digest_parts());
        parts.extend(branches.iter().map(|branch| branch.digest_part()));
        parts.extend(predicate_rows.iter().map(|row| row.digest_part()));
        let digest = WorthQueryBooleanSelectivityShapeDigest::from_parts(&parts);
        Self {
            digest,
            read_graph_digest,
            access_shape_digest,
            boolean_topology,
            anchor_posture,
            traversal_predicate_ordering_posture,
            admission_posture,
            branches,
            predicate_rows,
            counters,
        }
    }
}
