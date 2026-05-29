#[cfg(test)]
mod materializer_tests {
    use schema::facade::topology_authoring::seed_minimal_topology;

    use crate::derived_topology::materialized_graph::TopologyMaterializer;
    use crate::test_support::primitive_corpus::validated_topology::build_test_runtime;

    #[test]
    fn materialize_from_truth_reads_bootstrap_structure_from_relational_snapshot() {
        let mut runtime = build_test_runtime().expect(" milestone one runtime builder");

        let seeded = seed_minimal_topology(&mut runtime, "topo").expect("seed  topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect(" snapshot read");

        let topology = TopologyMaterializer::materialize_from_truth(&read_view)
            .expect(" topology materialization");

        assert!(topology.report().whole_view_materialization);
        assert_eq!(
            topology.report().fallback_class,
            Some(crate::derived_topology::materialized_graph::MaterializationFallbackClass::WholeViewRebuild)
        );
        assert_eq!(
            topology.report().breadth.entity_count,
            read_view.entities().len()
        );
        assert_eq!(
            topology.report().breadth.relation_count,
            read_view.relations().len()
        );
        assert_eq!(topology.report().breadth.topology_entity_count, 11);
        assert_eq!(topology.report().breadth.topology_relation_count, 14);

        let topology = topology.topology();

        assert_eq!(topology.models.len(), 1);
        assert_eq!(topology.bodies.len(), 1);
        assert_eq!(topology.lumps.len(), 1);
        assert_eq!(topology.regions.len(), 1);
        assert_eq!(topology.shells.len(), 1);
        assert_eq!(topology.faces.len(), 1);
        assert_eq!(topology.loops.len(), 1);
        assert_eq!(topology.wires.len(), 1);
        assert_eq!(topology.half_edges.len(), 1);
        assert_eq!(topology.edges.len(), 1);
        assert_eq!(topology.vertices.len(), 1);

        assert_eq!(topology.models[0].body_ids, vec![seeded.body]);
        assert_eq!(topology.bodies[0].model_id, Some(seeded.model));
        assert_eq!(topology.bodies[0].lump_ids, vec![seeded.lump]);
        assert_eq!(topology.lumps[0].body_id, Some(seeded.body));
        assert_eq!(topology.lumps[0].region_ids, vec![seeded.region]);
        assert_eq!(topology.regions[0].lump_id, Some(seeded.lump));
        assert_eq!(topology.regions[0].shell_ids, vec![seeded.shell]);
        assert_eq!(topology.shells[0].region_id, Some(seeded.region));
        assert_eq!(topology.shells[0].face_ids, vec![seeded.face]);
        assert_eq!(topology.faces[0].shell_id, Some(seeded.shell));
        assert_eq!(topology.faces[0].outer_loop_id, Some(seeded.outer_loop));
        assert_eq!(
            topology.faces[0].boundary_half_edge_ids,
            vec![seeded.half_edge]
        );
        assert_eq!(topology.loops[0].face_ids, vec![seeded.face]);
        assert_eq!(topology.loops[0].half_edge_ids, vec![seeded.half_edge]);
        assert_eq!(topology.wires[0].half_edge_ids, vec![seeded.half_edge]);
        assert_eq!(topology.half_edges[0].loop_id, Some(seeded.outer_loop));
        assert_eq!(topology.half_edges[0].wire_id, Some(seeded.wire));
        assert_eq!(
            topology.half_edges[0].next_half_edge_id,
            Some(seeded.half_edge)
        );
        assert_eq!(
            topology.half_edges[0].prev_half_edge_id,
            Some(seeded.half_edge)
        );
        assert_eq!(
            topology.half_edges[0].radial_next_half_edge_id,
            Some(seeded.half_edge)
        );
        assert_eq!(topology.half_edges[0].edge_id, Some(seeded.edge));
        assert_eq!(topology.half_edges[0].origin_vertex_id, Some(seeded.vertex));
        assert_eq!(topology.half_edges[0].target_vertex_id, Some(seeded.vertex));
        assert_eq!(topology.half_edges[0].face_id, Some(seeded.face));
    }
}




