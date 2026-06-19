use super::support::{
    assert_primitive_birth_compose_obligation_evidence, birth_synopsis,
    committed_birth_anchor_count, compose_family_cases, topology_workspace,
};
use crate::construction::{
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    TopologyConstructionQueryMutationSurface, TopologyPrimitiveConstructionBirthTopologyKind,
};

#[test]
fn admitted_birth_handoffs_execute_compose_graph_for_all_current_families() {
    for (family, descriptor, topology_birth_class, counts) in compose_family_cases() {
        let mut workspace = topology_workspace(family.as_str());
        let synopsis = birth_synopsis(family, descriptor, topology_birth_class, counts);
        let handoff = prepare_primitive_construction_query_admitted_handoff_from_synopsis(
            &synopsis,
            "birth-completeness",
            "birth-mapping",
            counts.supported_loop_count,
            counts.supported_body_count,
        )
        .expect("family synopsis should admit to topology handoff");

        let execution =
            super::super::execute_primitive_construction_birth_compose(&mut workspace, handoff)
                .expect("admitted handoff should execute through compose_graph");

        assert_eq!(
            execution.mutation_surface(),
            TopologyConstructionQueryMutationSurface::ComposeGraph
        );
        assert_eq!(execution.evidence().graph_obligation_selected_count(), 1);
        assert_eq!(
            execution.program().birth_entity_count(),
            counts.supported_vertex_count
        );
        assert_eq!(
            execution.receipt().write_receipts().len(),
            counts.supported_vertex_count
        );
        assert_eq!(
            execution
                .program()
                .materialization_coverage()
                .committed_topology_kinds(),
            &[TopologyPrimitiveConstructionBirthTopologyKind::Vertex]
        );
        assert_eq!(
            execution
                .program()
                .materialization_coverage()
                .unmaterialized_topology_kinds(),
            counts.expected_unmaterialized_topology_kinds()
        );
        assert_primitive_birth_compose_obligation_evidence(execution.evidence());
        assert_eq!(
            committed_birth_anchor_count(&mut workspace, family),
            counts.supported_vertex_count
        );
        assert!(!execution.evidence().batch_receipt_digest().is_empty());
        assert!(!execution.graph_obligation_envelope_digest().is_empty());
    }
}
