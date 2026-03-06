//! Diagnostic tests that produce ACTIONABLE output.

#[cfg(test)]
mod tests {
    use crate::b_rep::ShellKind;
    use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
    use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
    use crate::entity_lifecycle::split_edge::SplitEdge;
    use crate::operations::tests::invariant_checker::{diagnose_op_chain, dump_all_wiring};
    use crate::traverse::FaceEdgeIterator;

    /// Trace every step of the pole fan pattern and print EXACTLY
    /// what vertex is on what face, what edge connects what, and
    /// where the vertex orbit breaks.
    #[test]
    fn diagnose_pole_fan() {
        let report = diagnose_op_chain(|runner| {
            let mvf = runner.run("MVF", |d| {
                d.execute(MakeVertexFace {
                    shell_kind: ShellKind::Sheet,
                })
                .map(|r| r.into_value())
            })?;
            let pole = mvf.vertex;
            let mut current_edge = mvf.half_edge;

            for i in 0..3 {
                let se = runner.run(&format!("SE[{}]", i), |d| {
                    d.execute(SplitEdge { edge: current_edge })
                        .map(|r| r.into_value())
                })?;
                let face_id = runner.arena().get_half_edge(current_edge).unwrap().face();

                // Print vertex→face map BEFORE MEF
                print_vertex_face_map(runner.arena(), &format!("Before MEF[{}]", i));

                let mef = runner.run(
                    &format!("MEF[{}](V{}, V{})", i, pole.index(), se.new_vertex.index()),
                    |d| {
                        d.execute(MakeEdgeFace {
                            vertex_a: pole,
                            vertex_b: se.new_vertex,
                            face: face_id,
                        })
                        .map(|r| r.into_value())
                    },
                )?;

                // Print vertex→face map AFTER MEF
                print_vertex_face_map(runner.arena(), &format!("After MEF[{}]", i));
                println!(
                    "  mef.half_edge_ab=HE{} on F{}\n",
                    mef.half_edge_ab.index(),
                    runner
                        .arena()
                        .get_half_edge(mef.half_edge_ab)
                        .unwrap()
                        .face()
                        .index()
                );

                current_edge = mef.half_edge_ab;
            }
            Ok(())
        });

        println!("\n{}", report.summary());
    }

    fn print_vertex_face_map(arena: &crate::b_rep::TopologyArena, label: &str) {
        println!("  === {} ===", label);

        // For each vertex, list which faces it appears on
        for (vid, vdata) in arena.iter_vertices() {
            let mut faces_seen = std::collections::BTreeSet::new();
            let mut boundary_count = 0u32;

            // Walk vertex orbit
            let start = vdata.primary_disk();
            let mut cur = start;
            let bound = arena.half_edge_count();
            for step in 0..=bound {
                let hd = arena.get_half_edge(cur).unwrap();
                faces_seen.insert(hd.face().index());
                if hd.radial_next() == cur {
                    boundary_count += 1;
                }
                let twin = hd.radial_next();
                cur = arena.get_half_edge(twin).unwrap().next();
                if cur == start {
                    break;
                }
                if step == bound {
                    println!("    V{}: ORBIT OVERFLOW at step {}", vid.index(), step);
                    return;
                }
            }

            let faces: Vec<_> = faces_seen.into_iter().collect();
            println!(
                "    V{}: faces={:?} boundary_edges={}",
                vid.index(),
                faces,
                boundary_count
            );
        }

        // For each edge, show vertex pair
        for (eid, edata) in arena.iter_edges() {
            let he = edata.half_edge();
            let hd = arena.get_half_edge(he).unwrap();
            let origin = hd.origin().index();
            let dest = arena.get_half_edge(hd.next()).unwrap().origin().index();
            let twin = hd.radial_next();
            let boundary = if twin == he { " [BOUNDARY]" } else { "" };
            println!("    E{}: V{}↔V{}{}", eid.index(), origin, dest, boundary);
        }
    }
}
