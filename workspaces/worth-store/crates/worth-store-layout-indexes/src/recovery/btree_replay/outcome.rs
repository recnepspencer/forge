use super::{BTreeReplayDenialKind, BTreeReplayDenied};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BTreeReplayCaseId {
    Replayed,
    Denied(BTreeReplayDenialKind),
}

impl BTreeReplayCaseId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Replayed => "replayed",
            Self::Denied(denial) => denial.as_str(),
        }
    }
}

pub fn btree_replay_cases() -> impl Iterator<Item = BTreeReplayCaseId> {
    use BTreeReplayDenialKind as Denial;
    [
        BTreeReplayCaseId::Replayed,
        BTreeReplayCaseId::Denied(Denial::SecurityScope),
        BTreeReplayCaseId::Denied(Denial::Budget),
        BTreeReplayCaseId::Denied(Denial::Execution),
    ]
    .into_iter()
}

#[derive(Debug)]
enum BTreeReplayCase {
    Replayed(Box<crate::BaselineBTreeReplayRecoveryExecution>),
    Denied(BTreeReplayDenied),
}

#[derive(Debug)]
pub struct BTreeReplayOutcome {
    case: BTreeReplayCase,
}

#[derive(Debug)]
pub enum BTreeReplayView<'a> {
    Replayed(&'a crate::BaselineBTreeReplayRecoveryExecution),
    Denied(&'a BTreeReplayDenied),
}

impl BTreeReplayOutcome {
    pub(super) fn issue(
        result: Result<crate::BaselineBTreeReplayRecoveryExecution, BTreeReplayDenied>,
    ) -> Self {
        Self {
            case: match result {
                Ok(execution) => BTreeReplayCase::Replayed(Box::new(execution)),
                Err(denial) => BTreeReplayCase::Denied(denial),
            },
        }
    }
    pub const fn view(&self) -> BTreeReplayView<'_> {
        match &self.case {
            BTreeReplayCase::Replayed(execution) => BTreeReplayView::Replayed(execution),
            BTreeReplayCase::Denied(denial) => BTreeReplayView::Denied(denial),
        }
    }
    pub const fn case_id(&self) -> BTreeReplayCaseId {
        match &self.case {
            BTreeReplayCase::Replayed(_) => BTreeReplayCaseId::Replayed,
            BTreeReplayCase::Denied(denial) => BTreeReplayCaseId::Denied(denial.kind()),
        }
    }
    pub fn into_result(
        self,
    ) -> Result<crate::BaselineBTreeReplayRecoveryExecution, BTreeReplayDenied> {
        match self.case {
            BTreeReplayCase::Replayed(execution) => Ok(*execution),
            BTreeReplayCase::Denied(denial) => Err(denial),
        }
    }
}
