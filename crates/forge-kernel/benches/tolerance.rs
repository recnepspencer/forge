use criterion::{black_box, criterion_group, criterion_main, Criterion};

use forge_kernel::geometry_store::GeometryStore;
use forge_topo::arena::{
    EdgeData, FaceData, HalfEdgeData, LoopData, ShellData, ShellKind, ShellOrientation, VertexData,
};
use forge_topo::handles::HalfEdgeId;
use forge_topo::handles::{EdgeId, FaceId, LoopId, ShellId, VertexId};
use forge_topo::state::TopologyState;

/// Benchmarks the evaluation of `compute_model_scale` dynamically on a GeometryStore
/// populated with representative numbers of vertices.
fn bench_compute_model_scale(c: &mut Criterion) {
    let mut group = c.benchmark_group("compute_model_scale");

    for size in [10, 100, 1_000, 10_000].iter() {
        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            size,
            |b, &s| {
                let mut geom = GeometryStore::new();
                let mut topo = TopologyState::empty().into_mutation();

                // Populate some fake vertices spread out across a bounding box
                let placeholder_he = HalfEdgeId::from_raw_parts(u32::MAX, 0);

                for i in 0..s {
                    let v_id = topo.insert_vertex(VertexData::new(placeholder_he));
                    let x = (i as f64) * 0.1;
                    let y = (i as f64) * 0.2;
                    let z = (i as f64) * 0.3;
                    geom.set_vertex_position(v_id, [x, y, z]);
                }

                let arena = topo.commit().unwrap();

                b.iter(|| {
                    black_box(geom.compute_model_scale());
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_compute_model_scale);
criterion_main!(benches);
