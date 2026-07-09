use super::*;

#[test]
#[ignore = "performance baseline capture; run with -- --ignored --nocapture --test-threads=1"]
fn perf_durability_append_matrix() {
    let suite = "durability_append_matrix";

    let fresh_append_samples =
        capture_perf_samples(suite, "append_canonical_envelope_fresh_store", || {
            let mut source = runtime_with_test_schema();
            let envelope = create_entity_outcome(&mut source, "fresh-source")
                .publication
                .envelope
                .as_ref()
                .clone();
            let mut runtime = persisted_runtime_with_test_schema();

            let started_at = Instant::now();
            runtime
                .append_durable_envelope(&envelope)
                .expect("append canonical envelope to fresh store");
            let elapsed_micros = started_at.elapsed().as_micros();

            let store = runtime.durable_store().expect("durable store after append");
            let latest_segment = store
                .segments
                .last()
                .expect("segment manifest after append");
            PerfMeasurement {
                elapsed_micros,
                metrics: perf_metrics!({
                    "segment_count": store.segments.len(),
                    "latest_segment_commit_count": latest_segment.commit_count,
                    "durable_log_len": runtime.durable_log().len(),
                }),
            }
        });
    assert!(fresh_append_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &fresh_append_samples,
        "fresh durable append should create one segment with one canonical envelope",
        |metrics| {
            metrics["segment_count"].as_u64() == Some(1)
                && metrics["latest_segment_commit_count"].as_u64() == Some(1)
                && metrics["durable_log_len"].as_u64() == Some(1)
        },
    );

    let warm_append_samples =
        capture_perf_samples(suite, "append_canonical_envelope_existing_segment", || {
            let mut source = runtime_with_test_schema();
            let envelope_a = create_entity_outcome(&mut source, "warm-source-a")
                .publication
                .envelope
                .as_ref()
                .clone();
            let envelope_b = create_entity_outcome(&mut source, "warm-source-b")
                .publication
                .envelope
                .as_ref()
                .clone();
            let mut runtime = persisted_runtime_with_test_schema();
            runtime
                .append_durable_envelope(&envelope_a)
                .expect("seed durable append");

            let started_at = Instant::now();
            runtime
                .append_durable_envelope(&envelope_b)
                .expect("append canonical envelope to existing segment");
            let elapsed_micros = started_at.elapsed().as_micros();

            let store = runtime.durable_store().expect("durable store after append");
            let latest_segment = store
                .segments
                .last()
                .expect("segment manifest after append");
            PerfMeasurement {
                elapsed_micros,
                metrics: perf_metrics!({
                    "segment_count": store.segments.len(),
                    "latest_segment_commit_count": latest_segment.commit_count,
                    "durable_log_len": runtime.durable_log().len(),
                }),
            }
        });
    assert!(warm_append_samples
        .iter()
        .all(|sample| sample.elapsed_micros > 0));
    assert_budget(
        &warm_append_samples,
        "warm durable append should stay on the same segment up to capacity and extend the log by one",
        |metrics| {
            metrics["segment_count"].as_u64() == Some(1)
                && metrics["latest_segment_commit_count"].as_u64() == Some(2)
                && metrics["durable_log_len"].as_u64() == Some(2)
        },
    );
}
