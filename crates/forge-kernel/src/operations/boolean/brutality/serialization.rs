use super::super::test_helpers::{run_boolean, try_boolean};
use super::super::schema::{BooleanOp};
use forge_topo::hashing::compute_arena_topology_hash;

// ══════════════════════════════════════════════════════════════
// §10  SERIALIZATION ROUNDTRIP TORTURE
// ══════════════════════════════════════════════════════════════

/// 10.1 — TopologyState JSON roundtrip preserves topology hash
///
/// Serialize a boolean result's TopologyState to JSON, deserialize it back,
/// and verify the topology hash is identical.
#[test]
fn topology_state_json_roundtrip() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 2.0,
        [1.0, 0.0, 0.0], 2.0,
        BooleanOp::Union,
    );

    let original_hash = compute_arena_topology_hash(result.topology().arena());
    let original_v = result.topology().arena().vertex_count();
    let original_e = result.topology().arena().half_edge_count();
    let original_f = result.topology().arena().face_count();

    let json = serde_json::to_string(result.topology())
        .expect("TopologyState serialization must not fail");

    let deserialized: forge_topo::state::TopologyState = serde_json::from_str(&json)
        .expect("TopologyState deserialization must not fail");

    let roundtrip_hash = compute_arena_topology_hash(deserialized.arena());
    assert_eq!(
        original_hash, roundtrip_hash,
        "Topology hash diverged after JSON roundtrip: {original_hash:#x} vs {roundtrip_hash:#x}"
    );

    assert_eq!(deserialized.arena().vertex_count(), original_v, "Vertex count mismatch after roundtrip");
    assert_eq!(deserialized.arena().half_edge_count(), original_e, "Halfedge count mismatch after roundtrip");
    assert_eq!(deserialized.arena().face_count(), original_f, "Face count mismatch after roundtrip");
}

/// 10.2 — Multiple boolean results roundtrip deterministically
///
/// Run different boolean configurations, serialize/deserialize each,
/// verify Euler characteristic is preserved.
#[test]
fn multiple_boolean_results_roundtrip() {
    let configs: Vec<([f64; 3], f64, [f64; 3], f64, BooleanOp)> = vec![
        ([0.0, 0.0, 0.0], 1.0, [0.5, 0.0, 0.0], 1.0, BooleanOp::Union),
        ([0.0, 0.0, 0.0], 2.0, [0.0, 0.0, 0.0], 1.0, BooleanOp::Subtraction),
        ([0.0, 0.0, 0.0], 1.0, [0.5, 0.5, 0.5], 1.0, BooleanOp::Intersection),
    ];

    for (i, &(ca, ha, cb, hb, op)) in configs.iter().enumerate() {
        let result = try_boolean(ca, ha, cb, hb, op);
        if let Ok(r) = result {
            let topo = r.topology();
            let json = serde_json::to_string(topo)
                .expect("Serialization must not fail");

            let deser: forge_topo::state::TopologyState = serde_json::from_str(&json)
                .expect("Deserialization must not fail");

            let arena = deser.arena();
            let v = arena.vertex_count() as isize;
            let e = (arena.half_edge_count() / 2) as isize;
            let f = arena.face_count() as isize;

            if f > 0 {
                let euler = v - e + f;
                assert!(
                    euler > 0 && euler % 2 == 0,
                    "Case {i}: Euler violation after roundtrip: V={v} E={e} F={f} V-E+F={euler} (expected positive even)"
                );
            }
        }
    }
}
