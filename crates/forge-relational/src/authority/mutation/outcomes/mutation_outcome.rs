use super::{MutationEvent, RecordMutation};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct MutationOutcome {
    pub(crate) changes: Vec<RecordMutation>,
    pub(crate) events: Vec<MutationEvent>,
}

impl MutationOutcome {
    pub(crate) fn record_change(&mut self, change: RecordMutation) {
        self.changes.push(change);
    }

    pub(crate) fn record_event(&mut self, event: MutationEvent) {
        self.events.push(event);
    }
}
