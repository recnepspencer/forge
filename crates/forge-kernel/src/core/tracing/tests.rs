#[cfg(test)]
mod tracing_tests {
    use std::thread;
    use forge_core::{TracedDecision, DecisionKind, DecisionTier, DecisionContext, DecisionId};
    use crate::core::tracing::span::KernelSpan;

    /// Helper: build a TracedDecision with a given ID and kind.
    fn make_decision(id: u64, kind: DecisionKind) -> TracedDecision {
        TracedDecision::new(
            DecisionId(id),
            kind,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Tolerance { measured: 0.5, threshold: 1.0 },
        )
    }

    #[test]
    fn single_threaded_span_collects_decisions() {
        assert!(!KernelSpan::is_active());

        let guard = KernelSpan::enter("test_span");
        assert!(KernelSpan::is_active());

        KernelSpan::record_decision(make_decision(1, DecisionKind::Exact));
        KernelSpan::record_decision(make_decision(2, DecisionKind::Exact));

        let output = guard.finish();

        assert_eq!(output.decision_log.len(), 2, "span should contain both recorded decisions");
        assert_eq!(output.warnings.len(), 0);
        assert!(output.config_snapshot.is_none());
        assert!(!KernelSpan::is_active());
    }

    #[test]
    fn nested_spans_isolate_decisions() {
        let outer_guard = KernelSpan::enter("outer");
        KernelSpan::record_decision(make_decision(10, DecisionKind::Exact));

        {
            let inner_guard = KernelSpan::enter("inner");
            assert!(KernelSpan::is_active());

            KernelSpan::record_decision(make_decision(20, DecisionKind::Exact));
            KernelSpan::record_decision(make_decision(21, DecisionKind::Exact));

            let inner_output = inner_guard.finish();
            assert_eq!(
                inner_output.decision_log.len(), 2,
                "inner span should capture only inner decisions"
            );
        }

        KernelSpan::record_decision(make_decision(11, DecisionKind::Exact));

        assert!(KernelSpan::is_active(), "outer span should still be active");
        let outer_output = outer_guard.finish();
        assert_eq!(
            outer_output.decision_log.len(), 2,
            "outer span should capture only outer decisions (10 and 11)"
        );
        assert!(!KernelSpan::is_active());
    }

    #[test]
    fn cross_thread_attachment_routes_decisions_to_parent() {
        let guard = KernelSpan::enter("main");
        KernelSpan::record_decision(make_decision(100, DecisionKind::Exact));

        let handle = KernelSpan::current_handle()
            .expect("handle must exist when span is active");

        let t = thread::spawn(move || {
            assert!(!KernelSpan::is_active(), "new thread should have no active span");
            let _worker_guard = KernelSpan::attach(handle);
            assert!(KernelSpan::is_active(), "worker should be active after attach");
            KernelSpan::record_decision(make_decision(
                200,
                DecisionKind::Forced { reason: "worker_decision".into() },
            ));
        });

        t.join().unwrap();

        let output = guard.finish();
        assert_eq!(
            output.decision_log.len(), 2,
            "parent span should contain both main-thread (100) and worker-thread (200) decisions"
        );
    }
}
