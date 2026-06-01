use super::*;

#[test]
#[ignore = "performance harness audit; run with -- --ignored --nocapture --test-threads=1"]
fn perf_harness_measurement_matrix() {
    let suite = "harness_measurement_matrix";

    let samples = capture_perf_samples(
        suite,
        "post_measurement_metrics_do_not_pollute_elapsed",
        || {
            let started_at = Instant::now();
            measurement_from(started_at, || {
                let metrics_started_at = Instant::now();
                let metric_workload = (0..20_000u64)
                    .map(|index| {
                        perf_metrics!({
                            "id": index,
                            "label": format!("measurement-audit-{index}"),
                            "value": index % 97,
                        })
                    })
                    .collect::<Vec<_>>();
                let measurement_build_micros = metrics_started_at.elapsed().as_micros();
                perf_metrics!({
                    "measurement_build_micros": measurement_build_micros,
                    "measurement_item_count": metric_workload.len(),
                })
            })
        },
    );
    emit_metric_summaries(
        suite,
        "post_measurement_metrics_do_not_pollute_elapsed",
        &samples,
        &[
            ("measurement_build_micros", &["measurement_build_micros"]),
            ("measurement_item_count", &["measurement_item_count"]),
        ],
    );
    assert!(
        samples.iter().all(|sample| {
            metric_u64(&sample.metrics, "measurement_build_micros") as u128
                > sample.elapsed_micros.saturating_mul(5)
        }),
        "measurement workload construction should remain outside reported elapsed time"
    );
}
