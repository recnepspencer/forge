use super::super::schema::{BooleanInput, BooleanOp};
use super::super::test_helpers::{build_cube, execute_boolean_logged, run_boolean};

// ══════════════════════════════════════════════════════════════
// §4  TOPOLOGY INTEGRITY TORTURE
// ══════════════════════════════════════════════════════════════

/// 4.1 — Edge Split Storm
///
/// Build a cube and verify Euler characteristic.
#[test]
fn edge_split_storm() {
    let (topo, _geom) = build_cube([0.0, 0.0, 0.0], 1.0);
    let arena = topo.arena();

    let v = arena.vertex_count() as isize;
    let e = (arena.half_edge_count() / 2) as isize;
    let f = arena.face_count() as isize;
    let euler = v - e + f;

    assert_eq!(
        euler, 2,
        "Cube should satisfy Euler: V-E+F=2, got V={v} E={e} F={f} Euler={euler}"
    );
}

/// 4.2 — Boolean round-trip validity
#[test]
fn boolean_round_trip_validity() {
    let result = run_boolean([0.0, 0.0, 0.0], 2.0, [1.5, 0.0, 0.0], 2.0, BooleanOp::Union);

    let arena = result.topology().arena();
    let v = arena.vertex_count() as isize;
    let e = (arena.half_edge_count() / 2) as isize;
    let f = arena.face_count() as isize;

    assert_eq!(
        v - e + f,
        2,
        "Boolean result violates Euler: V={v} E={e} F={f}"
    );

    for (_he_id, he) in arena.iter_half_edges() {
        let twin = arena.get_half_edge(he.radial_next());
        assert!(twin.is_ok(), "Orphan halfedge");
    }
}

/// 4.3a — Fresh cube intersection works
///
/// Two overlapping cubes, intersection. No chaining.
#[test]
fn fresh_cube_intersection() {
    let result = run_boolean(
        [0.0, 0.0, 0.0],
        2.0,
        [0.5, 0.5, 0.0],
        2.0,
        BooleanOp::Intersection,
    );

    let arena = result.topology().arena();
    let v = arena.vertex_count() as isize;
    let e = (arena.half_edge_count() / 2) as isize;
    let f = arena.face_count() as isize;
    assert_eq!(
        v - e + f,
        2,
        "Intersection Euler violation: V={v} E={e} F={f}"
    );
}

/// 4.3b — Union result shape audit
///
/// Overlapping cubes union: verify output counts and Euler.
#[test]
fn union_result_shape_audit() {
    let result = run_boolean([0.0, 0.0, 0.0], 1.5, [1.0, 0.0, 0.0], 1.5, BooleanOp::Union);

    let (topo, _geom, _) = result.into_states();
    let arena = topo.arena();
    let v = arena.vertex_count() as isize;
    let e = (arena.half_edge_count() / 2) as isize;
    let f = arena.face_count() as isize;
    let euler = v - e + f;

    eprintln!("=== UNION RESULT SHAPE ===");
    eprintln!("  V={v} E={e} F={f} Euler={euler}");

    assert_eq!(euler, 2, "Union Euler violation: V={v} E={e} F={f}");
}

/// 4.3c — Chained Union→Union (simpler than Union→Intersection)
#[test]
fn chained_union_union() {
    let result_ab = run_boolean([0.0, 0.0, 0.0], 1.5, [1.0, 0.0, 0.0], 1.5, BooleanOp::Union);
    let (topo_ab, geom_ab, _) = result_ab.into_states();
    let (topo_c, geom_c) = build_cube([0.5, 0.5, 0.0], 1.0);

    let input = BooleanInput::new(
        topo_ab,
        geom_ab,
        BrepState::new(),
        topo_c,
        geom_c,
        BrepState::new(),
        BooleanOp::Union,
    );
    match execute_boolean_logged(input).into_result() {
        Ok(r) => {
            let arena = r.topology().arena();
            let v = arena.vertex_count() as isize;
            let e = (arena.half_edge_count() / 2) as isize;
            let f = arena.face_count() as isize;
            assert_eq!(
                v - e + f,
                2,
                "Chained Union→Union Euler violation: V={v} E={e} F={f}"
            );
        }
        Err(e) => panic!("Chained Union→Union must not fail: {e:?}"),
    }
}

/// 4.3d — Vertex provenance diagnosis
///
/// Check how many vertices of the A∪B result get 3-plane provenance.
/// If < 8 get provenance, cross-solid dedup will fail.
#[test]
fn vertex_provenance_audit() {
    use std::collections::{HashMap, HashSet};

    let result_ab = run_boolean([0.0, 0.0, 0.0], 1.5, [1.0, 0.0, 0.0], 1.5, BooleanOp::Union);
    let (topo_ab, geom_ab, _) = result_ab.into_states();
    let arena = topo_ab.arena();

    // Build face-plane index map (replicate what split_all_faces does)
    // Use unique plane indices by comparing normals/offsets
    let mut unique_planes: Vec<(crate::geom_facade::Plane, usize)> = Vec::new();
    let mut face_planes: HashMap<forge_topo::handles::FaceId, usize> = HashMap::new();

    for (fid, _) in arena.iter_faces() {
        if let Some(p) = geom_ab.get_face_plane(fid) {
            let idx = unique_planes
                .iter()
                .find_map(|(existing, idx)| {
                    if crate::geom_facade::coplanar_eq(existing, p) {
                        Some(*idx)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| {
                    let idx = unique_planes.len();
                    unique_planes.push((p.clone(), idx));
                    idx
                });
            face_planes.insert(fid, idx);
        }
    }

    eprintln!("=== VERTEX PROVENANCE AUDIT ===");
    eprintln!(
        "  {} faces, {} vertices, {} unique planes",
        arena.face_count(),
        arena.vertex_count(),
        unique_planes.len()
    );

    for (fid, pidx) in &face_planes {
        let plane = geom_ab.get_face_plane(*fid).unwrap();
        let n = plane.normal();
        eprintln!(
            "  F#{} -> plane {} normal=[{:.2}, {:.2}, {:.2}]",
            fid.index(),
            pidx,
            n[0],
            n[1],
            n[2]
        );
    }

    // For each vertex, walk its fan and collect adjacent face-plane indices
    let mut vertices_with_provenance = 0;
    let mut vertices_without = Vec::new();

    for (vid, _) in arena.iter_vertices() {
        let mut plane_set = HashSet::new();
        let start_he = arena.get_vertex(vid).unwrap().outgoing();
        let mut current = start_he;
        let mut faces_seen = Vec::new();

        for _ in 0..100 {
            let he_data = arena.get_half_edge(current).unwrap();
            let face = he_data.face();
            if let Some(&pidx) = face_planes.get(&face) {
                plane_set.insert(pidx);
                faces_seen.push(format!("F#{}", face.index()));
            }
            let twin = he_data.radial_next();
            let twin_data = arena.get_half_edge(twin).unwrap();
            current = twin_data.next();
            if current == start_he {
                break;
            }
        }

        let pos = geom_ab.get_vertex_position(vid).unwrap();
        if plane_set.len() >= 3 {
            vertices_with_provenance += 1;
            eprintln!(
                "  V#{}: [{:.2}, {:.2}, {:.2}] -> {} planes (faces: {}) ✓",
                vid.index(),
                pos[0],
                pos[1],
                pos[2],
                plane_set.len(),
                faces_seen.join(",")
            );
        } else {
            vertices_without.push(vid);
            eprintln!(
                "  V#{}: [{:.2}, {:.2}, {:.2}] -> {} planes (faces: {}) ✗ NO PROVENANCE",
                vid.index(),
                pos[0],
                pos[1],
                pos[2],
                plane_set.len(),
                faces_seen.join(",")
            );
        }
    }

    eprintln!(
        "  {}/{} vertices have provenance",
        vertices_with_provenance,
        arena.vertex_count()
    );
    // Don't assert — just diagnose. The fix target is the stitch, not provenance.
}

/// 4.3e — Split+classify audit for boolean result ∩ C
///
/// Manually runs split/classify/select to see which faces each solid has.
#[test]
fn split_classify_audit() {
    use crate::core::ModelingContext;
    use crate::geometry_state::GeometryState;
    use crate::operations::boolean::classify_schema::{FaceClassification, FaceOrigin};
    use crate::operations::boolean::_deprecated::parametric::assemble::select::select_faces;
    use crate::operations::shared_steps::classify_faces::classify_faces;
    use crate::operations::boolean::_deprecated::parametric::split::split_all_faces;

    let result_ab = run_boolean([0.0, 0.0, 0.0], 1.5, [1.0, 0.0, 0.0], 1.5, BooleanOp::Union);
    let (topo_ab, geom_ab, _) = result_ab.into_states();
    let (topo_c, geom_c) = build_cube([0.5, 0.5, 0.0], 1.0);

    let mut ctx = ModelingContext::default();

    let split_result =
        split_all_faces(topo_ab, geom_ab, topo_c, geom_c, &mut ctx).expect("split must succeed");

    let split_count = split_result.split_count();
    let (st, sg, tt, tg, sp, tp) = split_result.into_parts();

    eprintln!("=== SPLIT+CLASSIFY AUDIT ===");
    eprintln!("  Split count: {}", split_count);
    eprintln!(
        "  Target (A∪B) after split: V={} E={} F={}",
        st.arena().vertex_count(),
        st.arena().half_edge_count() / 2,
        st.arena().face_count()
    );
    eprintln!(
        "  Tool (C) after split: V={} E={} F={}",
        tt.arena().vertex_count(),
        tt.arena().half_edge_count() / 2,
        tt.arena().face_count()
    );

    // Print all face planes after splitting
    eprintln!("--- Target faces after split ---");
    for (fid, _) in st.arena().iter_faces() {
        if let Some(plane) = sg.get_face_plane(fid) {
            let n = plane.normal();
            eprintln!(
                "  F#{}: n=[{:.2},{:.2},{:.2}] d={:.2}",
                fid.index(),
                n[0],
                n[1],
                n[2],
                plane.offset()
            );
        }
    }
    eprintln!("--- Tool faces after split ---");
    for (fid, _) in tt.arena().iter_faces() {
        if let Some(plane) = tg.get_face_plane(fid) {
            let n = plane.normal();
            eprintln!(
                "  F#{}: n=[{:.2},{:.2},{:.2}] d={:.2}",
                fid.index(),
                n[0],
                n[1],
                n[2],
                plane.offset()
            );
        }
    }

    // Classify
    let tc = classify_faces(
        st.arena(),
        &sg,
        tt.arena(),
        &tg,
        FaceOrigin::Target,
        &mut ctx,
    )
    .expect("classify target");
    let tlc = classify_faces(tt.arena(), &tg, st.arena(), &sg, FaceOrigin::Tool, &mut ctx)
        .expect("classify tool");

    eprintln!("--- Target classification ---");
    for cf in &tc {
        eprintln!("  F#{}: {:?}", cf.face().index(), cf.classification());
    }
    eprintln!("--- Tool classification ---");
    for cf in &tlc {
        eprintln!("  F#{}: {:?}", cf.face().index(), cf.classification());
    }

    // Select for intersection
    let sel_t = select_faces(&tc, FaceOrigin::Target, BooleanOp::Intersection, &mut ctx);
    let sel_tl = select_faces(&tlc, FaceOrigin::Tool, BooleanOp::Intersection, &mut ctx);

    eprintln!(
        "--- Selected target faces for Intersection: {} ---",
        sel_t.len()
    );
    for fid in &sel_t {
        eprintln!("  F#{}", fid.index());
    }
    eprintln!(
        "--- Selected tool faces for Intersection: {} ---",
        sel_tl.len()
    );
    for fid in &sel_tl {
        eprintln!("  F#{}", fid.index());
    }

    // Print vertex positions for all selected faces (both solids)
    eprintln!("--- Target selected face vertices ---");
    for &fid in &sel_t {
        let edges: Vec<_> = forge_topo::traverse::FaceEdgeIterator::new(st.arena(), fid)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let verts: Vec<_> = edges
            .iter()
            .map(|&he| {
                let vid = st.arena().get_half_edge(he).unwrap().origin();
                let pos = sg.get_vertex_position(vid).unwrap();
                format!(
                    "V{}[{:.2},{:.2},{:.2}]",
                    vid.index(),
                    pos[0],
                    pos[1],
                    pos[2]
                )
            })
            .collect();
        eprintln!("  F#{}: {}", fid.index(), verts.join(" → "));
    }
    eprintln!("--- Tool selected face vertices ---");
    for &fid in &sel_tl {
        let edges: Vec<_> = forge_topo::traverse::FaceEdgeIterator::new(tt.arena(), fid)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let verts: Vec<_> = edges
            .iter()
            .map(|&he| {
                let vid = tt.arena().get_half_edge(he).unwrap().origin();
                let pos = tg.get_vertex_position(vid).unwrap();
                format!(
                    "V{}[{:.2},{:.2},{:.2}]",
                    vid.index(),
                    pos[0],
                    pos[1],
                    pos[2]
                )
            })
            .collect();
        eprintln!("  F#{}: {}", fid.index(), verts.join(" → "));
    }
}

/// 4.3f — Fresh cube ∩ C with same dimensions as chained test
///
/// If this passes but chained fails, the issue is with post-boolean topology.
#[test]
fn fresh_cube_intersection_same_dims() {
    // A∪B result is effectively [-0.75, 1.75] x [-0.75, 0.75] x [-0.75, 0.75]
    // Approximate as a ~2.5-width cube centered at [0.5, 0, 0]
    let (topo_ab, geom_ab) = build_cube([0.5, 0.0, 0.0], 2.5);
    let (topo_c, geom_c) = build_cube([0.5, 0.5, 0.0], 1.0);

    let input = BooleanInput::new(
        topo_ab,
        geom_ab,
        BrepState::new(),
        topo_c,
        geom_c,
        BrepState::new(),
        BooleanOp::Intersection,
    );
    match execute_boolean_logged(input).into_result() {
        Ok(r) => {
            let arena = r.topology().arena();
            let v = arena.vertex_count() as isize;
            let e = (arena.half_edge_count() / 2) as isize;
            let f = arena.face_count() as isize;
            eprintln!("fresh_cube_intersection_same_dims: V={v} E={e} F={f}");
            assert_eq!(
                v - e + f,
                2,
                "Fresh cube ∩ C Euler violation: V={v} E={e} F={f}"
            );
        }
        Err(e) => panic!("Fresh cube ∩ C must not fail: {e:?}"),
    }
}

/// 4.3e — Compare: boolean result directly used as topo input
///
/// Take actual A∪B result (with deleted slots) and intersect with C.
/// If fresh cube works but this doesn't, it's the deleted-slot arena.
#[test]
fn boolean_result_intersection() {
    let result_ab = run_boolean([0.0, 0.0, 0.0], 1.5, [1.0, 0.0, 0.0], 1.5, BooleanOp::Union);
    let (topo_ab, geom_ab, _) = result_ab.into_states();

    let arena = topo_ab.arena();
    eprintln!("=== BOOLEAN RESULT ARENA FOR INTERSECTION ===");
    eprintln!(
        "  HE: {}/{} active/slots",
        arena.half_edge_count(),
        arena.half_edge_slot_count()
    );
    eprintln!(
        "  V:  {}/{} active/slots",
        arena.vertex_count(),
        arena.vertex_slot_count()
    );
    eprintln!(
        "  F:  {}/{} active/slots",
        arena.face_count(),
        arena.face_slot_count()
    );

    // Print vertex positions
    for (vid, _vdata) in arena.iter_vertices() {
        if let Some(pos) = geom_ab.get_vertex_position(vid) {
            eprintln!(
                "  V#{}: [{:.4}, {:.4}, {:.4}]",
                vid.index(),
                pos[0],
                pos[1],
                pos[2]
            );
        }
    }

    // Print face planes
    for (fid, _) in arena.iter_faces() {
        if let Some(plane) = geom_ab.get_face_plane(fid) {
            let n = plane.normal();
            eprintln!(
                "  F#{}: normal=[{:.4}, {:.4}, {:.4}] offset={:.4}",
                fid.index(),
                n[0],
                n[1],
                n[2],
                plane.offset()
            );
        }
    }

    let (topo_c, geom_c) = build_cube([0.5, 0.5, 0.0], 1.0);

    let input = BooleanInput::new(
        topo_ab,
        geom_ab,
        BrepState::new(),
        topo_c,
        geom_c,
        BrepState::new(),
        BooleanOp::Intersection,
    );
    match execute_boolean_logged(input).into_result() {
        Ok(r) => {
            let arena = r.topology().arena();
            let v = arena.vertex_count() as isize;
            let e = (arena.half_edge_count() / 2) as isize;
            let f = arena.face_count() as isize;
            eprintln!("boolean_result_intersection: V={v} E={e} F={f}");
            assert_eq!(
                v - e + f,
                2,
                "Boolean result ∩ C Euler violation: V={v} E={e} F={f}"
            );
        }
        Err(e) => panic!("Boolean result ∩ C failed: {e:?}"),
    }
}

/// 4.3f — Chained Union→Intersection (the original failing case)
///
/// Chain: A ∪ B, then result ∩ C.
#[test]
fn chained_booleans_preserve_euler() {
    let result_ab = run_boolean([0.0, 0.0, 0.0], 1.5, [1.0, 0.0, 0.0], 1.5, BooleanOp::Union);

    let (topo_ab, geom_ab, _) = result_ab.into_states();

    // ═══════════════════════════════════════════════════════════════
    // EXHAUSTIVE HANDLE AUDIT of first boolean result
    // ═══════════════════════════════════════════════════════════════
    let arena = topo_ab.arena();
    eprintln!("=== EXHAUSTIVE HANDLE AUDIT ===");
    eprintln!(
        "  HE: {}/{} active/slots",
        arena.half_edge_count(),
        arena.half_edge_slot_count()
    );
    eprintln!(
        "  V:  {}/{} active/slots",
        arena.vertex_count(),
        arena.vertex_slot_count()
    );
    eprintln!(
        "  F:  {}/{} active/slots",
        arena.face_count(),
        arena.face_slot_count()
    );
    eprintln!("  L:  {} active", arena.loop_count());

    // Print every active HE with full pointer details
    let mut stale_count = 0;
    for (he_id, he_data) in arena.iter_half_edges() {
        let twin = he_data.radial_next();
        let next = he_data.next();
        let prev = he_data.prev();
        let origin = he_data.origin();
        let face = he_data.face();

        let twin_ok = arena.get_half_edge(twin).is_ok();
        let next_ok = arena.get_half_edge(next).is_ok();
        let prev_ok = arena.get_half_edge(prev).is_ok();
        let origin_ok = arena.get_vertex(origin).is_ok();
        let face_ok = arena.get_face(face).is_ok();

        if !twin_ok || !next_ok || !prev_ok || !origin_ok || !face_ok {
            stale_count += 1;
            eprintln!("  STALE HE#{}(gen{}): twin={}(gen{}):{} next={}(gen{}):{} prev={}(gen{}):{} origin=V{}(gen{}):{} face=F{}(gen{}):{}", 
                he_id.index(), he_id.generation(),
                twin.index(), twin.generation(), if twin_ok {"OK"} else {"BAD"},
                next.index(), next.generation(), if next_ok {"OK"} else {"BAD"},
                prev.index(), prev.generation(), if prev_ok {"OK"} else {"BAD"},
                origin.index(), origin.generation(), if origin_ok {"OK"} else {"BAD"},
                face.index(), face.generation(), if face_ok {"OK"} else {"BAD"},
            );
        }
    }
    eprintln!("  Total stale pointers in active HEs: {}", stale_count);

    // Check every face's loop pointer
    for (fid, fdata) in arena.iter_faces() {
        let loop_id = fdata.outer_loop();
        if arena.get_loop(loop_id).is_err() {
            eprintln!(
                "  STALE Face F#{}(gen{}) -> Loop L#{}(gen{}): BAD",
                fid.index(),
                fid.generation(),
                loop_id.index(),
                loop_id.generation()
            );
            stale_count += 1;
        }
    }

    // Check every loop's HE pointer
    for (lid, ldata) in arena.iter_loops() {
        let he = ldata.half_edge();
        if arena.get_half_edge(he).is_err() {
            eprintln!(
                "  STALE Loop L#{}(gen{}) -> HE#{}(gen{}): BAD",
                lid.index(),
                lid.generation(),
                he.index(),
                he.generation()
            );
            stale_count += 1;
        }
    }

    // Check every vertex's outgoing pointer
    for (vid, vdata) in arena.iter_vertices() {
        let out = vdata.outgoing();
        if arena.get_half_edge(out).is_err() {
            eprintln!(
                "  STALE Vertex V#{}(gen{}) -> HE#{}(gen{}): BAD",
                vid.index(),
                vid.generation(),
                out.index(),
                out.generation()
            );
            stale_count += 1;
        }
    }

    assert_eq!(
        stale_count, 0,
        "First boolean result contains {} stale handles!",
        stale_count
    );

    let (topo_c, geom_c) = build_cube([0.5, 0.5, 0.0], 1.0);

    let input = BooleanInput::new(
        topo_ab,
        geom_ab,
        BrepState::new(),
        topo_c,
        geom_c,
        BrepState::new(),
        BooleanOp::Intersection,
    );
    let result = execute_boolean_logged(input);

    match result.into_result() {
        Ok(r) => {
            let arena = r.topology().arena();
            let v = arena.vertex_count() as isize;
            let e = (arena.half_edge_count() / 2) as isize;
            let f = arena.face_count() as isize;
            let euler = v - e + f;
            assert_eq!(
                euler, 2,
                "Chained boolean Euler violation: V={v} E={e} F={f}"
            );
        }
        Err(e) => {
            panic!("Chained boolean must not fail: {e:?}");
        }
    }
}

/// 4.4 — Random Edge Split Storm
///
/// Build a cube, split edges, verify Euler holds after each split.
#[test]
fn random_edge_split_storm() {
    use forge_topo::euler::split_edge::SplitEdge;
    use forge_topo::lineage::OpSignature;
    use forge_topo::operator::EulerOperator;

    let (topo, _geom) = build_cube([0.0, 0.0, 0.0], 1.0);
    // Use clone to test iteration while keeping topo for reference if needed (though here it isn't used after).
    // Actually, into_mutation() is fine here as topo is not used after.
    let mut draft = topo.clone().into_mutation();
    let sig = OpSignature::new("split_storm");

    let mut split_count = 0usize;
    let half_edges: Vec<_> = topo.arena().iter_half_edges().map(|(id, _)| id).collect();

    for &he_id in half_edges.iter().take(24) {
        let op = SplitEdge {
            edge: he_id,
            parameter: 0.5,
        };
        let result = op.execute(&mut draft, &sig);
        if result.is_ok() {
            split_count += 1;

            let arena = draft.arena();
            let v = arena.vertex_count() as isize;
            let e = (arena.half_edge_count() / 2) as isize;
            let f = arena.face_count() as isize;
            let euler = v - e + f;
            assert_eq!(
                euler, 2,
                "Euler violation after {split_count} splits: V={v} E={e} F={f}"
            );
        }
    }

    assert!(split_count > 0, "Should have split at least one edge");
}

/// 4.5 — Operation Permutation Hash Equality
///
/// Union(A,B) then Union(result,C) vs Union(B,C) then Union(A,result)
/// should produce the same vertex/edge/face counts for associative operations.
#[test]
fn operation_permutation_counts() {
    let result_ab = run_boolean([0.0, 0.0, 0.0], 1.0, [0.5, 0.0, 0.0], 1.0, BooleanOp::Union);
    let (topo_ab, geom_ab, _) = result_ab.into_states();
    let (topo_c, geom_c) = build_cube([1.0, 0.0, 0.0], 1.0);

    let input_abc = BooleanInput::new(
        topo_ab,
        geom_ab,
        BrepState::new(),
        topo_c,
        geom_c,
        BrepState::new(),
        BooleanOp::Union,
    );
    let result_abc = execute_boolean_logged(input_abc)
        .into_result()
        .expect("(A∪B)∪C must not fail");

    let result_bc = run_boolean([0.5, 0.0, 0.0], 1.0, [1.0, 0.0, 0.0], 1.0, BooleanOp::Union);
    let (topo_bc, geom_bc, _) = result_bc.into_states();
    let (topo_a2, geom_a2) = build_cube([0.0, 0.0, 0.0], 1.0);

    let input_a_bc = BooleanInput::new(
        topo_a2,
        geom_a2,
        BrepState::new(),
        topo_bc,
        geom_bc,
        BrepState::new(),
        BooleanOp::Union,
    );
    let result_a_bc = execute_boolean_logged(input_a_bc)
        .into_result()
        .expect("A∪(B∪C) must not fail");

    assert_eq!(
        result_abc.topology().arena().face_count(),
        result_a_bc.topology().arena().face_count(),
        "(A∪B)∪C vs A∪(B∪C) face count mismatch"
    );
    assert_eq!(
        result_abc.topology().arena().vertex_count(),
        result_a_bc.topology().arena().vertex_count(),
        "(A∪B)∪C vs A∪(B∪C) vertex count mismatch"
    );
}
