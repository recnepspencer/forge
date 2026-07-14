use super::{denial::BTreeReplayDenied, request::BTreeReplayRequest};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutBTreeRecovery;

pub const fn layout_btree_recovery() -> LayoutBTreeRecovery {
    LayoutBTreeRecovery
}

impl LayoutBTreeRecovery {
    pub fn replay(self, request: BTreeReplayRequest<'_>) -> super::BTreeReplayOutcome {
        super::BTreeReplayOutcome::issue(self.replay_inner(request))
    }

    fn replay_inner(
        self,
        request: BTreeReplayRequest<'_>,
    ) -> Result<crate::BaselineBTreeReplayRecoveryExecution, BTreeReplayDenied> {
        let physical_source = super::source_admission::admit(&request)?;
        let admission = super::layout_admission::admit(&request, &physical_source)?;
        let source = crate::btree_replay_runtime()
            .bind_source(admission, physical_source)
            .map_err(|denial| BTreeReplayDenied::Execution(Box::new(denial)))?;
        crate::btree_replay_runtime()
            .execute(source)
            .map_err(|denial| BTreeReplayDenied::Execution(Box::new(denial)))
    }
}
