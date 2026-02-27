use criterion::{black_box, criterion_group, criterion_main, Criterion};

use forge_kernel::mesh_builder::make_cube;
use forge_kernel::operations::boolean::eval::build_face_coincidence_prepass;

/// Benchmarks the BVH-accelerated face coincidence prepass between two bounding
/// volumes modeled by convex cubes.
fn bench_face_coincidence_prepass(c: &mut Criterion) {
    let mut group = c.benchmark_group("coincidence_graph_prepass");

    // Two exactly overlapping cubes (maximum intersection workload)
    group.bench_function("sharing_faces", |b| {
        let (topo_a, geom_a) = make_cube([0.0, 0.0, 0.0], 10.0).unwrap().into_parts();
        let (topo_b, geom_b) = make_cube([10.0, 0.0, 0.0], 10.0).unwrap().into_parts();

        let target_arena = topo_a.arena();
        let tool_arena = topo_b.arena();

        b.iter(|| {
            black_box(build_face_coincidence_prepass(
                target_arena,
                &geom_a,
                tool_arena,
                &geom_b,
            ));
        });
    });

    // Two distant cubes (best case O(1) BVH exit)
    group.bench_function("distant_faces", |b| {
        let (topo_a, geom_a) = make_cube([0.0, 0.0, 0.0], 10.0).unwrap().into_parts();
        let (topo_b, geom_b) = make_cube([100.0, 0.0, 0.0], 10.0).unwrap().into_parts();

        let target_arena = topo_a.arena();
        let tool_arena = topo_b.arena();

        b.iter(|| {
            black_box(build_face_coincidence_prepass(
                target_arena,
                &geom_a,
                tool_arena,
                &geom_b,
            ));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_face_coincidence_prepass);
criterion_main!(benches);
