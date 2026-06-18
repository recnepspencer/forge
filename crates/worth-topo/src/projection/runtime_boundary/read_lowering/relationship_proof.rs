use forge_query::facade::{
    ExecutionBasisIntent, ForgeQueryReadGraph, RelationshipProofTopologyClass, SnapshotLineageClass,
};

use super::TopologyReadRelationshipProofPosture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RelationshipProofLowering {
    pub(super) posture: TopologyReadRelationshipProofPosture,
    pub(super) admission_identity: Option<String>,
    pub(super) topology_classes: Vec<RelationshipProofTopologyClass>,
    pub(super) admission_count: usize,
    pub(super) topology_width: usize,
    pub(super) support_profile_digest: String,
}

impl RelationshipProofLowering {
    fn support_profile_digest() -> String {
        forge_query::facade::runtime_backed_relationship_proof_support_profile()
            .profile_digest()
            .to_string()
    }

    fn deferred_until_query_runtime_graph() -> Self {
        Self {
            posture: TopologyReadRelationshipProofPosture::Deferred,
            admission_identity: None,
            topology_classes: Vec::new(),
            admission_count: 0,
            topology_width: 0,
            support_profile_digest: Self::support_profile_digest(),
        }
    }

    fn from_query_read_graph(read_graph: &ForgeQueryReadGraph) -> Self {
        let Some(admission) = read_graph.relationship_proof_admission() else {
            return Self::deferred_until_query_runtime_graph();
        };
        Self {
            posture: TopologyReadRelationshipProofPosture::Admitted,
            admission_identity: Some(admission.identity().as_str().to_string()),
            topology_classes: admission.topology_classes().to_vec(),
            admission_count: admission.descriptor_count(),
            topology_width: admission.budget().max_topology_width(),
            support_profile_digest: Self::support_profile_digest(),
        }
    }
}

pub(super) fn deferred_topology_read_relationship_proofs() -> RelationshipProofLowering {
    RelationshipProofLowering::deferred_until_query_runtime_graph()
}

pub(super) fn query_runtime_topology_read_relationship_proofs(
    read_graph: &ForgeQueryReadGraph,
) -> RelationshipProofLowering {
    RelationshipProofLowering::from_query_read_graph(read_graph)
}

pub(super) fn relationship_proof_boundary_diagnostic(
    read_graph: &ForgeQueryReadGraph,
) -> &'static str {
    if read_graph.relationship_proof_admission().is_some() {
        "worth-topo/runtime_boundary/read_lowering:query-read-graph-relationship-proof-authority-admitted"
    } else {
        "worth-topo/runtime_boundary/read_lowering:query-read-graph-relationship-proof-authority-missing"
    }
}

pub(super) fn runtime_basis_intent() -> ExecutionBasisIntent {
    ExecutionBasisIntent::new(
        forge_query::facade::BasisAuthorityFamily::Runtime,
        SnapshotLineageClass::CurrentHead,
        false,
    )
}

#[cfg(test)]
mod tests {
    use crate::projection::runtime_boundary::read_execution::query_shape::{
        identity_anchor_predicate, identity_ordering, identity_result_field, identity_selector,
        topology_kind_result_field, topology_kind_selector, TOPOLOGY_ENTITY_ROOT,
    };
    use crate::projection::runtime_boundary::read_execution::successor_relation_name;
    use crate::projection::runtime_boundary::read_lowering::schema::topology_read_schema_view;
    use forge_query::facade::TraversalSelector;

    #[test]
    fn diagnostic_names_query_read_graph_relationship_proof_authority() {
        let schema_view = topology_read_schema_view().expect("schema view");
        let graph = forge_query::facade::ForgeQueryReadBuilder::standalone()
            .anchored_collection(
                TOPOLOGY_ENTITY_ROOT,
                schema_view,
                |query| {
                    query
                        .project(identity_selector().expect("identity selector"))
                        .project(topology_kind_selector().expect("topology selector"))
                        .where_equal(
                            identity_anchor_predicate(".topology.half_edge.7")
                                .expect("identity predicate"),
                        )
                        .order_by(identity_ordering().expect("identity ordering"))
                        .traverse(
                            TraversalSelector::bounded_relation_name(successor_relation_name(), 2)
                                .expect("successor traversal"),
                        )
                },
                |shape| {
                    shape
                        .field(identity_result_field().expect("identity field"))
                        .field(topology_kind_result_field().expect("topology field"))
                },
            )
            .expect("query read graph");

        assert_eq!(
            super::relationship_proof_boundary_diagnostic(&graph),
            "worth-topo/runtime_boundary/read_lowering:query-read-graph-relationship-proof-authority-admitted"
        );
    }
}
