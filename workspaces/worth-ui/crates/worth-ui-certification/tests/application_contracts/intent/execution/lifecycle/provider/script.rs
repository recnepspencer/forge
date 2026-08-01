use std::collections::VecDeque;

pub(in crate::intent) struct ExecutionScript {
    pub(super) start: ScriptedStart,
    pub(super) polls: VecDeque<AttemptStep>,
    pub(super) cancellations: VecDeque<AttemptStep>,
    pub(super) recovery: VecDeque<RecoveryStep>,
}

#[derive(Clone, Copy)]
pub(in crate::intent) enum AttemptStep {
    PendingBeforeEffect,
    PendingEffectMayHaveBegun,
    Completed,
    RejectedBeforeEffect,
    FailedBeforeEffect,
    CancelledBeforeEffect,
    TimedOutBeforeEffect,
    PartialWithOutcome,
    PartialWithoutOutcome,
    Indeterminate,
}

#[derive(Clone, Copy)]
pub(in crate::intent) enum RecoveryStep {
    Pending,
    Completed,
    PartialWithOutcome,
    PartialWithoutOutcome,
    Indeterminate,
    Failed,
}

#[derive(Clone, Copy)]
pub(super) enum ScriptedStart {
    Started,
    RejectedBeforeEffect,
}

impl ExecutionScript {
    pub(in crate::intent) fn running(polls: impl IntoIterator<Item = AttemptStep>) -> Self {
        Self {
            start: ScriptedStart::Started,
            polls: polls.into_iter().collect(),
            cancellations: VecDeque::new(),
            recovery: VecDeque::new(),
        }
    }

    pub(in crate::intent) fn rejected() -> Self {
        Self {
            start: ScriptedStart::RejectedBeforeEffect,
            polls: VecDeque::new(),
            cancellations: VecDeque::new(),
            recovery: VecDeque::new(),
        }
    }

    pub(in crate::intent) fn with_cancellations(
        mut self,
        cancellations: impl IntoIterator<Item = AttemptStep>,
    ) -> Self {
        self.cancellations = cancellations.into_iter().collect();
        self
    }

    pub(in crate::intent) fn with_recovery(
        mut self,
        recovery: impl IntoIterator<Item = RecoveryStep>,
    ) -> Self {
        self.recovery = recovery.into_iter().collect();
        self
    }
}
