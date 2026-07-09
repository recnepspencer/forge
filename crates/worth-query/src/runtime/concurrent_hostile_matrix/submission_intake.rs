use std::sync::{Arc, Mutex};

use super::super::WorthQueryWriteCommand;

#[derive(Clone, Debug)]
pub struct WorthQueryConcurrentSubmissionIntake {
    records: Arc<Mutex<Vec<WorthQueryConcurrentSubmissionRecord>>>,
}

impl WorthQueryConcurrentSubmissionIntake {
    pub fn new() -> Self {
        Self {
            records: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn lane(&self, submitter_thread_ordinal: usize) -> WorthQueryConcurrentSubmissionLane {
        WorthQueryConcurrentSubmissionLane {
            submitter_thread_ordinal,
            records: Arc::clone(&self.records),
        }
    }

    pub fn drain_ordered(self) -> Vec<WorthQueryConcurrentSubmissionRecord> {
        let mut records = Arc::try_unwrap(self.records)
            .expect("all concurrent submission lanes should be dropped before drain")
            .into_inner()
            .expect("concurrent submission intake lock should not be poisoned");
        records.sort_by_key(|record| record.submission_ordinal);
        records
    }
}

impl Default for WorthQueryConcurrentSubmissionIntake {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct WorthQueryConcurrentSubmissionLane {
    submitter_thread_ordinal: usize,
    records: Arc<Mutex<Vec<WorthQueryConcurrentSubmissionRecord>>>,
}

impl WorthQueryConcurrentSubmissionLane {
    pub fn submit(&self, submission_ordinal: usize, command: WorthQueryWriteCommand) {
        let mut records = self
            .records
            .lock()
            .expect("concurrent submission intake lock should not be poisoned");
        records.push(WorthQueryConcurrentSubmissionRecord {
            submitter_thread_ordinal: self.submitter_thread_ordinal,
            submission_ordinal,
            command,
        });
    }
}

#[derive(Clone, Debug)]
pub struct WorthQueryConcurrentSubmissionRecord {
    submitter_thread_ordinal: usize,
    submission_ordinal: usize,
    command: WorthQueryWriteCommand,
}

impl WorthQueryConcurrentSubmissionRecord {
    pub fn submitter_thread_ordinal(&self) -> usize {
        self.submitter_thread_ordinal
    }

    pub fn submission_ordinal(&self) -> usize {
        self.submission_ordinal
    }

    pub fn into_command(self) -> WorthQueryWriteCommand {
        self.command
    }
}
