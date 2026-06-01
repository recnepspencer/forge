#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TestPostCommitFault {
    ConsumerFailureNonAuthoritative,
}

static TEST_POST_COMMIT_FAULT: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

pub(super) fn current_test_post_commit_fault() -> Option<TestPostCommitFault> {
    match TEST_POST_COMMIT_FAULT.load(std::sync::atomic::Ordering::SeqCst) {
        1 => Some(TestPostCommitFault::ConsumerFailureNonAuthoritative),
        _ => None,
    }
}

pub(crate) fn with_test_post_commit_fault<T>(
    fault: TestPostCommitFault,
    run: impl FnOnce() -> T,
) -> T {
    struct ResetGuard<'a> {
        fault: &'a std::sync::atomic::AtomicU8,
        _lock: std::sync::MutexGuard<'a, ()>,
    }

    impl Drop for ResetGuard<'_> {
        fn drop(&mut self) {
            self.fault.store(0, std::sync::atomic::Ordering::SeqCst);
        }
    }

    let guard = crate::testing::fault_injection_lock()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let _reset = ResetGuard {
        fault: &TEST_POST_COMMIT_FAULT,
        _lock: guard,
    };
    TEST_POST_COMMIT_FAULT.store(
        match fault {
            TestPostCommitFault::ConsumerFailureNonAuthoritative => 1,
        },
        std::sync::atomic::Ordering::SeqCst,
    );
    run()
}
