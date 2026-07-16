//! Benchmarks for `worth-math` predicates.
//!
//! Measures Stage 1 throughput and fast-path resolution rate
//! for the filtered evaluation pipeline.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use worth_math::predicates::{orient2d, orient3d};

fn orient2d_random(c: &mut Criterion) {
    let points: Vec<[f64; 2]> = (0..30000)
        .map(|i| {
            let x = (i as f64 * 0.7071).sin() * 100.0;
            let y = (i as f64 * 1.2247).cos() * 100.0;
            [x, y]
        })
        .collect();

    c.bench_function("orient2d_random_10k", |b| {
        b.iter(|| {
            for chunk in points.chunks(3) {
                if chunk.len() == 3 {
                    black_box(
                        orient2d(chunk[0], chunk[1], chunk[2])
                            .expect("generated 2D benchmark points should be valid"),
                    );
                }
            }
        });
    });
}

fn orient3d_random(c: &mut Criterion) {
    let points: Vec<[f64; 3]> = (0..40000)
        .map(|i| {
            let x = (i as f64 * 0.7071).sin() * 100.0;
            let y = (i as f64 * 1.2247).cos() * 100.0;
            let z = (i as f64 * 0.5774).sin() * 100.0;
            [x, y, z]
        })
        .collect();

    c.bench_function("orient3d_random_10k", |b| {
        b.iter(|| {
            for chunk in points.chunks(4) {
                if chunk.len() == 4 {
                    black_box(
                        orient3d(chunk[0], chunk[1], chunk[2], chunk[3])
                            .expect("generated 3D benchmark points should be valid"),
                    );
                }
            }
        });
    });
}

fn orient2d_near_collinear(c: &mut Criterion) {
    let eps = 1e-14;
    let points: Vec<([f64; 2], [f64; 2], [f64; 2])> = (0..10000)
        .map(|i| {
            let t = i as f64 / 10000.0;
            ([0.0, 0.0], [1.0, 0.0], [t, eps * (i as f64).sin()])
        })
        .collect();

    c.bench_function("orient2d_near_collinear_10k", |b| {
        b.iter(|| {
            for (a, b_pt, c) in &points {
                black_box(
                    orient2d(*a, *b_pt, *c)
                        .expect("generated near-collinear benchmark points should be valid"),
                );
            }
        });
    });
}

criterion_group!(
    benches,
    orient2d_random,
    orient3d_random,
    orient2d_near_collinear
);
criterion_main!(benches);
