#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TestPreparationFault {
    PlanningProofInsufficient,
    PublicationIsolationViolation,
    ReductionIdentityConflict,
    WorkerEvaluationFailure,
}

thread_local! {
    static TEST_PREPARATION_FAULT: std::cell::Cell<Option<TestPreparationFault>> =
        const { std::cell::Cell::new(None) };
}

pub(crate) fn current_test_preparation_fault() -> Option<TestPreparationFault> {
    TEST_PREPARATION_FAULT.with(std::cell::Cell::get)
}

pub(crate) fn with_test_preparation_fault<T>(
    fault: TestPreparationFault,
    run: impl FnOnce() -> T,
) -> T {
    struct ResetGuard {
        previous: Option<TestPreparationFault>,
    }

    impl Drop for ResetGuard {
        fn drop(&mut self) {
            TEST_PREPARATION_FAULT.with(|slot| slot.set(self.previous));
        }
    }

    let previous = TEST_PREPARATION_FAULT.with(|slot| {
        let previous = slot.get();
        slot.set(Some(fault));
        previous
    });
    let _reset = ResetGuard { previous };
    run()
}
