use criterion::{black_box, criterion_group, criterion_main, Criterion};

use forge_kernel::mesh_builder::make_cube;
use forge_spatial::integrity::gap::{measure_gap, GapSampleDensity};

/// Benchmarks the execution of `measure_gap` using Halton sampling with
/// various precision density presets.
fn bench_gap_measurement(c: &mut Criterion) {
    let mut group = c.benchmark_group("gap_measurement");

    let (topo_a, geom_a) = make_cube([0.0, 0.0, 0.0], 10.0).unwrap().into_parts();
    let (topo_b, geom_b) = make_cube([11.5, 0.0, 0.0], 10.0).unwrap().into_parts();

    // Extracted overlapping faces (+X from A, -X from B)
    let face_a = topo_a
        .arena()
        .iter_faces()
        .find(|(f, _)| {
            geom_a
                .get_face_plane(*f)
                .map_or(false, |p| p.normal()[0] > 0.9)
        })
        .unwrap()
        .0;

    let face_b = topo_b
        .arena()
        .iter_faces()
        .find(|(f, _)| {
            geom_b
                .get_face_plane(*f)
                .map_or(false, |p| p.normal()[0] < -0.9)
        })
        .unwrap()
        .0;

    let arena_a = topo_a.arena();
    let position_fn = |v: forge_topo::handles::VertexId| -> Option<[f64; 3]> {
        geom_a.get_vertex_position(v).copied()
    };
    let plane_fn =
        |f: forge_topo::handles::FaceId| -> Option<worth_geom::primitives::plane::Plane> {
            geom_b.get_face_plane(f).cloned()
        };

    group.bench_function("density_coarse", |b| {
        b.iter(|| {
            black_box(measure_gap(
                face_a,
                arena_a,
                face_b,
                &position_fn,
                &plane_fn,
                GapSampleDensity::Coarse,
            ));
        });
    });

    group.bench_function("density_medium", |b| {
        b.iter(|| {
            black_box(measure_gap(
                face_a,
                arena_a,
                face_b,
                &position_fn,
                &plane_fn,
                GapSampleDensity::Medium,
            ));
        });
    });

    group.bench_function("density_fine", |b| {
        b.iter(|| {
            black_box(measure_gap(
                face_a,
                arena_a,
                face_b,
                &position_fn,
                &plane_fn,
                GapSampleDensity::Fine,
            ));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_gap_measurement);
criterion_main!(benches);
