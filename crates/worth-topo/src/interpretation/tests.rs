#[cfg(test)]
mod interpretation_tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use forge_relational::facade::runtime::RelationalRuntimeApi;
    use worth_schema::facade::topology_authoring::seed_minimal_topology;
    use worth_schema::facade::worth_bootstrap_schema_registry;

    use crate::data::topology_view::{
        WorthTopologyBody, WorthTopologyEdge, WorthTopologyFace, WorthTopologyHalfEdge,
        WorthTopologyLoop, WorthTopologyLump, WorthTopologyModel, WorthTopologyRegion,
        WorthTopologyShell, WorthTopologyVertex, WorthTopologyView, WorthTopologyWire,
    };
    use crate::facade::{
        build_topology_read_artifact, certify_topology_view, interpret_topology_view,
        WorthTopologyMaterializer,
    };
    use worth_schema::facade::{WorthShellInterpretationClass, WorthWireInterpretationClass};

    #[test]
    fn seeded_bootstrap_interprets_as_open_sheet_with_one_wire() {
        let mut runtime = RelationalRuntimeApi::builder()
            .schema_registry(
                worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
            )
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "interpret").expect("seed worth topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("worth snapshot read");

        let topology = WorthTopologyMaterializer::materialize_from_truth(&read_view)
            .expect("worth topology materialization");
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
            WorthWireInterpretationClass::OpenChain
        );

        assert_eq!(interpretation.interpretations().shells.len(), 1);
        assert_eq!(
            interpretation.interpretations().shells[0].class,
            WorthShellInterpretationClass::OpenSheet
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
            .schema_registry(
                worth_bootstrap_schema_registry().expect("worth bootstrap schema registry"),
            )
            .build();

        let seeded = seed_minimal_topology(&mut runtime, "certify").expect("seed worth topology");
        let read_view = runtime
            .read_truth()
            .read_snapshot(&seeded.snapshot)
            .expect("worth snapshot read");

        let topology = WorthTopologyMaterializer::materialize_from_truth(&read_view)
            .expect("worth topology materialization");
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
            &crate::materialization::MaterializedTopologyView::whole_view(topology),
        );

        assert_eq!(interpretation.interpretations().wires.len(), 1);
        let wire = &interpretation.interpretations().wires[0];
        assert_eq!(wire.connected_component_count, 1);
        assert_eq!(wire.class, WorthWireInterpretationClass::ClosedCycle);
        assert!(wire.terminal_vertex_ids.is_empty());
        assert!(wire.branch_vertex_ids.is_empty());
    }

    #[test]
    fn longer_open_wire_chain_interprets_as_open_chain_with_two_terminals() {
        let topology = open_wire_chain_view(4);
        let interpretation = interpret_topology_view(
            &crate::materialization::MaterializedTopologyView::whole_view(topology),
        );

        assert_eq!(interpretation.interpretations().wires.len(), 1);
        let wire = &interpretation.interpretations().wires[0];
        assert_eq!(wire.class, WorthWireInterpretationClass::OpenChain);
        assert_eq!(wire.connected_component_count, 1);
        assert_eq!(wire.terminal_vertex_ids.len(), 2);
        assert!(wire.branch_vertex_ids.is_empty());
    }

    #[test]
    fn larger_closed_wire_cycle_interprets_as_closed_cycle() {
        let topology = closed_wire_cycle_of_size(4);
        let interpretation = interpret_topology_view(
            &crate::materialization::MaterializedTopologyView::whole_view(topology),
        );

        assert_eq!(interpretation.interpretations().wires.len(), 1);
        let wire = &interpretation.interpretations().wires[0];
        assert_eq!(wire.class, WorthWireInterpretationClass::ClosedCycle);
        assert_eq!(wire.connected_component_count, 1);
        assert!(wire.terminal_vertex_ids.is_empty());
        assert!(wire.branch_vertex_ids.is_empty());
    }

    #[test]
    fn larger_connected_wire_branch_interprets_as_connected_branch() {
        let topology = connected_wire_branch_view(4);
        let interpretation = interpret_topology_view(
            &crate::materialization::MaterializedTopologyView::whole_view(topology),
        );

        assert_eq!(interpretation.interpretations().wires.len(), 1);
        let wire = &interpretation.interpretations().wires[0];
        assert_eq!(wire.class, WorthWireInterpretationClass::ConnectedBranch);
        assert_eq!(wire.connected_component_count, 1);
        assert_eq!(wire.branch_vertex_ids.len(), 1);
        assert_eq!(wire.terminal_vertex_ids.len(), 4);
    }

    #[test]
    fn open_shell_with_nmt_edge_fan_interprets_as_open_and_non_manifold() {
        let topology = open_shell_nmt_fan_view(3);
        let interpretation = interpret_topology_view(
            &crate::materialization::MaterializedTopologyView::whole_view(topology),
        );

        assert_eq!(interpretation.interpretations().shells.len(), 1);
        let shell = &interpretation.interpretations().shells[0];
        assert_eq!(shell.class, WorthShellInterpretationClass::OpenNonManifold);
        assert_eq!(shell.face_count, 3);
        assert!(shell.boundary_half_edge_count > 0);
        assert_eq!(shell.non_manifold_edge_ids.len(), 1);
    }

    #[test]
    fn larger_open_shell_nmt_edge_fan_interprets_as_open_and_non_manifold() {
        let topology = open_shell_nmt_fan_view(4);
        let interpretation = interpret_topology_view(
            &crate::materialization::MaterializedTopologyView::whole_view(topology),
        );

        assert_eq!(interpretation.interpretations().shells.len(), 1);
        let shell = &interpretation.interpretations().shells[0];
        assert_eq!(shell.class, WorthShellInterpretationClass::OpenNonManifold);
        assert_eq!(shell.face_count, 4);
        assert!(shell.boundary_half_edge_count > 0);
        assert_eq!(shell.non_manifold_edge_ids.len(), 1);
    }

    #[test]
    fn single_face_open_sheet_interprets_as_sheet_disk_member() {
        let topology = single_face_sheet_disk_view(5);
        let interpretation = interpret_topology_view(
            &crate::materialization::MaterializedTopologyView::whole_view(topology),
        );

        assert_eq!(interpretation.interpretations().shells.len(), 1);
        let shell = &interpretation.interpretations().shells[0];
        assert_eq!(interpretation.boundary_summaries().len(), 1);
        assert_eq!(
            interpretation.boundary_summaries()[0].boundary_component_count,
            1
        );
        assert_eq!(shell.class, WorthShellInterpretationClass::OpenSheet);
        assert_eq!(shell.face_count, 1);
        assert_eq!(shell.boundary_component_count, 1);
        assert_eq!(shell.boundary_half_edge_count, 5);
        assert!(shell.non_manifold_edge_ids.is_empty());
    }

    #[test]
    fn multi_face_open_shell_interprets_as_sheet_patch_member() {
        let topology = open_sheet_patch_view(3);
        let interpretation = interpret_topology_view(
            &crate::materialization::MaterializedTopologyView::whole_view(topology),
        );

        assert_eq!(interpretation.interpretations().shells.len(), 1);
        let shell = &interpretation.interpretations().shells[0];
        assert_eq!(interpretation.boundary_summaries().len(), 1);
        assert!(interpretation.boundary_summaries()[0].boundary_component_count >= 1);
        assert_eq!(shell.class, WorthShellInterpretationClass::OpenSheet);
        assert_eq!(shell.face_count, 3);
        assert!(shell.boundary_component_count >= 1);
        assert!(shell.boundary_half_edge_count >= 5);
        assert!(shell.non_manifold_edge_ids.is_empty());
    }

    fn closed_wire_cycle_view() -> WorthTopologyView {
        closed_wire_cycle_of_size(3)
    }

    fn open_wire_chain_view(length: usize) -> WorthTopologyView {
        assert!(
            length >= 2,
            "open wire chain requires at least two half-edges"
        );

        let wire_id = entity(60);
        let mut half_edges = Vec::new();
        let mut edges = Vec::new();
        let mut vertices = Vec::new();
        let mut half_edge_ids = Vec::new();

        for index in 0..=length {
            vertices.push(vertex(&format!("v{index}"), entity(61 + index as u64)));
        }

        for index in 0..length {
            let edge_id = entity(80 + index as u64);
            let half_edge_id = entity(100 + index as u64);
            let next_half_edge_id = if index + 1 < length {
                Some(entity(100 + (index + 1) as u64))
            } else {
                None
            };
            let prev_half_edge_id = if index == 0 {
                None
            } else {
                Some(entity(100 + (index - 1) as u64))
            };

            edges.push(edge(&format!("e{index}"), edge_id));
            half_edges.push(half_edge_full(
                half_edge_id,
                None,
                Some(wire_id),
                next_half_edge_id,
                prev_half_edge_id,
                Some(half_edge_id),
                Some(edge_id),
                Some(entity(61 + index as u64)),
                Some(entity(61 + index as u64 + 1)),
                None,
            ));
            half_edge_ids.push(half_edge_id);
        }

        WorthTopologyView {
            wires: vec![WorthTopologyWire {
                entity_id: wire_id,
                label: "open-chain".into(),
                half_edge_ids,
            }],
            half_edges,
            edges,
            vertices,
            ..WorthTopologyView::default()
        }
    }

    fn connected_wire_branch_view(branch_count: usize) -> WorthTopologyView {
        assert!(
            branch_count >= 3,
            "connected wire branch requires at least three arms"
        );

        let wire_id = entity(500);
        let center_vertex = entity(501);
        let mut vertices = vec![vertex("center", center_vertex)];
        let mut edges = Vec::new();
        let mut half_edges = Vec::new();
        let mut half_edge_ids = Vec::new();

        for index in 0..branch_count {
            let outer_vertex = entity(510 + index as u64);
            let edge_id = entity(530 + index as u64);
            let half_edge_id = entity(550 + index as u64);
            vertices.push(vertex(&format!("leaf{index}"), outer_vertex));
            edges.push(edge(&format!("branch{index}"), edge_id));
            half_edges.push(half_edge_full(
                half_edge_id,
                None,
                Some(wire_id),
                Some(half_edge_id),
                Some(half_edge_id),
                Some(half_edge_id),
                Some(edge_id),
                Some(center_vertex),
                Some(outer_vertex),
                None,
            ));
            half_edge_ids.push(half_edge_id);
        }

        WorthTopologyView {
            wires: vec![WorthTopologyWire {
                entity_id: wire_id,
                label: "branch".into(),
                half_edge_ids,
            }],
            half_edges,
            edges,
            vertices,
            ..WorthTopologyView::default()
        }
    }

    fn closed_wire_cycle_of_size(length: usize) -> WorthTopologyView {
        assert!(
            length >= 3,
            "closed wire cycle requires at least three half-edges"
        );

        let wire_id = entity(1);
        let mut half_edges = Vec::new();
        let mut edges = Vec::new();
        let mut vertices = Vec::new();
        let mut half_edge_ids = Vec::new();

        for index in 0..length {
            vertices.push(vertex(&format!("v{index}"), entity(2 + index as u64)));
        }

        for index in 0..length {
            let edge_id = entity(20 + index as u64);
            let half_edge_id = entity(40 + index as u64);
            let next_half_edge_id = entity(40 + ((index + 1) % length) as u64);
            let prev_half_edge_id = entity(40 + ((index + length - 1) % length) as u64);
            let origin_vertex_id = entity(2 + index as u64);

            edges.push(edge(&format!("e{index}"), edge_id));
            half_edges.push(half_edge_full(
                half_edge_id,
                None,
                Some(wire_id),
                Some(next_half_edge_id),
                Some(prev_half_edge_id),
                Some(half_edge_id),
                Some(edge_id),
                Some(origin_vertex_id),
                Some(entity(2 + ((index + 1) % length) as u64)),
                None,
            ));
            half_edge_ids.push(half_edge_id);
        }

        WorthTopologyView {
            wires: vec![WorthTopologyWire {
                entity_id: wire_id,
                label: "cycle".into(),
                half_edge_ids,
            }],
            half_edges,
            edges,
            vertices,
            ..WorthTopologyView::default()
        }
    }

    fn open_shell_nmt_fan_view(fan_size: usize) -> WorthTopologyView {
        assert!(
            fan_size >= 3,
            "nmt fan requires at least three incident faces"
        );

        let model_id = entity(20);
        let body_id = entity(21);
        let lump_id = entity(22);
        let region_id = entity(23);
        let shell_id = entity(24);
        let edge_shared = entity(31);
        let v1 = entity(32);
        let v2 = entity(33);

        let mut faces = Vec::new();
        let mut loops = Vec::new();
        let mut half_edges = Vec::new();
        let mut edges = vec![edge("shared", edge_shared)];
        let mut vertices = vec![vertex("v1", v1), vertex("v2", v2)];
        let mut face_ids = Vec::new();
        let mut shared_half_edge_ids = Vec::new();

        for index in 0..fan_size {
            let face_id = entity(100 + index as u64);
            let loop_id = entity(120 + index as u64);
            let third_vertex = entity(140 + index as u64);
            let first_shared_he = entity(160 + index as u64 * 3);
            let second_he = entity(161 + index as u64 * 3);
            let third_he = entity(162 + index as u64 * 3);
            let edge_a = entity(200 + index as u64 * 2);
            let edge_b = entity(201 + index as u64 * 2);

            let (shared_origin, shared_target) = if index == 0 { (v1, v2) } else { (v2, v1) };
            let (second_origin, second_target) = if index == 0 {
                (v2, third_vertex)
            } else {
                (v1, third_vertex)
            };
            let (third_origin, third_target) = if index == 0 {
                (third_vertex, v1)
            } else {
                (third_vertex, v2)
            };

            face_ids.push(face_id);
            shared_half_edge_ids.push(first_shared_he);
            vertices.push(vertex(&format!("vf{index}"), third_vertex));
            edges.push(edge(&format!("ea{index}"), edge_a));
            edges.push(edge(&format!("eb{index}"), edge_b));

            faces.push(WorthTopologyFace {
                entity_id: face_id,
                label: format!("f{index}"),
                shell_id: Some(shell_id),
                outer_loop_id: Some(loop_id),
                inner_loop_ids: vec![],
                boundary_half_edge_ids: vec![first_shared_he, second_he, third_he],
            });
            loops.push(WorthTopologyLoop {
                entity_id: loop_id,
                label: format!("l{index}"),
                face_ids: vec![face_id],
                half_edge_ids: vec![first_shared_he, second_he, third_he],
            });
            half_edges.push(half_edge_full(
                first_shared_he,
                Some(loop_id),
                None,
                Some(second_he),
                Some(third_he),
                None,
                Some(edge_shared),
                Some(shared_origin),
                Some(shared_target),
                Some(face_id),
            ));
            half_edges.push(half_edge_full(
                second_he,
                Some(loop_id),
                None,
                Some(third_he),
                Some(first_shared_he),
                Some(second_he),
                Some(edge_a),
                Some(second_origin),
                Some(second_target),
                Some(face_id),
            ));
            half_edges.push(half_edge_full(
                third_he,
                Some(loop_id),
                None,
                Some(first_shared_he),
                Some(second_he),
                Some(third_he),
                Some(edge_b),
                Some(third_origin),
                Some(third_target),
                Some(face_id),
            ));
        }

        for index in 0..fan_size {
            let next_shared = shared_half_edge_ids[(index + 1) % fan_size];
            let current_shared = shared_half_edge_ids[index];
            if let Some(record) = half_edges
                .iter_mut()
                .find(|record| record.entity_id == current_shared)
            {
                record.radial_next_half_edge_id = Some(next_shared);
            }
        }

        WorthTopologyView {
            models: vec![WorthTopologyModel {
                entity_id: model_id,
                label: "model".into(),
                body_ids: vec![body_id],
            }],
            bodies: vec![WorthTopologyBody {
                entity_id: body_id,
                label: "body".into(),
                model_id: Some(model_id),
                lump_ids: vec![lump_id],
            }],
            lumps: vec![WorthTopologyLump {
                entity_id: lump_id,
                label: "lump".into(),
                body_id: Some(body_id),
                region_ids: vec![region_id],
            }],
            regions: vec![WorthTopologyRegion {
                entity_id: region_id,
                label: "region".into(),
                lump_id: Some(lump_id),
                shell_ids: vec![shell_id],
            }],
            shells: vec![WorthTopologyShell {
                entity_id: shell_id,
                label: "sheet".into(),
                region_id: Some(region_id),
                face_ids,
            }],
            faces,
            loops,
            half_edges,
            edges,
            vertices,
            ..WorthTopologyView::default()
        }
    }

    fn single_face_sheet_disk_view(edge_count: usize) -> WorthTopologyView {
        assert!(
            edge_count >= 3,
            "sheet disk requires at least three boundary edges"
        );

        let model_id = entity(700);
        let body_id = entity(701);
        let lump_id = entity(702);
        let region_id = entity(703);
        let shell_id = entity(704);
        let face_id = entity(705);
        let loop_id = entity(706);

        let mut vertices = Vec::new();
        let mut edges = Vec::new();
        let mut half_edges = Vec::new();
        let mut half_edge_ids = Vec::new();

        for index in 0..edge_count {
            vertices.push(vertex(
                &format!("disk-v{index}"),
                entity(710 + index as u64),
            ));
        }

        for index in 0..edge_count {
            let edge_id = entity(730 + index as u64);
            let half_edge_id = entity(750 + index as u64);
            let origin = entity(710 + index as u64);
            let target = entity(710 + ((index + 1) % edge_count) as u64);
            let next = entity(750 + ((index + 1) % edge_count) as u64);
            let prev = entity(750 + ((index + edge_count - 1) % edge_count) as u64);

            edges.push(edge(&format!("disk-e{index}"), edge_id));
            half_edges.push(half_edge_full(
                half_edge_id,
                Some(loop_id),
                None,
                Some(next),
                Some(prev),
                Some(half_edge_id),
                Some(edge_id),
                Some(origin),
                Some(target),
                Some(face_id),
            ));
            half_edge_ids.push(half_edge_id);
        }

        WorthTopologyView {
            models: vec![WorthTopologyModel {
                entity_id: model_id,
                label: "model".into(),
                body_ids: vec![body_id],
            }],
            bodies: vec![WorthTopologyBody {
                entity_id: body_id,
                label: "body".into(),
                model_id: Some(model_id),
                lump_ids: vec![lump_id],
            }],
            lumps: vec![WorthTopologyLump {
                entity_id: lump_id,
                label: "lump".into(),
                body_id: Some(body_id),
                region_ids: vec![region_id],
            }],
            regions: vec![WorthTopologyRegion {
                entity_id: region_id,
                label: "region".into(),
                lump_id: Some(lump_id),
                shell_ids: vec![shell_id],
            }],
            shells: vec![WorthTopologyShell {
                entity_id: shell_id,
                label: "sheet-disk".into(),
                region_id: Some(region_id),
                face_ids: vec![face_id],
            }],
            faces: vec![WorthTopologyFace {
                entity_id: face_id,
                label: "disk-face".into(),
                shell_id: Some(shell_id),
                outer_loop_id: Some(loop_id),
                inner_loop_ids: vec![],
                boundary_half_edge_ids: half_edge_ids.clone(),
            }],
            loops: vec![WorthTopologyLoop {
                entity_id: loop_id,
                label: "disk-loop".into(),
                face_ids: vec![face_id],
                half_edge_ids,
            }],
            half_edges,
            edges,
            vertices,
            ..WorthTopologyView::default()
        }
    }

    fn open_sheet_patch_view(face_count: usize) -> WorthTopologyView {
        assert!(face_count >= 2, "sheet patch requires at least two faces");
        assert!(
            face_count <= 3,
            "test helper currently supports up to three faces"
        );

        let model_id = entity(800);
        let body_id = entity(801);
        let lump_id = entity(802);
        let region_id = entity(803);
        let shell_id = entity(804);

        let vertex_ids = [
            entity(810),
            entity(811),
            entity(812),
            entity(813),
            entity(814),
        ];
        let edge_ids = [
            entity(820),
            entity(821),
            entity(822),
            entity(823),
            entity(824),
            entity(825),
            entity(826),
        ];
        let face_ids = [entity(830), entity(831), entity(832)];
        let loop_ids = [entity(840), entity(841), entity(842)];
        let half_edge_ids = [
            entity(850),
            entity(851),
            entity(852),
            entity(853),
            entity(854),
            entity(855),
            entity(856),
            entity(857),
            entity(858),
        ];

        let all_faces = vec![
            WorthTopologyFace {
                entity_id: face_ids[0],
                label: "patch-f0".into(),
                shell_id: Some(shell_id),
                outer_loop_id: Some(loop_ids[0]),
                inner_loop_ids: vec![],
                boundary_half_edge_ids: vec![half_edge_ids[0], half_edge_ids[1], half_edge_ids[2]],
            },
            WorthTopologyFace {
                entity_id: face_ids[1],
                label: "patch-f1".into(),
                shell_id: Some(shell_id),
                outer_loop_id: Some(loop_ids[1]),
                inner_loop_ids: vec![],
                boundary_half_edge_ids: vec![half_edge_ids[3], half_edge_ids[4], half_edge_ids[5]],
            },
            WorthTopologyFace {
                entity_id: face_ids[2],
                label: "patch-f2".into(),
                shell_id: Some(shell_id),
                outer_loop_id: Some(loop_ids[2]),
                inner_loop_ids: vec![],
                boundary_half_edge_ids: vec![half_edge_ids[6], half_edge_ids[7], half_edge_ids[8]],
            },
        ];
        let all_loops = vec![
            WorthTopologyLoop {
                entity_id: loop_ids[0],
                label: "patch-l0".into(),
                face_ids: vec![face_ids[0]],
                half_edge_ids: vec![half_edge_ids[0], half_edge_ids[1], half_edge_ids[2]],
            },
            WorthTopologyLoop {
                entity_id: loop_ids[1],
                label: "patch-l1".into(),
                face_ids: vec![face_ids[1]],
                half_edge_ids: vec![half_edge_ids[3], half_edge_ids[4], half_edge_ids[5]],
            },
            WorthTopologyLoop {
                entity_id: loop_ids[2],
                label: "patch-l2".into(),
                face_ids: vec![face_ids[2]],
                half_edge_ids: vec![half_edge_ids[6], half_edge_ids[7], half_edge_ids[8]],
            },
        ];
        let all_half_edges = vec![
            half_edge_full(
                half_edge_ids[0],
                Some(loop_ids[0]),
                None,
                Some(half_edge_ids[1]),
                Some(half_edge_ids[2]),
                Some(half_edge_ids[0]),
                Some(edge_ids[0]),
                Some(vertex_ids[0]),
                Some(vertex_ids[1]),
                Some(face_ids[0]),
            ),
            half_edge_full(
                half_edge_ids[1],
                Some(loop_ids[0]),
                None,
                Some(half_edge_ids[2]),
                Some(half_edge_ids[0]),
                Some(half_edge_ids[5]),
                Some(edge_ids[1]),
                Some(vertex_ids[1]),
                Some(vertex_ids[4]),
                Some(face_ids[0]),
            ),
            half_edge_full(
                half_edge_ids[2],
                Some(loop_ids[0]),
                None,
                Some(half_edge_ids[0]),
                Some(half_edge_ids[1]),
                Some(half_edge_ids[2]),
                Some(edge_ids[2]),
                Some(vertex_ids[4]),
                Some(vertex_ids[0]),
                Some(face_ids[0]),
            ),
            half_edge_full(
                half_edge_ids[3],
                Some(loop_ids[1]),
                None,
                Some(half_edge_ids[4]),
                Some(half_edge_ids[5]),
                Some(half_edge_ids[3]),
                Some(edge_ids[3]),
                Some(vertex_ids[1]),
                Some(vertex_ids[2]),
                Some(face_ids[1]),
            ),
            half_edge_full(
                half_edge_ids[4],
                Some(loop_ids[1]),
                None,
                Some(half_edge_ids[5]),
                Some(half_edge_ids[3]),
                Some(half_edge_ids[8]),
                Some(edge_ids[4]),
                Some(vertex_ids[2]),
                Some(vertex_ids[4]),
                Some(face_ids[1]),
            ),
            half_edge_full(
                half_edge_ids[5],
                Some(loop_ids[1]),
                None,
                Some(half_edge_ids[3]),
                Some(half_edge_ids[4]),
                Some(half_edge_ids[1]),
                Some(edge_ids[1]),
                Some(vertex_ids[4]),
                Some(vertex_ids[1]),
                Some(face_ids[1]),
            ),
            half_edge_full(
                half_edge_ids[6],
                Some(loop_ids[2]),
                None,
                Some(half_edge_ids[7]),
                Some(half_edge_ids[8]),
                Some(half_edge_ids[6]),
                Some(edge_ids[5]),
                Some(vertex_ids[2]),
                Some(vertex_ids[3]),
                Some(face_ids[2]),
            ),
            half_edge_full(
                half_edge_ids[7],
                Some(loop_ids[2]),
                None,
                Some(half_edge_ids[8]),
                Some(half_edge_ids[6]),
                Some(half_edge_ids[7]),
                Some(edge_ids[6]),
                Some(vertex_ids[3]),
                Some(vertex_ids[4]),
                Some(face_ids[2]),
            ),
            half_edge_full(
                half_edge_ids[8],
                Some(loop_ids[2]),
                None,
                Some(half_edge_ids[6]),
                Some(half_edge_ids[7]),
                Some(half_edge_ids[4]),
                Some(edge_ids[4]),
                Some(vertex_ids[4]),
                Some(vertex_ids[2]),
                Some(face_ids[2]),
            ),
        ];
        let all_edges = vec![
            edge("patch-e0", edge_ids[0]),
            edge("patch-e1", edge_ids[1]),
            edge("patch-e2", edge_ids[2]),
            edge("patch-e3", edge_ids[3]),
            edge("patch-e4", edge_ids[4]),
            edge("patch-e5", edge_ids[5]),
            edge("patch-e6", edge_ids[6]),
        ];
        let vertices = vertex_ids
            .iter()
            .enumerate()
            .map(|(index, entity_id)| vertex(&format!("patch-v{index}"), *entity_id))
            .collect::<Vec<_>>();

        WorthTopologyView {
            models: vec![WorthTopologyModel {
                entity_id: model_id,
                label: "model".into(),
                body_ids: vec![body_id],
            }],
            bodies: vec![WorthTopologyBody {
                entity_id: body_id,
                label: "body".into(),
                model_id: Some(model_id),
                lump_ids: vec![lump_id],
            }],
            lumps: vec![WorthTopologyLump {
                entity_id: lump_id,
                label: "lump".into(),
                body_id: Some(body_id),
                region_ids: vec![region_id],
            }],
            regions: vec![WorthTopologyRegion {
                entity_id: region_id,
                label: "region".into(),
                lump_id: Some(lump_id),
                shell_ids: vec![shell_id],
            }],
            shells: vec![WorthTopologyShell {
                entity_id: shell_id,
                label: "sheet-patch".into(),
                region_id: Some(region_id),
                face_ids: face_ids[..face_count].to_vec(),
            }],
            faces: all_faces[..face_count].to_vec(),
            loops: all_loops[..face_count].to_vec(),
            half_edges: all_half_edges[..(face_count * 3)].to_vec(),
            edges: all_edges,
            vertices,
            ..WorthTopologyView::default()
        }
    }

    fn entity(slot: u64) -> EntityId {
        EntityId::new(PartitionId::main(), slot, 1)
    }

    fn edge(label: &str, entity_id: EntityId) -> WorthTopologyEdge {
        WorthTopologyEdge {
            entity_id,
            label: label.into(),
        }
    }

    fn vertex(label: &str, entity_id: EntityId) -> WorthTopologyVertex {
        WorthTopologyVertex {
            entity_id,
            label: label.into(),
        }
    }

    fn half_edge_full(
        entity_id: EntityId,
        loop_id: Option<EntityId>,
        wire_id: Option<EntityId>,
        next_half_edge_id: Option<EntityId>,
        prev_half_edge_id: Option<EntityId>,
        radial_next_half_edge_id: Option<EntityId>,
        edge_id: Option<EntityId>,
        origin_vertex_id: Option<EntityId>,
        target_vertex_id: Option<EntityId>,
        face_id: Option<EntityId>,
    ) -> WorthTopologyHalfEdge {
        WorthTopologyHalfEdge {
            entity_id,
            label: "he".into(),
            loop_id,
            wire_id,
            next_half_edge_id,
            prev_half_edge_id,
            radial_next_half_edge_id,
            edge_id,
            origin_vertex_id,
            target_vertex_id,
            face_id,
        }
    }
}
