#[cfg(test)]
mod tracing_tests {
    use std::thread;
    use forge_core::TracedDecision;
    use crate::core::tracing::span::KernelSpan;

    #[test]
    fn test_single_threaded_span() {
        assert!(!KernelSpan::is_active());
        
        let guard = KernelSpan::enter("test_span");
        assert!(KernelSpan::is_active());

        // We can't directly instantiate `TracedDecision` because it might have private fields,
        // but we can use `KernelSpan::check_tolerance` as a proxy if it logged, or just verify
        // that calling `is_active` works. We'll verify the guard returns an empty output.
        let output = guard.finish();
        
        assert_eq!(output.decision_log.len(), 0);
        assert_eq!(output.warnings.len(), 0);
        assert!(output.config_snapshot.is_none());

        assert!(!KernelSpan::is_active());
    }

    #[test]
    fn test_nested_spans() {
        let guard1 = KernelSpan::enter("outer");
        assert!(KernelSpan::is_active());

        let handle_outer = KernelSpan::current_handle().unwrap();

        {
            let guard2 = KernelSpan::enter("inner");
            assert!(KernelSpan::is_active());
            
            // The inner guard has a different handle internally. Let's finish inner.
            let _output2 = guard2.finish();
        }

        // We are back to outer span.
        assert!(KernelSpan::is_active());
        let _output1 = guard1.finish();

        assert!(!KernelSpan::is_active());
    }

    #[test]
    fn test_cross_thread_attachment() {
        let guard = KernelSpan::enter("main");
        
        // Log something? No, let's just test handles.
        let handle = KernelSpan::current_handle().expect("Handle must exist when span is active");
        
        let t = thread::spawn(move || {
            assert!(!KernelSpan::is_active(), "New thread should have no active span initially");
            let _worker_guard = KernelSpan::attach(handle);
            assert!(KernelSpan::is_active(), "Worker thread should be active after attachment");
            // Would log a decision here, which goes to main thread's collector
        });
        
        t.join().unwrap();
        
        let _output = guard.finish();
    }
}
