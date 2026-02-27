use criterion::{black_box, criterion_group, criterion_main, Criterion};

use forge_kernel::analysis::gap::{measure_gap, GapSampleDensity};
use forge_kernel::mesh_builder::make_cube;

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

    // We reuse this context. It tracks operations implicitly but for
    // raw execution time we accept the minor allocation/write overhead
    // or we can recreate it per-iteration (which we do for accuracy below).

    group.bench_function("density_coarse", |b| {
        b.iter(|| {
            let mut ctx = forge_kernel::core::ModelingContext::new();
            black_box(measure_gap(
                face_a,
                &topo_a,
                &geom_a,
                face_b,
                &topo_b,
                &geom_b,
                GapSampleDensity::Coarse,
                &mut ctx,
            ));
        });
    });

    group.bench_function("density_medium", |b| {
        b.iter(|| {
            let mut ctx = forge_kernel::core::ModelingContext::new();
            black_box(measure_gap(
                face_a,
                &topo_a,
                &geom_a,
                face_b,
                &topo_b,
                &geom_b,
                GapSampleDensity::Medium,
                &mut ctx,
            ));
        });
    });

    group.bench_function("density_fine", |b| {
        b.iter(|| {
            let mut ctx = forge_kernel::core::ModelingContext::new();
            black_box(measure_gap(
                face_a,
                &topo_a,
                &geom_a,
                face_b,
                &topo_b,
                &geom_b,
                GapSampleDensity::Fine,
                &mut ctx,
            ));
        });
    });

    group.finish();
}

criterion_group!(benches, bench_gap_measurement);
criterion_main!(benches);
