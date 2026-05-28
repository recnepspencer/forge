#[cfg(test)]
mod interpretation_tests {

    use forge_relational::facade::runtime::RelationalRuntimeApi;
    use schema::facade::bootstrap_schema_registry;
    use schema::facade::topology_authoring::seed_minimal_topology;

    use crate::facade::{
        build_topology_read_artifact, certify_topology_view, interpret_topology_view,
        TopologyMaterializer,
    };
    use crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::{
        closed_wire_cycle_of_size, closed_wire_cycle_view, connected_wire_branch_view,
        open_sheet_patch_view, open_shell_nmt_fan_view, open_wire_chain_view,
        single_face_sheet_disk_view,
    };
    use schema::facade::platform::authority::{ShellInterpretationClass, WireInterpretationClass};

    #[test]
    fn seeded_bootstrap_interprets_as_open_sheet_with_one_wire() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_registry(bootstrap_schema_registry().expect(" bootstrap schema registry"))
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "interpret").expect("seed  topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect(" snapshot read");

        let topology = TopologyMaterializer::materialize_from_truth(&read_view)
            .expect(" topology materialization");
        let interpretation = interpret_topology_view(&topology);

        assert_eq!(interpretation.report().interpreted_wire_count, 1);
        assert_eq!(interpretation.report().interpreted_shell_count, 1);
        assert_eq!(interpretation.report().boundary_interpretation_count, 1);
        assert_eq!(interpretation.report().radial_interpretation_count, 1);
        assert_eq!(interpretation.interpretations().wires.len(), 1);
        assert_eq!(
            interpretation.interpretations().wires[0].connected_component_count,
            1
        );
        assert_eq!(
            interpretation.interpretations().wires[0]
                .terminal_vertex_ids
                .len(),
            1
        );
        assert_eq!(
            interpretation.interpretations().wires[0].class,
            WireInterpretationClass::OpenChain
        );

        assert_eq!(interpretation.interpretations().shells.len(), 1);
        assert_eq!(
            interpretation.interpretations().shells[0].class,
            ShellInterpretationClass::OpenSheet
        );
        assert_eq!(interpretation.interpretations().shells[0].face_count, 1);
        assert_eq!(
            interpretation.interpretations().shells[0].boundary_component_count,
            1
        );
        assert_eq!(
            interpretation.interpretations().shells[0].boundary_half_edge_count,
            1
        );
        assert_eq!(interpretation.boundary_summaries().len(), 1);
        assert_eq!(interpretation.radial_summaries().len(), 1);
    }

    #[test]
    fn seeded_bootstrap_certification_retains_interpretation_records() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_registry(bootstrap_schema_registry().expect(" bootstrap schema registry"))
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "certify").expect("seed  topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect(" snapshot read");

        let topology = TopologyMaterializer::materialize_from_truth(&read_view)
            .expect(" topology materialization");
        let interpreted = interpret_topology_view(&topology);
        let read_artifact = build_topology_read_artifact(&seeded.read_basis, &interpreted);
        let certified = certify_topology_view(seeded.read_basis.clone(), &interpreted);

        assert_eq!(read_artifact.snapshot, seeded.snapshot);
        assert_eq!(read_artifact.interpretations.wires.len(), 1);
        assert_eq!(read_artifact.interpretations.shells.len(), 1);
        assert_eq!(certified.read_basis.snapshot(), &seeded.snapshot);
        assert_eq!(certified.interpretations, read_artifact.interpretations);
    }

    #[test]
    fn closed_wire_cycle_interprets_as_closed_with_no_terminals() {
        let topology = closed_wire_cycle_view();
        let interpretation = interpret_topology_view(
            &crate::derived_topology::materialized_graph::MaterializedTopologyView::whole_view(
                topology,
            ),
        );

        assert_eq!(interpretation.interpretations().wires.len(), 1);
        let wire = &interpretation.interpretations().wires[0];
        assert_eq!(wire.connected_component_count, 1);
        assert_eq!(wire.class, WireInterpretationClass::ClosedCycle);
        assert!(wire.terminal_vertex_ids.is_empty());
        assert!(wire.branch_vertex_ids.is_empty());
    }

    #[test]
    fn longer_open_wire_chain_interprets_as_open_chain_with_two_terminals() {
        let topology = open_wire_chain_view(4);
        let interpretation = interpret_topology_view(
            &crate::derived_topology::materialized_graph::MaterializedTopologyView::whole_view(
                topology,
            ),
        );

        assert_eq!(interpretation.interpretations().wires.len(), 1);
        let wire = &interpretation.interpretations().wires[0];
        assert_eq!(wire.class, WireInterpretationClass::OpenChain);
        assert_eq!(wire.connected_component_count, 1);
        assert_eq!(wire.terminal_vertex_ids.len(), 2);
        assert!(wire.branch_vertex_ids.is_empty());
    }

    #[test]
    fn larger_closed_wire_cycle_interprets_as_closed_cycle() {
        let topology = closed_wire_cycle_of_size(4);
        let interpretation = interpret_topology_view(
            &crate::derived_topology::materialized_graph::MaterializedTopologyView::whole_view(
                topology,
            ),
        );

        assert_eq!(interpretation.interpretations().wires.len(), 1);
        let wire = &interpretation.interpretations().wires[0];
        assert_eq!(wire.class, WireInterpretationClass::ClosedCycle);
        assert_eq!(wire.connected_component_count, 1);
        assert!(wire.terminal_vertex_ids.is_empty());
        assert!(wire.branch_vertex_ids.is_empty());
    }

    #[test]
    fn larger_connected_wire_branch_interprets_as_connected_branch() {
        let topology = connected_wire_branch_view(4);
        let interpretation = interpret_topology_view(
            &crate::derived_topology::materialized_graph::MaterializedTopologyView::whole_view(
                topology,
            ),
        );

        assert_eq!(interpretation.interpretations().wires.len(), 1);
        let wire = &interpretation.interpretations().wires[0];
        assert_eq!(wire.class, WireInterpretationClass::ConnectedBranch);
        assert_eq!(wire.connected_component_count, 1);
        assert_eq!(wire.branch_vertex_ids.len(), 1);
        assert_eq!(wire.terminal_vertex_ids.len(), 4);
    }

    #[test]
    fn open_shell_with_nmt_edge_fan_interprets_as_open_and_non_manifold() {
        let topology = open_shell_nmt_fan_view(3);
        let interpretation = interpret_topology_view(
            &crate::derived_topology::materialized_graph::MaterializedTopologyView::whole_view(
                topology,
            ),
        );

        assert_eq!(interpretation.interpretations().shells.len(), 1);
        let shell = &interpretation.interpretations().shells[0];
        assert_eq!(shell.class, ShellInterpretationClass::OpenNonManifold);
        assert_eq!(shell.face_count, 3);
        assert!(shell.boundary_half_edge_count > 0);
        assert_eq!(shell.non_manifold_edge_ids.len(), 1);
    }

    #[test]
    fn larger_open_shell_nmt_edge_fan_interprets_as_open_and_non_manifold() {
        let topology = open_shell_nmt_fan_view(4);
        let interpretation = interpret_topology_view(
            &crate::derived_topology::materialized_graph::MaterializedTopologyView::whole_view(
                topology,
            ),
        );

        assert_eq!(interpretation.interpretations().shells.len(), 1);
        let shell = &interpretation.interpretations().shells[0];
        assert_eq!(shell.class, ShellInterpretationClass::OpenNonManifold);
        assert_eq!(shell.face_count, 4);
        assert!(shell.boundary_half_edge_count > 0);
        assert_eq!(shell.non_manifold_edge_ids.len(), 1);
    }

    #[test]
    fn single_face_open_sheet_interprets_as_sheet_disk_member() {
        let topology = single_face_sheet_disk_view(5);
        let interpretation = interpret_topology_view(
            &crate::derived_topology::materialized_graph::MaterializedTopologyView::whole_view(
                topology,
            ),
        );

        assert_eq!(interpretation.interpretations().shells.len(), 1);
        let shell = &interpretation.interpretations().shells[0];
        assert_eq!(interpretation.boundary_summaries().len(), 1);
        assert_eq!(
            interpretation.boundary_summaries()[0].boundary_component_count,
            1
        );
        assert_eq!(shell.class, ShellInterpretationClass::OpenSheet);
        assert_eq!(shell.face_count, 1);
        assert_eq!(shell.boundary_component_count, 1);
        assert_eq!(shell.boundary_half_edge_count, 5);
        assert!(shell.non_manifold_edge_ids.is_empty());
    }

    #[test]
    fn multi_face_open_shell_interprets_as_sheet_patch_member() {
        let topology = open_sheet_patch_view(3);
        let interpretation = interpret_topology_view(
            &crate::derived_topology::materialized_graph::MaterializedTopologyView::whole_view(
                topology,
            ),
        );

        assert_eq!(interpretation.interpretations().shells.len(), 1);
        let shell = &interpretation.interpretations().shells[0];
        assert_eq!(interpretation.boundary_summaries().len(), 1);
        assert!(interpretation.boundary_summaries()[0].boundary_component_count >= 1);
        assert_eq!(shell.class, ShellInterpretationClass::OpenSheet);
        assert_eq!(shell.face_count, 3);
        assert!(shell.boundary_component_count >= 1);
        assert!(shell.boundary_half_edge_count >= 5);
        assert!(shell.non_manifold_edge_ids.is_empty());
    }
}




